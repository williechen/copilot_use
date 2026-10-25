# 每日工程工作包：隔離三案資料與 Pet 原生 CI

來源：rust_ai_ring `9b89656f964b99e0cb89ba421855704a35c5ecb9`，`.github/workflows/persistence.yml` blob `5c866333d630070a4941e2f8ad36828df9852d31`。這是全專案每日漸進重構的一包，獨立於 Day38 功能教學、另計時間、不推進 Day。

現行同一 job 順序測 Chat→Pet server/native→Analytics PostgreSQL；前段失敗會使後段不執行。此包只重整這個 workflow 的責任與失敗隔離；不宣稱 Application／Domain、fixture 或全部 CI 已完成重構。最新來源已補 Linux 套件，不代表責任已拆開。

[完整 patch](ci-isolation.patch)由使用者在產品來源自行套用。若來源漂移先比對、重做套用檢查，不覆寫現有變更。完整取代 YAML 如下：

```yaml
name: Persistence
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
permissions:
  contents: read
concurrency:
  group: persistence-${{ github.ref }}
  cancel-in-progress: true
env:
  CARGO_TERM_COLOR: always
jobs:
  chat-sqlite:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Chat SQLite adapter tests
        working-directory: chat_apps
        env:
          DATABASE_URL: "sqlite::memory:"
        run: cargo test --locked -p chat-persistence

  pet-sqlite:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Pet SQLite adapter tests (no Tauri)
        working-directory: pet_apps
        env:
          DATABASE_URL: "sqlite::memory:"
        run: cargo test --locked -p pet-persistence

  pet-native:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Tauri native dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y build-essential pkg-config curl wget file \
            libssl-dev libxdo-dev librsvg2-dev libwebkit2gtk-4.1-dev \
            libayatana-appindicator3-dev
      - uses: dtolnay/rust-toolchain@stable
      - name: Pet native/server tests (not GUI acceptance)
        working-directory: pet_apps
        env:
          DATABASE_URL: "sqlite::memory:"
        run: cargo test --locked -p pet-server

  analytics-postgres:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:18
        env:
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: analytics_ci
        ports: ["5432:5432"]
        options: >-
          --health-cmd "pg_isready -U postgres -d analytics_ci"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Analytics SQLx adapter + real Router integration
        working-directory: analytics_apps
        env:
          DATABASE_URL: postgres://postgres:postgres@localhost:5432/analytics_ci
          ANALYTICS_DATABASE_URL: postgres://postgres:postgres@localhost:5432/analytics_ci
          ANALYTICS_CHAT_TOKEN: local-ci-chat-token
          ANALYTICS_PET_TOKEN: local-ci-pet-token
        run: cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration

```

| 舊命令／斷言 | 新執行入口 | 保留範圍 |
| --- | --- | --- |
| chat-persistence | chat-sqlite | SQLite adapter 原測試 |
| pet-persistence | pet-sqlite | SQLite adapter 原測試 |
| pet-server | pet-native | 原生編譯與 server 原測試，不能當 GUI 驗收 |
| analytics-server + analytics-persistence，postgres-integration | analytics-postgres | SQLx 真 PG、正式 Router、原 persistence 測試 |

四個 job 不互相 needs，無 continue-on-error；其中一案失敗仍使 workflow 失敗，但不取消其他案。原 workflow 的 push/PR、只讀權限及分支 concurrency 保留。同層同契約去重需後續完整斷言映射，本包不刪任何測試。

適用條件：在乾淨來源先 `git apply --check ci-isolation.patch`，再套用並跑四個 job；資料庫必須用隔離測試角色。原分支規則若 required check 指定 test-persistence，使用者同步改成四個新 job，否則 PR 可能等待不存在的名稱；本次未修改分支規則。採 PostgreSQL 18 major image，實際 minor 及服務版本需從新 job 記錄，不冒稱已執行18.6。

驗證方案：以一次 Pet native 的可控失敗確認 Chat、Pet SQLite、Analytics PG 仍執行且整體失敗；恢復後重跑並檢查四案原斷言、feature 與 DB migration。確認 PR trigger 與 required checks 均可結束。該故障注入僅在使用者隔離分支進行。

本次實際：完整 patch 在精確來源 git apply --check 通過；YAML 解析通過、四獨立 job、無吞失敗。產品未套用，Cargo／GitHub Actions／required checks／GUI 全未執行或修改。因此狀態是「方案交付／未套用／未驗收」。回復時由使用者還原此一 workflow，並同步還原 required checks；不回退其他產品變更。

下一包：盤點四 workspace 的正式 caller／Port／Adapter／測試斷言與 feature 相依，沿既有工程入口接續；不因本包而另建一套 FIFO，也不以此工作流拆分宣稱全專案分層完成。
