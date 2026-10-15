# 現行教學文章同步

本目錄保存最新既有 Notion Day 12 的四節教學鏡像，完整保留原文與程式區塊；來源、頁面、內容身分見 [manifest.json](manifest.json)。下一 Day 編號維持 learning-state.json 的13，不因同步重複發布或前移。

- [Chat：WebSocket 房間 Typing 即時通知](CHAT.md)
- [Pet：桌面漫遊明確啟用控制](PET.md)
- [Analytics：HTTP 來源驗證與 PostgreSQL 持久化接線](ANALYTICS.md)

三篇的來源均為 rust_ai_chat `5a0a8cee0d1fb4d0cd248869d7b24ebb1d653399`；文中新增功能是供使用者套用的教學，未寫入學習庫。最新來源 `bded382` 的本地正式驗證與同SHA GitHub CI皆通過；原教學撰寫基底仍保留5a0a8ce。本次來源驗證與教學新功能實作／驗收分開，結果見 [參考庫驗證](../../source-review/rust-ai-chat-validation.json)。
