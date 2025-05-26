mod inlinehook;

use windows::Win32::{
    Foundation::{
        BOOL, 
        HINSTANCE},
    System::{
        SystemServices::DLL_PROCESS_ATTACH,
        LibraryLoader::DisableThreadLibraryCalls,
    },
};
use std::ffi::c_void;

#[unsafe(no_mangle)]
pub extern "system" fn DllMain(
    hinst: HINSTANCE,
    reason: u32,
    _reserved: *mut c_void,
) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        unsafe { let _ = DisableThreadLibraryCalls(hinst); };
        std::thread::spawn(|| {
            inlinehook::inject().expect("[!] Inline hook failed");
        });
    }
    BOOL(1)
}
