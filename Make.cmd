REM Run in Visual Studio x64 Native Tools Command Prompt

set ROOT_DIR=%~dp0
set OUT_DIR=%ROOT_DIR%\bin
set BIND_DIR=%ROOT_DIR%\bindings
set CBIND_DIR=%BIND_DIR%\c
set CORE_DIR=%ROOT_DIR%\core
set EXAMPLES_DIR=%ROOT_DIR%\examples
set CEXAMPLE_DIR=%EXAMPLES_DIR%\c
set DOTNETEXAMPLE_DIR=%EXAMPLES_DIR%\dotnet

@echo ## Cleaning "%OUT_DIR%"
if exist "%OUT_DIR%" ( del "%OUT_DIR%" /y )

if not exist "%OUT_DIR%" ( mkdir "%OUT_DIR%" )

@echo.
@echo ## Building C bindings
cd "%CBIND_DIR%"
call "%CBIND_DIR%\Make.cmd"

@echo.
@echo ## Building Rust bindings
cd "%CORE_DIR%"
call "%CORE_DIR%\Make.cmd"

@echo.
@echo ## Building C example
cd "%CEXAMPLE_DIR%"
call "%CEXAMPLE_DIR%\Make.cmd"

@echo.
@echo ## Building dotnet example
cd "%DOTNETEXAMPLE_DIR%"
call "%DOTNETEXAMPLE_DIR%\Make.cmd"
