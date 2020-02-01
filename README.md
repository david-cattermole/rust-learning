# rust-learning

A personal project for a journey of self-discovery; Learning the Rust
Programming Language.

# Goals

- Create Rust library to be called by C++ (FFI).
- Create Rust executable to dynamically link and call a C++ library (FFI).
- Compile on both Windows and Linux (only Windows currently tested).
- Create a Least-Recently-Used (LRU) cach in Rust.

# Build on Windows

Requirements:

- [CMake 2.8+](https://cmake.org/)
- [Rust](https://www.rust-lang.org/)
  - [cbindgen](https://crates.io/crates/cbindgen)
- [Visual Studio 2012 (MSVC 11.0)](https://visualstudio.microsoft.com/downloads/) or [Visual Studio 2015 (MSVC 14.0)](https://visualstudio.microsoft.com/downloads/)


Compile:
```cmd
> cd <root directory>

:: Build Rust, generate C headers and compile C++.
> build_windows64.bat 
```
