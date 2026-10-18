#!/bin/bash
set +e
ROOT=/home/willie/Documents/Codex/2026-10-18/referenced-chatgpt-conversation-this-is-an-2
REPO=/home/willie/Documents/rust_ai_chat
OUT=$ROOT/outputs/full-9a95a10
cd "$REPO"
cargo leptos build --cargo-locked > "$OUT/main-leptos.log" 2>&1
echo LEPTOS_EXIT=$?
(cd apps/desktop_pet/frontend && NO_COLOR=true trunk build --locked --release) > "$OUT/main-pet-frontend.log" 2>&1
echo FRONTEND_EXIT=$?
cargo build --locked -p stats-server -p desktop-pet --features tauri/custom-protocol > "$OUT/main-native-build.log" 2>&1
echo NATIVE_EXIT=$?
mkdir -p "$ROOT/work/bin9a"
cp target/debug/server "$ROOT/work/bin9a/main-server"
cp target/debug/stats-server "$ROOT/work/bin9a/main-stats"
cp target/debug/desktop-pet "$ROOT/work/bin9a/main-pet"
