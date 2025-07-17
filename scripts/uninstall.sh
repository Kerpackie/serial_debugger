#!/bin/bash
# scripts/uninstall.sh
APP_NAME="serial_debugger"
INSTALL_DIR="$HOME/.local/bin"
EXE_PATH="$INSTALL_DIR/$APP_NAME"

echo "Starting uninstallation of $APP_NAME..."

if [ -f "$EXE_PATH" ]; then
    echo "Removing executable: $EXE_PATH"
    rm "$EXE_PATH"
else
    echo "Executable not found."
fi

if [ -n "$BASH_VERSION" ]; then
    CONFIG_FILE="$HOME/.bashrc"
elif [ -n "$ZSH_VERSION" ]; then
    CONFIG_FILE="$HOME/.zshrc"
else
    CONFIG_FILE="$HOME/.profile"
fi

echo "Cleaning up PATH in $CONFIG_FILE..."
if [ -f "$CONFIG_FILE" ]; then
    sed -i "/# Add user-local executables to PATH/d" "$CONFIG_FILE"
    sed -i "/export PATH=\"\$HOME\/.local\/bin:\$PATH\"/d" "$CONFIG_FILE"
    echo "PATH cleanup complete."
else
    echo "Config file $CONFIG_FILE not found. No changes needed."
fi

echo -e "\nUninstallation complete! Please restart your terminal for changes to take full effect."