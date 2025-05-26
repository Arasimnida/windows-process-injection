# Windows Code Injection Techniques in Rust

> **⚠️ WARNING**  
>    This is a project for demonstration, research and learning purposes only. 
>    Don't use it for anything illegal, please respect the law.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85+-orange.svg)](https://www.rust-lang.org/)


A **non-exhaustive** collection of Windows code-injection and API-hooking techniques, implemented in Rust.
Each technique lives in its own module, so you can plug-and-play the ones you need, or use the provided CLI tool for testing.

## Table of Contents
1. [Features](#features)
2. [Requirements & Build](#requirements--build)
3. [Usage](#usage)
4. [TO DO](#to-do)
5. [References](#references)

## 📂 Repository Layout

```
.
├── Cargo.lock
├── Cargo.toml
├── common/
│   ├── alertable/                                  # Example alertable thread for apc injection
│   ├── hook_payload/                               # MessageBox DLL payload for CBTProc hooking
│   ├── inline_hook                                 # Inline-hooking DLL for CreateFileA
│   ├── inlinehook_test/                            # Binary test for inline-hook injection
│   └── payload_messagebox/                         # Simple MessageBox payload DLL
├── injector/
│   ├── Cargo.toml
│   └── src
│       ├── main.rs                                 # CLI entry point
│       ├── dll_injection
│       │   ├── loadlibrary_createremotethread.rs   # DLL Injection using LoadLibrary and CreateRemoteThread 
│       │   └── queueuserapc.rs                     # QueueUserAPC based DLL injection
│       ├── hook_user
│       │   ├── mod.rs
│       │   └── setwindowshookex.rs                 # SetWindowsHookEx CBT hook
│       └── shellcode_injection
│           ├── createremotethread.rs               # Shellcode injection using CreateRemoteThread
│           ├── mod.rs
│           └── ntcreatethreadex.rs                 # Shellcode injection using NtCreateThreadEx
│           └── sirthread.rs                 # Shellcode injection using Thread hijacking (SIR)
└── README.md
```

## Features

- **DLL Injection**
  - `CreateRemoteThread` && `LoadLibraryA`
  - `QueueUserAPC` && `LoadLibraryA`
- **Shellcode Injection**
  - `CreateRemoteThread` with raw shellcode
  - `NtCreateThreadEx` with raw shellcode
- **In-Process Hooking**
  - Inline-hooking of `CreateFileA`
  - `SetWindowsHookExA` (CBT hook)
- **Hijacking de thread (Suspend/Inject/Resume)**
  - For ARM64 processors

## TO DO

- **DLL Injection**
  - `NtCreateThreadEx` && `LoadLibraryA`
- **Shellcode Injection**
  - `QueueUserAPC` with raw shellcode
- **In-Process Hooking**
  - IAT hooking
- **Hijacking de thread (Suspend/Inject/Resume)**
  - Architecture: X86, AMD64, ARM
- **Registery keys**
- **Reflective DLL & manual mapping**
- **Process Hollowing**
- **Process Doppelgänging (TxF Ghosting)**
- **AtomBombing (APC via GlobalAtom Tables)**
- **Sham (Shim Databases)**
- **Extra Window Memory Injection (EWMI)**
- **Injection via file mapping (Remote File Mapping)**
- **Threadless injection (Fibers)**

## Requirements & Build

- **Rust**, installed via rustup
- **Windows SDK / Build Tools** (if building on Windows for the MSVC target)
    ```Powershell
    # Build injector and common on Windows
    cd windows-process-injection
    cargo build --release
    ```
- **Cross-compilation on Linux (optional)**:
    ```bash
    # Add the Windows GNU target
    rustup target add x86_64-pc-windows-gnu

    # Ensure mingw-w64 is installed (Debian/Ubuntu)
    sudo apt install mingw-w64

    # Build injector and common for Windows
    cd windows-process-injection
    cargo build --release --target x86_64-pc-windows-gnu
    ```

All the DLLs and test executables you need to verify each injection technique live in the `common/` directory.

## Usage

To test the injection techniques have first to launch a process using `Start-Process` and get its PID using `Get-Process` in powershell terminal. After getting the PID you can launch the injector in another powershell terminal to inject code into this process.

```
USAGE:
    injector.exe <COMMAND> [ARGS]

COMMANDS:
    dll-createremotethread <PID> <DLL_PATH>
        Inject a DLL via CreateRemoteThread + LoadLibraryA

    dll-queueuserapc <PID> <DLL_PATH>
        Inject a DLL via QueueUserAPC + LoadLibraryA

    shellcode-createremotethread <PID>
        Inject inline shellcode via CreateRemoteThread

    shellcode-ntcreatethreadex <PID>
        Inject inline shellcode via NtCreateThreadEx
    
    shellcode-sirthread <PID>
        Hijack one thread of the target process using the suspend inject resume (SIR) technique

    setwindowshookex <HOOK_DLL_PATH>
        Set a systemwide CBT hook via SetWindowsHookEx

    inline-hook <PID> <HOOK_DLL_PATH>
        Install inline hook (e.g. CreateFileA trampoline)

OPTIONS:
    -h, --help       Print this help message
```

## Example

Execute the following in Powershell:

```
PS> cd common\inlinehook_test\target\release\
PS> .\inlinehook_test.exe
inlinehook_test running with PID: 4321
Enter filename (no extension):
MyFile
Full path: C:\Users\You\Desktop\MyFile.txt
[+] File created/opened: HANDLE(0x1234)
```

In another terminal run:

```
PS> injector.exe inline-hook 4321 ..\..\common\inline_hook\target\release\inline_hook.dll
[+] inline-hook succeeded.
```

You can now try to create a file again using `.\inlinehook_test.exe` and the payload should executes.

## References

- [10 process injection techniques technical survey common and trending process](https://www.elastic.co/blog/ten-process-injection-techniques-technical-survey-common-and-trending-process#:~:text=As%20shown%20in%20Figure%201%2C,APIs%20so%20that%20a%20remote)