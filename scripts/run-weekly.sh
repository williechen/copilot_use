#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
unset OPENAI_API_KEY
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}"
if ! command -v node >/dev/null; then
  bundled_node="$HOME/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin"
  export PATH="$bundled_node:$PATH"
fi
command -v node >/dev/null || { echo '缺少 Node.js' >&2; exit 1; }
evidence="target/weekly/$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$evidence"
python3 scripts/tests/weekly-compare.py
python3 scripts/weekly-compare.py --reference "${COPILOT_REFERENCE:-../rust_ai_chat}" --output "$evidence/comparison.json"
VERIFY_REPORT="$PWD/$evidence/functional.json" python3 scripts/local-verify.py
printf '功能檢查完成，證據：%s；語意整合與正式驗收須另核對\n' "$evidence"
