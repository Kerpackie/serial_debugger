#!/bin/bash
# scripts/install.sh
APP_NAME="serial_debugger"
INSTALL_DIR="$HOME/.local/bin"
SOURCE_PATH="release/$APP_NAME"

echo "🔧 Starting installation for $APP_NAME..."
mkdir -p "$INSTALL_DIR"

echo "Copying executable to $INSTALL_DIR"
cp "$SOURCE_PATH" "$INSTALL_DIR/"

if [ -n "$BASH_VERSION" ]; then
    CONFIG_FILE="$HOME/.bashrc"
elif [ -n "$ZSH_VERSION" ]; then
    CONFIG_FILE="$HOME/.zshrc"
else
    CONFIG_FILE="$HOME/.profile"
fi

echo "Updating PATH in $CONFIG_FILE..."
if ! grep -q "export PATH=\"\$HOME/.local/bin:\$PATH\"" "$CONFIG_FILE"; then
    echo '' >> "$CONFIG_FILE"
    echo '# Add user-local executables to PATH' >> "$CONFIG_FILE"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$CONFIG_FILE"
    echo "Successfully added $INSTALL_DIR to your PATH."
else
    echo "Directory is already in your PATH."
fi

echo -e "\nInstallation complete! Please restart your terminal or run 'source $CONFIG_FILE' to use the '$APP_NAME' command."