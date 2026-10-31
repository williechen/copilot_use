# Day45 真實GUI驗收補記｜Pet

產品精確SHA `a7a8267f8c2f9003dd0d91d9a8cd25daba2517e4`。已實際啟動產品debug Tauri/Wry原生程式，在Xvfb／xfwm4用WebKitWebDriver及X11事件驗200×200單圖、六姿態、回籠、原生關閉、雙寵物狀態隔離、原生菜單、拖曳、找回及同DB重啟恢復。不是DOM fixture；但實體／跨OS包、透明合成、自動漫遊仍不接受／未驗。

[完整範圍／限制／截圖](../../../source-review/day45-gui-2026-10-31-a7a8267/README.md) · [實際操作資料](../../../source-review/day45-gui-2026-10-31-a7a8267/gui-review.json)。此補記優先於下方較早的「尚未跑瀏覽器／桌面」描述；本次未加Day，next_day仍46，FIFO及last_full_comparison不變。

---

# Day45 最後來源追驗｜Pet

產品`d784f32c6bff86249dd84fd4294d055a4c774a49`。使用者已在d784修正browser斷言，最新來源重新實跑UI11項與browser六pose，皆通過。28版TypeError及修正patch保留為歷史，d784不需再套該patch。DOM fixture仍不等於真Tauri視窗驗收。

[本回合完整證據](../../../source-review/day45-2026-10-31-d784f32/README.md) · [精確SHA／實測區分](../../../source-review/day45-2026-10-31-d784f32/latest-source.json)。本段優先於以下28初次驗證及更早歷史原文。

---

# Day45 最新驗證補記｜Pet｜2026-10-31

產品精確SHA `28b71b67907062f804b2358c41fde349c9b18ca9`；本補記優先於下方歷史候選敘述。產品已採用200×200與pose Refactor，UI11項通過；但產品browser腳本的xresult.image.width錯字令測試TypeError中斷，不能稱原樣六pose通過。[完整修正patch](../../../source-review/day45-2026-10-31-d784f32/pet-browser-fix.patch)修為x + width，修正候選六pose成功且越界負向控制會正確失敗。此為DOM fixture，仍非真Tauri視窗驗收。首次編譯磁碟空間不足的log保留，清快取後重跑11通過；不得把環境錯誤當成功。

[三案驗證／修正條件](../../../source-review/day45-2026-10-31-d784f32/README.md) · [命令／exit與限制](../../../source-review/day45-2026-10-31-d784f32/review.json) · [清暫存紀錄](../../../source-review/day45-2026-10-31-d784f32/cleanup.json)。本次不新增Day；next_day仍46，FIFO與last_full_comparison未變；原Red／Green／獨立Refactor與歷史證據保留。

---

## 本回合完整逐檔修正patch

```diff
diff --git a/pet_apps/pet_ui/tests/browser/canvas.cjs b/pet_apps/pet_ui/tests/browser/canvas.cjs
index 30071a3..c9019f9 100644
--- a/pet_apps/pet_ui/tests/browser/canvas.cjs
+++ b/pet_apps/pet_ui/tests/browser/canvas.cjs
@@ -13,7 +13,7 @@ const root=process.argv[2] || path.resolve(__dirname,'../../../..');
   const result=await page.evaluate(()=>{const f=document.querySelector('.pet-face').getBoundingClientRect();const i=document.querySelector('img').getBoundingClientRect();return {frame:{width:f.width,height:f.height},image:{x:i.x,y:i.y,width:i.width,height:i.height},count:document.querySelectorAll('.pet-face img').length,scroll:document.documentElement.scrollHeight}});
   console.log(pose,JSON.stringify(result));
   if(result.frame.width!==200||result.frame.height!==200||result.count!==1)throw Error('200x200 single-image canvas missing');
-  if(result.image.x<0||result.image.y<0||result.image.xresult.image.width>200.5||result.image.y+result.image.height>200.5)throw Error('pose clipped');
+  if(result.image.x<0||result.image.y<0||result.image.x+result.image.width>200.5||result.image.y+result.image.height>200.5)throw Error('pose clipped');
  }
  await page.screenshot({path:process.env.PET_SCREENSHOT || 'pet-day45-canvas.png'});} finally { await b.close(); }
 })().catch(e=>{console.error(e);process.exitCode=1});
\ No newline at end of file
```

