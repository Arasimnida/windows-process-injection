use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{
            MessageBoxA, 
            MB_OK
            }
    }
};

/// Called by the loader; `reason == 1` ➜ DLL_PROCESS_ATTACH
#[unsafe(no_mangle)]
pub extern "system" fn DllMain(
    _module: *mut core::ffi::c_void,
    reason: u32,
    _reserved: *mut core::ffi::c_void,
) -> i32 {
    if reason == 1 {
        unsafe {
            let text    = b"Injection succeeded!\0";
            let caption = b"Malware is illegal and for nerds\0";

            MessageBoxA(
                HWND(0),
                PCSTR(text.as_ptr()),
                PCSTR(caption.as_ptr()),
                MB_OK,
            );
        }
    }
    1 // TRUE
}
