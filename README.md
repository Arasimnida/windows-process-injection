# Variety of Injection Windowsx64 Primitives (VIWP)
---
**VIWP is a non-exhaustive** collection of process injection techniques implemented in Rust for Windows x64.  
*I've created VIWP to learn how each injection method work and get hands-on understanding of Windows internals.*

## Table of Contents
1. [Features](#features)
2. [Lab Environment](#lab-environment)
3. [Usage](#usage)
4. [Architecture](#architecture)
5. [References](#references)


## Features

  - `CreateRemoteThread` (DLL and shellcode)
  - `QueueUserAPC` (DLL)
  - `NtCreateThreadEx` (shellcode)
  - Inline-hooking of`CreateFileA`
  - CBT hook `SetWindowsHookExA`
  - Thread hijacking (Suspend/Inject/Resume) for ARM64 processors

## Lab Environment

- Build: Rust 1.70+ (`cargo build --release`)
- Target: Windows x64
- Testing:
  - Rust cross-compile on Linux: `$ rustup target add x86_64-pc-windows-gnu && cargo build --release --target x86_64-pc-windows-gnu`)
  - Windows 10 VM with Defender disabled
  - CLI flags to select method, payload type, target PID: `\> injector.exe <COMMAND> [ARGS]`

If needed DLLs and test executables live in `common/`.

## Usage
```
USAGE:
    injector.exe <COMMAND> [ARGS]

COMMANDS:
    dll-createremotethread <PID> <DLL_PATH>
    dll-queueuserapc <PID> <DLL_PATH>
    shellcode-createremotethread <PID>
    shellcode-ntcreatethreadex <PID>
    shellcode-sirthread <PID>
    setwindowshookex <HOOK_DLL_PATH>
    inline-hook <PID> <HOOK_DLL_PATH>

OPTIONS:
    -h, --help       Print this help message
```

## Architecture

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

## References

- [10 process injection techniques technical survey common and trending process](https://www.elastic.co/blog/ten-process-injection-techniques-technical-survey-common-and-trending-process#:~:text=As%20shown%20in%20Figure%201%2C,APIs%20so%20that%20a%20remote)

## Next steps

- Clean up the archi
- Upgrade all features with manual parsing
- Bypass defender and AV/EDR
- Process injection methods:
  - `NtCreateThreadEx` && `LoadLibraryA` (DLL)
  - `QueueUserAPC` (shellcode)
  - IAT hooking
  - Thread hijacking (Suspend/Inject/Resume) for other processors
  - Using Registery keys
  - A lot of other cool stuff to explore: Process Hollowing, Process Doppelgänging (TxF Ghosting), AtomBombing (APC via GlobalAtom Tables), Sham (Shim Databases), Extra Window Memory Injection (EWMI), Injection via file mapping (Remote File Mapping), Threadless injection (Fibers)...
