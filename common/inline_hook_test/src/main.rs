use std::{ffi::CString, io::{self, Write}};
use windows::core::PCSTR;
use windows::Win32::{
    Foundation::{HANDLE, INVALID_HANDLE_VALUE},
    Storage::FileSystem::{
        CreateFileA, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
        FILE_SHARE_NONE, OPEN_ALWAYS,
    },
};

fn main() -> windows::core::Result<()> {
    let pid = std::process::id();
    println!("inline_hook_test running with PID: {}", pid);
    loop {
        print!(">>> Enter filename (no extension): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let name = input.trim();
        if name.is_empty() {
            eprintln!("[!] No filename given, exiting.");
            return Ok(());
        }

        let full = format!("C:\\Users\\Marie\\Desktop\\{name}.txt");
        println!(">>> Full path: {}", full);

        let cpath = CString::new(full.clone()).unwrap();
        let handle = unsafe {
            CreateFileA(
                PCSTR(cpath.as_ptr() as _),
                FILE_GENERIC_READ.0 | FILE_GENERIC_WRITE.0,
                FILE_SHARE_NONE,
                None,
                OPEN_ALWAYS,
                FILE_ATTRIBUTE_NORMAL,
                HANDLE(0),
            )?
        };

        if handle == INVALID_HANDLE_VALUE {
            eprintln!("[!] Failed to create or open file: {}", full);
        } else {
            println!(">>> File created/opened: {:?}", handle);
        }
    }
}
