# scripts/uninstall.ps1
$AppName = "serial_debugger"
$InstallDir = "$([Environment]::GetFolderPath('UserProfile'))\.local\bin"
$ExePath = Join-Path $InstallDir "$AppName.exe"

Write-Host "Starting uninstallation of $AppName..."

$currentUser = New-Object Security.Principal.WindowsPrincipal $(New-Object Security.Principal.WindowsIdentity).GetCurrent()
if (-not $currentUser.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Host "Error: Please run this script as an Administrator." -ForegroundColor Red
    Start-Process powershell.exe "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
    exit
}

if (Test-Path $ExePath) {
    Write-Host "Removing executable: $ExePath"
    Remove-Item -Path $ExePath -Force
} else {
    Write-Host "Executable not found."
}

Write-Host "Cleaning up PATH environment variable..."
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($UserPath -like "*$InstallDir*") {
    $NewPath = ($UserPath.Split(';') | Where-Object { $_ -ne $InstallDir }) -join ';'
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Host "Successfully removed $InstallDir from your PATH." -ForegroundColor Green
} else {
    Write-Host "Directory was not found in your PATH. No changes needed."
}

Write-Host "`nUninstallation complete! Please restart your terminal for changes to take full effect."