$ErrorActionPreference = "Stop"

$Repo = "zeMinng/runpkg"
$InstallDir = "$env:USERPROFILE\.runpkg\bin"
$Package = "runpkg-windows-x64.zip"

if ($env:RUNPKG_VERSION) {
    $Version = $env:RUNPKG_VERSION
    $url = "https://github.com/$Repo/releases/download/$Version/$Package"
    $checksumsUrl = "https://github.com/$Repo/releases/download/$Version/checksums.txt"
} else {
    $Version = "latest"
    $url = "https://github.com/$Repo/releases/latest/download/$Package"
    $checksumsUrl = "https://github.com/$Repo/releases/latest/download/checksums.txt"
}

Write-Host "Installing runpkg $Version..."

Write-Host "Downloading:"
Write-Host $url

New-Item -ItemType Directory -Force -Path "$InstallDir" | Out-Null

$tmp = Join-Path $env:TEMP "runpkg-$([guid]::NewGuid()).zip"

try {
    curl.exe -fL --retry 3 --retry-delay 2 $url -o "$tmp"
    if ($LASTEXITCODE -ne 0) { throw "Download failed: $url" }

    curl.exe -fL --retry 3 --retry-delay 2 $checksumsUrl -o "$tmp.checksums"
    if ($LASTEXITCODE -ne 0) { throw "Download failed: $checksumsUrl" }

    $pattern = [regex]::Escape($Package) + '$'
    $match = Select-String -Path "$tmp.checksums" -Pattern $pattern | Select-Object -First 1
    if (-not $match) { throw "Checksum entry not found for $Package" }

    $expected = $match.Line.Split()[0].ToLower()
    $actual   = (Get-FileHash -Algorithm SHA256 -Path "$tmp").Hash.ToLower()
    if ($expected -ne $actual) { throw "Checksum verification failed for $Package" }

    Expand-Archive -Path "$tmp" -DestinationPath "$InstallDir" -Force

    if (-not (Test-Path "$InstallDir\runpkg.exe")) {
        throw "runpkg.exe not found after extraction"
    }
} finally {
    Remove-Item -Force -ErrorAction SilentlyContinue "$tmp", "$tmp.checksums"
}

$oldPath = [Environment]::GetEnvironmentVariable("Path", "User")
if (-not $oldPath) { $oldPath = "" }

if (($oldPath.TrimEnd(';') -split ';') -notcontains $InstallDir) {
    $newPath = ($oldPath.TrimEnd(';') + ";" + $InstallDir).TrimStart(';')
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
}

Write-Host ""
Write-Host "Installed successfully!"
Write-Host "Restart terminal and run:"
Write-Host "runpkg"
