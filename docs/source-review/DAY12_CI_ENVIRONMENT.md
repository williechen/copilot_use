# Day 12 CI 測試環境：已修正

來源 `rust_ai_chat@d05438950b96e03c9c6c6f8f2c7e3573cc191a25`；使用者已在 `.github/workflows/test.yml` 的 `test-workspace` job 配置獨立 PostgreSQL、ANALYTICS_DATABASE_URL 與兩個不同的合成來源 token。[該工作已成功](https://github.com/williechen/rust_ai_chat/actions/runs/38765474606/job/116527415647)，同 SHA 6／6 workflows、18／18 checks 全部成功。

目前實際設定（既有 test-workspace job 內）：

```yaml
    services:
      postgres:
        image: postgres:latest
        env:
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: rust_ai_chat_test
        ports:
          - 5432:5432
        options: >-
          --health-cmd "pg_isready -U postgres -d rust_ai_chat_test"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    env:
      ANALYTICS_DATABASE_URL: postgres://postgres:postgres@localhost:5432/rust_ai_chat_test
      ANALYTICS_CHAT_TOKEN: local-ci-chat-token
      ANALYTICS_PET_TOKEN: local-ci-pet-token
      CHAT_DATABASE_URL: "sqlite::memory:"
      PET_DATABASE_URL: "sqlite::memory:"
```

這些是隔離 CI 測試值，不需要真實 provider 密鑰。現行無 token／無效 payload 測試只驗證拒絕路徑；新增有效 HTTP 攝取測試時，仍須對獨立 DB 套用 `apps/stats_server/migrations`。

`persistence.yml` 與 `test.yml` 是不同 workflow，job 的 env／service 不互通。先前 a57e532 的 test-workspace 因環境變數缺失而出現 NotPresent；最新設定已直接補在正確 job，舊失敗不再列為待辦。助手未修改 rust_ai_chat 的 workflow。
