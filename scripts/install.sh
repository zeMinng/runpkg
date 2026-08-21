#!/bin/sh

set -e

REPO="zeMinng/runpkg"
INSTALL_DIR="$HOME/.runpkg/bin"

# Require the tools used below before doing anything else.
for cmd in curl tar; do
    command -v "$cmd" >/dev/null 2>&1 || { echo "Error: $cmd is required but not found."; exit 1; }
done

TMP_FILE=$(mktemp)
TMP_CHECKSUM=$(mktemp)
trap 'rm -f "$TMP_FILE" "$TMP_CHECKSUM"' EXIT

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
    CHECKSUMS_URL="https://github.com/$REPO/releases/download/$VERSION/checksums.txt"
else
    VERSION="latest"
    URL="https://github.com/$REPO/releases/latest/download/$PACKAGE"
    CHECKSUMS_URL="https://github.com/$REPO/releases/latest/download/checksums.txt"
fi

echo "Installing runpkg $VERSION..."

echo "Downloading:"
echo "$URL"

mkdir -p "$INSTALL_DIR"

curl -fL --retry 3 --retry-delay 2 "$URL" -o "$TMP_FILE"
curl -fL --retry 3 --retry-delay 2 "$CHECKSUMS_URL" -o "$TMP_CHECKSUM"

# Verify SHA256 checksum. Linux ships sha256sum; macOS ships shasum.
if command -v sha256sum >/dev/null 2>&1; then
    ACTUAL=$(sha256sum "$TMP_FILE" | awk '{print $1}')
else
    ACTUAL=$(shasum -a 256 "$TMP_FILE" | awk '{print $1}')
fi
EXPECTED=$(grep "  $PACKAGE\$" "$TMP_CHECKSUM" | awk '{print $1}')

if [ -z "$EXPECTED" ] || [ "$EXPECTED" != "$ACTUAL" ]; then
    echo "Checksum verification failed for $PACKAGE"
    exit 1
fi

tar -xzf "$TMP_FILE" -C "$INSTALL_DIR"

[ -f "$INSTALL_DIR/runpkg" ] || { echo "Error: runpkg binary not found after extraction."; exit 1; }
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
if grep -qF '.runpkg/bin' "$PROFILE_FILE" 2>/dev/null; then
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
