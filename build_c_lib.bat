echo off
REM build c_lib batch file
REM example call: build_c_lib.bat stable 64 .\clr_c_api\x64\ %OUT_DIR% Debug

set CHANNEL=%1
set BITS=%2
set PROJECT_DIR=%3
set OUT_DIR=%4
set MODE=%5

set SOLUTION_PATH=.\clr_c_api\clr_c_api.sln
set RUSTUP=%USERPROFILE%\.rustup
set TRIPLET=pc-windows-msvc
set SUB_MODE=x%BITS%

for /f "tokens=*" %%i in ('call "%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe" -format value -property productPath -latest -legacy') do set DEVENV=%%i

set VSMSBUILDCMD="%DEVENV:IDE\devenv.exe=Tools\VsMSBuildCmd.bat%"
call %VSMSBUILDCMD%
REM echo on

"%DEVENV%" %SOLUTION_PATH% /Clean "%MODE%|%SUB_MODE%" /out clean_debug.log
"%DEVENV%" %SOLUTION_PATH% /Build "%MODE%|%SUB_MODE%" /out build_debug.log

XCOPY %PROJECT_DIR%\%SUB_MODE%\%MODE%\clr_c_api.lib %OUT_DIR% /Y
XCOPY %PROJECT_DIR%\%SUB_MODE%\%MODE%\clr_c_api.dll %OUT_DIR% /Y