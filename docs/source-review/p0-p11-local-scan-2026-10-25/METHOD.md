# 掃描方法與保存口徑

1. 使用者要求更新後，先從 GitHub fetch main，再以精確 commit checkout；在新建的本地 copilot_use 副本工作，不覆蓋其他聊天的 checkout。
2. 重新遍歷 docs 下所有 Markdown（排除本次報告目錄），讀取完整 UTF-8 內容、行數／位元組／SHA256，對照 git ls-tree blob。沒有用舊索引的 full_text_verified 或 acceptance 欄位推導本次狀態。
3. 自檔案路徑辨識 daily/day-NN/{project}[-rN].md、history/day-NN/{project}/{github,notion/*}.md、同日 archive 的三產品正文、Pet r1 revisions，以及 teaching/archive/day-NN 的三產品正文。每一實體都在逐篇 JSON 有階段、摘錄、相似度與雜湊。
4. 閱讀優先序是 daily 最高 rN → history/github.md → 非 merge 的 notion → merge／archive。這是閱讀導航優先序，沒有宣稱哪一版功能已驗收。
5. 從每篇實際大綱及實作段落抽取帶行號證據，依 105 個主題人工整理對照正式 Roadmap。階段以主體為主；文中提及 AI／OTel／MCP 或舊 P9 不足以歸入相應新階段。主／相關階段只是教材分類，不是工程實作順序。
6. 全文搜尋庫名、路徑、舊階段、SSR API、501、測試預期／歷史通過語境等；命中只列候選，不等同錯誤。程式碼區塊未全部重新編譯，也未逐行獨立語義審查。
7. 同日變體對閱讀版本以逐行 SequenceMatcher 比較；全文 SHA256 相同才列完全重複。相似度不是刪除、功能等價或抄襲判定。
8. 證據目錄內嵌副本、補教／pending／support／版本補核等全部保留在 Markdown inventory；不算新的獨立 Day，未逐份人工審查其產品效力。壓縮檔及外部 URL 的內容未完整展開重驗。
9. JSON 對每篇分列本輪文件掃描、產品實作、CI、功能驗收；后三者全部未於本輪驗證。沿原 FUTURE_WORK anchor 追蹤，未建立重複 FIFO。
10. 舊索引 105／111 與本輪 105／157 是不同保存口徑。本輪 157 是 105 閱讀版、7 較早修訂（包括 Pet Day24 r1）、45 鏡像／封存變體；不是發現了 157 個不同教學日。

十三階段更正：舊階段字串命中已排除 P0–P12；正確階段數及原日曆保留。另補最新程式有限接線來源，不以此更新任何原文的功能驗收。
