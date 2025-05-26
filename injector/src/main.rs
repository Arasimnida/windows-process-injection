mod dll_injection;
mod shellcode_injection;
mod hook_user;

fn print_usage() {
    eprintln!("> Usage:");
    eprintln!(":\\>   injector.exe dll-createremotethread        <pid> <dll_path>");
    eprintln!(":\\>   injector.exe dll-queueuserapc              <pid> <dll_path>");
    eprintln!(":\\>   injector.exe shellcode-createremotethread  <pid>");
    eprintln!(":\\>   injector.exe shellcode-sirthread           <pid>");
    eprintln!(":\\>   injector.exe shellcode-ntcreatethreadex    <pid>");
    eprintln!(":\\>   injector.exe setwindowshookex              <hook_dll_path>");
    eprintln!(":\\>   injector.exe inline-hook                   <pid> <hook_dll_path>");
}

fn main() {
    let mut args = std::env::args().skip(1);
    let cmd = match args.next() {
        Some(c) => c.to_lowercase(),
        None => { print_usage(); std::process::exit(1); }
    };

    // On dispatch selon la sous-commande
    let result = match cmd.as_str() {
        "dll-createremotethread" => {
            let pid = args.next().and_then(|s| s.parse().ok())
                .unwrap_or_else(|| { eprintln!("[!] Missing or invalid PID"); print_usage(); std::process::exit(1) });
            let dll = args.next().unwrap_or_else(|| { eprintln!("[!] Missing <dll_path>"); print_usage(); std::process::exit(1) });
            dll_injection::loadlibrary_createremotethread::inject(pid, &dll)
        }
        "dll-queueuserapc" => {
            let pid = args.next().and_then(|s| s.parse().ok())
                .unwrap_or_else(|| { eprintln!("[!] Missing or invalid PID"); print_usage(); std::process::exit(1) });
            let dll = args.next().unwrap_or_else(|| { eprintln!("[!] Missing <dll_path>"); print_usage(); std::process::exit(1) });
            dll_injection::queueuserapc::inject(pid, &dll)
        }
        "shellcode-createremotethread" => {
            let pid = args.next().and_then(|s| s.parse().ok())
                .unwrap_or_else(|| { eprintln!("[!] Missing or invalid PID"); print_usage(); std::process::exit(1) });
            shellcode_injection::createremotethread::inject(pid)
        }
        "shellcode-sirthread" => {
            let pid = args.next().and_then(|s| s.parse().ok())
                .unwrap_or_else(|| { eprintln!("[!] Missing or invalid PID"); print_usage(); std::process::exit(1) });
            shellcode_injection::createremotethread::inject(pid)
        }
        "shellcode-ntcreatethreadex" => {
            let pid = args.next().and_then(|s| s.parse().ok())
                .unwrap_or_else(|| { eprintln!("[!] Missing or invalid PID"); print_usage(); std::process::exit(1) });
            shellcode_injection::ntcreatethreadex::inject(pid)
        }
        "setwindowshookex" => {
            hook_user::setwindowshookex::inject()
        }
        "inline-hook" => {
            let pid = args.next().and_then(|s| s.parse().ok())
                .unwrap_or_else(|| { eprintln!("[!] Missing or invalid PID"); print_usage(); std::process::exit(1) });
            let dll = args.next().unwrap_or_else(|| { eprintln!("[!] Missing <hook_dll_path>"); print_usage(); std::process::exit(1) });
            dll_injection::loadlibrary_createremotethread::inject(pid, &dll)
        }
        _ => {
            eprintln!("[!] Unknown command: {}", cmd);
            print_usage();
            std::process::exit(1);
        }
    };

    match result {
        Ok(_) => println!("[+] {} succeeded.", cmd),
        Err(e) => eprintln!("[!] {} failed: {e:?}", cmd),
    }
}
