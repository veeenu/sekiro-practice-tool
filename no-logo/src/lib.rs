use std::ffi::c_void;
use std::mem;
use std::ptr::null_mut;

use libsekiro::prelude::*;
use once_cell::sync::Lazy;
use u16cstr::u16str;
use widestring::U16CString;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress, LoadLibraryW};
use windows::Win32::System::Memory::{
    VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
};
use windows::Win32::System::SystemInformation::GetSystemDirectoryW;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

type FnDirectInput8Create = unsafe extern "stdcall" fn(
    _: HMODULE,
    _: u32,
    _: *const c_void,
    _: *const *const c_void,
    _: *const c_void,
) -> HRESULT;

type FnHResult = unsafe extern "stdcall" fn() -> HRESULT;
type FnGetClassObject =
    unsafe extern "stdcall" fn(*const c_void, *const c_void, *const c_void) -> HRESULT;

static SYMBOLS: Lazy<(FnDirectInput8Create, FnHResult, FnGetClassObject, FnHResult, FnHResult)> =
    Lazy::new(|| unsafe {
        apply_patch();

        type ProcType = unsafe extern "system" fn() -> isize;
        let module = LoadLibraryW(PCWSTR(dinput8_path().as_ptr() as _)).unwrap();

        (
            mem::transmute::<ProcType, FnDirectInput8Create>(
                GetProcAddress(module, PCSTR("DirectInput8Create\0".as_ptr())).unwrap(),
            ),
            mem::transmute::<ProcType, FnHResult>(
                GetProcAddress(module, PCSTR("DllCanUnloadNow\0".as_ptr())).unwrap(),
            ),
            mem::transmute::<ProcType, FnGetClassObject>(
                GetProcAddress(module, PCSTR("DllGetClassObject\0".as_ptr())).unwrap(),
            ),
            mem::transmute::<ProcType, FnHResult>(
                GetProcAddress(module, PCSTR("DllRegisterServer\0".as_ptr())).unwrap(),
            ),
            mem::transmute::<ProcType, FnHResult>(
                GetProcAddress(module, PCSTR("DllUnregisterServer\0".as_ptr())).unwrap(),
            ),
        )
    });

fn dinput8_path() -> U16CString {
    let mut sys_path = vec![0u16; 320];
    let len = unsafe { GetSystemDirectoryW(Some(&mut sys_path)) as usize };

    widestring::U16CString::from_vec_truncate(
        sys_path[..len]
            .iter()
            .chain(u16str!("\\dinput8.dll\0").as_slice().iter())
            .copied()
            .collect::<Vec<_>>(),
    )
}

unsafe fn apply_patch() {
    let module_base = GetModuleHandleW(PCWSTR(null_mut())).unwrap();
    let offset = base_addresses::BaseAddresses::from(*VERSION).no_logo;

    let ptr = (module_base.0 as usize + offset) as *mut [u8; 2];
    let mut old = PAGE_PROTECTION_FLAGS(0);
    if *ptr == [0x74, 0x30] {
        VirtualProtect(ptr as _, 2, PAGE_EXECUTE_READWRITE, &mut old);
        (*ptr) = [0x75, 0x30];
        VirtualProtect(ptr as _, 2, old, &mut old);
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "stdcall" fn DirectInput8Create(
    a: HMODULE,
    b: u32,
    c: *const c_void,
    d: *const *const c_void,
    e: *const c_void,
) -> HRESULT {
    (SYMBOLS.0)(a, b, c, d, e)
}

#[no_mangle]
unsafe extern "C" fn DllMain(
    _hmodule: windows::Win32::Foundation::HMODULE,
    reason: u32,
    _: *mut std::ffi::c_void,
) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        apply_patch();
    }

    BOOL(1)
}
