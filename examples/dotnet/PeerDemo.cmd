REM Run in Visual Studio x64 Native Tools Command Prompt
REM Needs dotnet core installed

set ROOT_DIR=%~dp0..\..
set OUT_DIR=%ROOT_DIR%\bin

REM set OUT_DIR=C:\code\mothra\bin

@echo ## Opening first instance
start "Session 1" dotnet "%OUT_DIR%\dotnet\Example.dll"

timeout /t 5

@echo ## Opening second instance
set ENR_PATH=%HOMEDRIVE%%HOMEPATH%\.mothra\network\enr.dat
set /p BOOT_NODES=<%ENR_PATH%
start "Session 2" dotnet "%OUT_DIR%\dotnet\Example.dll" -- --boot-nodes %BOOT_NODES% --listen-address 127.0.0.1 --port 9001 --datadir "%temp%\mothra2"
