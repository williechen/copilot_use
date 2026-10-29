# 2026-10-23｜首次每週全專案重新比對：靜態查核與未完成 Gate

> **結論：未完成全專案完整比對。** 本次已取得兩庫完整檔案樹並核對關鍵來源、Day30 及 CI；但沒有逐行檢查全部原始碼、所有 migrations／測試／deployment，也沒有重新執行全部 DB、WASM、HTTP／WS、GUI、Node 及端到端測試。因此不得標記 complete，不更新 `last_full_comparison`，不改 `next_day` 或 makeup_queue。

## 精確來源

- 程式（唯讀）：`williechen/rust_ai_chat/main@9c2b7cf5a4c60a08254445911916c4ed9b02d7b3`。
- 規則／教學：`williechen/copilot_use/main@a11af0f45c033abd7f7c361bc9a09ee9a7fda966`（查核起始 SHA；本報告後續提交會產生新文件 SHA）。
- 兩庫 recursive Git tree：`rust_ai_chat` 232 個樹項目、`copilot_use` 2431 個樹項目，均 `truncated=false`；樹清冊完整不代表檔案內容全數核實。
- 狀態：`docs/learning-state.json` 三案 `next_day=31`；舊 `last_full_comparison=2026-10-16T22:47:14.399027+08:00`，本次不得更新。
- 最新教學：Chat Day30 r3、Pet Day30 r2、Analytics Day30 r2；`manifest.json` 三案為 `github_article_readback_verified`，非主體程式驗收。

## 跨專案架構／產品需求檢查

已讀取 `P9_TARGET.md`、`FUTURE_WORK.md`、`SYSTEM_ARCHITECTURE.md`、`UI_ALIGNMENT_REQUIREMENTS.md`、共同規則與目前進度。三案共同目標維持：Leptos UI（Chat SSR/hydrate、Pet CSR/Tauri、Analytics 後台 UI）、Axum HTTP／WS 或 Tauri IPC、Application／Domain、SQLx Adapter；Chat/Pet SQLite 與 Analytics PostgreSQL 相互隔離。此為文件要求，**不是全部實作已符合的驗收聲明**。

## 本次已證實的差異與風險

| 專案 | 程式／證據 | 差異與處置 | 判定 |
| --- | --- | --- | --- |
| Chat | `apps/server/src/lib.rs` 中 `.route("/api/rooms", get(http::list_rooms).post(http::create_room))` | 生產 POST 仍與 GET 放在公開 route；Day30 r3 教學要求將 POST 加會員 Session middleware。需先套用後測無 Cookie 401／有效會員 201／GET 200，以及 WS／ACL 相關 Gate。 | source gap；Gate 2 未驗收 |
| Chat | `guarded_room`、`guarded_ws` 已存在、採 `require_member` middleware | 不要重教已存在的 Session guard；仍須以真 HTTP/WS Upgrade／SQLite 檢查有效／偽造／過期／撤銷 Session 與逐房權限。 | 部分已接線、整合驗收未完成 |
| Pet | `apps/desktop_pet/src/pet_windows.rs::cage_saved_pet`：先 `set_caged(true)` 再 `window.close().map_err(...)?` | 關窗同步失敗時直接傳回，**沒有回復 DB `set_caged(false)`**，可能留下 DB 與生活窗不一致；Day30 r2 提案未套用。 | confirmed defect path；原生失敗注入待測 |
| Analytics | `apps/stats_server/src/password_login.rs::login_page() -> Html<&'static str>`，內含兩個 doctype | 仍以 Axum 手寫頁面呈現登入；Day30 r2 Leptos SSR 提案未套用。Session／Argon2／PG 鏈不應藉改頁重做。 | confirmed UI alignment gap |
| Analytics | `apps/stats_server/src/dashboard_ui.rs` 已有 `DashboardView`、`render_dashboard` | 可重用既有 Leptos SSR；登入與改密碼正式路由、帳號 ACL 仍需個別實測。 | 部分已接線 |
| 文件 | `docs/FUTURE_WORK.md` 仍有「週六工程驗收」歷史用語 | 與已改為週日的每週完整比對排程用語不一致；應更新現行待辦文字，但不得改寫歷史 evidence。 | documentation drift |
| 教學版本 | 現行共同規則第③節強制官方最新穩定版本／MSRV／manifest／lock 及相容性核對 | Day30 文章仍記錄 resolved 版本或 MSRV 未確認，**不應事後宣稱已符合新增的 Gate 1 版本要求**。需版本化補查，舊提交證據保留。 | policy mismatch/pending |

## GitHub Actions：精確程式 SHA

同一主體 SHA（2026-10-23 02:51:29 UTC 建立）的六次 workflow run：

| Workflow | Run ID | 結果 |
| --- | ---: | --- |
| Rust AI Chat | [39891849782](https://github.com/williechen/rust_ai_chat/actions/runs/39891849782) | success |
| Rust Desktop Pet | [39891849805](https://github.com/williechen/rust_ai_chat/actions/runs/39891849805) | success |
| Rust Dashboard | [39891849841](https://github.com/williechen/rust_ai_chat/actions/runs/39891849841) | success |
| Persistence | [39891849758](https://github.com/williechen/rust_ai_chat/actions/runs/39891849758) | success |
| Test All | [39891849769](https://github.com/williechen/rust_ai_chat/actions/runs/39891849769) | success |
| Node | [39891849761](https://github.com/williechen/rust_ai_chat/actions/runs/39891849761) | **failure** |

Node run 的 `test-node` job（ID `120213214766`）在「Run Node.js tests」步驟失敗。本次未讀取完整 job logs，無法斷言根因。所有成功均僅覆蓋各 workflow 實際執行的測試，不能替代 Day30 尚未套用的新 Green。

## 待完成的完整比對與驗證 Gate

1. 依程式樹逐一核對所有 source／Cargo manifests、`Cargo.lock`、Rust toolchain、Leptos／Axum／SQLx／Tauri、前端 Node 相依及其官方最新穩定版；本文**尚未證明最新穩定版完整符合要求**。
2. 逐一比對 Chat 全部 endpoints、Room/WS/Session/ACL、SQLite migrations、SSR/hydrate 與實際使用者流程及相關測試。
3. 逐一比對 Pet 所有 IPC、持久化／migration、生活視窗、控制窗、重啟、每幀單一圖像及三 ID 原生 GUI 情境。
4. 逐一比對 Analytics PostgreSQL migrations、bootstrap/login/change-password、角色來源 ACL、後台 SSR、事件／query endpoints、完整 postgres-integration。
5. 重查 deployment／環境／密鑰載入政策／權限邊界與相容性，不接觸正式秘密。
6. 獨立執行（或取得與精確 SHA 對應且覆蓋完整的）測試、DB／HTTP／WS／WASM／GUI／Node 驗證；分析 Node 失敗 logs 並追蹤其他已知測試失敗。
7. 將發現逐項映射 `FUTURE_WORK`、Day、雙軌補教及去重清單。證據不足的項目維持 pending，不把本報告當作產品驗收。

## 狀態與不可變性

- 本報告是**首次全專案比對的部分結果／差異基準**，不冒充「已完整成功」。
- `williechen/rust_ai_chat` 未寫入；未新建 Day、未修改歷史文件、補教結案、`next_day`、`last_full_comparison`。
- 後續比對完成並有實際全面證據，才可依預期 branch SHA 條件同步成功時間；不能因報告提交成功自動更新。
