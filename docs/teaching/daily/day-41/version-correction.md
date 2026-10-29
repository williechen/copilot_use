# Day41 版本更正｜Rust stable 1.99.1

核對日期：2026-10-27。Chat／Pet／Analytics Day41 三篇原文誤把 **Rust 1.99.0（2026-10-01）** 列為當日最新 stable；正確為 **Rust 1.99.1（2026-10-15）**。官方公告：https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/；官方版本總表：https://blog.rust-lang.org/releases/ 。

1.99.1 修正 `wcslen`、x86_64 FFI bool zero-extension、macOS rust-lld rpath 等回歸。三篇原 Cargo manifest/lock 與 patch 不因工具鏈版本更正而改動；使用者套用時應記錄 `rustc --version` 並在隔離副本驗證。原文章、commit、blob SHA、SHA256 保留，本檔為**追加勘誤，不靜默覆寫**。Day41 仍是未套用、未測試、未驗收方案。

原文：[Chat](chat.md) · [Pet](pet.md) · [Analytics](analytics.md)；[SHA256 證據](delivery-verification.json)。
