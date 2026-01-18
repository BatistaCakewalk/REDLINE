#!/bin/bash

# REDLINE Installer for Linux and macOS

INSTALL_DIR="$HOME/.redline"
BIN_DIR="/usr/local/bin"
REPO_URL="https://github.com/REDTOPS-Enterprise/REDLINE.git"

echo "Installing REDLINE..."

# 1. Remove old installation if it exists
if [ -d "$INSTALL_DIR" ]; then
    echo "Removing existing installation..."
    rm -rf "$INSTALL_DIR"
fi
if [ -L "$BIN_DIR/redline" ]; then
    echo "Removing old symlink..."
    rm "$BIN_DIR/redline"
fi

# 2. Clone the repository
echo "Cloning REDLINE repository into $INSTALL_DIR..."
git clone "$REPO_URL" "$INSTALL_DIR"
if [ $? -ne 0 ]; then
    echo "Error: Failed to clone repository. Aborting."
    exit 1
fi

# 3. Make the main script executable
echo "Setting permissions..."
chmod +x "$INSTALL_DIR/redline.py"

# 4. Create a symlink
echo "Creating symlink in $BIN_DIR..."
ln -s "$INSTALL_DIR/redline.py" "$BIN_DIR/redline"
if [ $? -ne 0 ]; then
    echo "Error: Failed to create symlink. You may need to run this script with sudo."
    echo "Try: sudo ./install.sh"
    exit 1
fi

# 5. Initialize the compiler core
echo "Initializing REDLINE compiler core..."
"$INSTALL_DIR/redline.py" init

echo ""
echo "REDLINE installed successfully!"
echo "You can now use the 'redline' command anywhere on your system."
