#!/bin/sh

set -e

REPO="zeMinng/runpkg"
INSTALL_DIR="$HOME/.runpkg/bin"
TMP_FILE="/tmp/runpkg.tar.gz"

trap 'rm -f "$TMP_FILE"' EXIT

OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Darwin)
        case "$ARCH" in
            arm64)  PACKAGE="runpkg-macos-arm64.tar.gz" ;;
            x86_64) PACKAGE="runpkg-macos-x64.tar.gz" ;;
            *)
                echo "Unsupported architecture: $ARCH"
                exit 1
                ;;
        esac
        ;;

    Linux)
        case "$ARCH" in
            x86_64)  PACKAGE="runpkg-linux-x64.tar.gz" ;;
            aarch64) PACKAGE="runpkg-linux-arm64.tar.gz" ;;
            *)
                echo "Unsupported architecture: $ARCH"
                exit 1
                ;;
        esac
        ;;

    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

if [ -n "$RUNPKG_VERSION" ]; then
    VERSION="$RUNPKG_VERSION"
    URL="https://github.com/$REPO/releases/download/$VERSION/$PACKAGE"
else
    VERSION="latest"
    URL="https://github.com/$REPO/releases/latest/download/$PACKAGE"
fi

echo "Installing runpkg $VERSION..."

echo "Downloading:"
echo "$URL"

mkdir -p "$INSTALL_DIR"

curl -fL "$URL" -o "$TMP_FILE"

tar -xzf "$TMP_FILE" -C "$INSTALL_DIR"

chmod +x "$INSTALL_DIR/runpkg"

# Detect the user's shell and pick the matching profile file.
SHELL_BIN="${SHELL##*/}"

case "$SHELL_BIN" in
    fish)
        PROFILE_FILE="$HOME/.config/fish/config.fish"
        PATH_LINE="fish_add_path \"$INSTALL_DIR\""
        ;;
    zsh)
        PROFILE_FILE="$HOME/.zshrc"
        PATH_LINE="export PATH=\"$INSTALL_DIR:\$PATH\""
        ;;
    bash)
        if [ "$OS" = "Darwin" ]; then
            PROFILE_FILE="$HOME/.bash_profile"
        else
            PROFILE_FILE="$HOME/.bashrc"
        fi
        PATH_LINE="export PATH=\"$INSTALL_DIR:\$PATH\""
        ;;
    *)
        PROFILE_FILE="$HOME/.profile"
        PATH_LINE="export PATH=\"$INSTALL_DIR:\$PATH\""
        ;;
esac

# Ensure the parent directory exists (needed for the fish config path).
mkdir -p "${PROFILE_FILE%/*}"

# Append the PATH line idempotently (skip if it is already configured).
if grep -qF "$INSTALL_DIR" "$PROFILE_FILE" 2>/dev/null; then
    echo ""
    echo "PATH already configured in:"
    echo "$PROFILE_FILE"
else
    printf '\n# runpkg\n%s\n' "$PATH_LINE" >> "$PROFILE_FILE"
    echo ""
    echo "Added runpkg to PATH in:"
    echo "$PROFILE_FILE"
fi

echo ""
echo "Installed to:"
echo "$INSTALL_DIR"
echo ""
echo "Restart your terminal (or run: source \"$PROFILE_FILE\"), then run:"
echo "runpkg"
