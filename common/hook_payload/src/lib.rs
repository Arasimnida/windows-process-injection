use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{
            BOOL, 
            HINSTANCE, 
            LPARAM, 
            LRESULT, 
            WPARAM
        },
        UI::WindowsAndMessaging::{
            CallNextHookEx, 
            MessageBoxA, 
            MB_OK
        },
    },
};

#[unsafe(no_mangle)]
pub extern "system" fn CBTProc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        MessageBoxA(
            None,
            PCSTR(b"Hooking succeeded!\0".as_ptr()),
            PCSTR(b"Malware is illegal and for nerds\0".as_ptr()),
            MB_OK,
        );
        CallNextHookEx(None, n_code, w_param, l_param)
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn DllMain(_: HINSTANCE, reason: u32, _: *mut core::ffi::c_void) -> BOOL {
    if reason == 1 /* DLL_PROCESS_ATTACH */ {
        // Optional logging or setup
    }
    BOOL(1)
}
