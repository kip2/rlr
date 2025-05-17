@echo off
setlocal

:: Cargo.tomlからversionを抽出
for /f "tokens=2 delims== " %%v in ('findstr /b /c:"version =" Cargo.toml') do (
    set "TAG=%%~v"
)
set "TAG=%TAG:"=%"
set "TAG=v%TAG: =%"

set REPO=kip2/rlr
set FILE=rlr-x86_64-pc-windows-msvc.zip
set INSTALL_DIR=%USERPROFILE%\rlr-bin
set URL=https://github.com/%REPO%/releases/download/%TAG%/%FILE%

echo Downloading %URL%
curl -L -o %FILE% %URL%

echo Creating install directory: %INSTALL_DIR%
mkdir "%INSTALL_DIR%"

echo Extracting %FILE%
powershell -Command "Expand-Archive -Path '%CD%\%FILE%' -DestinationPath '%INSTALL_DIR%' -Force"

echo Deleting archive
del "%FILE%"

for /f "tokens=*" %%i in ('powershell -command "[System.Environment]::GetEnvironmentVariable('Path', 'User')"') do set "CURRENT_PATH=%%i"

echo %CURRENT_PATH% | find /i "%INSTALL_DIR%" >nul
if errorlevel 1 (
    echo Adding %INSTALL_DIR% to PATH
    powershell -Command "[Environment]::SetEnvironmentVariable('Path', '%CURRENT_PATH%;%INSTALL_DIR%', 'User')"
) else (
    echo PATH already contains %INSTALL_DIR%
)

echo Done. You may need to restart your terminal.
endlocal
