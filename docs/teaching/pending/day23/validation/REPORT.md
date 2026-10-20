# Day23 Chat 本地更新與候選查證

查證時間（UTC）：2026-10-20T13:29:39.259633+00:00。本輪範圍為本地更新、候選修正與驗證；未正式發布。

## 來源及本地保護

| Repo | 本地位置 | 分支 | 精確 SHA |
|---|---|---|---|
| rust_ai_chat | /home/willie/Documents/rust_ai_chat | main | f38360cd537daf6dc20e1d3868221bb56390cc7c |
| copilot_use | /home/willie/Documents/copilot_use | teaching/rust-ai-chat-alignment | fc4a72825dac985e7d0a8e8e72edb7848349caa0 |

兩庫 origin 均為 git@github.com:williechen/{repo}.git。操作前 rust_ai_chat 為 main，HEAD da5155265bf66ef9d495fd38913ad6cb93e1bac8，有 README.md 修改及未追蹤 docs/；與上游8個提交的 tracked 更新不重疊，fetch後 merge --ff-only 成功。既有 README 差異仍為43行新增／227行刪除，docs/仍未追蹤，沒有stash、reset或覆寫。copilot_use原 main乾淨，HEAD 0bf1d460ea8f20dfaa3bcdaf14b7a895d1b3aacd，fetch後新建追蹤指定遠端教學分支；產品 main 不變。結束前 ls-remote 確認兩個遠端 SHA 與上表相同。

測試程式僅放本聊天 work/source 的精確 git archive 隔離副本；共用原 repo ignored target 作編譯快取，未寫產品原始碼、未提交產品修改。教學修改只在pending/day23及本目錄，沒有commit或push。

## 規則、封存與需求核對

已讀 COMMON_LEARNING_RULES、learning-state、P9_TARGET、FUTURE_WORK、SYSTEM_ARCHITECTURE、UI_ALIGNMENT_REQUIREMENTS、current/manifest，及 Chat Day22原稿／封存／補教候選。規則 daily-tdd-minimum-v1；GitHub規則blob `4ca972ac43e6b47a08e7a9b70e6382bb3590258d`。

Notion共同總頁 3f2e51d4509381ddb8a8ce7cb4cd3e60（last edited 2026-10-20T12:32:57.531Z）、Chat入口 3f3e51d45093807da638c336e857543a（12:33:02.658Z）及 Chat Day22 3fee51d45093814385cdcdc9e693789a（11:54:10.262Z）已實讀；Day22仍標已封存，時間與archive manifest相符。Pet／Analytics入口亦已唯讀取得，未進行其候選開發。

Notion與GitHub鏡像均要求Day22含以前封存、下一Day的新Red／Green／Refactor後才補教、FIFO每篇一單元、發布全文讀回前不加next_day。三案next_day仍23，makeup_queue／last_full_comparison／manifest未修改，無Notion寫入、無排程操作。

## 修正的問題

1. 舊註冊候選引用5bcef702的NOT NULL假設已過時；最新main已nullable。刪除不再必要的DROP TABLE／RENAME候選，避免不必要重建與歷史資料／外鍵風險。
2. 登入候選兩處`format!("{:x}", Sha256::digest(...))`在SHA2 0.11.1得到E0277，改用逐byte hex helper；提供實際完整程式及明確Red imports。
3. 新增sha2後先在隔離副本解析lock；server只新增sha2 0.11.1直接依賴，來源lock不變。精確版本與crates.io官方穩定版核對留packages-latest.json。
4. 用完整migration runner與檔案SQLite驗證真實signup→login→account、pool關閉重開，補測過期／偽造／停用拒絕及舊庫checksum阻擋。原候選只測記憶體DB，不能證明持久Session。

## 實際驗證

| 命令／對象 | 結果 | 證據 |
|---|---|---|
| 原main cargo test --locked -p server --features ssr | library 8 passed／6 failed，之後target未執行 | baseline-clean.log |
| 原候選 cargo test --offline -p server day23_password_login | E0277，2處LowerHex型別錯誤 | login-candidate.log |
| 正確Red cargo test --locked -p server --lib day23_password_login | E0425，缺routes，預期符號缺失 | login-red.log |
| 修正候選 cargo test --locked -p server day23 -- --nocapture | 5 passed／0 failed | day23-final.log |
| 修正候選 cargo check --locked -p server --features ssr | PASS，有既有ui未使用符號warnings | ssr.log |
| 精確main archive git apply --check candidate.patch | PASS | patch-check.log |

原main失敗：signup短帳號409≠422；http房間4項及ws房間1項因rooms.desc NOT NULL。這些不是新Session修正造成，亦未在本輪改產品來掩蓋。候選針對性測試通過不代表全server全綠。

5項包含登入單元1、schema契約2、檔案SQLite／checksum2。覆蓋fresh註冊303與Argon2驗證、欄位NULL、歷史schema500、密碼大小寫帳號、cookie屬性、錯密碼／錯Origin不建Session、digest非明文、新pool重開、未登入／偽造／過期／停用拒絕、完整runner與foreign_key_check、舊SHA全套migration升級checksum拒絕。

## 同SHA CI（唯讀快照）

| Workflow | Status | Conclusion | Updated UTC |
|---|---|---|---|
| [Node](https://github.com/williechen/rust_ai_chat/actions/runs/39428806798) | completed | success | 2026-10-20T13:15:52Z |
| [Persistence](https://github.com/williechen/rust_ai_chat/actions/runs/39428806698) | completed | failure | 2026-10-20T13:18:09Z |
| [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39428806748) | completed | failure | 2026-10-20T13:19:11Z |
| [Rust Dashboard](https://github.com/williechen/rust_ai_chat/actions/runs/39428806718) | completed | success | 2026-10-20T13:17:59Z |
| [Rust Desktop Pet](https://github.com/williechen/rust_ai_chat/actions/runs/39428806744) | completed | success | 2026-10-20T13:26:13Z |
| [Rust AI Chat](https://github.com/williechen/rust_ai_chat/actions/runs/39428806840) | completed | success | 2026-10-20T13:26:52Z |

CI來源SHA均為上表main；本地候選不在CI內。jobs細節未逐項讀取，不以workflow結論宣稱特定功能已驗收。

## 可重現方式

在該main SHA的另一乾淨副本套用candidate.patch，**勿套用到正式DB或原始唯讀repo**：

```bash
git apply --check /absolute/path/to/candidate.patch
git apply /absolute/path/to/candidate.patch
cargo test --locked -p server day23 -- --nocapture
cargo check --locked -p server --features ssr
```

patch SHA256：`c1f89a4ebff298f09755eb02682104b4561dc6044a316b674214db9e511f78f7`。patch含Cargo/server/UI接線、完整signin.rs、additive Session migration、測試及歷史唯讀SQL fixtures；不含產品signup修正。歷史fixture只給隔離測試DB，不能當正式migration重送。

## 尚未完成

固定24小時Session與現行「活動續期、後台設定閒置逾時」仍有差異；個人管理、房間API／WS授權、選房導回、註冊獨立成功訊息頁、改密碼撤銷／登入等待未接通。舊庫checksum阻擋尚無通用安全修復，不能改checksum或刪庫。未驗真瀏覽器、OS程序重啟、MSRV／WASM／跨平台或實際使用者DB。Day22補教只留既有下一Day候選，未宣稱整合或結案。

此次完成本地更新及兩份候選查證修正；未做Notion發布讀回，因此不推進next_day，也不開始Pet或Analytics教學。
