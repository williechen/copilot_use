# 2026-10-20 Day 23 最新來源核對與續接證據

狀態：pending / source-contract-audit；**不是已發布 Day23，也不是 Green 驗收**。只作續接證據；不得藉此推進 next_day、清除 makeup_queue、覆寫 Day22 封存文章，或更新 last_full_comparison。

核對來源（Asia/Taipei 2026-10-20 晚間）：
- rust_ai_chat/main: `5bcef70219b480a7999fdd0fbed5384d8d28d9be`（唯讀）
- copilot_use teaching/rust-ai-chat-alignment（寫入本證據前）：`3fb9072366caa1ed0b2848709efea6e0d663657f`
- docs/learning-state.json blob: `5a50543f6c50ef09a815016ed50745e58d98de1c`; next_day chat/pet/analytics 各 23；原有 makeup_queue 不變。
- docs/COMMON_LEARNING_RULES.md blob: `4ca972ac43e6b47a08e7a9b70e6382bb3590258d`; daily-tdd-minimum-v1 與 2026-10-20 Day22 以前封存條款。

## Chat Day23：登入之前的現有契約核對
- `apps/server/src/lib.rs` 已將 `signup::routes(db.clone())` merge 進 Axum Router，`crates/ui/src/lib.rs` 的 /signin 目前只有建立會員與回首頁連結，沒有帳密登入／Session。
- `apps/server/src/signup.rs` 已有 Argon2 0.6 單參數 `hash_password(password.as_bytes())`、POST /signup、SQLite INSERT。
- **真實 schema 差異**：`migrations/20261018073931_create_auth_user.sql` 的 `mbr_user.user_sexy TEXT NOT NULL` 與 `mbr_user.user_birthday TIMESTAMP NOT NULL` 無 DEFAULT，而 signup.rs 的 INSERT 並未提供兩欄。必須先核對既有/後續 migration 及在真實遷移資料庫上的結果，不能聲稱註冊可完成。已列出的後續 root migrations 沒有明顯更改這兩欄的 migration。不得直接修改舊 migration 或清空 DB。
- 既有 signup.rs 的短帳號檢查與既有 Day22 測試預期 422 不一致：對非空 1 字元帳號可能通過入口驗證，後續 DB 結果不同；相關補教 pending: `docs/teaching/pending/day23/chat-day22-corrections.md`。本項是舊 Day 勘誤，不能冒稱 Day23 新 Red。
- Day23 仍應以可登入使用者的最早必要前置契約為題，在確定 migration/註冊可行後補最小完整 Red/Green；維持後續 Session、房間 ACL 未完成的明示。

## Pet Day23
- `apps/desktop_pet/src/pet_store.rs` 已有 SQLite `PetStore::create` 與 Day22 fixture，不能重複新增。
- 後續需核對 Tauri command 註冊、前端 invoke、生活窗顯示、重啟載入。只見 store 或 `PetMachine::default()` 不足以宣稱使用者可建立第一隻寵物。
- `apps/desktop_pet/migrations/20261018091733_create_pet.sql` 定義 `pet_profile` 參照 `pet_owner`；`20261018095626_create_pet_owner.sql` 才建立 `pet_owner`。需核對 SQLite migration fresh/install/upgrade 及 foreign key policy。

## Analytics Day23
- `apps/stats_server/src/bootstrap.rs` 的 `initialize`、`apps/stats_server/src/bin/bootstrap_admin.rs` 的 CLI 已存在；不得重複實作 admin 初始化。下一最早缺失環節是帳密登入／持久化 Session；是否需先補密碼驗證介面，依最新 HTTP/PG 契約確定。
- 舊版固定 key 登入與人員帳密不可視為同一驗證流程。

## 套件與 CI
- source workspace 0.2.22／edition 2024；`argon2 = "0.6"`、`tokio = 1.54`、`sqlx = 0.9`。
- 最新來源 SHA 的 GitHub Actions Node run 39426649542：success（只限 Node workflow）。
- Persistence run 39426649469：failure；job test-persistence https://github.com/williechen/rust_ai_chat/actions/runs/39426649469/job/118640798490 失敗，尚未讀取完整 log，不能推斷根因。
- Rust AI Chat run 39426649577 當時 in_progress。以上 CI 不是 Day23 候選測試證據。
- 本次未執行新 Red/Green 測試、未發布 Notion Day23、未讀回新文章、未部署、未產品驗收。原因是當前取得的原始碼契約存在註冊 schema 不一致，且尚無可供發布的完整 Day23 候選；不是 Notion 或 GitHub 工具報錯。

## 確切續接點
1. 重新取兩庫 SHA、state blob、規則及 Notion Day23 頁，避免重複頁／分支競態。
2. Chat：檢查完整 migration/實際註冊測試和 CI 原因，產生與既有檔案相容的最小登入前置修補及 Day23 新 Red/完整 Green；追加 FIFO 的 Day22 更正。
3. Notion 發布 Chat Day23 四節並全文讀回；再以 latest blob SHA 條件更新 chat 23→24，保留 makeup_queue 其他待補項；同步 manifest/鏡像並讀回。再按同法 Pet、Analytics。
