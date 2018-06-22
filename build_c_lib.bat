REM echo off
REM build c_lib batch file
REM example call: build_c_lib.bat .\clr_c_api\clr_c_api.sln %USERPROFILE%\.rustup stable 64 .\clr_c_api\x64\
set SOLUTION_PATH=%1
set RUSTUP=%2
set CHANNEL=%3
set BITS=%4
set PROJECT_DIR=%5

REM echo off
for /f "tokens=*" %%i in ('call "%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe" -format value -property productPath -latest -legacy') do set DEVENV=%%i
REM echo %DEVENV%

set VSMSBUILDCMD="%DEVENV:IDE\devenv.exe=Tools\VsMSBuildCmd.bat%"
REM echo %VSMSBUILDCMD%
call %VSMSBUILDCMD%
echo on
"%DEVENV%" %SOLUTION_PATH% /Clean "static_debug|x%BITS%" /Project clr_c_api /out Clean_static_debug.log
"%DEVENV%" %SOLUTION_PATH% /Build "static_debug|x%BITS%" /Project clr_c_api /out Build_static_debug.log
"%DEVENV%" %SOLUTION_PATH% /Clean "dylib_debug|x%BITS%" /Project clr_c_api /out Clean_dylib_debug.log
"%DEVENV%" %SOLUTION_PATH% /Build "dylib_debug|x%BITS%" /Project clr_c_api /out Build_dylib_debug.log

type Build_static_debug.log
type Build_dylib_debug.log
XCOPY %PROJECT_DIR%static_debug\clr_c_api.lib %RUSTUP%\toolchains\%CHANNEL%-x86_%BITS%-pc-windows-msvc\lib\rustlib\x86_%BITS%-pc-windows-msvc\lib /Y
XCOPY %PROJECT_DIR%static_debug\clr_c_api.lib . /Y
XCOPY %PROJECT_DIR%dylib_debug\clr_c_api.dll %RUSTUP%\toolchains\%CHANNEL%-x86_%BITS%-pc-windows-msvc\lib\rustlib\x86_%BITS%-pc-windows-msvc\lib /Y
XCOPY %PROJECT_DIR%dylib_debug\clr_c_api.dll . /Y
