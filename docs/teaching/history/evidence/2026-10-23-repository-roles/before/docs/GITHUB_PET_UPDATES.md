# GitHub 原生 Pet 自動更新

本機 Linux 使用既有 XDG 安裝與同一份寵物資料。**啟動時檢查一次，或由控制面板按「檢查更新」**；沒有週期性更新 timer。再次開啟既有控制視窗不觸發新的啟動檢查。

來源只接受 `williechen/copilot_use` 的最新已發布正式 Release；略過草稿、預發行、同版與降版。更新包先用本機固定公鑰驗證 Minisign 簽章與版本，再讀取 archive；只允許五個指定普通檔案，拒絕 symlink／路徑穿越／重複檔名與超出大小。新程式需回報相符的乾淨 release 來源 SHA／版號及 SHA256。

更新既有安裝時，保留 `.local/bin/copilot-desktop-pet`、XDG desktop entry、app_data 及 runtime.env；更新前備份 SQLite，新增不可變版本目錄，原子切換 current。原本正在執行才會自動重新啟動；若程式已關閉則只更新。啟動失敗會切回原版本，保留資料及備份，不自動降版資料庫。

一般控制設定即時保存與套用，不依賴程式重新啟動。只有程式版本更新需要重新啟動。

## 發行端資產

每個新正式 Release 上傳以下兩份資產：

- `copilot-pet-linux-x86_64.json`
- `copilot-pet_<版本>_linux-x86_64.tar.gz`

以原生乾淨 release 建置後執行：

```sh
source scripts/native-env.sh
python3 scripts/package-pet-update.py \
  --private-key "$HOME/.config/copilot-desktop-pet/release-signing.key" \
  --output target/pet-update-assets
```

私鑰只留本機模式 0600，不入 Git，也不包含在更新包。`deploy/pet-update-public-key.pub` 為對應公鑰，首次設定時固定到使用者 config；更新包不能自行替換信任公鑰。更新包由固定私鑰簽署，簽章包含版本；manifest 保存該簽章，內容再與已簽署的更新包核對。

目前已發布版本以 [GitHub Releases](https://github.com/williechen/copilot_use/releases) 即時結果為準；既有發行資產與驗收範圍保持固定。本文規定後續原生更新格式，不以本機建置或測試包冒稱 GitHub 已發行。

## 本機服務

`copilot-pet-update.service` 是 user oneshot，入口為安裝目錄 current/github-update.py。程式啟動／使用者按更新時呼叫一次，不 enable timer，不需要 root。日誌：`journalctl --user -u copilot-pet-update.service`；UI 只顯示不含憑證的更新狀態。

啟動檢查設定位於 `~/.config/copilot-desktop-pet/update-on-start`，控制面板 checkbox 即時保存。關閉後仍可按更新。
