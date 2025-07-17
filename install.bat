@echo off
setlocal enabledelayedexpansion

:: Define app name and paths
set "APP_NAME=serial_debugger"
set "TARGET_DIR=%LOCALAPPDATA%\Programs\%APP_NAME%\bin"
set "EXE_NAME=serial_debugger.exe"
set "TARGET_EXE=%TARGET_DIR%\%EXE_NAME%"

:: Ensure bin directory exists
if not exist "%TARGET_DIR%" (
    mkdir "%TARGET_DIR%"
)

:: Copy EXE
echo Copying %EXE_NAME% to %TARGET_DIR%...
copy /Y "release\%EXE_NAME%" "%TARGET_EXE%" >nul
if %ERRORLEVEL% NEQ 0 (
    echo Failed to copy executable.
    pause
    exit /b 1
)

:: Add to PATH if not present (user-level)
echo Checking if %TARGET_DIR% is in user PATH...
powershell -NoProfile -Command ^
  "$path = [Environment]::GetEnvironmentVariable('Path', 'User');" ^
  "if (-not ($path -split ';' | Where-Object { $_ -eq '%TARGET_DIR%' })) {" ^
    "$new = ($path + ';%TARGET_DIR%').Trim(';');" ^
    "[Environment]::SetEnvironmentVariable('Path', $new, 'User');" ^
    "Write-Output 'PATH updated.';" ^
  "} else { Write-Output 'PATH already contains directory.' }"

:: Confirm it's runnable
echo Verifying installation...
where serial_debugger >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo serial_debugger is now available from the command line.
) else (
    echo echo You may need to restart your terminal (or log out and back in) to use 'serial_debugger' from anywhere.

)

echo.
echo Installation complete.
pause
