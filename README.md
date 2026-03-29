# railway-rs

Deploy your applications from the CLI — like Railway, but yours.

## Installation

### Linux & macOS
```bash
curl -fsSL https://raw.githubusercontent.com/danieldzansi/Railway-rs-Cli/master/install.sh | bash
```

### Windows (PowerShell — run as Administrator)
```powershell
irm https://raw.githubusercontent.com/danieldzansi/Railway-rs-Cli/master/install.ps1 | iex
```

### Windows (Manual)
1. Download `railway-rs-x86_64-windows.exe` from [Releases](https://github.com/danieldzansi/Railway-rs-Cli/releases/latest)
2. Rename it to `railway-rs.exe`
3. Move it to a folder in your PATH (e.g. `C:\Windows\System32`)

### Via Cargo (all platforms)
```bash
cargo install railway-rs
```

## Getting Started
```bash
# Login with your email
railway-rs login your@email.com

# Deploy your app
railway-rs deploy ./my-app

# View running apps
railway-rs ps

# View logs
railway-rs logs <container-id>

# Stop an app
railway-rs stop <container-id>
```

## How it works

1. CLI compresses your source code
2. Uploads to the Railway RS API
3. Nixpacks detects your language and builds a Docker image
4. Image is pushed to a registry
5. Worker server pulls and runs your container
6. Your app is live at a subdomain of danieldzansi.me
