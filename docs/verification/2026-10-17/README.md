# 2026-10-17 缺口修正與驗證

程式驗證基準：copilot_use `9be34f2d7c89842ec7c0aa39a4a60f0186da6c3e`；本紀錄所屬後續提交另修正單例驗收子程序、timer 與文件。rust_ai_chat 基準 `3e58e650bd63bcd21ba58e7aabf751542401ef80`，保持唯讀。

## 已修正與實際證據

- GitHub main 已由 `49e1d94` 推進至 `763aee4`，再同步 `9be34f2`；保留原有提交、tag 與 release。後續修正以本紀錄所屬提交為準。
- 週六 workflow cron 改為 `0 16 * * 5`，對應台北週六00:00；本機 timer 範本與操作手冊同步。`systemd-analyze calendar` 得到下一次2026-10-24 00:00 Asia/Taipei。
- README 移除過時 Day12／CI 發布門檻並指向教學總文章；歷史驗收資料保留原 SHA／當時排程。Day14 原文鏡像沒有回填修改，當中的舊 CI 說法不再是現行規則。
- Windows CI 真實失敗為讀取中文 Tauri JSON 使用系統預設編碼；明訂 UTF-8 並加非 UTF-8 locale 的子程序回歸測試。`python3 scripts/tests/versioning.py` 通過，三平台 CI 都執行此測試。
- 上一輪 Linux/macOS GUI 均在第二次啟動失敗：子程序繼承驗收模式，遇到主程序已建立的 state 被「必須為空白 fixture」檢查拒絕，未到單例插件。子程序改用正常啟動模式；主程序仍完整保留新 fixture 驗證，GUI 斷言仍要求第二程序退出及原控制面板顯示。此修改待新 CI 實際 GUI 驗證。

| 本機驗證 | 結果 | 證據 |
|---|---|---|
| Analytics 原生 PostgreSQL 全部 lib 測試 | 16 passed，0 failed，0 ignored | [完整日誌](analytics-db-gap-tests.log) |
| Chat WASM hydrate 編譯 | 通過 | [日誌](compatibility-ui-hydrate.log) |
| Chat native 編譯 | 通過 | [日誌](compatibility-chat-native.log) |
| Pet 前端 WASM 編譯 | 通過 | [日誌](compatibility-pet-hydrate.log) |
| rustfmt、workflow YAML、版本編碼回歸、參考庫 patch 套用檢查 | 通過 | 對應命令見下文 |

Analytics 測試涵蓋來源／事件類型聚合、gRPC 交易後 ACK、SSE、冪等與衝突、UUID/VARCHAR、封存還原、SQLite Outbox 重試。首次編譯因本機磁碟滿而失敗；只清除可重建的 Pet debug 快取後重跑通過。原生測試使用暫存 PostgreSQL，沒有讀取正式資料或執行付費 AI。

## 套件安全

cargo-audit 0.22.2，官方二進位 SHA256 `7fb9497f8594b389e5fce5ef9b92db08432996895b2e0c5a0167a69ed445c428`，RustSec DB `479c888ce4401313bca61ba4aba2d0f6244c4a09`。四份獨立 lockfile 都啟用漏洞及 yanked 檢查。

root/Pet lockfile 的 rustls 0.23.45 原有4個已公告漏洞，已升級至0.23.46；對應 [RustSec 公告](https://rustsec.org/advisories/RUSTSEC-2026-0349.html)及 RUSTSEC-2026-0346、0347、0348。升級後四份 lockfile 已知漏洞數均為0；CI 加入相同四份檢查。

- [root audit](security-root-audit-after.json)：0漏洞，paste 1.0.15 停止維護警告。
- [Pet audit](security-pet-audit-after.json)：0漏洞，proc-macro-error 1.0.4 停止維護、zip 4.6.1 已撤回警告。
- [Pet frontend audit](security-pet-frontend-audit-after.json)：0漏洞，paste 停止維護警告。
- [Chat frontend audit](security-chat-frontend-audit-after.json)：0漏洞，paste 停止維護警告。

0已知漏洞不等於完整安全驗收；以上相依警告尚未解決。沒有為消除警告盲目升級上游不相容的大版本。架構文件依 lockfile 標示實際 Tauri 2.12.3；沒有將可用新版本寫成已安裝。lockfile 固定值見 [SHA256](lock-sha256.json)。所有套件／MSRV／Windows及macOS原生、安裝包與更新相容性仍須匹配 SHA 的實際結果，不能以 Linux/WASM 編譯替代。

## 參考庫 CI 尚待使用者套用

文章要求 → rust_ai_chat 的 `source_summary_tests`／`source_kind_summary_tests` → copilot_use 的來源／類型聚合與16項 lib 測試 → 上表原生日誌。

缺口在 rust_ai_chat：Persistence CI 只指定 `--test postgres_event_store`，不涵蓋 feature 控制的 lib DB 測試。成品現有 `cargo test --workspace --all-targets --locked` 已涵蓋其 lib 測試，不需新增假對應 target。

已備妥[最小修正 patch](rust_ai_chat-Analytics-DB-CI.patch)，`git apply --check` 已對參考 SHA 通過，**未套用、未執行參考庫新增 CI**。由使用者在 rust_ai_chat 執行：

```sh
git apply --check /home/willie/Documents/copilot_use/docs/verification/2026-10-17/rust_ai_chat-Analytics-DB-CI.patch
git apply /home/willie/Documents/copilot_use/docs/verification/2026-10-17/rust_ai_chat-Analytics-DB-CI.patch
# 在既有 PostgreSQL 測試環境中執行；不填正式資料庫憑證。
DATABASE_URL="$ANALYTICS_DATABASE_URL" cargo test --locked -p stats-server --features postgres-integration --lib
```

此項作為 Day15或下一篇適用 Day 的「Refactor 後：舊 Day 更正補充」素材，不另算新功能、不回填舊 Day，也不宣稱已發布新文章。

## 遠端 CI 狀態

[763aee4 原始 CI](https://github.com/williechen/copilot_use/actions/runs/38941906999)：Rust/PostgreSQL及故障復原通過；Windows版本檔編碼失敗，Linux/macOS第二次啟動GUI失敗，Compose尚未完成。這些失敗已保留，不改寫成通過。

[9be34f2 修正後 CI](https://github.com/williechen/copilot_use/actions/runs/38942947148) 已啟動，尚未完成；本紀錄所屬後續 GUI 修正還需以新的 main CI 判定。未建立新 release／tag，未宣稱完整跨平台驗收完成。
