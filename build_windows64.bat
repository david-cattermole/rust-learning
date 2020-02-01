@ECHO OFF
SETLOCAL
:: Builds project under MS Windows

:: Install directory
SET INSTALL_DIR="%USERPROFILE%\dev\rust-learning"

:: The root of this project.
SET THIS_DIR=%~dp0
SET ROOT=%THIS_DIR%
ECHO Package Root: %ROOT%
CHDIR %ROOT%

:: Build Rust
CHDIR rust
:: cargo install --force cbindgen
cbindgen --config cbindgen.toml --crate rustlearningrs --output include/lib.h
cargo build --release
CHDIR "%ROOT%"

:: Build C++
MKDIR build
CHDIR build
DEL /S /Q *
FOR /D %%G in ("*") DO RMDIR /S /Q "%%~nxG"

cmake -G "NMake Makefiles" ^
    -DCMAKE_BUILD_TYPE=Release ^
    -DCMAKE_INSTALL_PREFIX=%INSTALL_DIR% ^
    ..

nmake /F Makefile clean
nmake /F Makefile all
nmake /F Makefile install

:: Return back project root directory.
CHDIR "%ROOT%"
