# scripts/install.ps1
$AppName = "serial_debugger"
$InstallDir = "$([Environment]::GetFolderPath('UserProfile'))\.local\bin"
$SourcePath = "..\release\$AppName.exe"

Write-Host "Starting installation for $AppName..."

$currentUser = New-Object Security.Principal.WindowsPrincipal $(New-Object Security.Principal.WindowsIdentity).GetCurrent()
if (-not $currentUser.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Host "Error: Please run this script as an Administrator." -ForegroundColor Red
    Start-Process powershell.exe "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
    exit
}

if (-not (Test-Path $InstallDir)) {
    Write-Host "Creating directory: $InstallDir"
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

Write-Host "Copying executable to $InstallDir"
Copy-Item -Path $SourcePath -Destination $InstallDir -Force

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