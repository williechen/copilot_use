# Day22 封存／最新程式與套件核對

核對時間：2026-10-20 Asia/Taipei。來源 main `cabacb75e946b03c67c455ce13a7a4ea85652f8e`，workspace 0.2.22；原教學候選基底 da5155265bf66ef9d495fd38913ad6cb93e1bac8。教學分支讀取基底 fd1efa18942406879d976632f87d48220e8abafc。

## 封存

三篇既有 Notion 頁只在標題加【已封存】，全文讀回確認正文逐字未變。封存的是本次讀到、已含既有版本勘誤的目前正文；其第3節已與前次 GitHub 鏡像不同。兩種版本分開保存於 archive/day-22/notion/ 與 github/，不倒退或覆寫任一原文。前次通過紀錄仍屬 43d445e 候選，不能挪作新版 main 成功證據。

本輪不發布 Day23、不遞增 next_day、不變更 last_full_comparison 或排程。下一 Day 在第2節的新 Red／Green／Refactor 後加「舊 Day 更正補充（程式／套件差異）」，每專案一個 Day22 單元，先讀最新來源，依 FIFO 處理，完整發布讀回才結案。

## 最新程式驗證

| 範圍 | 命令（來源目錄的 Cargo.toml） | 實際結果 |
|---|---|---|
| Chat | cargo test --locked -p server --features ssr | 13 passed／1 failed；signup 測試在短帳號時收到 409，預期 422；執行到此 target 後停止，未跑後續 target |
| Pet | cargo test --locked -p desktop-pet | 42 passed／0 failed／0 ignored，24 個 library＋18 個 integration |
| Analytics | 專用 DATABASE_URL 與 ANALYTICS_TEST_DATABASE_URL；cargo test --locked -p stats-server --features postgres-integration -- --include-ignored | 26 passed／0 failed／0 ignored；PostgreSQL 18.6，包含原先忽略的登入／查詢／登出 DB 測試，但它仍是舊 key 登入 |
| 前端 | cargo check --locked -p desktop-pet-frontend -p ui --features ui/hydrate --target wasm32-unknown-unknown | 通過，保留既有警告，不代表瀏覽器／GUI 驗收 |
| migration | 獨立 Rust SQLx probe；舊／新 migrator 只接專用 SQLite fixtures | 最新 fresh Chat／Pet 都通過；舊庫升級分別在 20261018094242、20261018091733 因 checksum 修改中止；sentinel 資料保留 |
| Chat 修正候選 | 同版本隔離 correction 目錄 cargo test --locked -p server --features ssr | 19 passed／0 failed／0 ignored，15 library＋4 integration；僅供下一 Day 補教，未套用 main |

所有命令、錯誤與完整輸出在同目錄 .log。新舊 migrate probe 是專用重現工具，不修改來源 migration、不連正式 DB。臨時 PG 已停止。未執行：真瀏覽器／Tauri GUI、服務重啟載入、互動 CLI、MSRV／跨平台、產品整合或完整功能驗收。沒有執行付費 AI或讀取正式秘密。

## 已確認差異與下一 Day 修正

### Chat

已接通 /signup 與 mbr_user、Argon2 0.6；後端驗證現只檢查空值，短帳號測試失敗。8-byte 密碼下界是目前 HTML 與 Analytics 的行為，與原教學 12 不同；隔離修正候選保留最新版 8-byte 政策、恢復其他伺服器欄位限制，不能只靠 HTML 驗證。新增測試用空 DB／不同欄位，避免重複 Email 的 409 掩蓋驗證漏洞。原 Day22 migration 檔名也與 main 不同，沿用現有 20261019221857_member_identity.sql，不再新增另一份相同索引。

原 20261018094242_create_ai_person.sql 被加入 ai_person_activity、has_use_ai，造成舊庫 checksum mismatch。後續須先核對實際 DB 的已套用歷史，再保留該歷史原 migration 並新增前向 migration；已套用修改版與舊版的 DB 不能以同一組還原檔直接通吃。此項未自動修補，不改 checksum、不刪庫、不宣稱相容已解決。

### Pet

控制台→既有 bridge 名稱 validateNewPetName→create_pet→同一 SQLite pool→提交讀回已在 main。生活窗仍共用 PetMachine::default()；逐寵物視窗身分／初始外觀／重啟自動載入未接通。原 20261018091733_create_pet.sql 增加 is_use_ai／has_use_ai，舊庫 checksum mismatch 已重現；不是新增一份 create_pet migration。後續與 Chat 相同先核對 DB 歷史，再用前向 migration，不覆寫封存 Day22。

### Analytics

