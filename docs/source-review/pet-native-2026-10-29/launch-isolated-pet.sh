#!/usr/bin/env bash
set -euo pipefail
task_root='/home/willie/Documents/Codex/2026-10-28/referenced-chatgpt-conversation-this-is-an'
test_output="$task_root/outputs/pet-native-restart-2026-10-29"
frontend="$task_root/work/acceptance-product/pet_apps/pet_ui/dist"
database="$task_root/work/acceptance-db/pet-gui-restart-v2.sqlite"
binary="$task_root/work/latest/pet_apps/target/debug/pet-server"
export PATH="/home/willie/.cargo/bin:$PATH"
test -f "$frontend/index.html"
test -f "$database"
test -x "$binary"
mkdir -p "$test_output"
python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1",1420)); s.close()'
python3 -m http.server 1420 --bind 127.0.0.1 --directory "$frontend" >> "$test_output/frontend-runtime.txt" 2>&1 &
pet_http_pid=$!
python3 "$test_output/watch_pet_gui.py" "$database" "$test_output/database-events.jsonl" &
pet_watch_pid=$!
trap 'kill "$pet_http_pid" "$pet_watch_pid" 2>/dev/null || true' EXIT
for pet_wait in $(seq 1 20); do
  if curl --silent --fail http://127.0.0.1:1420/ > /dev/null; then break; fi
  sleep 0.5
done
curl --silent --fail http://127.0.0.1:1420/ > /dev/null
cd "$task_root/work/acceptance-product/pet_apps/pet_server"
export PET_DATABASE_URL="sqlite://$database"
export PET_ENABLE_ROAM=0
# Process-local fallback for this VMware validation session; no system changes.
export LIBGL_ALWAYS_SOFTWARE=true
export GALLIUM_DRIVER=llvmpipe
export WEBKIT_DISABLE_DMABUF_RENDERER=1
unset OPENAI_API_KEY ANTHROPIC_API_KEY GEMINI_API_KEY
printf '\nSESSION_START %s\n' "$(date --iso-8601=seconds)" >> "$test_output/native-runtime.txt"
printf 'RENDER_MODE software llvmpipe; WebKit DMA-BUF disabled for this process\n' >> "$test_output/native-runtime.txt"
printf '專用 SQLite：%s\n請只操作「重新驗收A-a1」與「重新驗收B-b2」，勿使用聊天或檔案功能。\n' "$database"
set +e
"$binary" 2>&1 | tee -a "$test_output/native-runtime.txt"
pet_exit=${PIPESTATUS[0]}
set -e
printf 'SESSION_END exit=%s %s\n' "$pet_exit" "$(date --iso-8601=seconds)" >> "$test_output/native-runtime.txt"
exit "$pet_exit"
