# 各專案Refactor

各篇同篇獨立的Refactor段落只處理所屬專案，與Red／Green並列、按自身累積相依接續，不能只提供說明。以下入口保存範圍及歷史對照，實際程式／patch與測試調整隨版本化文章交付。

| 專案 | 自身責任範圍 | 本輪完整變更 |
| --- | --- | --- |
| [Chat](chat.md) | UI／HTTP／WS、用例、Domain、SQLite與本案測試接線 | Day38 r4抽出room_list_state，合併重複測試並承接舊helper有效斷言 |
| [Pet](pet.md) | 生命週期、Application／SQLite／Tauri、Runtime／鎖與本案測試 | Day38 r4把開窗補償流程與純測試移到Application，真鎖／IPC留server |
| [Analytics](analytics.md) | 帳號／Session、查詢／攝取、PG／UI與本案測試fixture | Day38 r4共用三組PG整合測試準備，原斷言與DB隔離保留 |

完整patch、候選測試範圍與未完成項見[各案進度](PROGRESS.md)。全庫workflow、共用crates、跨workspace及跨案契約另列[全專案獨立項目](../../engineering/README.md)，不納入各案Refactor，也不重複計功。

各案以完整程式／patch、caller與依賴接線、舊斷言承接、實際／未執行、回復與下一步交付；產品保持唯讀，使用者套用。不同故障／跨層契約保留；fixture不複製業務實作，不刪測試換全綠。核心60分鐘只計Red／Green，重構另計、不增加Day、不更新last_full_comparison。
