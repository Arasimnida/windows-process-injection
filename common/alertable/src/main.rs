use windows::Win32::System::Threading::SleepEx;

fn main() {
    println!("[i] Starting alertable loop for APC testing...");

    loop {
        unsafe {
            SleepEx(1000, true);
        }
    }
}
