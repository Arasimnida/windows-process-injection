use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{
            BOOL, 
            CloseHandle
        },
        System::{
            LibraryLoader::{
                GetModuleHandleA, 
                GetProcAddress
            },
            Memory::{
                VirtualAllocEx, 
                MEM_COMMIT, 
                MEM_RESERVE, 
                PAGE_READWRITE
            },
            Threading::{
                CreateRemoteThread, 
                OpenProcess, 
                PROCESS_ALL_ACCESS, 
                WaitForSingleObject
            },
            Diagnostics::Debug::WriteProcessMemory,
        },
    },
};
use std::ffi::CString;

pub fn inject(pid: u32, dll_path: &str) -> windows::core::Result<()> {
    unsafe {
        let h_process = OpenProcess(PROCESS_ALL_ACCESS, BOOL(0), pid)?;
        assert!(h_process.0 != 0, "OpenProcess failed");

        let dll_cstring = CString::new(dll_path).unwrap();
        let dll_bytes = dll_cstring.as_bytes_with_nul();

        let alloc_address = VirtualAllocEx(
            h_process,
            None,
            dll_bytes.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );
        assert!(!alloc_address.is_null(), "VirtualAllocEx failed");

        let result = WriteProcessMemory(
            h_process,
            alloc_address,
            dll_bytes.as_ptr() as _,
            dll_bytes.len(),
            None,
        );
        result.expect("WriteProcessMemory failed");
        
        let kernel32_name = PCSTR(b"kernel32.dll\0".as_ptr());
        let loadlibrary_name = PCSTR(b"LoadLibraryA\0".as_ptr());
        let kernel32 = GetModuleHandleA(kernel32_name)?;

        let loadlibrary_addr = GetProcAddress(kernel32, loadlibrary_name);
        loadlibrary_addr.expect("Failed to get LoadLibraryA address");

        let func_ptr = loadlibrary_addr.unwrap() as *const std::ffi::c_void;
        
        let thread = CreateRemoteThread(
            h_process,
            None,
            0,
            Some(std::mem::transmute(func_ptr)),
            Some(alloc_address),
            0,
            None,
        );
        assert!(thread.is_ok(), "CreateRemoteThread failed");

        WaitForSingleObject(thread.unwrap(), 5000);
        return CloseHandle(h_process)
    }
}
