# 主體程式本地操作入口

`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_chat` 是唯一程式、測試、CI 與功能驗收主體。

操作前讀取 [rust_ai_chat/main README](https://github.com/williechen/rust_ai_chat/blob/main/README.md)、當時的 Cargo.toml／Cargo.lock 與各 app 原始碼；以精確 SHA 決定 package、feature、路由、埠號、migration 及資料目錄，不從文件庫的舊腳本推導。

本次靜態讀取 `f61ea1c59667f3c92c78e576e3c48038a8ce030a`：Chat 入口為 [apps/server/src/lib.rs](https://github.com/williechen/rust_ai_chat/blob/f61ea1c59667f3c92c78e576e3c48038a8ce030a/apps/server/src/lib.rs)，Analytics 為 [apps/stats_server/src/main.rs](https://github.com/williechen/rust_ai_chat/blob/f61ea1c59667f3c92c78e576e3c48038a8ce030a/apps/stats_server/src/main.rs)，Pet 為 [apps/desktop_pet](https://github.com/williechen/rust_ai_chat/blob/f61ea1c59667f3c92c78e576e3c48038a8ce030a/apps/desktop_pet/src/lib.rs)。這是來源定位，不是已啟動或驗收聲明；實際操作時仍重取最新程式。

舊庫 local-services.py、day9-*、Chat 3001、token Dashboard 與 copilot-desktop-pet 安裝步驟不作 rust_ai_chat 啟動指令；[原文保留](teaching/history/evidence/2026-10-23-repository-roles/before/docs/LOCAL_START.md)。不因本次文件同步安裝、啟動、停止服務或操作現有 DB。

[驗收方式](LOCAL_VALIDATION.md) · [操作索引](DEPLOYMENT.md) · [分工同步範圍](REPOSITORY_ROLE_TRANSITION.md)
