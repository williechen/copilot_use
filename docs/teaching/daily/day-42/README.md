# Day42 三專案 TDD 候選交付（2026-10-28）

來源：`williechen/rust_ai_ring@0b2288e4996b6d80b00e7a77e2cd023835306528`（產品唯讀）。規則來源：`copilot_use/docs/COMMON_LEARNING_RULES.md` blob `ba9e6cbd6763d245e60800a02c60888ba6215bd5`。三案原始 `next_day=42`，目前仍為 42；**未通過完整 Gate1／Gate2，故不推進 Day**。

| 專案 | 新功能 Red／Green | 同篇獨立 Refactor | 原文與完整 patch |
| --- | --- | --- | --- |
| Chat | 進房後從正式 `GET /api/rooms/{id}` 顯示真房名與失敗回饋 | 房間列表顯示狀態決策搬入 `room_list_state` | [文章](chat.md) · [Refactor](chat-day42-refactor.patch) · [Green](chat-day42-green.patch) |
| Pet | 控制台逐 UUID 讀取 SQLite 回籠狀態，失敗不冒稱成功 | 漫遊環境開關政策移入 `roam` | [文章](pet.md) · [Refactor](pet-day42-refactor.patch) · [Green](pet-day42-green.patch) |
| Analytics | 點選真實事件類型列，導向既有精確篩選 | URL percent encoder 移至 `dashboard_url` | [文章](analytics.md) · [Refactor](analytics-day42-refactor.patch) · [Green](analytics-day42-green.patch) |

[精確 commit／blob／SHA256 與靜態 patch 套用證據](delivery-verification.json) · [四份 Cargo.lock 與當日官方版本](versions.md)。

## 驗證邊界

已執行：讀取兩庫最新精確 SHA；四份 Cargo workspace manifest／lock 主要版本；三篇文章與六份 patch 分別 GitHub 提交、從各自精確 commit 全文讀回；以原始產品檔案逐行檢查六份 unified patch 的 hunk 上下文、數量與 Refactor→Green 順序，**全部靜態重建通過**。

未執行：真正 `git apply --check`、Cargo metadata/fmt/check/test、Node、SQLite/PG、SSR/WASM、Tauri native、瀏覽器／GUI、CI；全部遞迴套件 MSRV／本機 toolchain／native 相容未驗。靜態重建不是編譯或 Gate2。方案由使用者在隔離產品 checkout 套用，`rust_ai_ring` main 未修改。

## 下一步

先在產品來源精確 SHA 的隔離副本套用各案 Refactor→Green，依文章列出的測試命令實跑；修復任何編譯／行為差異並驗證官方版本相容，保留原文章與版本化勘誤。Gate1 完整核對後再依 expected `next_day=42` 條件推進至 43；不得因本索引存在就提前遞增。各案 Refactor 不抵算 [全專案 G-01／G-02](../../../engineering/README.md)，FIFO 與 `last_full_comparison` 保持原值。
