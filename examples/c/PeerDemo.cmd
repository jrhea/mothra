REM Run in Visual Studio x64 Native Tools Command Prompt

set ROOT_DIR=%~dp0..\..
set OUT_DIR=%ROOT_DIR%\bin
set ENR_PATH=%HOMEDRIVE%%HOMEPATH%\.mothra\network\enr.dat

REM set OUT_DIR=C:\code\mothra\bin

@echo ## Opening first instance
start "Session 1" "%OUT_DIR%\Example.exe"

timeout /t 2

@echo ## Opening second instance
set /p BOOT_NODES=<%ENR_PATH%
start "Session 2" "%OUT_DIR%\Example.exe" --boot-nodes %BOOT_NODES% --listen-address 127.0.0.1 --port 9001 --datadir "%temp%\mothra2"

