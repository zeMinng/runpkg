$ErrorActionPreference = "Stop"

$Repo = "zeMinng/runpkg"
$InstallDir = "$env:USERPROFILE\.runpkg\bin"
$Package = "runpkg-windows-x64.zip"

if ($env:RUNPKG_VERSION) {
    $Version = $env:RUNPKG_VERSION
    $url = "https://github.com/$Repo/releases/download/$Version/$Package"
} else {
    $Version = "latest"
    $url = "https://github.com/$Repo/releases/latest/download/$Package"
}

Write-Host "Installing runpkg $Version..."

Write-Host "Downloading:"
Write-Host $url

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$tmp = "$($env:TEMP)\runpkg.zip"

Invoke-WebRequest $url -OutFile $tmp

Expand-Archive $tmp $InstallDir -Force

Remove-Item $tmp -Force

$oldPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($oldPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$oldPath;$InstallDir",
        "User"
    )
}

Write-Host ""
Write-Host "Installed successfully!"
Write-Host "Restart terminal and run:"
Write-Host "runpkg"
