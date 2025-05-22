#![allow(non_snake_case)]

use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{HANDLE, HWND},
        System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
        System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
        UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
    },
};
use std::{mem, ptr, sync::Mutex};

type FnCreateFileA = unsafe extern "system" fn(
    PCSTR, 
    u32, 
    u32, 
    *mut core::ffi::c_void, 
    u32, 
    u32, 
    HANDLE
) -> HANDLE;

#[allow(dead_code)]
struct InlineHook {
    // Forwarder stub address in kernel32.dll              
    target_addr: usize,
    // CreateFileA address in kernelbase.dll
    real_addr: usize,
}

const STUB_LEN: usize = 6;


static HOOK_STATE: Mutex<Option<InlineHook>> = Mutex::new(None);

pub fn inject() -> windows::core::Result<()> {
    let hmod = unsafe { GetModuleHandleA(PCSTR(b"kernel32.dll\0".as_ptr())).expect("[!] GetModuleHandleA Failed") };
    let addr = unsafe { GetProcAddress(hmod, PCSTR(b"CreateFileA\0".as_ptr())).expect("[!] GetProcAddress Failed") as *mut u8 };

    let disp = unsafe {
        std::ptr::read_unaligned(addr.add(2) as *const i32)
    } as isize;

    let ptr_addr = unsafe { addr.add(6).offset(disp) } as *const usize;

    let real_addr = unsafe {
        std::ptr::read_unaligned(ptr_addr)
    };

    let hook_fn = hook_create_file_a as *const () as usize;
    let rel = (hook_fn as isize - (addr as isize + 5)) as i32;
    let mut jmp = [0xE9u8; 5];
    jmp[1..].copy_from_slice(&rel.to_le_bytes());

    let mut old_prot = PAGE_PROTECTION_FLAGS(0);
    unsafe {
        VirtualProtect(addr as *mut _, STUB_LEN, PAGE_EXECUTE_READWRITE, &mut old_prot)?;
        ptr::copy_nonoverlapping(jmp.as_ptr(), addr as *mut u8, 5);
        ptr::write((addr.wrapping_add(5)) as *mut u8, 0x90);
    }

    let mut guard = HOOK_STATE.lock().unwrap();
    *guard = Some(InlineHook {
        target_addr: addr as usize,
        real_addr,
    });
    drop(guard);

    Ok(())
    
}

unsafe extern "system" fn hook_create_file_a(
    lpFileName: PCSTR, 
    dwDesiredAccess: u32, 
    dwShareMode: u32,
    lpSecurityAttributes: *mut core::ffi::c_void,
    dwCreationDisposition: u32, 
    dwFlagsAndAttributes: u32, 
    hTemplateFile: HANDLE,
) -> HANDLE {
    unsafe {
        // payload
        MessageBoxA(
            HWND(0),
            PCSTR(b"Injection succeeded!\0".as_ptr()),
            PCSTR(b"Hooked using CreateFileA, Malware is still illegal and for nerds.\0".as_ptr()),
            MB_OK,
        );

        let guard = HOOK_STATE.lock().unwrap();
        let hook = guard.as_ref().unwrap();
        let original_fn: FnCreateFileA = mem::transmute(hook.real_addr as *const ());
        let result = original_fn(
            lpFileName,
            dwDesiredAccess,
            dwShareMode,
            lpSecurityAttributes,
            dwCreationDisposition,
            dwFlagsAndAttributes,
            hTemplateFile,
        );
        result
    }
}
