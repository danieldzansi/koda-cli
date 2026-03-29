$ErrorActionPreference = "Stop"

$repo = "danieldzansi/Railway-rs-Cli"
$binary = "railway-rs-x86_64-windows.exe"
$installDir = "$env:USERPROFILE\.railway-rs"
$installPath = "$installDir\railway-rs.exe"

Write-Host "Installing railway-rs..."

# Create install directory
New-Item -ItemType Directory -Force -Path $installDir | Out-Null

# Download latest binary
$url = "https://github.com/$repo/releases/latest/download/$binary"
Write-Host "Downloading from $url..."
Invoke-WebRequest -Uri $url -OutFile $installPath

# Add to PATH
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$installDir", "User")
    Write-Host "Added railway-rs to PATH"
}

# Save API URL
$configDir = "$env:USERPROFILE\.railway-rs"
$configFile = "$configDir\config.json"
if (-not (Test-Path $configFile)) {
    '{"api_url": "https://api.danieldzansi.me"}' | Out-File -FilePath $configFile -Encoding utf8
}

Write-Host ""
Write-Host "railway-rs installed successfully!"
Write-Host "Restart your terminal then run:"
Write-Host "  railway-rs login your@email.com"
Write-Host "  railway-rs deploy ./my-app"
