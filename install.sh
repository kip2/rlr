#!/bin/bash
set -e

TAG="v$(grep '^version =' Cargo.toml | sed -E 's/version = \"(.*)\"/\1/')"
REPO="kip2/rlr"
BINARY="rlr"
TARGET="x86_64-unknown-linux-gnu"

if [[ "$(uname)" == "Darwin" ]]; then
  TARGET="x86_64-apple-darwin"
fi

FILE="${BINARY}-${TARGET}.tar.gz"
URL="https://github.com/${REPO}/releases/download/${TAG}/${FILE}"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "Downloading $URL"
curl -L -o "$FILE" "$URL"
tar -xzf "$FILE" -C "$INSTALL_DIR"
chmod +x "$INSTALL_DIR/$BINARY"

rm "$FILE"

if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  SHELL_RC="$HOME/.bashrc"
  [[ "$SHELL" =~ zsh ]] && SHELL_RC="$HOME/.zshrc"

  echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_RC"
  echo "Added $INSTALL_DIR to PATH in $SHELL_RC. Restart your terminal or run:"
  echo "source $SHELL_RC"
else
  echo "$INSTALL_DIR is already in PATH"
fi

