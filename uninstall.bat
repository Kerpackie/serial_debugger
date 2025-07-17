@echo off
setlocal enabledelayedexpansion

:: Define app name and paths
set "APP_NAME=serial_debugger"
set "TARGET_DIR=%LOCALAPPDATA%\Programs\%APP_NAME%\bin"
set "EXE_NAME=serial_debugger.exe"
set "TARGET_EXE=%TARGET_DIR%\%EXE_NAME%"

echo Uninstalling %APP_NAME%...

:: Delete the EXE
if exist "%TARGET_EXE%" (
    del "%TARGET_EXE%"
    echo Removed %EXE_NAME%
) else (
    echo EXE not found at %TARGET_EXE%
)

:: Remove bin dir if empty
dir /b "%TARGET_DIR%" 2>nul | findstr . >nul
if %ERRORLEVEL% NEQ 0 (
    rmdir "%TARGET_DIR%"
    echo Removed empty bin folder.
)

:: Remove path from user PATH
echo Removing %TARGET_DIR% from user PATH...
powershell -NoProfile -Command ^
  "$path = [Environment]::GetEnvironmentVariable('Path', 'User');" ^
  "$new = ($path -split ';' | Where-Object { $_ -ne '%TARGET_DIR%' }) -join ';';" ^
  "[Environment]::SetEnvironmentVariable('Path', $new, 'User');"

:: Confirm removal
echo Verifying uninstall...
where serial_debugger >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo serial_debugger is still found in PATH. Try restarting terminal or check for other copies.
) else (
    echo serial_debugger is no longer found in PATH.
)

echo.
echo Uninstallation complete.
pause
