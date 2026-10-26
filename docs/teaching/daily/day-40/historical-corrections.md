# Day39／Day40歷史內容更正（2026-10-26）

更正採版本化正文與本頁對照，原發布檔案、commit／blob／驗證JSON保留其當時範圍；不倒推當時已有測試，不回寫Day25含以前封存文。

| 原敘述／問題 | 更正與證據 | 範圍 |
| --- | --- | --- |
| Day39仍未套用／只測Refactor | 前輪bb259aa已實際跑完整Green與最新來源，Chat57、Pet70＋Node6、Analytics52通過；[三篇r3](../day-39/chat-r3.md)與其原始log保留 | 結果僅bb259aa，不挪作Day40 |
| Day39 Analytics pg_store有public re-export | 最新Day39已是私有use，各caller直接引用Persistence；CSV回應需保留Session續期 | [Analytics r3](../day-39/analytics-r3.md) |
| Day40靜態hunk檢查等於可套用 | 原Chat Green／Analytics Refactor及Green標準apply失敗，因preimage多預期EOF空白行；原失敗log及修訂patch皆保留 | [修正紀錄](local-validation/patch-corrections.json) |
| Day40 Chat只新增端點不影響舊入口 | 最新碼誤刪/account，舊登入測試404≠401；候選保留並存，最新SHA修復patch另附 | [Chat r1](chat-r1.md) |
| Day40 Pet功能已全套用／舊斷言全保留 | 最新ff14ca6沒有真生活窗CloseRequested，且刪除3個舊測試；候選與最新碼分別驗證，補接patch另附 | [Pet r1](pet-r1.md) |
| 測試通過即產品完整驗收 | fmt／check／test／真SQLite／PG、GUI及CI分列，失敗與未跑項不隱藏 | [本次驗證](local-validation/README.md) |

先前本地r3的「未commit／push」及JSON表示其原驗證時點；本次才依使用者要求將歷史更正一併提交GitHub。三案next_day維持41，FIFO及last_full_comparison不變，全專案G-01／G-02不因本次驗證結案。

## 發布前補正：b0e06b7

ff14ca6後又出現b0e06b7，僅補Pet原生CloseRequested；因此初輪「未接」保留歷史，不能當最終狀態。Pet新SHA重跑69項通過，3項舊測試仍缺；恢復候選fmt及server60通過。Chat／Analytics／crates Git tree不變，舊失敗及PG結果仍屬ff14，不冒稱新SHA重跑。[刷新證據](local-validation/source-refresh.json)。

## 來源9c76797：先前兩個缺口已套用修復

最新 `9c767978b20bd56e1c2182dac2f37e6ae9b7d988` 補回/account並恢復三個Pet舊斷言。原ff／b0失敗與缺測描述保留當時SHA；現況不再寫仍未修復。三案fmt實際通過，本輪未跑check/test／GUI／CI。[來源對照](source-update-9c76797/README.md)。前輪候選結果不可改標成新SHA已跑，Day41與FIFO不變。
