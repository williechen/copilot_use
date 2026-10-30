#!/usr/bin/env bash
set -euo pipefail
BIN=/home/willie/Documents/Codex/2026-10-28/referenced-chatgpt-conversation-this-is-an/work/gate2-20261031/binaries/pet-server-2c7ff8b
DB=/home/willie/Documents/Codex/2026-10-28/referenced-chatgpt-conversation-this-is-an/work/gate2-20261031/pet-manual.sqlite
RUN_DIR=/home/willie/Documents/Codex/2026-10-28/referenced-chatgpt-conversation-this-is-an/work/gate2-20261031/manual-run
EXPECTED=8f826a6709c45c6d84e913a31cc67b640976c2b5e2952c1e50bf20a454aae5c6
if [ ! -x "$BIN" ]; then echo "缺少已驗證的原生執行檔：$BIN" >&2; exit 1; fi
if ! printf '%s  %s\n' "$EXPECTED" "$BIN" | sha256sum --check --status; then echo "原生產物雜湊不符，停止啟動" >&2; exit 1; fi
if [ ! -e "$DB" ]; then
  python3 - <<'PYDB'
import sqlite3
source=sqlite3.connect('file:/home/willie/Documents/Codex/2026-10-28/referenced-chatgpt-conversation-this-is-an/work/gate2-20261031/pet-gui.sqlite?mode=ro',uri=True)
copy=sqlite3.connect('/home/willie/Documents/Codex/2026-10-28/referenced-chatgpt-conversation-this-is-an/work/gate2-20261031/pet-manual.sqlite')
source.backup(copy)
copy.close()
source.close()
PYDB
fi
echo "來源：rust_ai_ring@2c7ff8b；專用資料庫：$DB"
if [ "${1:-}" = "--check" ]; then echo "產物與隔離資料庫預檢完成；尚未啟動GUI"; exit 0; fi
mkdir -p "$RUN_DIR"
cd "$RUN_DIR"
if [ ! -e .env ]; then : > .env; fi
mkdir -p xdg-data xdg-cache xdg-config
exec 9>acceptance.lock
if ! flock -n 9; then echo "本驗收視窗已有程序執行，請先在原終端結束" >&2; exit 1; fi
LOG="$PWD/manual-startup.log"
echo "僅操作Gate2-A／Gate2-B／Gate2-C；關閉控制台只隱藏，終端Ctrl+C結束整個程序。"
echo "本輪待人工確認：實際桌面拖曳、縮放／可讀性、原生右鍵及多螢幕。日誌：$LOG"
env -u TAURI_WEBVIEW_AUTOMATION -u OPENAI_API_KEY -u ANTHROPIC_API_KEY -u GEMINI_API_KEY \
  XDG_DATA_HOME="$PWD/xdg-data" XDG_CACHE_HOME="$PWD/xdg-cache" XDG_CONFIG_HOME="$PWD/xdg-config" \
  LIBGL_ALWAYS_SOFTWARE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 WEBKIT_DISABLE_COMPOSITING_MODE=1 \
  PET_ENABLE_ROAM=true PET_DATABASE_URL="sqlite://$DB?mode=rwc" "$BIN" 2>&1 | tee -a "$LOG"
