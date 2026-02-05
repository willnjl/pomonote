#!/bin/zsh
set -euo pipefail

BIN_DIR="$HOME/bin"
BIN_PATH="$BIN_DIR/pomonote"
RELEASE_BIN="$(pwd)/target/release/pomonote"
BACKUP_PATH="$BIN_PATH.bak.$(date +%s)"

echo "[INFO] Starting safe installation of pomonote..."

# Check for cargo
if ! command -v cargo >/dev/null 2>&1; then
  echo "[ERROR] cargo is not installed. Aborting." >&2
  exit 1
fi

# Check if pomonote is running and kill it
if pgrep -f "$BIN_PATH" > /dev/null; then
  echo "[INFO] pomonote is currently running. Killing the process before reinstalling..."
  pkill -f "$BIN_PATH" || { echo "[WARN] Failed to kill pomonote. Continuing..."; }
fi

# Build the Rust application
echo "[INFO] Building pomonote with cargo..."
cargo build --release

# Check if build succeeded and binary exists
if [ ! -f "$RELEASE_BIN" ]; then
  echo "[ERROR] Build failed or binary not found at $RELEASE_BIN. Aborting." >&2
  exit 1
fi

# Prepare bin directory
mkdir -p "$BIN_DIR"

# Backup existing binary if present
if [ -f "$BIN_PATH" ]; then
  echo "[INFO] Backing up existing binary to $BACKUP_PATH"
  cp "$BIN_PATH" "$BACKUP_PATH"
fi

# Copy new binary
cp "$RELEASE_BIN" "$BIN_PATH"
chmod 755 "$BIN_PATH"

echo "[SUCCESS] Installed pomonote to $BIN_PATH. Make sure $BIN_DIR is in your PATH."