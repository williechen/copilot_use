#!/usr/bin/env bash
# 供 build／weekly source，優先使用系統套件，必要時使用專案私有工具。
_native_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
_native_tools="$_native_root/target/native-tools/root"
if [[ -x "$_native_tools/usr/bin/x86_64-linux-gnu-gcc-15" ]]; then
  export PATH="$_native_tools/usr/bin:$PATH"
  export LD_LIBRARY_PATH="$_native_tools/usr/lib/x86_64-linux-gnu${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  export CC="$_native_tools/usr/bin/x86_64-linux-gnu-gcc-15"
  export CXX="$_native_tools/usr/bin/x86_64-linux-gnu-g++-15"
  export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="$CC"
  export OPENSSL_DIR="$_native_tools/usr"
  export PKG_CONFIG_PATH="$_native_tools/usr/lib/x86_64-linux-gnu/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
  export PKG_CONFIG_SYSROOT_DIR="$_native_tools"
  export CFLAGS="-I$_native_tools/usr/include/x86_64-linux-gnu${CFLAGS:+ $CFLAGS}"
fi
if ! command -v node >/dev/null; then
  export PATH="$HOME/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin:$PATH"
fi
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}"
unset _native_root _native_tools