# Pet Day45｜200×200 生活畫布與不裁切的單圖動畫

## ① 目標、現況與正式流程

產品 `64eff527283fd05657841297879aefeaadd31f64` 唯讀，規則／需求基準 `copilot_use@738aeab0e689ebcc9699d0555a598480c8cede46`；最新 learning-state 與 manifest 的 Pet next_day=45。已讀 Day44 活動模型與 migration、UI 條件及 caller，基本新增→SQLite→出籠→重載已存在，不重寫後端活動契約。

接續可操作旅程：控制台新增或選擇寵物→open_saved_pet 原生200×200窗→App 的 pet-life 分支→PetStatus→單張 SVG 依 snapshot／clock 切換 pose。現行 img 只有80×56、face56px高，無法提供要求的200×200生活畫布。本篇擴充畫布，圖片置於168×168等比安全區，留出旋轉／跳動範圍；畫布是200×200，不宣稱每個有色像素填滿整窗。拖曳把手、原生右鍵、回籠與控制台按鈕保留既有 IPC，錯誤覆蓋層不擠壓畫布。

## Refactor｜Pet UI pose 投影自時鐘與 IPC 元件移出

累積問題：activity.rs 同時含純投影、CSS class、時鐘生命週期、contextmenu IPC 與元件。將 PetPose、project_pose 及四組原有效斷言完整搬至 component/pose.rs。activity 僅由 super::pose 匯入，component/mod.rs 私有宣告；沒有新增 domain→UI 依賴，PetState 仍屬 domain。Cargo 與 CSR feature 沿用現有 pet-ui，不碰其他專案或全庫 CI。

四組斷言逐一承接：idle 眨眼優先、互動兩相、睡眠兩相、六個互異 class，名稱及期待值皆保留，執行入口改成 component::pose::tests；原 snapshot／scan fixture 不刪。以下是逐檔完整搬移，不以附件代替正文：

