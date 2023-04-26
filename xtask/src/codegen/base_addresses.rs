use std::collections::HashSet;
use std::env;
use std::ffi::c_void;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::ptr::null_mut;

use heck::AsSnakeCase;
use lazy_static::lazy_static;
use textwrap::dedent;
use widestring::{U16CStr, U16CString};
use windows::core::PCWSTR;
use windows::imp::CloseHandle;
use windows::Win32::Storage::FileSystem::{
    GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, VS_FIXEDFILEINFO,
};
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32First, Module32Next, Process32FirstW, Process32NextW,
    MODULEENTRY32, PROCESSENTRY32W, TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};

lazy_static! {
    static ref AOBS: Vec<Aob<'static>> = vec![
        Aob::new("Quitout", &["48 8B 05 ?? ?? ?? ?? 48 63 C9 89 54 88 20 C3"], 3, 7, true),
        Aob::new("RenderWorld", &["80 3D ?? ?? ?? ?? 00 0F 10 00 0F 11 45 D0"], 2, 7, true),
        Aob::new(
            "DebugRender",
            &["44 0F B6 3D ?? ?? ?? ?? 0F 29 74 24 20 0F 28 F1 E8"],
            4,
            8,
            true
        ),
        Aob::new(
            "Igt",
            &[
                r#"48 8B 0D ?? ?? ?? ?? 0F 28 C6 F3 0F 59 05 ?? ?? ?? ?? F3 48 0F 2C C0 01 81 ?? ?? ?? ??"#
            ],
            3,
            7,
            true
        ),
        Aob::new(
            "PlayerPosition",
            &["48 83 3D ?? ?? ?? ?? 00 0F 84 ?? ?? ?? ?? F3 41 0F 10 47 78 F3 0F 5C C7"],
            3,
            8,
            true
        ),
        Aob::new("DebugFlags", &["80 3D ?? ?? ?? ?? 00 75 08 32 C0 48 83 C4 20"], 2, 7, true),
        Aob::new(
            "ShowCursor",
            &[r#"40 38 3D ?? ?? ?? ?? 0F B6 DB 0F 44 DF 84 DB 0F 94 C3 83 7D 40 FF"#],
            3,
            7,
            true
        ),
        Aob::new("NoLogo", &["74 30 48 8D 54 24 30 48 8B CD E8"], 0, 0, false)

    ];
}

pub struct Aob<'a> {
    name: &'a str,
    patterns: Vec<Vec<Option<u8>>>,
    offset: usize,
    deref_offset: u32,
    deref: bool,
}

impl<'a> Aob<'a> {
    pub fn new(
        name: &'a str,
        patterns: &[&str],
        offset: usize,
        deref_offset: u32,
        deref: bool,
    ) -> Self {
        Self {
            name,
            patterns: patterns.iter().copied().map(into_needle).collect(),
            offset,
            deref_offset,
            deref,
        }
    }

