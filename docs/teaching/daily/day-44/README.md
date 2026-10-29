# Day44｜三案 TDD 正式文件入口（2026-10-30）

**Pet Day44 修訂更正：** 原新增 `auto_release_on_start` 欄位的 Green 已撤回封存；新規則要求出／回籠依 `pet_activity` 任一 `is_activity=true` 判斷，不限散步、不新增欄位。替代 Green 待完成／待驗，原 Gate1 歷史讀回不代表修訂已完成。next_day 保持三案 45，先補齊 Pet Day44 修訂，不重複加 Day。

Chat／Analytics Day44 保留原 Gate1 交付與未驗界線；Pet Day44 原新增欄位 Green 已撤回，現在是資料契約修訂，替代 Green 待完成。產品保持唯讀；原始讀回證據只屬原發布版本，三案 next_day 維持 Day45。

| 專案 | Day44 正文 | Refactor patch | Green patch | 精確文章 commit |
| --- | --- | --- | --- | --- |
| Chat | [Chat](chat.md) | [Refactor](chat-day44-refactor.patch) | [Green](chat-day44-green.patch) | [`26705cb8a185`](https://github.com/williechen/copilot_use/commit/26705cb8a185be4a13ff7e777d0860edb6b0db4a) |
| Pet | [Pet](pet.md) | [Refactor](pet-day44-refactor.patch) | 已撤回，見 [修訂](pet.md) | [`971f0e3e17a6`](https://github.com/williechen/copilot_use/commit/971f0e3e17a6705916a324ac9675f68ebb136fac) |
| Analytics | [Analytics](analytics.md) | [Refactor](analytics-day44-refactor.patch) | [Green](analytics-day44-green.patch) | [`585384c39d3a`](https://github.com/williechen/copilot_use/commit/585384c39d3aeac9c535992e110ebd5c9d1f2854) |

[逐 commit blob／SHA256／Gate 紀錄](delivery-verification.json) · [ENTRY](ENTRY.md) · [上一篇 Day43](../day-43/README.md) · [目前進度](../../current/STATUS.md) · [各案 Refactor](../../functional-refactoring/PROGRESS.md) · [跨案工程](../../../engineering/README.md)