```diff
diff --git a/pet_apps/pet_ui/src/component/activity.rs b/pet_apps/pet_ui/src/component/activity.rs
index 83c6f18..e634521 100644
--- a/pet_apps/pet_ui/src/component/activity.rs
+++ b/pet_apps/pet_ui/src/component/activity.rs
@@ -5,54 +5,7 @@ use wasm_bindgen_futures::spawn_local;
 
 use crate::bridge;
 
-#[derive(Debug, Clone, Copy, PartialEq, Eq)]
-pub enum PetPose {
-    Idle,
-    Blink,
-    InteractA,
-    InteractB,
-    SleepA,
-    SleepB,
-}
-
-impl PetPose {
-    fn css_class(self) -> &'static str {
-        match self {
-            Self::Idle => "pose-idle",
-            Self::Blink => "pose-blink",
-            Self::InteractA => "pose-interact-a",
-            Self::InteractB => "pose-interact-b",
-            Self::SleepA => "pose-sleep-a",
-            Self::SleepB => "pose-sleep-b",
-        }
-    }
-}
-
-fn project_pose(state: PetState, blinking: bool, animation_phase: bool) -> PetPose {
-    match state {
-        PetState::Idle => {
-            if blinking {
-                PetPose::Blink
-            } else {
-                PetPose::Idle
-            }
-        }
-        PetState::Interacting => {
-            if animation_phase {
-                PetPose::InteractB
-            } else {
-                PetPose::InteractA
-            }
-        }
-        PetState::Sleeping => {
-            if animation_phase {
-                PetPose::SleepB
-            } else {
-                PetPose::SleepA
-            }
-        }
-    }
-}
+use super::pose::{PetPose, project_pose};
 
 // 拖曳桌面小寵物的控制把手組件
 #[component]
@@ -165,55 +118,3 @@ pub fn PetStatus(
         </section>
     }
 }
-
-#[cfg(test)]
-mod tests {
-    use super::*;
-
-    #[test]
-    fn idle_blink_has_priority() {
-        assert_eq!(project_pose(PetState::Idle, true, true,), PetPose::Blink,);
-    }
-
-    #[test]
-    fn interacting_uses_animation_phase() {
-        assert_eq!(
-            project_pose(PetState::Interacting, false, false,),
-            PetPose::InteractA,
-        );
-
-        assert_eq!(
-            project_pose(PetState::Interacting, false, true,),
-            PetPose::InteractB,
-        );
-    }
-
-    #[test]
-    fn sleeping_uses_animation_phase() {
-        assert_eq!(
-            project_pose(PetState::Sleeping, false, false,),
-            PetPose::SleepA,
-        );
-
-        assert_eq!(
-            project_pose(PetState::Sleeping, false, true,),
-            PetPose::SleepB,
-        );
-    }
-
-    #[test]
-    fn day26_each_pose_selects_single_image_class() {
-        use std::collections::HashSet;
-        let all = [
-            PetPose::Idle,
-            PetPose::Blink,
-            PetPose::InteractA,
-            PetPose::InteractB,
-            PetPose::SleepA,
-            PetPose::SleepB,
-        ];
-        let classes: HashSet<&'static str> = all.iter().map(|p| p.css_class()).collect();
-        assert_eq!(classes.len(), 6);
-        assert!(classes.iter().all(|s| s.starts_with("pose-")));
-    }
-}
diff --git a/pet_apps/pet_ui/src/component/mod.rs b/pet_apps/pet_ui/src/component/mod.rs
index d6f3ab5..1f3a3dd 100644
--- a/pet_apps/pet_ui/src/component/mod.rs
+++ b/pet_apps/pet_ui/src/component/mod.rs
@@ -1,6 +1,7 @@
 pub mod activity;
 pub mod chat;
 pub mod control;
+mod pose;
 
 mod snapshot;
 
diff --git a/pet_apps/pet_ui/src/component/pose.rs b/pet_apps/pet_ui/src/component/pose.rs
new file mode 100644
index 0000000..e408516
--- /dev/null
+++ b/pet_apps/pet_ui/src/component/pose.rs
@@ -0,0 +1,102 @@
+use pet_domain::PetState;
+
+#[derive(Debug, Clone, Copy, PartialEq, Eq)]
+pub enum PetPose {
+    Idle,
+    Blink,
+    InteractA,
+    InteractB,
+    SleepA,
+    SleepB,
+}
+
+impl PetPose {
+    pub(super) fn css_class(self) -> &'static str {
+        match self {
+            Self::Idle => "pose-idle",
+            Self::Blink => "pose-blink",
+            Self::InteractA => "pose-interact-a",
+            Self::InteractB => "pose-interact-b",
+            Self::SleepA => "pose-sleep-a",
+            Self::SleepB => "pose-sleep-b",
+        }
+    }
+}
+
+pub(super) fn project_pose(state: PetState, blinking: bool, animation_phase: bool) -> PetPose {
+    match state {
+        PetState::Idle => {
+            if blinking {
+                PetPose::Blink
+            } else {
+                PetPose::Idle
+            }
+        }
+        PetState::Interacting => {
+            if animation_phase {
+                PetPose::InteractB
+            } else {
+                PetPose::InteractA
+            }
+        }
+        PetState::Sleeping => {
+            if animation_phase {
+                PetPose::SleepB
+            } else {
+                PetPose::SleepA
+            }
+        }
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    #[test]
+    fn idle_blink_has_priority() {
+        assert_eq!(project_pose(PetState::Idle, true, true,), PetPose::Blink,);
+    }
+
+    #[test]
+    fn interacting_uses_animation_phase() {
+        assert_eq!(
+            project_pose(PetState::Interacting, false, false,),
+            PetPose::InteractA,
+        );
+
+        assert_eq!(
+            project_pose(PetState::Interacting, false, true,),
+            PetPose::InteractB,
+        );
+    }
+
+    #[test]
+    fn sleeping_uses_animation_phase() {
+        assert_eq!(
+            project_pose(PetState::Sleeping, false, false,),
+            PetPose::SleepA,
+        );
+
+        assert_eq!(
+            project_pose(PetState::Sleeping, false, true,),
+            PetPose::SleepB,
+        );
+    }
+
+    #[test]
+    fn day26_each_pose_selects_single_image_class() {
+        use std::collections::HashSet;
+        let all = [
+            PetPose::Idle,
+            PetPose::Blink,
+            PetPose::InteractA,
+            PetPose::InteractB,
+            PetPose::SleepA,
+            PetPose::SleepB,
+        ];
+        let classes: HashSet<&'static str> = all.iter().map(|p| p.css_class()).collect();
+        assert_eq!(classes.len(), 6);
+        assert!(classes.iter().all(|s| s.starts_with("pose-")));
+    }
+}
```

