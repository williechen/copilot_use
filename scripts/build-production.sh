#!/usr/bin/env bash
# 原生 release 建置；不啟動服務、不修改資料庫。
set -euo pipefail
cd "$(dirname "$0")/.."
for tool in cargo rustup trunk python3; do
  command -v "$tool" >/dev/null || { echo "缺少建置工具：$tool" >&2; exit 1; }
done
source_sha=$(git rev-parse HEAD)
if [[ -n $(git status --porcelain) ]]; then
  echo '正式建置要求乾淨的已提交來源，以便追溯 SHA' >&2
  exit 1
fi
cargo fmt --all -- --check
bash scripts/build-chat-ui.sh
cargo build --locked --release -p day9-chat -p day9-analytics
release_dir="target/native-release/$source_sha"
mkdir -p "$release_dir/bin" "$release_dir/site/pkg"
cp target/release/day9-chat target/release/day9-analytics "$release_dir/bin/"
cp target/site/pkg/* "$release_dir/site/pkg/"
cp -R deploy/systemd "$release_dir/"
python3 - "$release_dir" "$source_sha" <<'PYTHON'
import hashlib, json, pathlib, sys
root = pathlib.Path(sys.argv[1])
files = {str(p.relative_to(root)): hashlib.sha256(p.read_bytes()).hexdigest()
         for p in sorted(root.rglob("*")) if p.is_file() and p.name != "manifest.json"}
(root / "manifest.json").write_text(json.dumps({"source_sha": sys.argv[2],
    "files": files, "deployment": "not deployed"}, indent=2) + "\n")
PYTHON
printf '原生成品：%s；尚未部署或完成驗收\n' "$release_dir"
