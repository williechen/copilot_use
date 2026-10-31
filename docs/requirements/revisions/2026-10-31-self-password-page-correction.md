# 自身密碼 ACL page與登入顯示規則更正（2026-10-31）

先前助理將「共用修改密碼，不單獨一頁」誤讀成移除自身密碼ACL page，該解讀已撤回。使用者已明確更正並確認以下功能樹。

自身密碼保留為 ACL page。首次登入需強制修改時先顯示自身密碼頁，完成修改後，後續登入才預設顯示首頁；未完成者重登仍須先完成，不以單純登入次數繞過。

首次與日後本人修改使用同一個自身密碼 ACL page及共用元件／後端流程，不再另建第二個首次修改頁。保留目前密碼驗證、新密碼確認、CSRF、成功後清除失敗等待及撤銷所有 Session 再登入；身分取自 Session，不可藉此修改其他帳號。

```text
會員管理                 ← category，至少一個可用page才顯示
└─ 自身密碼              ← 保留的ACL page
   └─ 修改密碼           ← update action，有權限才顯示
```


若只有自身密碼page權限，不連帶顯示帳號管理等其他page；category由page權限推導，action依所屬page與自己的CRUD權限顯示。首次／後續登入只決定預設開啟內容，功能樹仍按ACL顯示。需要首次強制修改的人員必須能完成此page必要操作，不能被唯讀設定擋住，也不因此取得管理其他帳密的權限。

本次更正需求，不寫rust_ai_ring、不增加Day，不改next_day／FIFO／last_full_comparison，未重跑或宣稱產品驗收。原文件commit f6bad0e9fc78d7990e38562ba508e9fe492bba39及讀回證據保留歷史；其中移除ACL page的需求解讀無效，原SQL來源／測試仍保留原SHA。[現行共用驗證](../AUTHENTICATION.md) · [Analytics ACL](../ANALYTICS_ACL.md)。
