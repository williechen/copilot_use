# 正式階段覆蓋、缺口與證據要求

分類已依使用者更正為 [P0–P12 十三階段 Roadmap](../../ROADMAP_P0_P12.md)；以下「未找到」限本輪 105 個閱讀版本的教材主體，並非產品未實作。

|階段|交付／邊界|閱讀版本主題數／相關數|缺口與下一個證據|
|---|---|---|---|
|P0 Foundation & MVP|三產品非 AI 使用者旅程、授權、持久化與重啟|86 / 9|按產品補精確 SHA、HTTP／Browser／DB／原生重啟的整條旅程證據。|
|P1.0 Common Logging Baseline|P1 首工作包；三程序共用結構化日誌、redaction、runtime smoke|0 / 0|本輪未找到以完整共用 Logging Baseline 為主體的 Day 文章；零散日誌提及不算完成。|
|P1 Security & Identity|Session、ACL、CSRF、rate limit，基礎安全仍是 P0 前置|0 / 36|有基礎登入／Session 文章；完整角色／來源／房間 ACL、撤銷競態及負向證據需另核。|
|P2 Reliability & Persistence|transaction、冪等、恢復、備份|8 / 10|有記憶體與 PG 冪等、Pet 補償切片；完整故障注入、跨程序恢復、備份還原證據需補。|
|P3 Real-time Communication|reconnect、presence、typing、history sync|2 / 0|有 RoomHub 與 Typing 切片；重連補訊、presence、歷史同步的整條教材與驗收未由此掃描建立。|
|P4 AI Integration|Responses、HTTP/SSE、structured output、function calling|0 / 0|未找到以真 Provider 完整接線為主體的 Day；AI 目標／模型宣告不作交付。|
|P5 AI Persona & Conversation|persona、context、turn-taking、cooldown/budget|0 / 0|Profile／外觀設定為前置，不能等同多角色有界會話。|
|P6 Memory & RAG|persistent memory、embeddings、retriever、授權|0 / 0|未找到以授權跨重啟記憶／RAG 為主體的 Day；保留文件缺口，實作未判定。|
|P7 MCP & Tool Integration|MCP client、registry、permission、audit|0 / 0|FileOrganizer IPC／安全掃描不等同 MCP；需真協議與工具權限教材／驗證。|
|P8 Agent Runtime|有界執行、取消、checkpoint、error recovery|0 / 0|PetMachine／Runtime 名稱不等同新 Agent Runtime；需按正式交付重建證據鏈。|
|P9 Learning & Teaching Agent|learner state、active recall、spaced repetition|0 / 0|舊 P9_TARGET 與教學管理 JSON 不作新 P9 的產品完成證據。|
|P10 Observability & Operations|distributed trace、metrics、propagation、export、alert|0 / 0|產品 Analytics 事件統計不等同 OTel；P1.0 亦不能推遲至此。|
|P11 Production Readiness & Deployment|部署、staging、備份恢復、營運就緒|0 / 0|需部署與故障恢復證據；本輪未執行。|
|P12 Release & Lifecycle|release、installer、updater、upgrade、rollback|0 / 0|需精確 tag／產物、真平台安裝更新與保資料回退證據；與 P11 分開驗。|

## 重複與遞進判讀

|文章群|本輪判讀|處理|
|---|---|---|
|Chat 9→10→11|房間存在、ID 一致性、WS caller 接線是不同切片|保留；回到 chat-history／chat-auth 原項，不另排新 Day|
|Pet 2→3；11→13→14→15|bridge、安全契約、分組 UI、一般掃描、確認 UI 各有不同目的|同領域延伸非相同功能完成；P0 Gate 排除 FileOrganizer 強制要求|
|Analytics 5→7→10→11→12|記憶體契約、測試整理、接線、PG 冪等、HTTP commit ACK 遞進|不得把記憶體去重視為持久冪等；舊 501 只留歷史|
|Pet 32→33|右鍵入口與生活窗按鈕入口不同|選取自身的共同規則可集中說明；兩個入口各自需證據|
|Chat34／Analytics35／Day36 修訂群|同日閱讀版本與較早修訂不同|保留全部；按各文件修訂標示追溯，不以新檔數增加學習進度|
|Day12–15 archive、Day13 merge、Day19–26 GitHub／Notion|同日鏡像／封存變體，部分文字不完全相同|全列實體行；雜湊相同才稱完全重複；不同者保留差異|

[EXACT_DUPLICATES.json](EXACT_DUPLICATES.json) 是全 docs Markdown 的完全重複群組；未涵蓋二進位／壓縮檔內部。證據內可能另含文章副本，列入全檔清單但沒有逐份提升為獨立文章。

## 過時與證據不足

1. P0–P12 是正確十三階段；舊庫名、舊 apps 路徑、Phase2／舊 P9 依歷史語境解讀。使用新階段附表，不改原正文，也不把舊 P9 指標算入新 P9。
2. 舊 501／來源 token 查詢設計須與新攝取成功 ACK、人員 Session 查詢分離規劃對照；這是文章設計與規劃差異，未據此宣稱最新產品有缺陷。
3. `render_to_string` 等字串只標版本適配候選。本輪沒有重新編譯文章範例，不能斷言所有命中皆錯，也不宣稱範例原樣可用。
4. Day1 三產品原文在更新後 tracked docs 仍缺。舊索引與 Notion 連結不能補成原文已讀。
5. 產品 SHA／測試／CI／E2E 需另建同 SHA 證據鏈；本輪只有文章正文與本地文獻掃描，完整產品完成度仍未知。
6. 全 docs 逐檔機器讀取與段落對照已完成；每個程式碼區塊逐行人工語義審查、所有外部證據連結內容及壓縮證據內文未完整重驗，明列未完成。
7. 既有索引自述 105 閱讀版本／111 修訂；本輪檔案系統另找到 Pet Day24 r1 等封存實體，故本輪實體檔分母較大。這是保存口徑差異，沒有沿用舊索引當作新查核。

本次同步最新程式只補有限 caller／接線對照，詳 [來源基準](../phase-plan-merge-2026-10-25/SOURCE_BASELINE.md)；文章中的歷史結果仍不升格為本次測試。
