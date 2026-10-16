#!/usr/bin/env bash
# Ubuntu 26.04：無 sudo 時解包官方 APT 套件至 target，系統套件資料庫不變。
set -euo pipefail
cd "$(dirname "$0")/.."
source /etc/os-release
[[ "$ID" == ubuntu && "$VERSION_ID" == 26.04 ]] || { echo '此私有工具 bootstrap 僅驗證 Ubuntu 26.04' >&2; exit 1; }
mkdir -p target/native-tools/debs target/native-tools/root
(
  cd target/native-tools/debs
  apt-get download gcc-15-x86-64-linux-gnu cpp-15-x86-64-linux-gnu libgcc-15-dev libstdc++-15-dev g++-15-x86-64-linux-gnu cmake cmake-data pkgconf pkgconf-bin libpkgconf7 libssl-dev libssl3t64 libjsoncpp26 librhash1 libarchive13t64
)
if [[ "${1:-}" == --desktop ]]; then
  apt-get -o Debug::NoLocking=1 -o "Dir::Cache::archives=$PWD/target/native-tools/debs" --download-only --yes install libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev patchelf xvfb
fi
for package in target/native-tools/debs/*.deb; do
  dpkg-deb -x "$package" target/native-tools/root
done
python3 - <<'PYTHON'
from pathlib import Path
root=Path('target/native-tools/root').resolve()
for file in (root/'usr/lib').rglob('*'):
    if file.is_symlink() and not file.exists():
        system=Path('/')/file.relative_to(root)
        if not system.exists():
            link=file.readlink()
            system=link if link.is_absolute() else Path('/')/file.parent.relative_to(root)/link
        if system.exists():
            file.unlink()
            file.symlink_to(system.resolve())
PYTHON
source scripts/native-env.sh
printf 'int main(void) { return 0; }\n' > target/native-tools/probe.c
"$CC" target/native-tools/probe.c -o target/native-tools/probe
target/native-tools/probe
cmake --version
pkg-config --modversion openssl
