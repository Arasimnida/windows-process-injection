use windows::{
    Win32::{
        Foundation::{
            BOOL, 
            CloseHandle
        },
        System::{
            LibraryLoader::{
                GetModuleHandleA, 
                GetProcAddress},
            Memory::{
                VirtualAllocEx, 
                MEM_COMMIT, 
                MEM_RESERVE, 
                PAGE_READWRITE
            },
            Threading::{
                OpenProcess, 
                QueueUserAPC, 
                OpenThread,
                PROCESS_ALL_ACCESS, 
                THREAD_SET_CONTEXT, 
                THREAD_QUERY_INFORMATION, 
                THREAD_SUSPEND_RESUME
            },
            Diagnostics::{
                Debug::WriteProcessMemory, 
                ToolHelp::{
                    CreateToolhelp32Snapshot, 
                    TH32CS_SNAPTHREAD, 
                    THREADENTRY32, 
                    Thread32First, 
                    Thread32Next
                }
            },
        },
    },
    core::PCSTR,
};
use std::{ffi::CString};

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
        
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0)?;
        let mut entry = THREADENTRY32 {
            dwSize: std::mem::size_of::<THREADENTRY32>() as u32,
            ..Default::default()
        };

        if Thread32First(snapshot, &mut entry).is_ok() {
            loop {
                if entry.th32OwnerProcessID == pid {
                    let h_thread = OpenThread(
                        THREAD_SET_CONTEXT | THREAD_QUERY_INFORMATION | THREAD_SUSPEND_RESUME,
                        false,
                        entry.th32ThreadID,
                    );
                
                    if let Ok(h_thread) = h_thread {
                        let result = QueueUserAPC(
                            Some(std::mem::transmute(func_ptr)),
                            h_thread,
                            alloc_address as usize,
                        );
                    
                        if result == 0 {
                            println!("QueueUserAPC failed on thread {}", entry.th32ThreadID);
                        } else {
                            println!("APC queued on thread {}", entry.th32ThreadID);
                        }
                    
                        let _ = CloseHandle(h_thread);
                    }
                }
            
                if !Thread32Next(snapshot, &mut entry).is_ok() {
                    break;
                }
            }
        }
        return CloseHandle(snapshot);
    }
}
