@echo off
setlocal

set INSTALL_DIR=%USERPROFILE%\rlr-bin
set BINARY_NAME=rlr.exe

for /f "usebackq delims=" %%p in (`powershell -nologo -noprofile -command "try { & '%INSTALL_DIR%\%BINARY_NAME%' cookie-path } catch { exit 1 }"`) do (
    set "COOKIE_PATH=%%p"
)

if defined COOKIE_PATH (
    if exist "%COOKIE_PATH%" (
        echo Deleting "%COOKIE_PATH%"
        del "%COOKIE_PATH%"
    )
) else (
    echo Failed to retrieve cookie path
)

if exist "%INSTALL_DIR%\%BINARY_NAME%" (
    echo Deleting %INSTALL_DIR%\%BINARY_NAME%
    del "%INSTALL_DIR%\%BINARY_NAME%"
) else (
    echo Binary not found in %INSTALL_DIR%
)

REM PATHから削除
for /f "tokens=*" %%i in ('powershell -command "[System.Environment]::GetEnvironmentVariable('Path', 'User')"') do set "CURRENT_PATH=%%i"

echo %CURRENT_PATH% | find /i "%INSTALL_DIR%" >nul
if not errorlevel 1 (
    echo Removing %INSTALL_DIR% from PATH
    powershell -Command "[Environment]::SetEnvironmentVariable('Path', ($env:Path -split ';' | Where-Object { $_ -ne '%INSTALL_DIR%' }) -join ';', 'User')"
) else (
    echo %INSTALL_DIR% not found in PATH
)

endlocal
