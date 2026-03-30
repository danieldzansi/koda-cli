$ErrorActionPreference = "Stop"

$repo = "danieldzansi/koda-cli"
$binary = "koda-x86_64-windows.exe"
$installDir = "$env:USERPROFILE\.koda"
$installPath = "$installDir\koda.exe"

Write-Host "Installing koda..."

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
    Write-Host "Added koda to PATH"
}

# Save API URL
$configDir = "$env:USERPROFILE\.koda"
$configFile = "$configDir\config.json"
if (-not (Test-Path $configFile)) {
    '{"api_url": "https://api.danieldzansi.me"}' | Out-File -FilePath $configFile -Encoding utf8
}

Write-Host ""
Write-Host "koda installed successfully!"
Write-Host "Restart your terminal then run:"
Write-Host "  koda login your@email.com"
Write-Host "  koda deploy ./my-app"
