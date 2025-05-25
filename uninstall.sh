#!/bin/bash
set -e

INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="rlr"
COOKIE_PATH=$(rlr cookie-path)

if [[ -f $COOKIE_PATH ]]; then
  echo "Removing $COOKIE_PATH"
  rm "$COOKIE_PATH"
fi

if [[ -f "$INSTALL_DIR/$BINARY_NAME" ]]; then
  echo "Removing $INSTALL_DIR/$BINARY_NAME"
  rm "$INSTALL_DIR/$BINARY_NAME"
else
  echo "File not found in $INSTALL_DIR"
fi
