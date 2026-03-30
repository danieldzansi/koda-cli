# koda

Deploy your applications from the CLI — like Railway, but yours.

## Installation

### Linux & macOS
```bash
curl -fsSL https://raw.githubusercontent.com/danieldzansi/koda-cli/master/install.sh | bash
```

### Windows (PowerShell — run as Administrator)
```powershell
irm https://raw.githubusercontent.com/danieldzansi/koda-cli/master/install.ps1 | iex
```

### Windows (Manual)
1. Download `koda-x86_64-windows.exe` from [Releases](https://github.com/danieldzansi/koda-cli/releases/latest)
2. Rename it to `koda.exe`
3. Move it to a folder in your PATH (e.g. `C:\Windows\System32`)

### Via Cargo (all platforms)
```bash
cargo install koda
```

## Getting Started
```bash
# Login with your email
koda login your@email.com

# Deploy your app
koda deploy ./my-app

# View running apps
koda ps

# View logs
koda logs <container-id>

# Stop an app
koda stop <container-id>
```

## How it works

1. CLI compresses your source code
2. Uploads to the Railway RS API
3. Nixpacks detects your language and builds a Docker image
4. Image is pushed to a registry
5. Worker server pulls and runs your container
6. Your app is live at a subdomain of danieldzansi.me
