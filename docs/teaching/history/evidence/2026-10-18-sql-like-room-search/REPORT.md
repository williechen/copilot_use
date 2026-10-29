# 名稱搜尋改用資料庫 SQL LIKE

記錄時間 2026-10-18T23:20:45.119287+08:00。依使用者更正，以後端SQL執行名稱包含查詢，取代先取全量再前端contains的候選方案。既有主線不改；本輪只驗證名稱搜尋相關切片，不宣稱全產品驗收。

|程式|版本／位置|實際驗證|
|---|---|---|
|rust_ai_chat main|fa0888ea1e1ea93415a4ba7f16972938969b1be3，唯讀|GET /api/rooms?q=Rust 技術交流仍回全部5筆；預期3的assert exit1，[Red](red.log)|
|rust_ai_chat SQL候選|4c1e5f710ade90a84dad83555a3e06ae6027ad77，本地codex/sql-like-room-search，增量基底2f60c73|相關套件34／0／0，嚴格Clippy、SSR／WASM建置、真HTTP12案、瀏覽器3筆／空結果／503保留與重試通過|
|copilot_use產品|78d5796200547bf9c1776b8dc82cbc0d842bf512，本地codex/p0-table-trial，延續948685f|相關套件22／0／0，嚴格Clippy、native／frontend建置、真HTTP5案，公開／成員權限與先SQL查詢再LIMIT通過|

兩個工作分支都未推送／合併main；本地沒有候選CI，不能套用舊main的CI success或先前129通過作為本次全量驗收。原fa0888e的完整重測與阻擋仍見[歷史報告](../2026-10-18-full-fa0888e/REPORT.md)。

## 實際接線與 SQL

參考庫：RoomList查詢按鈕 → URLSearchParams編碼q → GET /api/rooms?q=... → Query<RoomSearch> → RoomRepository::search_by_name → SqliteRoomRepository SQLx bind → JSON → rows直接顯示與真實ID連結。UI不呼叫正式的client filter；舊filter_rooms僅保留歷史純函式測試，不算SQL覆蓋。查詢失敗保留舊rows；重新整理清空q後由後端查全部。歷史未接通的SSR提示仍保留。

```sql
SELECT id, category_room_id, name
FROM rooms
WHERE name LIKE ?1 ESCAPE '\'
ORDER BY name, id
```

先trim，依序escape反斜線、%與_，外包%作為參數，例如`%Rust 技術交流%`；不把輸入拼進SQL。空白參數為%%，查全部；不相符回[]。API拒絕超過200 UTF-8 bytes或NUL。LIKE採SQLite英文ASCII不分大小寫語意，中文子字串可查，不宣稱完整Unicode case folding或FTS效能；前置%通常不能使用一般B-tree加速，此次未增無效索引。

產品：公開首頁／登入房間列表 → ?q → 現有HTTP Adapter SQL。公開WHERE保留NOT is_private；登入清單保留原Session／公開或成員資格條件，先將原OR授權條件加括號，再AND名稱LIKE，最後ORDER BY／LIMIT100。沒有為了搜尋繞過授權，沒有用先回前100筆再client filter的方式。UI有搜尋輸入及按鈕，直接顯示API rows，不過濾私房權限或重新contains。

## 命令與原始證據

```sh
# rust_ai_chat SQL候選
cargo test --locked -p chat-application -p persistence -p server -p ui --features ui/ssr
cargo clippy --locked -p chat-application -p persistence -p server -p ui --all-targets --features ui/ssr -- -D warnings
cargo leptos build --cargo-locked
# copilot_use
cargo test --locked -p day9-chat -p chat-ui -p persistence
cargo clippy --locked -p day9-chat -p chat-ui -p persistence --all-targets -- -D warnings
cargo build --locked -p day9-chat
NO_COLOR=true bash scripts/build-chat-ui.sh
```

- 參考庫：[tests](tests.log)、[Clippy](clippy.log)、[build](build.log)。SQLite測試涵蓋同名2＋延伸1、部分中文、trim／空白、大小寫、無匹配、字面萬用字元及引號／注入字串；HTTP驗NUL／過長回400。[真HTTP12案](http.json)、[script](reference-http.py)。新專用DB須先啟動測試服務，以--seed建立9筆fixture；--existing只重驗，不重複seed。
- 產品：[tests](copilot-tests.log)、[Clippy](copilot-clippy.log)、[build](copilot-build.log)、[frontend](copilot-frontend.log)。整合測試加入110個不相符公開房間，仍找到後方3個匹配房間；公開及一般使用者看不到私房，合法房主可查自己的私房；%／_／反斜線按字面查詢，注入字串不查全庫。真HTTP公開3／一般使用者3／管理者4、匿名受保護query401及%字面查詢通過。[真HTTP5案](copilot-http.json)。
- 真瀏覽器：[參考庫3筆AX](reference-browser.txt)、[空結果](reference-empty.txt)、[503保留3筆](reference-error.txt)、[重試](reference-retry.txt)、[含URL編碼q的網路紀錄](network.jsonl)；[產品3筆AX](copilot-browser.txt)。截圖存於本地封裝，皆為合成測試資料。

Ubuntu26.04.1、Rust／Cargo1.99.1，原Cargo.lock未改；僅web-sys既有UrlSearchParams feature新增。SQLite fixture皆在本任務work/，參考庫loopback3000／測試代理3002，產品3111，服務身分為合成fixture、Analytics指向本地未啟用端點、OTel停用。沒有正式秘密、付費AI或日常資料庫操作；不是PG／Pet／跨平台全量重測。

## 文章與可恢復程式

既有Chat Day18第2節重寫為完整SQL／Repository／Query DTO／URL helper／UI接線，保留既有action-log與SSR限制更正。Chat Day17歷史程式保留，只補目前適用性；Chat入口與共同總頁同步。四頁全文讀回，Rust／SQL區塊與預計內容核對，v1正文及子頁保留；next_day各19、makeup_queue與last_full_comparison未變。

參考庫[SQL增量patch](reference.patch)以2f60c73為基底；[完整候選patch](reference-complete.patch)以fa0888e為基底，另含先前Cookie／action-log ID／原生探針修改，不能誤當只改搜尋。產品[patch](copilot.patch)以948685f為基底。恢復commit的bundle在本地封裝。功能驗證後將參考庫日常執行檔重新建置為main，避免共享target留下候選。
