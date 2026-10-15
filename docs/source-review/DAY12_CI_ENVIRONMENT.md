# Day 12 GitHub CI 測試環境缺口

來源 `rust_ai_chat@a57e532c125f5f18d7fd2a250401efde0c9d2612`。本文件只提供診斷與待套用設定，不修改學習倉庫。

[`Test All / test-workspace` 失敗日誌](https://github.com/williechen/rust_ai_chat/actions/runs/38745289793/job/116457220577)指出 `tests/analytics/event_kind_tests.rs:47、68` 的環境變數讀取發生 `NotPresent`。兩處讀取的是 `ANALYTICS_DATABASE_URL`；其後也會讀取兩個來源 token 並連線 PostgreSQL。

本地以獨立 PostgreSQL 與合成 token 跑完整 workspace 通過。移除這三個變數後，在隔離副本重現相同兩個失敗；不是用真實 provider 或私人 `.env` 測試。

待使用者套用的最小設定：在 `.github/workflows/test.yml` 的既有 `test-workspace` job 加入以下 `services` 與 `env`，保留既有 checkout、toolchain、Linux 依賴及 Test workspace 步驟：

```yaml
    services:
      postgres:
        image: postgres:18
        env:
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: local-ci-only
          POSTGRES_DB: rust_ai_chat_test
        ports:
          - 5432:5432
        options: >-
          --health-cmd "pg_isready -U postgres -d rust_ai_chat_test"
          --health-interval 5s
          --health-timeout 5s
          --health-retries 10

    env:
      ANALYTICS_DATABASE_URL: postgres://postgres:local-ci-only@127.0.0.1:5432/rust_ai_chat_test
      ANALYTICS_CHAT_TOKEN: local-ci-chat-token
      ANALYTICS_PET_TOKEN: local-ci-pet-token
```

以上全部是可拋棄測試值，不使用 GitHub Secrets／真實來源憑證。現有無 token／無效 payload 測試不會成功寫入事件；若要新增有效攝取 HTTP 測試，須先對這個獨立 DB 套用 `apps/stats_server/migrations`。

`persistence.yml` 與 `test.yml` 是不同 workflow，job 的 env／service 不互通。只改 `persistence.yml` 不能修復 `Test All / test-workspace`。後續提交必須以新 SHA 重新查驗 CI，本文件的範例尚未在 GitHub 套用或驗證成功。
