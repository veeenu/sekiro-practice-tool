use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::ptr::{null, null_mut};

use heck::AsSnakeCase;
use lazy_static::lazy_static;
use textwrap::dedent;
use widestring::U16CString;
use windows::core::{PCWSTR, PWSTR};
use windows::imp::{CloseHandle, GetLastError};
use windows::Win32::Foundation::{BOOL, DBG_CONTINUE};
use windows::Win32::Storage::FileSystem::{
    GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, VS_FIXEDFILEINFO,
};
use windows::Win32::System::Diagnostics::Debug::{
    ContinueDebugEvent, ReadProcessMemory, WaitForDebugEventEx, DEBUG_EVENT,
};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32, TH32CS_SNAPMODULE,
};
use windows::Win32::System::Threading::{
    CreateProcessW, OpenProcess, TerminateProcess, DEBUG_PROCESS, DETACHED_PROCESS,
    PROCESS_ALL_ACCESS, PROCESS_INFORMATION, STARTUPINFOW,
};

lazy_static! {
    static ref AOBS: Vec<Aob<'static>> = vec![];
}

pub struct Aob<'a> {
    name: &'a str,
    patterns: Vec<Vec<Option<u8>>>,
    offset: usize,
    deref: bool,
}

impl<'a> Aob<'a> {
    pub fn new(name: &'a str, patterns: &[&str], offset: usize, deref: bool) -> Self {
        Self { name, patterns: patterns.iter().copied().map(into_needle).collect(), offset, deref }
    }

    pub fn find(&self, bytes: &[u8]) -> Option<(&'a str, usize)> {
        if let Some(base) = self.patterns.iter().find_map(|aob| naive_search(&bytes, &aob)) {
            if self.deref {
                let index_range = base + self.offset..base + self.offset + 4;
                let address = u32::from_le_bytes(bytes[index_range].try_into().unwrap());
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
                    module_entry.modBaseAddr as *mut _,
                    buf.as_mut_ptr() as *mut _,
                    module_entry.modBaseSize as usize,
                    Some(&mut bytes_read),
                )
            };
            println!("Read {:x} out of {:x} bytes", bytes_read, module_entry.modBaseSize);
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
    let mut process_info = PROCESS_INFORMATION::default();
    let startup_info =
        STARTUPINFOW { cb: std::mem::size_of::<STARTUPINFOW>() as _, ..Default::default() };

    let mut exe = U16CString::from_str(exe_path.to_str().unwrap()).unwrap().into_vec();
    exe.push(0);

    let process = unsafe {
        CreateProcessW(
            PCWSTR(exe.as_ptr()),
            PWSTR(null_mut()),
            None,
            None,
            BOOL::from(false),
            DEBUG_PROCESS | DETACHED_PROCESS,
            None,
            PCWSTR(null()),
            &startup_info,
            &mut process_info,
        )
    };

    if !process.as_bool() {
        eprintln!("Could not create process: {:x}", unsafe { GetLastError() });
        return None;
    }

    println!("Process handle={:x} pid={}", process_info.hProcess.0, process_info.dwProcessId);

    let mut debug_event = DEBUG_EVENT::default();

    loop {
        unsafe { WaitForDebugEventEx(&mut debug_event, 1000) };
        unsafe {
            ContinueDebugEvent(process_info.dwProcessId, process_info.dwThreadId, DBG_CONTINUE)
        };
        if debug_event.dwDebugEventCode.0 == 2 {
            break;
        }
    }

    let ret = read_base_module_data(
        exe_path.file_name().unwrap().to_str().unwrap(),
        process_info.dwProcessId,
    );

    unsafe { TerminateProcess(process_info.hProcess, 0) };

    ret
}

fn find_aobs(bytes: Vec<u8>) -> Vec<(&'static str, usize)> {
    AOBS.iter().filter_map(|aob| aob.find(&bytes)).collect()
}

fn get_file_version(file: &Path) -> Version {
    let mut file_path = file.to_string_lossy().to_string();
    file_path.push(0 as char);
    let file_path = widestring::U16CString::from_str(file_path).unwrap();
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

    let mut version_info: *mut VS_FIXEDFILEINFO = null_mut();
    unsafe {
        VerQueryValueW(
            version_info_buf.as_ptr() as _,
            PCWSTR(widestring::U16CString::from_str("\\\\\0").unwrap().as_ptr()),
            &mut version_info as *mut *mut _ as _,
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