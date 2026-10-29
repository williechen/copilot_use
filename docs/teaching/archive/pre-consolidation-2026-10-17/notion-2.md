Here is the result of "fetch" for the Page with URL https://app.notion.com/p/3f3e51d450938067923ae5a424d30950 as of 2026-10-16T16:47:28.783Z:
<page url="https://app.notion.com/p/3f3e51d450938067923ae5a424d30950">
<ancestor-path>
<parent-page url="https://app.notion.com/p/3f2e51d4509381ddb8a8ce7cb4cd3e60" title="Rust AI 三專案｜共同學習規則與前進目標"/>
</ancestor-path>
<properties>
{"title":"桌面寵物"}
</properties>
<iconMetadata>null</iconMetadata>
<content>
## 1. 大綱
目錄／程式銜接以 rust_ai_chat 最新 main 為主，目標／前進項目以 copilot_use 實際產品為主，其餘遵循共同主頁。以下為未來學習銜接項目，產品驗收與學習套用分開。
## 2. 最終目標
Tauri／Leptos 多寵物：五欄設定與可選覆寫、獨立 TOML／素材／SQLite、AI 角色一致性素材與 200×200 等比動畫、記憶／學習／回想／複習／提醒、RAG／MCP／Agent、安全 FileOrganizer、備份／還原／可逆封存及 updater。
## 3. 後續缺口
- 不預排下一 Day；依最新 main 與已核實缺口採差異驅動、TDD Red → 最小 Green → Refactor → 驗證，選擇可實作的最小功能切片；必要測試／重構與阻塞修正補在原 Day 第 2 節，教學固定四節。原開發項目跨階段保留，功能完成後回填原階段證據；CI 通過不等於學習或正式驗收完成。
- 銜接 Profile／Application Port／Schedule 純層、SQLite／HTTP／OS runtime、Tauri／frontend IPC。
- 銜接多寵物獨立資料與五欄／選用外觀／品種／顏色／動畫；明確覆寫優先，生成素材存 manifest，不覆寫使用者設定。
- 銜接 AI 素材／200×200、記憶／複習／提醒、政策確認／回退、事件、三平台安裝與更新。
每次先核對最新 source，排除已套用項目；不把成品成功算成學習已完成。
## 4. CI/CD
測不合法 Profile 不到 Adapter、投影失敗非成功、非法評分不改排程、未確認不操作、non-root／symlink／TOCTOU／opaque ID／ACL／journal、receipt／Outbox、隔離／備份、動畫邊界、真實 GUI／IPC、installer／簽章 updater／冷啟動／資料保留。
歷史 CI 不累積於主頁，真正結果與來源分開保存。
## 5. 開發與執行
rust_ai_chat 只讀；copilot_use 對應 apps/desktop_pet、frontend、crates/pet_domain 與 pet_runtime。資料為可攜式 SQLite／每寵物檔案；原始圖像與顯示尺寸分開。五欄為名字、種類、喜好、個性、興趣；外觀／動畫細項選用。
[操作設定](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/DEPLOYMENT.md) · [未來清單](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/FUTURE_WORK.md) · [依當日差異選題](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/FUTURE_WORK.md)
<page url="https://app.notion.com/p/3fbe51d4509381e484f4eb9e1f98fc68">2026-10-16｜P0 Pet Day 14｜主動掃描授權目錄與摘要顯示</page>
<page url="https://app.notion.com/p/3fae51d45093814dbdabee6deac48554">2026-10-16｜P0 Pet Day 13｜重複預覽顯示群組大小與檔案數</page>
<page url="https://app.notion.com/p/3fae51d4509381d0a80ddbbaf251fb03">2026-10-15｜P0 Pet Day 12｜桌面漫遊明確啟用控制</page>
<page url="https://app.notion.com/p/3f9e51d450938152a147c931e3e945be">2026-10-15｜P0 Pet Day 11｜SHA256 重複預覽接入 Tauri 與 UI</page>
<page url="https://app.notion.com/p/3f9e51d4509381f7ab32d0dc8b389a86">2026-10-14｜P0 Pet Day 10｜Profile 驗證後才解析外觀</page>
<page url="https://app.notion.com/p/3f8e51d4509381c3b989f2365fd0c0af">2026-10-14｜P0 Pet Day 9｜Profile 外觀覆寫與預設值合併</page>
<page url="https://app.notion.com/p/3f8e51d4509381bf9da4c44750ffc726">2026-10-13｜正式補跑｜Pet｜成功事件契約測試</page>
<page url="https://app.notion.com/p/3f8e51d45093815fbec6ec85bc547bd6">2026-10-13｜手動補跑｜Pet｜既有 CleanupOperation 狀態機的重入防護</page>
</content>
</page>