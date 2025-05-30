#!/bin/bash

set -e  # エラーが出たら即終了

if [ $# -ne 1 ]; then
    echo "Usage: ./setup.sh 1"
    exit 1
fi

PROBLEM_NUMBER=$1
DIRECTORY_SUFFIX="p-"
DIRECTORY_NAME="${DIRECTORY_SUFFIX}${PROBLEM_NUMBER}"

# .vscodeディレクトリの、このシェルスクリプトを実行するパスからの相対パスを記載して下さい
VSCODE_SETTINGS=".vscode/settings.json"

if [ -d "$DIRECTORY_NAME" ]; then
    echo "ディレクトリが既に存在しています。"
    exit 1
fi

# settings.json がなければ初期化
if [ ! -f "$VSCODE_SETTINGS" ]; then
    mkdir -p "$(dirname "$VSCODE_SETTINGS")"
    echo '{ "rust-analyzer.linkedProjects": [], "files.watcherExclude": { "**/target": true } }' > "$VSCODE_SETTINGS"
fi

# json形式をチェックする
error_output=$(mktemp)

if ! jq empty "$VSCODE_SETTINGS" 2> "$error_output"; then
    echo "settings.json に構文エラーがあります"
    cat "$error_output"
    rm "$error_output"
    exit 1
fi

# ダウンロード
rlr d $PROBLEM_NUMBER
# cargo run -- d $PROBLEM_NUMBER


# === VSCode Rust-analyzer settings 追記処理 ===

CARGO_PATH="${DIRECTORY_NAME}/sample/Cargo.toml"

if ! grep -q "$CARGO_PATH" "$VSCODE_SETTINGS"; then
    tmpfile=$(mktemp)
    jq --arg path "$CARGO_PATH" '
        .["rust-analyzer.linkedProjects"] += [$path]
    ' "$VSCODE_SETTINGS" > "$tmpfile" && mv "$tmpfile" "$VSCODE_SETTINGS"
    echo "$CARGO_PATH を .vscode/settings.json に追加しました"
else
    echo "️$CARGO_PATH はすでに .vscode/settings.json に含まれています"
fi
