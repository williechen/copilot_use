#!/usr/bin/env bash
# 僅驗證配置及建置，不啟動服務、不改 DB、不讀 AI key。
set -euo pipefail
cd "$(dirname "$0")/.."
command -v docker >/dev/null || { echo '缺少 Docker Engine 與 Compose v2' >&2; exit 1; }
[[ -f .env ]] || { echo '請先依 .env.example 建立私有 .env' >&2; exit 1; }
export COPILOT_BUILD_SHA
COPILOT_BUILD_SHA=$(git rev-parse HEAD)
if [[ -n $(git status --porcelain) ]]; then
  echo '正式建置要求乾淨的已提交來源，以便追溯 SHA' >&2
  exit 1
fi
compose=(docker compose --env-file .env -f compose.yaml -f deploy/compose.production.yaml)
"${compose[@]}" config --quiet
"${compose[@]}" build chat analytics
mkdir -p target
docker image inspect "copilot-chat:$COPILOT_BUILD_SHA" "copilot-analytics:$COPILOT_BUILD_SHA" > target/production-images.json
printf '建置來源 %s；尚未部署或完成驗收\n' "$COPILOT_BUILD_SHA"
