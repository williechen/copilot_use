# 當前文件與正式 Roadmap 的衝突

更正：先前將本地 13 階段／P12 判作衝突的結論撤回；直接使用者更正優先於舊十二階段文件。

本輪只記錄差異，不回寫歷史文章、不自動改排日期或學習進度。

|項目|本輪重新讀取證據|衝突／處理建議|
|---|---|---|
|現行 curriculum 階段數|[docs/learning-state.json:L138](https://github.com/williechen/copilot_use/blob/4a3c43187e1ffd7cf4ba8d44cd4f8529f0d9d6d4/docs/learning-state.json#L138)|本地 13／P0–P12 正確。使用者已更正先前 12 階段說法；本次新增十三階段詳細 Roadmap，保留原日曆。|
|現行 stage_topics|[docs/learning-state.json:L162](https://github.com/williechen/copilot_use/blob/4a3c43187e1ffd7cf4ba8d44cd4f8529f0d9d6d4/docs/learning-state.json#L162)|P0–P12 範圍正確；已補現行 Roadmap 引用，舊 P9 規則仍不恢復。|
|daily 尾段進度|[docs/teaching/daily/README.md:L64](https://github.com/williechen/copilot_use/blob/4a3c43187e1ffd7cf4ba8d44cd4f8529f0d9d6d4/docs/teaching/daily/README.md#L64)|尾段寫 Day35，但同頁最新交付表為 Day37，當前 learning-state.next_day 三產品亦均為 37。這是導航文字過時；本次只同步尾段為 Day37，next_day 及原證據不變。|
|FileOrganizer／AI 延伸|[docs/ROADMAP_P0_P11.md:L40](https://github.com/williechen/copilot_use/blob/4a3c43187e1ffd7cf4ba8d44cd4f8529f0d9d6d4/docs/ROADMAP_P0_P11.md#L40)|歷史 Pet Day 的延伸教學保留；不由其完成與否阻塞正式 P0 Gate。|

這些是本輪文件對照的發現；不代表其中任何產品功能已實作或未實作。只在附表記錄，以便後續集中修正現行規劃，避免擅改原始文章及歷史學習紀錄。
