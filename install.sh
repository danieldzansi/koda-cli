#!/bin/bash
set -e

REPO="danieldzansi/koda-cli"
BINARY="koda"
INSTALL_DIR="/usr/local/bin"

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux)
    case "$ARCH" in
      x86_64) TARGET="x86_64-linux" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  darwin)
    case "$ARCH" in
      x86_64) TARGET="x86_64-macos" ;;
      arm64) TARGET="aarch64-macos" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $OS"
    echo "For Windows download from: https://github.com/$REPO/releases/latest"
    exit 1
    ;;
esac

echo "Downloading koda for $TARGET..."
LATEST_URL="https://github.com/$REPO/releases/latest/download/koda-$TARGET"
curl -fsSL "$LATEST_URL" -o "/tmp/koda"
chmod +x /tmp/koda
sudo mv /tmp/koda "$INSTALL_DIR/$BINARY"

# Save API URL permanently
SHELL_RC="$HOME/.bashrc"
[ -f "$HOME/.zshrc" ] && SHELL_RC="$HOME/.zshrc"
grep -q "API_URL" "$SHELL_RC" || echo 'export API_URL=https://api.danieldzansi.me' >> "$SHELL_RC"
export API_URL=https://api.danieldzansi.me

echo ""
echo "koda installed successfully!"
echo ""
echo "Get started:"
echo "  koda login your@email.com"
echo "  koda deploy ./your-app"