    pub fn find(&self, bytes: &[u8]) -> Option<(&'a str, usize)> {
        if let Some(base) = self.patterns.iter().find_map(|aob| naive_search(bytes, aob)) {
            if self.deref {
                let index_range = base + self.offset..base + self.offset + 4;
                let address = u32::from_le_bytes(bytes[index_range].try_into().unwrap());
                let address = address + base as u32 + self.deref_offset;
                Some((self.name, address as usize))
            } else {
                Some((self.name, base + self.offset))
            }
        } else {
            eprintln!("{:24} not found", self.name);
            None
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Version(u32, u32, u32);

impl Version {
    fn to_fromsoft_string(self) -> String {
        format!("{}.{:02}.{}", self.0, self.1, self.2)
    }
}

struct VersionData {
    version: Version,
    aobs: Vec<(&'static str, usize)>,
}

fn szcmp(source: &[u8], s: &str) -> bool {
    source.iter().zip(s.chars()).all(|(a, b)| *a == b as u8)
}

fn into_needle(pattern: &str) -> Vec<Option<u8>> {
    pattern
        .split(' ')
        .map(|byte| match byte {
            "?" | "??" => None,
            x => u8::from_str_radix(x, 16).ok(),
        })
        .collect::<Vec<_>>()
}

fn naive_search(bytes: &[u8], pattern: &[Option<u8>]) -> Option<usize> {
    bytes.windows(pattern.len()).position(|wnd| {
        wnd.iter().zip(pattern.iter()).all(|(byte, pattern)| match pattern {
            Some(x) => byte == x,
            None => true,
        })
    })
}

fn read_base_module_data(proc_name: &str, pid: u32) -> Option<(usize, Vec<u8>)> {
    let module_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid).unwrap() };
    let mut module_entry =
        MODULEENTRY32 { dwSize: std::mem::size_of::<MODULEENTRY32>() as _, ..Default::default() };

    unsafe { Module32First(module_snapshot, &mut module_entry) };

    loop {
        if szcmp(&module_entry.szModule, proc_name) {
            let process = unsafe { OpenProcess(PROCESS_ALL_ACCESS, true, pid).unwrap() };
            let mut buf = vec![0u8; module_entry.modBaseSize as usize];
            let mut bytes_read = 0usize;
            unsafe {
                ReadProcessMemory(
                    process,
                    module_entry.modBaseAddr as *const c_void,
                    buf.as_mut_ptr() as *mut c_void,
                    module_entry.modBaseSize as usize,
                    Some(&mut bytes_read),
                )
            };
            println!(
                "Read {:x} out of {:x} bytes {:x}-{:x}",
                bytes_read,
                module_entry.modBaseSize,
                module_entry.modBaseAddr as usize,
                module_entry.modBaseSize as usize + module_entry.modBaseAddr as usize,
            );
            unsafe { CloseHandle(process.0) };
            return Some((module_entry.modBaseAddr as usize, buf));
        }
        if !unsafe { Module32Next(module_snapshot, &mut module_entry).as_bool() } {
            break;
        }
    }
    None
}

fn get_base_module_bytes(exe_path: &Path) -> Option<(usize, Vec<u8>)> {
    // This was a great strategy but unfortunately it's over now

    // let mut process_info = PROCESS_INFORMATION::default();
    // let startup_info =
    //     STARTUPINFOW { cb: std::mem::size_of::<STARTUPINFOW>() as _,
    // ..Default::default() };
    //
    // let mut exe =
    // U16CString::from_str(exe_path.to_str().unwrap()).unwrap().into_vec();
    // exe.push(0);
    //
    // let mut exe_dir =
    //     U16CString::from_str(exe_path.parent().unwrap().to_str().unwrap()).
    // unwrap().into_vec(); exe_dir.push(0);
    //
    // let process = unsafe {
    //     CreateProcessW(
    //         PCWSTR(exe.as_ptr()),
    //         PWSTR(null_mut()),
    //         None,
    //         None,
    //         BOOL::from(false),
    //         DEBUG_PROCESS | DEBUG_ONLY_THIS_PROCESS | DETACHED_PROCESS,
    //         None,
    //         PCWSTR(exe_dir.as_ptr()),
    //         &startup_info,
    //         &mut process_info,
    //     )
    // };
    //
    // if !process.as_bool() {
    //     eprintln!("Could not create process: {:x}", unsafe { GetLastError() });
    //     return None;
    // }
    //
    // println!("Process handle={:x} pid={}", process_info.hProcess.0,
    // process_info.dwProcessId);
    //
    // let mut debug_event = DEBUG_EVENT::default();
    //
    // loop {
    //     if !unsafe { WaitForDebugEventEx(&mut debug_event, INFINITE).as_bool() }
    // {         break;
    //     }
    //     unsafe {
    //         ContinueDebugEvent(process_info.dwProcessId, process_info.dwThreadId,
    // DBG_CONTINUE)     };
    //     match debug_event.dwDebugEventCode.0 {
    //         1 => println!("{:#?}", unsafe { debug_event.u.Exception }),
    //         2 => println!("{:#?}", unsafe { debug_event.u.CreateThread }),
    //         3 => println!("{:#?}", unsafe { debug_event.u.CreateProcessInfo }),
    //         4 => println!("{:#?}", unsafe { debug_event.u.ExitThread }),
    //         5 => println!("{:#?}", unsafe { debug_event.u.ExitProcess }),
    //         6 => println!("{:#?}", unsafe { debug_event.u.LoadDll }),
    //         7 => println!("{:#?}", unsafe { debug_event.u.UnloadDll }),
    //         8 => println!("{:#?}", unsafe { debug_event.u.DebugString }),
    //         9 => println!("{:#?}", unsafe { debug_event.u.RipInfo }),
    //         _ => unreachable!(),
    //     }
    //     if debug_event.dwDebugEventCode.0 == 2 {
    //         break;
    //     }
    // }

    // Enter asking the user to double click the hecking game
    println!("Start \n\n{}\n\n and press enter please...", exe_path.display());
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();

    let process = unsafe { get_process_by_name64("sekiro.exe").unwrap() };

    let ret = read_base_module_data(
        exe_path.file_name().unwrap().to_str().unwrap(),
        process, // process_info.dwProcessId,
    );

    // unsafe { TerminateProcess(process_info.hProcess, 0) };

    ret
}

unsafe fn get_process_by_name64(name: &str) -> Result<u32, ()> {
    let name = U16CString::from_str_truncate(name);

    let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap();
    let mut pe32 = PROCESSENTRY32W {
        dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };

    if !Process32FirstW(snapshot, &mut pe32).as_bool() {
        CloseHandle(snapshot.0);
        return Err(());
    }

    let pid = loop {
        let proc_name =
            U16CStr::from_ptr_truncate(pe32.szExeFile.as_ptr(), pe32.szExeFile.len()).unwrap();

        if proc_name == name.as_ucstr() {
            break Ok(pe32.th32ProcessID);
        }

        if !Process32NextW(snapshot, &mut pe32).as_bool() {
            CloseHandle(snapshot.0);
            break Err(());
        }
    }
    .unwrap();

    CloseHandle(snapshot.0);

    Ok(pid)
}

fn find_aobs(bytes: Vec<u8>) -> Vec<(&'static str, usize)> {
    AOBS.iter().filter_map(|aob| aob.find(&bytes)).collect()
}

fn get_file_version(file: &Path) -> Version {
    let file_path = file.canonicalize().unwrap().to_string_lossy().to_string();
    let file_path = U16CString::from_str(file_path).unwrap();

    let mut version_info_size =
        unsafe { GetFileVersionInfoSizeW(PCWSTR(file_path.as_ptr()), None) };

    let mut version_info_buf = vec![0u8; version_info_size as usize];
    unsafe {
        GetFileVersionInfoW(
            PCWSTR(file_path.as_ptr()),
            0,
            version_info_size,
            version_info_buf.as_mut_ptr() as _,
        )
    };
    println!("{:?}", version_info_size);

    let mut version_info: *mut VS_FIXEDFILEINFO = null_mut();
    unsafe {
        VerQueryValueW(
            version_info_buf.as_ptr() as _,
            PCWSTR(U16CString::from_str("\\\\\0").unwrap().as_ptr()),
            &mut version_info as *mut *mut _ as *mut *mut c_void,
            &mut version_info_size,
        )
    };
    let version_info = unsafe { version_info.as_ref().unwrap() };
    let major = (version_info.dwFileVersionMS >> 16) & 0xffff;
    let minor = (version_info.dwFileVersionMS) & 0xffff;
    let patch = (version_info.dwFileVersionLS >> 16) & 0xffff;

    Version(major, minor, patch)
}

// Codegen routine
//

/// Generate the `BaseAddresses` struct.
fn codegen_base_addresses_struct() -> String {
    let mut generated = String::new();

    generated.push_str("// **********************************\n");
    generated.push_str("// *** AUTOGENERATED, DO NOT EDIT ***\n");
    generated.push_str("// **********************************\n");

    generated.push_str("#[derive(Debug)]\n");
    generated.push_str("pub struct BaseAddresses {\n");
    generated.push_str(
        &AOBS
            .iter()
            .map(|aob| format!("    pub {}: usize,\n", AsSnakeCase(aob.name)))
            .collect::<Vec<_>>()
            .join(""),
    );
    generated.push_str("}\n\n");
    generated.push_str("impl BaseAddresses {\n");
    generated.push_str("    pub fn with_module_base_addr(self, base: usize) -> BaseAddresses {\n");
    generated.push_str("        BaseAddresses {\n");
    generated.push_str(
        &AOBS
            .iter()
            .map(|aob| {
                format!(
                    "            {}: self.{} + base,\n",
                    AsSnakeCase(aob.name),
                    AsSnakeCase(aob.name)
                )
            })
            .collect::<Vec<_>>()
            .join(""),
    );
    generated.push_str("        }\n    }\n}\n\n");
    generated
}

/// Generate `BaseAddresses` instances.
fn codegen_base_addresses_instances(ver: &Version, aobs: &[(&str, usize)]) -> String {
    use std::fmt::Write;
    let mut string = aobs.iter().fold(
        format!(
            "pub const BASE_ADDRESSES_{}_{:02}_{}: BaseAddresses = BaseAddresses {{\n",
            ver.0, ver.1, ver.2
        ),
        |mut o, (name, offset)| {
            writeln!(o, "    {}: 0x{:x},", AsSnakeCase(name), offset).unwrap();
            o
        },
    );
    string.push_str("};\n\n");
    string
}

/// Generate the `Version` enum and `From<Version> for BaseAddresses`.
fn codegen_version_enum(ver: &[VersionData]) -> String {
    use std::fmt::Write;
    let mut string = String::new();

    // pub enum Version

    string.push_str("#[derive(Clone, Copy)]\n");
    string.push_str("pub enum Version {\n");

    for v in ver {
        writeln!(string, "    V{}_{:02}_{},", v.version.0, v.version.1, v.version.2).unwrap();
    }

    string.push_str("}\n\n");

    // impl From<(u32, u32, u32)> for Version

    string.push_str("impl From<(u32, u32, u32)> for Version {\n");
    string.push_str("    fn from(v: (u32, u32, u32)) -> Self {\n");
    string.push_str("        match v {\n");

    for v in ver {
        let Version(maj, min, patch) = v.version;
        writeln!(
            string,
            "            ({maj}, {min}, {patch}) => Version::V{maj}_{min:02}_{patch},"
        )
        .unwrap();
    }

    string.push_str("            (maj, min, patch) => {\n");
    string.push_str(
        "                tracing::error!(\"Unrecognized version {maj}.{min:02}.{patch}\");\n",
    );
    string.push_str("                panic!()\n");
    string.push_str("            }\n");
    string.push_str("        }\n");
    string.push_str("    }\n");
    string.push_str("}\n\n");

    // impl Version

    string.push_str("impl Version {\n");
    string.push_str("    pub fn tuple(&self) -> (u8, u8, u8) {\n");
    string.push_str("        match self {\n");

    for v in ver {
        let Version(maj, min, patch) = v.version;
        writeln!(
            string,
            "            Version::V{maj}_{min:02}_{patch} => ({maj}, {min}, {patch}),"
        )
        .unwrap();
    }
    string.push_str("        }\n");
    string.push_str("    }\n");
    string.push_str("}\n\n");

    // impl From<Version> for BaseAddresses

    string.push_str("impl From<Version> for BaseAddresses {\n");
    string.push_str("    fn from(v: Version) -> Self {\n");
    string.push_str("        match v {\n");

    for v in ver {
        let Version(maj, min, patch) = v.version;
        let stem = format!("{maj}_{min:02}_{patch}");
        writeln!(string, "            Version::V{stem} => BASE_ADDRESSES_{stem},").unwrap();
    }

    string.push_str("        }\n");
    string.push_str("    }\n");
    string.push_str("}\n\n");

    string
}

fn patches_paths() -> impl Iterator<Item = PathBuf> {
    let base_path = PathBuf::from(
        env::var("SEKPT_PATCHES_PATH").unwrap_or_else(|_| panic!("{}", dedent(r#"
            SEKPT_PATCHES_PATH environment variable undefined.
            Check the documentation: https://github.com/veeenu/sekiro-practice-tool/README.md#building
        "#))),
    );
    println!("{base_path:?}");
    base_path
        .read_dir()
        .expect("Couldn't scan patches directory")
        .map(Result::unwrap)
        .map(|dir| dir.path().join("sekiro.exe"))
}

fn codegen_base_addresses_path() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
        .join("libsekiro")
        .join("src")
        .join("codegen")
        .join("base_addresses.rs")
}

pub(crate) fn codegen_base_addresses() {
    let mut processed_versions: HashSet<Version> = HashSet::new();

    let version_data = patches_paths()
        .filter(|p| p.exists())
        .filter_map(|exe| {
            println!("{exe:?}");
            let version = get_file_version(&exe);
            if processed_versions.contains(&version) {
                None
            } else {
                let exe = exe.canonicalize().unwrap();
                println!("\nVERSION {}: {:?}", version.to_fromsoft_string(), exe);

                let (_base_addr, bytes) = get_base_module_bytes(&exe).unwrap();

                let aobs = find_aobs(bytes);
                processed_versions.insert(version);
                Some(VersionData { version, aobs })
            }
        })
        .collect::<Vec<_>>();

    let mut codegen = codegen_base_addresses_struct();
    codegen.push_str(&codegen_version_enum(&version_data));

    let codegen = version_data.iter().fold(codegen, |mut o, i| {
        o.push_str(&codegen_base_addresses_instances(&i.version, &i.aobs));
        o
    });

    File::create(codegen_base_addresses_path()).unwrap().write_all(codegen.as_bytes()).unwrap();
}
