#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
(cd "$ROOT/apps/chat_frontend" && NO_COLOR=true trunk build --release=true --no-color)
mkdir -p "$ROOT/target/site/pkg"
cp "$ROOT"/apps/chat_frontend/dist/*.js "$ROOT/target/site/pkg/chat_ui.js"
cp "$ROOT"/apps/chat_frontend/dist/*_bg.wasm "$ROOT/target/site/pkg/chat_ui_bg.wasm"
cp "$ROOT/crates/chat_ui/style.css" "$ROOT/target/site/pkg/chat.css"
