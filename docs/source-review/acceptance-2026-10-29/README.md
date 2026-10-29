# 網頁排程、GUI／CI／舊 DB 驗收｜2026-10-29

產品固定來源 `8b882696c5548accb9d98a32b27db35e11109c74`；文件 parent `726eaa4fe5118f3bf803831b837e3b399c5062c5`。產品主庫唯讀；使用者在本輪再次指定只交付補丁與證據，不建立產品分支／PR、不合併 main。隔離副本新增的 probes 是驗收工具，不是第二套產品。

## 網頁排程已實際修正

已開啟既有雲端任務「三專案每日 TDD 教學」，只更新提示詞；儲存後刷新並全文比對一致，任務仍啟用，時間／頻率未改，未建立重複任務。[精確提示詞](schedule-prompt.txt) · [SHA256／讀回](schedule-readback.json)。

新提示詞分開 Gate1 文件與 Gate2 產品驗收，先確認可用工具，環境不足不單獨阻止合格教學發布；仍須修正真正程式／patch／文件缺陷，保存精確提交與未執行項，不重複推進 next_day。

## 真實 GUI

| 範圍 | 實測結果 | 界線 |
|---|---|---|
| Chat 原樣 | 初始房間載入成功；空搜尋後清除無法恢復清單 | 正式搜尋 API 以篩選結果覆蓋 rows，舊清除只重設 query；新增功能缺陷 |
| Chat 修補候選 | UI 21 項／Leptos SSR＋hydrate build 通過；真瀏覽器清除恢復清單、網路失敗保留 query／rows、重試成功 | 修補只在隔離副本；未推產品 |
| Analytics 原樣 | 升級後管理員登入、中文／& 精確篩選、切來源保留條件、空結果、清除、CSV 下載解析、登出後拒絕後台 | 初次改密碼 GUI、完整角色／來源 ACL 未驗收 |
| Pet | Trunk 前端與 native binary 建置通過，提供專用 SQLite 的啟動腳本 | 原生視窗不能由目前工具操控，人工 A/B 操作回報仍待完成 |

[逐情境結果](gui-scenarios.json)；本地 outputs 保存排程與 GUI 畫面，未將使用者的瀏覽器畫面或實際 DB 備份發布到 GitHub。

## CI：已真正重跑，仍有產品缺陷

同一精確 SHA 的 7 個 workflow 中，6 成功、Test All 失敗。[原 run](https://github.com/williechen/rust_ai_ring/actions/runs/40837663978)。connector 重跑先回 403（[原始回覆](ci-rerun-result.json)）；之後使用已登入的 GitHub 網頁選 Re-run failed jobs，實際啟動 attempt #2，完成後 test-ui 仍失敗。

兩次日誌均指向 day43_room_list_has_clear_search_control 在 matched Route 外呼叫 RoomList：[attempt #1](ci-ui-log.txt) · [attempt #2](ci-attempt2-log.txt) · [精確 job／attempt](ci-attempt2.json)。本地補修已通過，但未套入產品；不能將遠端標為全綠，也不要求擴張 integration 權限來掩蓋程式問題。

[可審閱的兩檔完整修補](chat-gui-ci-fix.patch)：RoomList 重用正式 refresh 路徑，成功才清除 query，失敗維持快照；SSR fixture 補 Routes／Route，原斷言保留。需由使用者套入產品後再依新 SHA 驗 CI；本輪保留唯讀要求。

## 舊 DB：真實備份副本與有資料的舊 schema

| 測試資料 | 結果 | 可主張範圍 |
|---|---|---|
| 本地 .local/chat.sqlite 唯讀 backup 副本 | 最新 Migrator 與重跑通過，使用者資料表摘要／筆數不變 | 既有 9 migration，房間／訊息／會員皆 0；不是有訊息舊庫的最終證明 |
| 本地 .local/pet.sqlite 唯讀 backup 副本 | 最新 Migrator 與重跑通過，1 筆既有寵物及資料摘要保持不變 | 既有 3 migration；未操作原庫 |
| Chat／Pet 前兩個 migration 的有資料舊 schema 副本 | 升至最新 migration，房名／訊息及寵物資料保留；重跑冪等；checksum 漂移拒絕且資料仍存 | 合成歷史 schema fixture，明確不是使用者正式備份 |
| Analytics 前兩個 migration 的有資料舊 schema 副本 | 升至 6 migration；既有密碼雜湊不變、2 筆事件保留、重跑／checksum 拒絕通過 | 隔離 PostgreSQL TCP 55482 |
| Analytics pg_dump／pg_restore | 真正還原至另一個隔離 DB；原 2 migration／2 事件可讀，再升級成功 | 實際使用者的舊 PG 備份尚未提供，仍待驗 |

[真實本地庫驗證](actual-local-db-verification.json) · [備份 metadata](actual-local-db-backups.json) · [舊 schema fixture 精確來源](old-schema-prefix.json)。實際 SQLite／PG 備份只留本地 work，不上傳；沒有修改正式 DB、環境秘密或舊 migration/checksum。

## 未結案與進度

Gate2 尚未全部通過：產品補修未套、遠端 CI 仍失敗、Pet 人工原生 GUI 與真實舊 PostgreSQL 備份未驗。next_day 三案 44、FIFO、last_full_comparison 不變；驗收補教不增加 Day。所有失敗、初次編輯語法錯誤與修正後結果分開保留，未以候選成功追認原樣產品成功。

## 使用者後續確認

使用者確認目前沒有 PostgreSQL 備份檔，因此實際舊 PG 庫升級維持「無備份可驗」；不是要求再提供本地環境才交付教學。使用者同意協助 Pet 原生操作，但尚無 A/B 查詢／出入籠／重啟的具體結果，仍待人工驗收。啟動腳本已修正終端 PATH，改用 Trunk 絕對路徑；原 trunk not found 記錄保留。

## Pet 人工回報範圍更正（2026-10-29）

使用者後續明確確認「僅完成腳本修正或啟動」，因此不將「已完成」解讀為 A/B 查詢、出籠隔離、回籠與重啟情境通過。SQLite 唯讀快照仍見 A/B 均回籠、未選取寵物；與初始狀態相容，不能證明中間操作。原生 GUI 繼續待驗，先前建置通過僅代表建置。[人工回報與快照界線](pet-human-clarification.json)。
