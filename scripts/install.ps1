# scripts/install.ps1

# --- Configuration ---
$AppName = "serial_debugger"
$InstallDir = "$([Environment]::GetFolderPath('UserProfile'))\.local\bin"
$SourcePath = "..\release\$AppName.exe"

# --- Script Body ---
Write-Host "Starting installation for $AppName..."

# 1. Check for Administrator Privileges (Robust Method)
if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Host "Error: This script needs to be run with Administrator privileges." -ForegroundColor Red
    Write-Host "Please right-click the install.bat file and choose 'Run as administrator'."
    Read-Host "Press Enter to exit..."
    exit
}


# 2. Create the installation directory if it doesn't exist
if (-not (Test-Path $InstallDir)) {
    Write-Host "Creating directory: $InstallDir"
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

# 3. Copy the executable
Write-Host "Copying executable to $InstallDir"
Copy-Item -Path $SourcePath -Destination $InstallDir -Force

# 4. Add the directory to the User's PATH if it's not already there
Write-Host "Updating PATH environment variable..."
$CurrentUserPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($CurrentUserPath -notlike "*$InstallDir*") {
    $NewPath = "$CurrentUserPath;$InstallDir"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Host "Successfully added $InstallDir to your PATH." -ForegroundColor Green
} else {
    Write-Host "Directory is already in your PATH." -ForegroundColor Green
}

Write-Host "`nInstallation complete! Please restart your terminal to use the '$AppName' command."
# Add a pause so the user can read the output
Read-Host "Press Enter to close."