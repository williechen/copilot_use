# 系統架構與責任

> **2026-10-30 最新增量：** 產品 `d2612fcd154d2132d850086d83a48578abbb07fa` 已安全快轉並在隔離副本重新驗證。[四 workspace／三案測試、清理與原始證據](source-review/local-validation-2026-10-30-day43/README.md)。一般測試／WASM／Node／隔離 PG 通過；Chat 原樣 SSR 缺 imports，最小補丁後 UI 21 項通過，補丁未寫入產品；新 SHA 遠端 test-ui 仍失敗。Pet 人工 GUI 保留 8b88269 原驗收範圍，本輪未重跑。next_day 三案 44、FIFO、last_full_comparison 不變，完整 Gate2 未結案。下方較早日期與 SHA 為歷史快照。

> 2026-10-28 固定產品來源 `c6708c42f21ac919e57a379848b4977d27a1c3d6`；[本輪原始命令與結果](source-review/local-validation-2026-10-28-c6708c4/README.md)。本輪只增量重測 fixture、metadata／fmt；其他來源實測維持原 SHA，完整驗收與版本 Gate 尚未完成。


依[共用規則](COMMON_LEARNING_RULES.md)，全專案架構／測試／CI重構每日漸進，與功能教學分開。本文只維護技術責任，各專案自己的重構見[各案入口](teaching/functional-refactoring/README.md)，跨案／共用依賴與全庫CI見[全專案獨立項目](engineering/README.md)；目錄與package見[SOURCE_LAYOUT](SOURCE_LAYOUT.md)。

<a id="functional-layering"></a>

## 分層與依賴方向

| 層／邊界 | 責任 | 禁止反向依賴或替代 |
| --- | --- | --- |
| Leptos UI | 元件／表單／狀態／事件，透過已確認API或IPC互動 | 不查DB；前台隱藏不等於授權 |
| Delivery：Axum／Tauri | HTTP／WS／IPC協定、輸入輸出、身份上下文、錯誤映射，呼叫用例並強制授權 | 不堆業務SQL，不把Tauri型別傳入Domain |
| Application | 使用案例、流程／授權決策、交易／補償意圖，定義Port及邊界DTO／錯誤 | 不依賴Axum、Leptos、Tauri、SQLx／Pool／Row |
| Domain | 規則、值物件、狀態機與不變條件 | 不依賴Application或IO driver |
| Persistence Adapter | SQLx、查詢／交易／migration／Row映射，實作內層Port | 不決定HTTP／UI結果，不讓內層依賴sqlx::FromRow DTO |
| 原生／外部 Adapter | 視窗／OS／檔案／模型等實作Port | 不隱藏另一套無caller的業務流程 |
| Composition root | 建pool與adapter、注入use case／state並組裝啟動 | 不能成為所有handler繞過用例的理由 |

依賴向內：Delivery→Application→Domain；Persistence／原生Adapter實作Application Port。沿各既有crate內按功能組織模組，不先加空crate；公開契約、features、mod／imports、相容入口與真caller須完整銜接。

## 來源所見與工程範圍

以下靜態來源為rust_ai_ring `9b89656f964b99e0cb89ba421855704a35c5ecb9`，不是本次新增產品驗收。

| 範圍 | 已有邊界 | 尚需工程銜接 |
| --- | --- | --- |
| Chat | Application Repository／部分用例、Domain、SQLx Adapter、Leptos UI | 剩餘Session／認證SQL與流程、正式HTTP／WS身份與ACL；每日UI修補可重用既有邊界 |
| Pet | Domain狀態機、Application名稱規則、Persistence部分flag／清單查詢 | Store SQL、生命週期用例／Port、Tauri／Runtime／鎖／補償接線 |
| Analytics | 正式Session／Leptos頁面、PG查詢與部分Persistence聚合 | Application／Domain／UI仍有範本，Session／查詢用例、DTO與Adapter邊界 |
| 共用crates | shared等既有共用契約 | shared依賴chat-domain等跨workspace邊界需按真caller盤點，不能假設已完全解耦 |

## 測試與CI

Domain測規則；Application以Port fake測流程／補償；Adapter測真SQL／交易或native契約；正式Router／IPC測授權、錯誤映射與接線；UI／瀏覽器測可見互動。相同情境跨層有不同責任時保留，同層同契約的重疊才整併，fixture不複製業務實作。

CI分開純內層、各案DB、正式整合、Pet原生及UI／產物；三案無實際相依不串接，必要失敗不可吞掉。Persistence job若選入pet-server就含Tauri原生建置，不能當純SQLite證據。[來源分析](source-review/2026-10-26-ci-layer-boundaries.md)保留其當次範圍；補原生套件不等於工程完成。

工程先完成斷言／caller承接，再清舊實作與獨占測試；以精確SHA實測記錄完成，目錄、re-export、metadata或cargo check都不足以取代DB／GUI驗收。當次功能的小整理併Green說明；各案Refactor只在該案同篇獨立段落交付自身的完整程式與測試調整；跨案共用crates、跨workspace依賴與全庫CI另立全專案項目，不塞入各案段落。
