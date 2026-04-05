#!/bin/sh
# Install agent-runbooks
# Usage: curl -fsSL https://raw.githubusercontent.com/btakita/agent-runbooks/main/install.sh | sh

set -e

REPO="btakita/agent-runbooks"
BIN="agent-runbooks"

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux) OS="unknown-linux-gnu" ;;
  darwin) OS="apple-darwin" ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
  x86_64|amd64) ARCH="x86_64" ;;
  aarch64|arm64) ARCH="aarch64" ;;
  *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

TARGET="${ARCH}-${OS}"

# Get latest release tag
LATEST=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed 's/.*"tag_name": "\(.*\)".*/\1/')

if [ -z "$LATEST" ]; then
  echo "Failed to fetch latest release"
  exit 1
fi

URL="https://github.com/$REPO/releases/download/$LATEST/$BIN-$TARGET.tar.gz"

echo "Installing $BIN $LATEST ($TARGET)..."

# Download and extract to ~/.local/bin (or /usr/local/bin with sudo)
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
mkdir -p "$INSTALL_DIR"

curl -fsSL "$URL" | tar xz -C "$INSTALL_DIR"
chmod +x "$INSTALL_DIR/$BIN"

echo "Installed $BIN to $INSTALL_DIR/$BIN"

# Check if in PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  echo ""
  echo "Add to your PATH: export PATH=\"$INSTALL_DIR:\$PATH\""
fi
