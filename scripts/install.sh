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

echo ""
echo "Installed to:"
echo "$INSTALL_DIR"
echo ""
echo "Add this to your shell profile:"
echo ""
echo "export PATH=\"$INSTALL_DIR:\$PATH\""
