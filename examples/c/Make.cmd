REM Run in Visual Studio x64 Native Tools Command Prompt

set ROOT_DIR=%~dp0..\..
set BIND_DIR=%ROOT_DIR%\bindings
set CBIND_DIR=%BIND_DIR%\c
set OUT_DIR=%ROOT_DIR%\bin

set CFLAGS=/W4 /O2 /EHsc /MD
set IFLAGS=-I%CBIND_DIR%
set LFLAGS=/LIBPATH:"%OUT_DIR%" /LIBPATH:"%OUT_DIR%\release" /LIBPATH:"%OUT_DIR%\release\deps"
set OBJ=%OUT_DIR%\example.obj
set TARGET=%OUT_DIR%\example.exe

@echo ## Cleaning "%TARGET%"
if not exist "%OUT_DIR%" ( mkdir "%OUT_DIR%" )
if exist "%TARGET%" ( del "%TARGET%" )

@echo ## Compiling "%OBJ%"
cl /c %CFLAGS% "example.c" %IFLAGS% /Fo"%OBJ%"

@echo ## Linking "%TARGET%"
link mothra.dll.lib "%OBJ%" %LFLAGS% /OUT:"%TARGET%"
