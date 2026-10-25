# 教學與全專案工程銜接

## 每日功能教學

依[共用規則](COMMON_LEARNING_RULES.md)選一個可操作功能缺口，提供Red、完整Green、正式caller與直接相關測試；Refactor接續所屬專案整體工程，正文引用完整工作包與累積狀態。路徑、Cargo／feature、imports與命令依[現行目錄](SOURCE_LAYOUT.md)核對，不以API／helper通過冒稱畫面與整條流程已驗收。

核心60分鐘只計Red／Green，工程與其他作業另計。當日切片的小整理併Green說明，不以局部整理抵算Refactor。跨功能搬層、測試去重／遷移、共用fixture與CI重整不排入當日教材；需要工程相依時明列阻礙，由工程規劃追蹤，不刪測試、不以龐大重構擴充每日教學範圍。

## 每日漸進的全專案獨立工程

[工程整理入口](teaching/functional-refactoring/README.md)涵蓋三案、共用crates及跨workspace邊界。先讀三案各自進度，按整體架構／測試／CI依賴每天完成一個完整小工作包，接續前日未完成事項，不依當日教學切片重新選題，記錄於[工程進度](teaching/functional-refactoring/PROGRESS.md)。工程交付須含：

1. 依賴／正式caller／測試與CI現況，目標責任及工作包相依順序。
2. 必要完整程式、Port／DTO／錯誤、Adapter與組裝、公開介面相容處理；Cargo normal／build／dev／target依賴、features、module／import同步。
3. 舊測試→保留／合併／參數化／遷移／確認淘汰的映射；有效斷言與不同故障模式保留，fixture不複製業務實作。
4. CI的純內層、各案DB、正式HTTP／IPC、Pet原生與UI驗證分責任；三案無實際相依不串接，不能吞掉失敗。拆分時同步required checks與觸發條件。
5. 精確來源SHA、實際測試／CI／DB／GUI結果、未執行、風險與回復方式；工程文件完成不等於產品工程已完成。

## 文件與驗收

copilot_use維護需求、規則、工程文件、教學與證據；rust_ai_ring是產品／測試／CI主體，本次保持唯讀。工程工作與教學進度分開，不因工程規劃、搬移或驗收增加Day，不改FIFO或全面比對時間。新教學仍依獨立文章提交、精確commit全文讀回及條件同步規則。

保留已發布文章與原始commit／blob／SHA256。全專案重構不是固定搬移11個測試的舊計畫，也不是在copilot_use建立第二套產品。
