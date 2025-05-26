use windows::{
    core::PCSTR,
    Win32::{
        System::LibraryLoader::{
            LoadLibraryA, 
            GetProcAddress}, 
        UI::WindowsAndMessaging::{
            SetWindowsHookExA, 
            UnhookWindowsHookEx, 
            GetMessageA,
            WH_CBT, 
            HHOOK, 
            MSG, 
            HOOKPROC,
        },
    },
};
use std::ffi::CString;

const HOOK_DLL_PATH: &str = "C:\\CBT_hook.dll";

pub fn inject() -> windows::core::Result<()> {
    unsafe {
        let path_cstr = CString::new(HOOK_DLL_PATH).expect("CString conversion failed");
        let hinst = LoadLibraryA(PCSTR(path_cstr.as_ptr() as _))?;
        let name_cstr = CString::new("CBTProc").expect("CString conversion failed");
        let raw_addr = GetProcAddress(
            hinst,
            PCSTR(name_cstr.as_ptr() as _)
        ).expect("GetProcAddress failed for CBTProc");

        let hook_proc: HOOKPROC = std::mem::transmute(raw_addr);

        let hook: HHOOK = SetWindowsHookExA(
            WH_CBT,
            hook_proc,
            hinst,
            0,
        )?;

        assert!(hook.0 != 0, "Failed to set Windows hook");
        let mut msg = MSG::default();
        while GetMessageA(&mut msg, None, 0, 0).as_bool() {}

        return UnhookWindowsHookEx(hook)
    }
}
