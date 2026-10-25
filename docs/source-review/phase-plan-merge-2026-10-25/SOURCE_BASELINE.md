# 更新後本地程式基準與規劃取證

程式：`rust_ai_ring@5c713553daee4055738239f047d6c774fcc7a7a1`；文章：`copilot_use@4a3c43187e1ffd7cf4ba8d44cd4f8529f0d9d6d4`，均於 2026-10-25 從 GitHub main 確認後在本地讀取。程式工作樹維持乾淨。

已重新讀取 202 個 tracked 檔案計算雜湊；下列是重新定位的有限來源與接線證據，不是整庫逐行審查或功能驗收。本輪未執行 Cargo／DB／Browser／GUI／模型／CI run。

|所屬階段|重新讀取位置|本輪所見與限制|
|---|---|---|
|P1.0|[crates/telemetry/src/lib.rs:L1](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/crates/telemetry/src/lib.rs#L1)|檔案只有空白，不能當三程序共用初始化已實作。|
|P4|[crates/ai_core/src/lib.rs:L37](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/crates/ai_core/src/lib.rs#L37)|目前此共用模型檔有 MockChatModel；不能作真 Provider 接線證據。|
|P0/P1|[chat_apps/chat_server/src/lib.rs:L53](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/chat_apps/chat_server/src/lib.rs#L53)|正式 WS router 掛 Session middleware；有入口不等於逐房 ACL 完成。|
|P0/P1|[chat_apps/chat_server/src/ws.rs:L90](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/chat_apps/chat_server/src/ws.rs#L90)|既有 WS 定期與收／發前重驗 Session；本輪不重跑競態測試。|
|P0/P1|[chat_apps/chat_server/src/room_auth.rs:L16](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/chat_apps/chat_server/src/room_auth.rs#L16)|守門函式只見 active_member 後放行；此函式未核對 room_id 的管理／成員授權。|
|P0/P3|[chat_apps/chat_ui/src/lib.rs:L280](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/chat_apps/chat_ui/src/lib.rs#L280)|UI 仍明示歷史未接通；不能把「提示已做好」當歷史讀回完成。|
|P0/P3|[chat_apps/chat_application/src/repository.rs:L25](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/chat_apps/chat_application/src/repository.rs#L25)|MessageRepository 只定義 create；現有此 Port 沒有訊息歷史讀取方法。|
|P0/P2|[pet_apps/pet_server/src/lib.rs:L192](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/pet_apps/pet_server/src/lib.rs#L192)|setup 已接未回籠清單迴圈；舊「僅恢復最後選擇」不是此 SHA 現況。|
|P0/P2|[pet_apps/pet_persistence/src/lib.rs:L35](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/pet_apps/pet_persistence/src/lib.rs#L35)|此查詢按未回籠旗標過濾；不等於需求中獨立預設資格已完成。|
|P0/P2|[pet_apps/pet_server/src/lib.rs:L170](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/pet_apps/pet_server/src/lib.rs#L170)|生活窗 Destroyed 路徑只清 Runtime；對照「直接關閉等同持久回籠」需求有差距。|
|P0|[pet_apps/pet_server/src/pet_windows.rs:L33](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/pet_apps/pet_server/src/pet_windows.rs#L33)|生活窗實際採 pet-UUID label。|
|P0|[pet_apps/pet_server/src/roam.rs:L12](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/pet_apps/pet_server/src/roam.rs#L12)|漫遊仍用固定 pet label；與動態生活窗 label 不同。|
|P0/P1|[analytics_apps/analytics_server/src/main.rs:L24](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/analytics_apps/analytics_server/src/main.rs#L24)|真正進程使用 password_login::build；不把 lib 裡未掛載的舊 helper 當正式入口。|
|P0/P1|[analytics_apps/analytics_server/src/lib.rs:L191](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/analytics_apps/analytics_server/src/lib.rs#L191)|精確類型查詢已改用人員 Session；來源 token 仍用在 ingest，不重做 Day36。|
|P0/P1|[analytics_apps/analytics_server/src/password_login.rs:L455](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/analytics_apps/analytics_server/src/password_login.rs#L455)|上述精確查詢 handler 確實掛在真正 build 的路由內。|
|P0/P11/P12|[.github/workflows/persistence.yml:L68](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/.github/workflows/persistence.yml#L68)|名為 analytics server 的步驟實際只選 analytics-persistence；不能據此追認 server PG targets。|

[完整檔案雜湊](source-inventory.json) · [精確段落](source-evidence.json)

教材先前引用的 runtime 與 CI 結果仍只屬其原 SHA／環境；不因此次 fetch 或程式 marker 存在而升格。P4 以後未提供完整 caller 稽核，狀態為待查核，不能一概寫未實作。
