# CI責任與功能分層核對

日期：2026-10-26。規則補充ci-layer-boundaries-v1。文件基準 `cca773b17b052672cc30350f320bb54292f95ab2`；程式來源 `9b89656f964b99e0cb89ba421855704a35c5ecb9`。本次僅修改copilot_use文件，不修改產品workflow／程式、不執行建置／測試、不推進三案next_day、不變更FIFO／last_full_comparison或舊文章。

## 這次具體問題

來源 `.github/workflows/persistence.yml` 的唯一job `test-persistence` 有三個順序step：

1. Chat：`chat_apps`，`cargo test --locked -p chat-persistence`。
2. Pet：`pet_apps`，`cargo test --locked -p pet-server -p pet-persistence`。
3. Analytics：`analytics_apps`，`cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration`。

Pet第二步包含Tauri server，會編譯normal依賴tauri與build依賴tauri-build，需要Linux原生開發套件。648b903的該步失敗，後續Analytics被skipped。這是同job預設失敗傳遞造成的執行耦合，與Analytics本身測試是否正確無直接因果；也不能以Persistence名稱判斷只測了SQLx。

9b89656加入Linux依賴安裝。此次讀取 [新Persistence run 40236329539](https://github.com/williechen/rust_ai_ring/actions/runs/40236329539) 時，安裝已成功、Pet測試執行中、Analytics尚未開始；不宣稱新CI失敗，也不宣稱已修復或全綠。無論該run最終是否成功，三案在單job串接的責任混合仍存在。

## 能證實與不能推論的範圍

- 可證實：CI把SQLite Adapter與Tauri server混選，且三案無必要地串成單job，Pet環境／建置失敗會阻止Analytics產生結果。補系統套件不足以修正驗證邊界。
- 原始碼可證實：出籠用例／補償協調仍在pet_server，Application只有名稱規則等，後續按功能抽出用例／Port／Adapter有實際必要；沿既有functional-layering-v1處理。
- 不能只據此推論：pet-persistence反向依賴pet-server或Domain依賴Tauri。精確manifest中pet-persistence依賴SQLx而無server／Tauri；pet-application／domain沒有SQLx／Tauri。此次CI是命令明確選了server才拉入native，不是persistence library自己拉入server。

## 後續CI責任矩陣

以下是尚未套用的目標／驗證命令，不是已存在的新job。工作目錄必須分別指定，repository根沒有Cargo.toml。

| 目標job責任 | 工作目錄與明確選取 | 必要環境／保留覆蓋 |
| --- | --- | --- |
| Chat純內層 | chat_apps：`cargo test --locked -p chat-domain -p chat-application` | 無DB／native；檢查normal／build／target依賴 |
| Pet純內層 | pet_apps：`cargo test --locked -p pet-domain -p pet-application` | 無DB／native；新用例應可獨立測 |
| Analytics純內層 | analytics_apps：按實際功能測domain／application | 目前仍範本，不把add測試當業務覆蓋 |
| Chat SQLite | chat_apps：`cargo test --locked -p chat-persistence` | SQLite Adapter契約；與Pet／PG分開 |
| Pet SQLite | pet_apps：`cargo test --locked -p pet-persistence` | SQLite Adapter契約；不選pet-server |
| Analytics PostgreSQL | analytics_apps：`cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration` | 專用PG服務與測試DB，明列包含正式Router／Session整合；不依賴Pet |
| Pet原生server | pet_apps：`cargo test --locked -p pet-server` | 正確Linux依賴；保留補償／Runtime／caller測試，後續搬到內層的斷言先遷再清理 |
| UI／產品建置與GUI | 沿現有Chat Leptos、Pet Trunk／Tauri及Analytics實際路徑逐項核對 | SSR／WASM／bundle／GUI分列，不由server測試冒稱畫面驗收 |

Chat既有server HTTP／WS等測試仍須保留於相應整合job，不能因本表突出SQLite就漏掉。Analytics SQLx library與server Router可同屬該案PG整合job，名稱必須說清楚，與「純資料adapter單測」區分；未來需要更細結果再拆，不為目錄數量強造job。

獨立job之間沒有實際產物相依就不設needs；matrix跨案時避免fail-fast取消其它案。彙總job若採always執行，仍須檢查每個必要job結果，failure／cancelled／非預期skipped不能被包成success。拆分後required checks／paths觸發也要核對，不產生永遠pending的舊檢核。這些是完整Green需給的CI接線，本次尚未產出或套用產品YAML。

## 後續完成條件

依既有功能項，在同一最小功能切片提供純用例／Port、SQLx或native實作、正式caller、有效測試遷移、Cargo／module接線與必要完整CI修改。分別以精確SHA的內層、真DB、正式HTTP／IPC、native與必要GUI結果記錄通過／失敗／未執行。不得只安裝WebKit、刪server測試、移除PG feature或允許失敗來宣稱架構清楚或CI全綠。

現有測試結果與歷史失敗保留各自來源，當前三案next_day仍為38。規則／文件已更新不代表上述目標job或產品分層已實作。
