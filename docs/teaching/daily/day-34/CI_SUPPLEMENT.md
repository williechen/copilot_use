# 交付後取得的精確來源 CI 補充

來源 54f5d80a5678e68e164d56e6d9f11c57cdff1f36。Chat 正文發布時 connector 的 PR-only workflow 與 combined statuses 為空；後續透過 GitHub Actions REST 取得 push runs，補充於此並保留原正文與雜湊。

- [Test All](https://github.com/williechen/rust_ai_ring/actions/runs/40101549290)：failure；test-analytics-server 的 Test analytics server 失敗。其他八個 job 成功。
- [Persistence](https://github.com/williechen/rust_ai_ring/actions/runs/40101549347)：failure；test-persistence 的 Test analytics server with PostgreSQL 失敗。
- Node、Cargo workspaces（四案 metadata）、Rust AI Chat（check/build）、Rust Desktop Pet（check/build）、Rust Analytics（check/build）均 success。

完整 run/job/step 原始狀態見 [source-snapshot.json](source-snapshot.json)。尚未讀取失敗完整日誌，根因未確認；check/build 成功不能代替 PostgreSQL 測試。這些結果只屬原程式來源，未包含 Day34 Green；功能驗收仍未完成。