## ② Red → 完整 Green

核心 Red／Green 預估15分鐘，未實際計時；Refactor、版本、瀏覽器安裝與交付另計。Red 用正式 style.css 與 PetStatus SVG 的 DOM fixture；它測 layout 引擎而非字串搜尋。fixture 代表 pet-life 結構，並未啟動 Leptos WASM 或 Tauri，因此不能當正式畫面接線驗收。

在產品根目錄套 Refactor→Red，安裝 `pet_apps/pet_ui/tests/browser/package.json` 的 Playwright 1.64.1、`playwright install chromium`，再 `node pet_apps/pet_ui/tests/browser/canvas.cjs "$PWD"`。腳本可從任何 cwd 以 root 參數操作；無參數則由 __dirname 回到產品根目錄。以下完整 Red 附件：

```diff
diff --git a/pet_apps/pet_ui/tests/browser/canvas.cjs b/pet_apps/pet_ui/tests/browser/canvas.cjs
new file mode 100644
index 0000000..4a087f3
--- /dev/null
+++ b/pet_apps/pet_ui/tests/browser/canvas.cjs
@@ -0,0 +1,19 @@
+const { chromium } = require('playwright');
+const fs=require('node:fs');
+const path=require('node:path');
+const root=process.argv[2] || path.resolve(__dirname,'../../../..');
+(async()=>{
+ const b=await chromium.launch({headless:true});try { const page=await b.newPage({viewport:{width:200,height:200}});
+ const source=fs.readFileSync(path.join(root,'pet_apps/pet_ui/src/component/activity.rs'),'utf8');
+ const src=source.match(/src="(data:image[^\"]+)"/)[1];
+ const css=fs.readFileSync(path.join(root,'pet_apps/pet_ui/style.css'),'utf8');
+ await page.setContent(`<style>${css}</style><main class="pet-life"><header class="drag-handle">...</header><p>測試寵物</p><section class="pet-status"><div class="pet-face"><img class="pose-idle" src="${src}" alt="寵物動畫"></div><strong>待機</strong><small>revision 1</small></section><button>回籠子</button><p role="alert"></p><button>控制台</button></main>`);
+ for(const pose of ['idle','blink','interact-a','interact-b','sleep-a','sleep-b']){
+  await page.locator('img').evaluate((e,p)=>e.className='pose-'+p,pose);
+  const result=await page.evaluate(()=>{const f=document.querySelector('.pet-face').getBoundingClientRect();const i=document.querySelector('img').getBoundingClientRect();return {frame:{width:f.width,height:f.height},image:{x:i.x,y:i.y,width:i.width,height:i.height},count:document.querySelectorAll('.pet-face img').length,scroll:document.documentElement.scrollHeight}});
+  console.log(pose,JSON.stringify(result));
+  if(result.frame.width!==200||result.frame.height!==200||result.count!==1)throw Error('200x200 single-image canvas missing');
+  if(result.image.x<0||result.image.y<0||result.image.x+result.image.width>200.5||result.image.y+result.image.height>200.5)throw Error('pose clipped');
+ }
+ await page.screenshot({path:process.env.PET_SCREENSHOT || 'pet-day45-canvas.png'});} finally { await b.close(); }
+})().catch(e=>{console.error(e);process.exitCode=1});
diff --git a/pet_apps/pet_ui/tests/browser/package.json b/pet_apps/pet_ui/tests/browser/package.json
new file mode 100644
index 0000000..47d3a62
--- /dev/null
+++ b/pet_apps/pet_ui/tests/browser/package.json
@@ -0,0 +1,6 @@
+{
+  "private": true,
+  "devDependencies": {
+    "playwright": "1.64.1"
+  }
+}
```

