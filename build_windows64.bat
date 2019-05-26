@ECHO OFF
SETLOCAL
:: Builds project under MS Windows

:: The root of this project.
SET THIS_DIR=%~dp0
SET ROOT=%THIS_DIR%
ECHO Package Root: %ROOT%
CHDIR %ROOT% 

:: Build plugin
MKDIR build
CHDIR build
DEL /S /Q *
FOR /D %%G in ("*") DO RMDIR /S /Q "%%~nxG"

cmake -G "NMake Makefiles" ^
    -DCMAKE_BUILD_TYPE=Release ^
    ..

nmake /F Makefile clean
nmake /F Makefile all

:: Return back project root directory.
CHDIR "%ROOT%"
