# 週六同步與正式部署建置

此環境使用 `/home/willie/Documents/rust_ai_chat` 作唯讀參考，
`/home/willie/Documents/copilot_use` 的 `teaching/rust-ai-chat-alignment` 作工程工作區。
使用者於 2026-10-16 重新要求正式部署建置；本次新增建置準備，尚無外部上線證據。
既有 v0.1.0、release 附件與歷史驗收保持固定。

## 週六執行

`weekly-sync.yml` 在 main 安裝後，每週六台北時間 08:00 執行，也可手動觸發。
GitHub 排程可能延遲；未推送與整合 main 前不會啟用。
完整 tracked tree 比對保存兩庫 SHA、檔案 Git object ID、mode、新增／刪除／內容差異及 dirty 狀態。
搬移候選可用相同 object ID 查核；差異本身不代表必要整合。

```sh
python3 scripts/weekly-compare.py --reference /home/willie/Documents/rust_ai_chat --output target/weekly-comparison.json
```

接著逐模組核對介面、契約、DB、流程與 UI，把差異分類為合理差異、必要銜接、架構違反、產品缺口與未核實。
週六 workflow 呼叫既有 Verify（Rust／DB／Docker／故障復原／三平台 GUI）。
不傳入 AI key、不新增付費呼叫；不自動覆寫 source、merge 或推進 learning-state。
完整語意核對、必要修正、精確 SHA 的功能 Gate 與提交完成後，才能更新完整比對日期與驗收。
此工具只提供比對證據，不能代替人工或代理的語意核對。

## 正式建置

安裝 Docker Engine 與 Compose v2；依 `.env.example` 建立私有 `.env`，
每個 token／密碼使用獨立隨機值。AI key 可空白。
提交並完成該 SHA 的 Verify 後執行：

```sh
bash scripts/build-production.sh
```

此命令驗證 Compose 並建置 Chat／Analytics；不啟動服務。
Desktop Pet 沿用 Verify 的三平台原生安裝包建置。
production overlay 開啟 Secure cookie，服務仍只監聽 host loopback。
正式入口需主機 TLS reverse proxy：Chat 3001 支援 WebSocket、Analytics 3100 支援 HTTP/2 gRPC 與 SSE（停用 buffering）。
容器間沿用既有私有 network h2c，`ALLOW_INTERNAL_HTTP_DEV` 只供容器間 Analytics 傳輸。
禁止把這個 listener 直接公開；TLS cookie 必須經 HTTPS 入口驗證登入。

尚需部署主機／網域、TLS 憑證、監控告警、排程備份與實際還原演練、
精確 image digest 留存、舊版映像及 DB 相容性回滾演練，才可進行上線驗收。
目前配置與建置入口不代表以上項目通過。