本次真 Red exit1：idle frame=192×56、單圖80×56，期待200×200。Green 從已套 Refactor、未套 Red 的狀態套用，已包含同一 Red 測試；做過 Red 者先 reverse Red。CSS 只作用於 .pet-life，不改控制台尺寸、資料或活動狀態。

```diff
diff --git a/pet_apps/pet_ui/style.css b/pet_apps/pet_ui/style.css
index 352c075..7013485 100644
--- a/pet_apps/pet_ui/style.css
+++ b/pet_apps/pet_ui/style.css
@@ -279,4 +279,15 @@ body {
 
 @media (prefers-reduced-motion:reduce) {
   .pet-face > img { transition:none; }
-}
\ No newline at end of file
+}
+/* Life canvas fills the existing 200x200 native window; controls overlay it. */
+.pet-life { position:relative; padding:0; width:200px; height:200px; }
+.pet-life .pet-status { display:block; margin:0; }
+.pet-life .pet-face { position:absolute; inset:0; width:200px; height:200px; line-height:normal; overflow:hidden; }
+.pet-life .pet-face > img { width:168px; height:168px; object-fit:contain; }
+.pet-life .drag-handle { position:absolute; z-index:2; top:0; left:0; width:200px; height:20px; }
+.pet-life > p:not([role="alert"]) { position:absolute; top:20px; left:0; width:200px; pointer-events:none; }
+.pet-life .pet-status strong, .pet-life .pet-status small { position:absolute; width:1px; height:1px; padding:0; overflow:hidden; clip-path:inset(50%); white-space:nowrap; }
+.pet-life > button { position:absolute; z-index:2; bottom:0; left:4px; }
+.pet-life > button:last-child { left:auto; right:4px; }
+.pet-life > p[role="alert"] { position:absolute; z-index:3; bottom:24px; left:0; width:200px; white-space:normal; background:#522; }
diff --git a/pet_apps/pet_ui/tests/browser/canvas.cjs b/pet_apps/pet_ui/tests/browser/canvas.cjs
new file mode 100644
index 0000000..4a087f3
--- /dev/null
+++ b/pet_apps/pet_ui/tests/browser/canvas.cjs
@@ -0,0 +1,19 @@
+const { chromium } = require('playwright');
+const fs=require('node:fs');
+const path=require('node:path');
+const root=process.argv[2] || path.resolve(__dirname,'../../../..');
+(async()=>{
+ const b=await chromium.launch({headless:true});try { const page=await b.newPage({viewport:{width:200,height:200}});
+ const source=fs.readFileSync(path.join(root,'pet_apps/pet_ui/src/component/activity.rs'),'utf8');
+ const src=source.match(/src="(data:image[^\"]+)"/)[1];
+ const css=fs.readFileSync(path.join(root,'pet_apps/pet_ui/style.css'),'utf8');
+ await page.setContent(`<style>${css}</style><main class="pet-life"><header class="drag-handle">...</header><p>測試寵物</p><section class="pet-status"><div class="pet-face"><img class="pose-idle" src="${src}" alt="寵物動畫"></div><strong>待機</strong><small>revision 1</small></section><button>回籠子</button><p role="alert"></p><button>控制台</button></main>`);
+ for(const pose of ['idle','blink','interact-a','interact-b','sleep-a','sleep-b']){
+  await page.locator('img').evaluate((e,p)=>e.className='pose-'+p,pose);
+  const result=await page.evaluate(()=>{const f=document.querySelector('.pet-face').getBoundingClientRect();const i=document.querySelector('img').getBoundingClientRect();return {frame:{width:f.width,height:f.height},image:{x:i.x,y:i.y,width:i.width,height:i.height},count:document.querySelectorAll('.pet-face img').length,scroll:document.documentElement.scrollHeight}});
+  console.log(pose,JSON.stringify(result));
+  if(result.frame.width!==200||result.frame.height!==200||result.count!==1)throw Error('200x200 single-image canvas missing');
+  if(result.image.x<0||result.image.y<0||result.image.x+result.image.width>200.5||result.image.y+result.image.height>200.5)throw Error('pose clipped');
+ }
+ await page.screenshot({path:process.env.PET_SCREENSHOT || 'pet-day45-canvas.png'});} finally { await b.close(); }
+})().catch(e=>{console.error(e);process.exitCode=1});
diff --git a/pet_apps/pet_ui/tests/browser/package.json b/pet_apps/pet_ui/tests/browser/package.json
new file mode 100644
index 0000000..47d3a62
--- /dev/null
+++ b/pet_apps/pet_ui/tests/browser/package.json
@@ -0,0 +1,6 @@
+{
+  "private": true,
+  "devDependencies": {
+    "playwright": "1.64.1"
+  }
+}
```

