# rust-learn
understanding rust

## Platform
* Operating System: MacOSX

## Install RustUp

* https://github.com/rust-lang/rustup.rs
* https://rustup.rs/

if rust is alreayd install, and rustup is not found, just reinstall rustup:
```bash
breaw install rust-init
rust-init -y
```

if rust need to be updated, run:
```
rustup update
```

## Install VSCode
https://code.visualstudio.com/download

## Install VSCode Rust Extension
* Rust
	* desc: Rust language integration for VSCode
* Rust(rls)
	* desc: Rust language support 
		* code completion
		* jump to definition, peek definition, find all references, symbol search
		* tpes and documentation on hover
		* code formating
		* refactoring (renamem deglob)
		* error squiggles and apply suggestions from errors
		* snippets
		* build tasks
* CodeLLDB
	* desc: Native debugger based on LLDB

## Configure VS Code

Click Debug -> Add Configuration -> LLDB: Custom Launch
```JSON
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "(Windows) Launch",
            "type": "cppvsdbg",
            "request": "launch",
            "program": "${workspaceRoot}/target/debug/foo.exe",
            "args": [],
            "stopAtEntry": false,
            "cwd": "${workspaceRoot}",
            "environment": [],
            "externalConsole": true
        },
        {
            "name": "(OSX) Launch",
            "type": "lldb",
            "request": "launch",
            "program": "${workspaceRoot}/target/debug/foo",
            "args": [],
            "cwd": "${workspaceRoot}"
        }
    ]
}
```

VSCode Setting, search 'break', enable:
[ ]Allow Breakpoints Everywhere 

## Build

Build empty rust project
```bash
cargo build
```

If you catch
```
dyld: Library not loaded: /usr/local/opt/openssl/lib/libssl.1.0.0.dylib
  Referenced from: /usr/local/bin/cargo
  Reason: image not found
```

You need to switch open ssl version by home brew
```bash
brew switch openssl 1.0.2s
```

Build again
```bash
cargo build
```

You will got
```
error: could not find `Cargo.toml` in `/Users/{username}/Desktop/dev/starlab/rust-learn` or any parent directory
```

Then you SHOULD goto 

https://doc.rust-lang.org/book/ch01-02-hello-world.html



