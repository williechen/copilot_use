# Analytics Day42 r2｜2026-10-29 主分支接線讀回

> 本篇是 [Day42 原文](analytics.md) 的不可變追加修訂，保留 r1、原始 Refactor／Green patch；不是 Day43。產品唯讀來源 [rust_ai_ring@c6708c4](https://github.com/williechen/rust_ai_ring/tree/c6708c42f21ac919e57a379848b4977d27a1c3d6)。

## ① 目標與正式呼叫鏈

原 Day42 將管理後台的事件類型明細變成可點擊的精確篩選。最新 `analytics_apps/analytics_server/src/dashboard_ui.rs` 第82–89行已見事件類型連結的 URL 組裝，將來源及 `event_type` 帶回正式 Dashboard 查詢；不能將連結存在視為角色／來源 ACL 已通過。

## Refactor｜僅 Analytics

[獨立 Refactor patch](analytics-day42-refactor.patch) 將百分比編碼抽出至 `analytics_apps/analytics_server/src/dashboard_url.rs`；最新第1行可見 `encode_query_component`，第15行起測試包含空白、中文、特殊字元與 script 字串。正式 `dashboard_ui.rs` 已 import 該函式；保留舊 Query／PG／Session 的獨有斷言，不涉及 Chat／Pet 或跨案 crates。

## ② Red／Green 原文承接

[原文完整 Red／Green](analytics.md)及 [Green patch](analytics-day42-green.patch) 保留。Red 鎖定事件類型明細可點擊、URL 編碼與精確篩選；Green 以既有 Leptos Dashboard 的每列 `href`、編碼函式及現有 HTTP Query/PG 路徑完成，不新增平行頁面。原 patch 僅適用 `0b2288e`，須 Refactor → Green；最新 main 已含接線，不再重套。

## ③ 版本

2026-10-29 Rust 官方最新穩定版為 [1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)；四 workspace version `0.2.42`。舊來源 Axum／Leptos／SQLx resolved 版本見 [Day42 版本表](versions.md)；最新 lock 已變，完整官方版本／MSRV／PostgreSQL 相容 Gate 未完成。

## ④ 驗證與下一步

本輪以 GitHub 精確 SHA 靜態讀回 `dashboard_url.rs` 與 `dashboard_ui.rs` 的正式 caller／測試；**未執行** Red、Cargo、隔離 PostgreSQL、HTTP、瀏覽器或 CI。2026-10-28 [同 SHA metadata／fmt](../../../source-review/local-validation-2026-10-28-c6708c4/README.md)不代表 PG／ACL 通過。下一步重跑 `postgres-integration`、管理員 Session、事件類型精確查詢與角色／來源 ACL，Gate1／Gate2 未完整通過，next_day.analytics 維持42。
