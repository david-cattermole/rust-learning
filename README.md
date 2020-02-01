# -*- coding: utf-8 -*-

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

:: Build Rust, generate C headers and compile C++, and install
> build_windows64.bat

:: Go to installed path
> cd %userprofile%\dev\rust-learning

::
> tree.com /F
Folder PATH listing for volume OS
Volume serial number is ####-####
C:.
├───bin
│       rustlearningcpp.dll
│       rustlearningcpp.exe
│       rustlearningrs.dll
│       rustlearningrs.exe
│
├───include
│       cpplib.h
│       rustlib.h
│
└───lib
        rustlearningcpp.lib

:: Run C++ Executable
> bin\rustlearningcpp.exe
Rust Number is 1
C++ Number is 2
Rust Number is 2
C++ Number is 3
Rust Number is 40
C++ Number is 41
Rust Number is 41
C++ Number is 42
Rust Number is 42
C++ Number is 43
Rust Number is 43
C++ Number is 44
Rust Number is 42

:: Run Rust Executable
> bin\rustlearningrs.exe
New Cache
Set Capacity 3
With Capacity 3
New Cache
Set Capacity 3
Capacity
Capacity
============================
Insert 123=My String 001
Insert 124=My String 002
Get Length
Size of CacheA: 2
============================
Remove 123
Remove 125
Get Length
Size of CacheA: 1
============================
Insert 126=My String 004
Insert 125=My String 003
Insert 126=My String 004
Get Length
Get Length
Size of CacheA: 2
Size of CacheB: 2
============================
Insert 123=My String 001
Insert 124=My String 002
Evict One
Insert 125=My String 003
Evict One
Insert 126=My String 004
Evict One
Get Length
Size of CacheA: 3
Insert 123=My String 001
Insert 124=My String 002
Evict One
Insert 125=My String 003
Evict One
Insert 126=My String 004
Evict One
Get Length
Size of CacheB: 3

>

```
