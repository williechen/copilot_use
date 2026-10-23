# Analytics Day29｜Dashboard 事件來源切換的可測試白名單

## ① 學習大綱與現況

唯讀程式：`williechen/rust_ai_chat/main@143d6b3f5180b669860b1300286b89464c669434`。已核對 `apps/stats_server/src/dashboard_ui.rs` 的 Leptos `DashboardView` 已有 Chat／Pet 來源連結；`apps/stats_server/src/password_login.rs::dashboard` 目前從 `Query<DashboardSourceQuery>` 取 `source`，以 `web_chat`／`desktop_pet` 白名單判斷，呼叫 `pg_store::summary_for_source`（PostgreSQL 綁定參數），再由 `dashboard_ui::render_dashboard` 呈現事件總數。Day28 已有 SSR Dashboard，不重做。現有 UI 尚未標示哪個來源被選中；使用者無法從導覽直接辨識目前篩選條件。

本日最小切片：Dashboard URL `?source=desktop_pet` → Axum Query 白名單 → PostgreSQL `summary_for_source` → Leptos SSR 顯示**目前來源**及事件總數，並以 `aria-current="page"` 標記作用中的導覽連結。既有登入 Session、首次登入改密碼及 SQL 參數化不變。三案各 15 分鐘、彈性 15 分鐘，共 60 分鐘，整合測試另計。

## ② Red → Green → Refactor／FIFO

**Red：** 現有 `render_dashboard("Pet", 3)` 輸出雖有來源標籤，卻沒有 `aria-current="page"`；先加入下列失敗測試（放在 `apps/stats_server/src/dashboard_ui.rs` 末尾），確認選中狀態可被 DOM／輔助技術識別。以下測試在 Green 前預期失敗，**本次未執行**：

```rust
#[cfg(test)]
mod day29_tests {
    use super::*;

    #[test]
    fn active_source_is_marked_for_accessibility() {
        let html = render_dashboard("Pet", 3);
        assert!(html.contains("desktop_pet"));
        assert!(html.contains("aria-current=\"page\""));
        assert!(html.contains("Pet 已提交事件總數 3"));
    }
}
```

**Green：** 將 `apps/stats_server/src/dashboard_ui.rs` 的 `DashboardView` 導覽區塊替換成依 `source_label` 產生作用中狀態的 Leptos SSR 節點，保留原本頁面結構、登入登出表單與總數：

```rust
let chat_current = (source_label == "Chat").then_some("page");
let pet_current = (source_label == "Pet").then_some("page");
view! {
    <nav aria-label="事件來源">
        <a href="/dashboard?source=web_chat" aria-current=chat_current>
            "Chat 事件"
        </a>
        <a href="/dashboard?source=desktop_pet" aria-current=pet_current>
            "Pet 事件"
        </a>
    </nav>
}
```

此為**替換既有 `<nav>` 的片段**，不是完整替換 `DashboardView`；`chat_current` 與 `pet_current` 必須在原 `view!` 之前計算。Leptos `Option<&str>` 屬性行為需透過 SSR 測試確認：非作用中連結不應輸出 `aria-current`。不更動 `password_login::dashboard` 的來源白名單、Session 檢查、`summary_for_source` SQL 或路由。

**Refactor／FIFO：** 將「目前來源」顯示與 PostgreSQL 篩選使用相同的白名單映射，避免畫面和資料查詢不一致；若要擴增來源，應先加入後端白名單與測試，而非只增加 UI 連結。此切片不是角色 ACL、互動式篩選或資料庫權限隔離，這些仍屬待辦。Day25 以前不修改，歷史補教佇列不因本篇結案。

## ③ 套件版本

`rust_ai_chat` workspace `0.2.28`、edition `2024`；沿用既有 `leptos = "0.8"`、`axum = "0.8"`、`sqlx = "0.9"`，PostgreSQL 透過 `PgPool`。版本為 `Cargo.toml` manifest 約束，非 lockfile resolved 精確版本；未確認 MSRV 或本機工具鏈。本日不新增 crate、migration 或資料庫操作。

## ④ 驗證、預期與實際

| 操作／命令 | 預期 | 本次實際 |
| --- | --- | --- |
| `cargo test --locked -p stats_server active_source_is_marked_for_accessibility` | Pet 來源輸出作用中 aria-current | 未執行 |
| `cargo test --locked -p stats_server` | 原有登入、來源與聚合測試不退化 | 未執行 |
| `cargo check --locked -p stats_server` | Leptos SSR Option 屬性可編譯 | 未執行 |
| 已登入 `/dashboard?source=web_chat` | Chat 被標記，Pet 不被標記 | 未執行 |
| 已登入 `/dashboard?source=desktop_pet` | Pet 被標記，數值來自 PostgreSQL 綁定查詢 | 未執行 |
| `/dashboard?source=unknown` | HTTP 400，白名單不退化 | 未執行 |
| 未登入 Dashboard | 重導登入，不洩漏資料 | 未執行 |

CI：未取得 `143d6b3f5180b669860b1300286b89464c669434` 對應 Actions workflow/run/job 的完成證據；不得宣稱測試或 CI 通過。未修改 `rust_ai_chat`、未連線 PostgreSQL、未執行程式測試、未部署或完成產品功能驗收。本篇僅代表 GitHub 教學文件交付。
