# scripts/uninstall.ps1
# --- Configuration ---
$AppName = "serial_debugger"
$InstallDir = "$([Environment]::GetFolderPath('UserProfile'))\.local\bin"
$ExePath = Join-Path $InstallDir "$AppName.exe"

# --- Script Body ---
Write-Host "Starting uninstallation of $AppName..."

# 1. Check for Administrator Privileges
if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Host "Error: This script needs to be run with Administrator privileges." -ForegroundColor Red
    Write-Host "Please right-click the uninstall.bat file and choose 'Run as administrator'."
    Read-Host "Press Enter to exit..."
    exit
}


# 2. Remove the executable
if (Test-Path $ExePath) {
    Write-Host "Removing executable: $ExePath"
    Remove-Item -Path $ExePath -Force
} else {
    Write-Host "Executable not found."
}

# 3. Remove the directory from the User's PATH
Write-Host "Cleaning up PATH environment variable..."
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($UserPath -like "*$InstallDir*") {
    # Filter out the installation directory from the path
    $NewPath = ($UserPath.Split(';') | Where-Object { $_ -ne $InstallDir }) -join ';'
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Host "Successfully removed $InstallDir from your PATH." -ForegroundColor Green
} else {
    Write-Host "Directory was not found in your PATH. No changes needed."
}

Write-Host "`nUninstallation complete! Please restart your terminal for changes to take full effect."
# Add a pause so the user can read the output
Read-Host "Press Enter to close."