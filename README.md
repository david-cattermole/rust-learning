# rust-learning
A personal project for a journey of self-discovery; Learning the Rust Programming Language.

# Goals

- Create a Least-Recently-Used (LRU) cache.
- Create bindings for Python.
- Create bindings for C++ (FFI).
- Compile on both Windows and Linux.

# Build

```cmd
> cd <root directory>

:: Compile and run the Rust main.rs file. 
> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 3.87s
     Running `target\debug\rust-learning.exe`
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

:: Compile the project (including library)
> cargo build

:: For final release
> cargo build --release  

:: Build C++ executible
> build_windows64.bat 
```

Notes:
- C++ executable fails to link to the rust library.