Green 的定位關係：face絕對定位充滿畫布，圖置中，標籤仍供輔助閱讀但不占版面；拖曳在頂部、兩操作鈕在底部，錯誤在按鈕上方。六種 pose 的 transformed bounding box 逐一檢查皆在200×200內，單圖數=1；不是刪掉動畫以通過檢查。新增測試套件只屬 Pet UI，不新增全庫 CI job 或抵算 G-01。

### FIFO／上一篇承接

Pet Day25 最早 pending 的單圖部分：精確來源已是單 SVG，這次確認六相單圖與裁切界線；但三個真 native pet-id 生活窗、逐ID獨立事件與實機 frame 尚未在本篇精確候選執行，故 FIFO 不結案，也不更改封存 Day25。Day44 活動與 SQLite 既有斷言保留，本篇沒有變更後端契約。

## ③ 2026-10-31 官方穩定版與相容性

Rust/Cargo 本機 1.99.1；[官方 Rust 1.99.1（2026-10-15）](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)。下表當日直接查 crates.io 官方 registry，排除預發行及 yanked；完整 JSON、MSRV 與查核來源隨文保存。edition 2024；沒有升級 lock。

| 套件 | manifest | lock | 官方 stable | MSRV |
| --- | --- | --- | --- | --- |
| [async-trait](https://crates.io/api/v1/crates/async-trait) | 0.1 | 0.1.92 | 0.1.92 | 1.71 |
| [leptos](https://crates.io/api/v1/crates/leptos) | 0.8 | 0.8.22 | 0.8.22 | 1.88 |
| [leptos_config](https://crates.io/api/v1/crates/leptos_config) | 0.8 | 0.8.10 | 0.8.10 | 1.88 |
| [libsqlite3-sys](https://crates.io/api/v1/crates/libsqlite3-sys) | package manifest / transitive | 0.37.0 | 0.39.1 | 1.88.0 |
| [serde](https://crates.io/api/v1/crates/serde) | 1.0 | 1.0.229 | 1.0.229 | 1.56 |
| [serde_json](https://crates.io/api/v1/crates/serde_json) | 1.0 | 1.0.154 | 1.0.154 | 1.71 |
| [sha2](https://crates.io/api/v1/crates/sha2) | 0.11 | 0.11.1 | 0.11.1 | 1.88 |
| [sqlx](https://crates.io/api/v1/crates/sqlx) | 0.9 | 0.9.0 | 0.9.0 | 1.94.0 |
| [tauri](https://crates.io/api/v1/crates/tauri) | package manifest / transitive | 2.12.5 | 2.12.5 | 1.90 |
| [tauri-build](https://crates.io/api/v1/crates/tauri-build) | package manifest / transitive | 2.7.4 | 2.7.4 | 1.90 |
| [thiserror](https://crates.io/api/v1/crates/thiserror) | 2.0 | 2.0.21 | 2.0.21 | 1.77 |
| [tokio](https://crates.io/api/v1/crates/tokio) | 1.54 | 1.54.1 | 1.54.1 | 1.85 |
| [tower](https://crates.io/api/v1/crates/tower) | package manifest / transitive | 0.5.3 | 0.5.3 | 1.64.0 |
| [uuid](https://crates.io/api/v1/crates/uuid) | 1.29 | 1.29.0 | 1.29.0 | 1.89.0 |

SQLx 0.9 要求 Rust 1.94；Leptos 0.8 要求 1.88，目前編譯器符合。libsqlite3-sys lock 0.37.0 低於最新 0.39.1：SQLx SQLite driver 綁定其原生 links，不能單獨升級造成重複 sqlite3 links；本篇保留 SQLx 支援的 lock，升級須一起重核 driver、SQLite 編譯及舊庫。MSRV 是必要條件，不能據此推論 DB、WASM 或 GUI 已驗收。
Tauri 2.12.5／tauri-build 2.7.4 均符合 Rust 1.90；Linux 仍須 GTK3、WebKitGTK4.1、X11／Wayland。候選 native 檢查不代表實體桌面驗收。

Pet UI 直接工具補查（當日官方 registry，完整JSON見pet-extra-versions.json）：

| 套件 | lock／安裝 | 官方 stable | MSRV |
| --- | --- | --- | --- |
| [gloo-timers](https://crates.io/api/v1/crates/gloo-timers) | 0.4.0 | 0.4.0 | 1.82 |
| [wasm-bindgen](https://crates.io/api/v1/crates/wasm-bindgen) | 0.2.129 | 0.2.130 | 1.81 |
| [wasm-bindgen-futures](https://crates.io/api/v1/crates/wasm-bindgen-futures) | 0.4.79 | 0.4.80 | 1.81 |
| [js-sys](https://crates.io/api/v1/crates/js-sys) | 0.3.106 | 0.3.107 | 1.81 |
| [web-sys](https://crates.io/api/v1/crates/web-sys) | 0.3.106 | 0.3.107 | 1.81 |
| [serde-wasm-bindgen](https://crates.io/api/v1/crates/serde-wasm-bindgen) | 0.6.5 | 0.6.5 | 未宣告 |
| [trunk](https://crates.io/api/v1/crates/trunk) | trunk installed 0.21.14 | 0.22.1 | 1.96.1 |

wasm-bindgen／js-sys／web-sys／futures 是配對ABI工具鏈，建議整組升到本日 stable 並連同 CLI、WASM、Trunk 重驗，不能只換單一版本。本篇沿用鎖檔重現基準，沒有冒稱升級；目前 Rust1.99.1 滿足新 Trunk1.96.1最低要求，實際安裝0.21.14 尚未驗證新版本。serde-wasm-bindgen 未宣告 MSRV，以實際編譯為依據。

[Playwright 官方1.64說明](https://playwright.dev/docs/release-notes)／npm registry1.64.1（本次安裝1.64.1），Node安裝v22.22.1；瀏覽器為Playwright配對Chromium156.0.8078.4，這是測試配對版，不當作終端使用者穩定瀏覽器或 WebKitGTK。首次 npm 命令因未安裝 exit127，改用當次可用 pnpm11.25.0 安裝成功；原始失敗與重試已保存。沒有自動修改產品 lock。

## ④ 驗證、限制與下一步

真命令與exit見pet-validation.json／日誌：同案host UI11項通過；候選瀏覽器DOM Red1／Green0，六相逐一量測；從乾淨精確來源檢查 Refactor→Green 套用成功。螢幕圖為隔離DOM fixture，不是Tauri實機截圖。

產品SHA七workflow success見product-ci.json；該CI不包含本篇尚未套用候選。未跑：WASM重新建置、正式Tauri WebKitGTK、三窗逐ID事件／原生拖曳／右鍵與控制台按鈕、實體螢幕與縮放、SQLite舊庫／活動回歸。本篇沒動SQLite，不能把舊112項結果追認到候選。

Gate1：完整內容／版本／patch與精確commit讀回齊備後同步；Gate2=false，候選尚未由使用者套用。下一步在隔離Tauri用A/B/C三隻寵物驗證畫布、不同snapshot／revision、drag／右鍵／回籠只作用指定ID，再以新精確SHA補實機證據。回復以Green→Refactor反序 reverse；無migration與資料刪除。核心60分鐘只計三案主要Red／Green，last_full_comparison 不更新。
