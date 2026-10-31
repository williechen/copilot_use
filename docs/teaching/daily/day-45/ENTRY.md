# Day45 ENTRY｜最新產品採用驗證

1. [Chat](chat-r2.md)：d784產品SSR44項通過；28同子樹WASM check通過。原Session／排除／本人action接線已採用。
2. [Pet](pet.md)：d784產品UI11及六pose DOM通過；使用者已修測試錯字，舊Pet補正patch不必重套。真Tauri驗收未完成。
3. [Analytics](analytics-r4.md)：d784 postgres-integration編譯失敗；完整補正候選68通過、1 legacy ignored，與產品原樣結果分開。

[來源／失敗與修正／清暫存](../../../source-review/day45-2026-10-31-d784f32/README.md) · [逐檔精確讀回](../../../source-review/day45-2026-10-31-d784f32/publication.json) · [套用界線](README.md)。

產品原Day45內容已採用，不要對目前產品重套64／f6完整patch。Analytics runtime-only修正不動migration；完整修正只適用空DB或f6 ledger，已跑28改寫版者不能自動回復checksum。next_day三案46，FIFO與last_full_comparison未更動，完整Gate2=false。