bootstrap.rs／bootstrap_admin CLI 已在 main；Argon2 0.6 的單參數 hash_password 會自動生成隨機鹽，不能誤判成無鹽，也不重加 0.5 的 SaltString／rand_core 0.6 呼叫。官方 API：[PasswordHasher 0.6](https://docs.rs/argon2/0.6.0/argon2/trait.PasswordHasher.html)。0.6 API 與 PG 初始化測試已在本輪通過；8-byte 下界與舊教學 12 的差異須在下一 Day 明示。is_init_pd=true 只保存標記，現有 key 登入尚未讀 uac_user，首次改密碼／新帳密 Session／角色授權仍未接通。

## 官方套件版本

本輪只查版本，沒有 cargo update／更改任一來源 lock。共核對 28 個外部直接依賴（含 workspace／build／dev／WASM）。表中 latest 指官方 registry 最新非 prerelease 穩定版；舊的傳遞依賴可與新主版本共存，不因此宣稱直接依賴過期。

| 套件 | lock 中的版本（可含傳遞依賴） | 官方最新穩定版 |
|---|---|---|
| any_spawner | 0.3.0 | 0.3.0 |
| argon2 | 0.6.0 | 0.6.0 |
| async-trait | 0.1.92 | 0.1.92 |
| axum | 0.8.9 | 0.8.9 |
| dotenv | 0.15.0 | 0.15.0 |
| gloo-timers | 0.4.0 | 0.4.0 |
| js-sys | 0.3.106 | 0.3.106 |
| leptos | 0.8.22 | 0.8.22 |
| leptos_axum | 0.8.10 | 0.8.10 |
| leptos_config | 0.8.10 | 0.8.10 |
| leptos_router | 0.8.17 | 0.8.17 |
| libc | 0.2.191 | 0.2.191 |
| rpassword | 7.5.4 | 7.5.4 |
| serde | 1.0.229 | 1.0.229 |
| serde-wasm-bindgen | 0.6.5 | 0.6.5 |
| serde_json | 1.0.154 | 1.0.154 |
| sha2 | 0.10.9, 0.11.1 | 0.11.1 |
| sqlx | 0.9.0 | 0.9.0 |
| tauri | 2.12.4 | 2.12.4 |
| tauri-build | 2.7.3 | 2.7.3 |
| thiserror | 1.0.69, 2.0.21 | 2.0.21 |
| tokio | 1.54.0 | 1.54.0 |
| tower | 0.5.3 | 0.5.3 |
| tracing | 0.1.44 | 0.1.44 |
| uuid | 1.28.0 | 1.28.0 |
| wasm-bindgen | 0.2.129 | 0.2.129 |
| wasm-bindgen-futures | 0.4.79 | 0.4.79 |
| web-sys | 0.3.106 | 0.3.106 |

28 項的直接版本均已包含最新穩定版，沒有額外待升級版本；與 Day22 舊候選相比，主要為 Argon2 0.5.3→0.6.0、Tokio 1.53.3→1.54.0，其他 lock 差異見 packages.json。官方查詢 URL／時間、manifest 要求與 Rust 版本要求逐項保存於 packages.json。SQLx 0.9.0 宣告 rust-version 1.94.0，Argon2／Tokio 1.85；本輪用 rustc 1.99.1，來源只指定 stable，未宣告或測試專案 MSRV。

## CI 與證據邊界

- [Node #39404616507](https://github.com/williechen/rust_ai_chat/actions/runs/39404616507)：success；cabacb75e946b03c67c455ce13a7a4ea85652f8e；2026-10-20T09:58:51Z 至 2026-10-20T09:59:01Z。
- [Test All #39404616558](https://github.com/williechen/rust_ai_chat/actions/runs/39404616558)：failure；cabacb75e946b03c67c455ce13a7a4ea85652f8e；2026-10-20T09:58:51Z 至 2026-10-20T10:01:06Z。
- [Rust Dashboard #39404616426](https://github.com/williechen/rust_ai_chat/actions/runs/39404616426)：success；cabacb75e946b03c67c455ce13a7a4ea85652f8e；2026-10-20T09:58:51Z 至 2026-10-20T10:01:39Z。
- [Persistence #39404616431](https://github.com/williechen/rust_ai_chat/actions/runs/39404616431)：success；cabacb75e946b03c67c455ce13a7a4ea85652f8e；2026-10-20T09:58:51Z 至 2026-10-20T10:02:33Z。
- [Rust Desktop Pet #39404616349](https://github.com/williechen/rust_ai_chat/actions/runs/39404616349)：success；cabacb75e946b03c67c455ce13a7a4ea85652f8e；2026-10-20T09:58:51Z 至 2026-10-20T10:09:37Z。
- [Rust AI Chat #39404616488](https://github.com/williechen/rust_ai_chat/actions/runs/39404616488)：success；cabacb75e946b03c67c455ce13a7a4ea85652f8e；2026-10-20T09:58:51Z 至 2026-10-20T10:12:38Z。

Test All 的 test-server job 118567423767 失敗；其餘 job 狀態與時間見 ci.json。CI 成功不能代替未覆蓋的 PG／GUI／業務驗收。copilot_use 產品來源仍為教學分支 fd1efa1（相對 b3db92f 的 apps／crates／Cargo 程式未變）；本輪未重跑其產品驗收，也不把參考庫測試當成產品通過。
