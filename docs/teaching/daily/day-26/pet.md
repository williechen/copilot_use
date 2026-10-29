## ① 學習大綱與最新來源

<b>Pet Day 26：把已存在的每窗單一 PetStatus 元件之「多 emoji 字元」改為每幀只有一張寵物圖像（單一 img 元素）。</b> 以 <code>rust_ai_chat/main</code> <code>aaa9194a966aefae1ef4724f5d2b576b4289c550</code> 與教學分支 SHA <code>bee8540d036bbca6b7a79e1a79a176935a7f585a</code> 作為本日來源。規則版本 <code>daily-tdd-minimum-v1</code>、GitHub 規則 blob <code>143fd1592322acda2a2c7821006f87020eec3518</code>。

實際呼叫鏈：<code>apps/desktop_pet/frontend/src/main.rs::App</code> 對 <code>view=pet&pet_id=...</code> 每窗渲染一次 <code>PetStatus</code>，狀態透過 <code>frontend/index.html</code> 的 <code>get_saved_pet_state</code>／<code>saved-pet-state</code>（依 id 過濾），由 <code>apps/desktop_pet/src/pet_windows.rs::Runtime</code> 查獨立 FSM。已有 SQLite <code>pet_store.rs::create/list/profile</code> 與 Tauri 動態視窗。真正缺口是 <code>component/activity.rs::PetStatus</code> 的 <code>🐢·</code>、<code>🐢♪</code>、<code>🐢💤</code> 等複合字元，不能以「只有一個元件」冒充「一個圖像」。

本日只完成可套用的單圖像動畫呈現切片；<b>沒有在本次環境啟動原生 GUI</b>，三個不同 pet id 的視窗／事件／重啟證據仍未核實。

## ② 實作內容：Red → 完整最小 Green → Refactor／FIFO

### Red：驗證幀對應的單一圖像

在既有 <code>apps/desktop_pet/frontend/src/component/activity.rs</code> 的 <code>#[cfg(test)] mod tests</code> 內加上：

```rs
#[test]
fn day26_each_pose_selects_single_image_class() {
    use std::collections::HashSet;
    let all = [
        PetPose::Idle, PetPose::Blink, PetPose::InteractA,
        PetPose::InteractB, PetPose::SleepA, PetPose::SleepB,
    ];
    let classes: HashSet<&'static str> =
        all.iter().map(|p| p.css_class()).collect();
    assert_eq!(classes.len(), 6);
    assert!(classes.iter().all(|s| s.starts_with("pose-")));
}
```

原程式缺少 <code>css_class()</code>，因此是<b>預期編譯 Red</b>（尚未執行）。本測試只覆蓋 pose 對應，不代表 Tauri 原生視窗驗收。

### Green：真實生活窗只渲染一個 img

以下完整替換現有 <code>PetStatus</code>（保留既有 <code>project_pose</code>、<code>BlinkController</code>、<code>AnimationClock</code>、<code>DragHandle</code>）：

```rs
// 在既有 enum PetPose 後新增以下實作：
impl PetPose {
    fn css_class(self) -> &'static str {
        match self {
            Self::Idle => "pose-idle",
            Self::Blink => "pose-blink",
            Self::InteractA => "pose-interact-a",
            Self::InteractB => "pose-interact-b",
            Self::SleepA => "pose-sleep-a",
            Self::SleepB => "pose-sleep-b",
        }
    }
}

// 將既有 PetStatus 函式整段替換；保留其餘原生事件／FSM 邏輯。
#[component]
pub fn PetStatus(
    snapshot: ReadSignal<Option<PetSnapshot>>,
    blinking: ReadSignal<bool>,
    animation_phase: ReadSignal<bool>,
) -> impl IntoView {
    let pose = move || snapshot.get()
        .map(|s| project_pose(s.state, blinking.get(), animation_phase.get()));
    let label = move || match pose() {
        Some(PetPose::Idle) => "待機",
        Some(PetPose::Blink) => "眨眼",
        Some(PetPose::InteractA | PetPose::InteractB) => "互動",
        Some(PetPose::SleepA | PetPose::SleepB) => "睡覺",
        None => "載入中",
    };
    let revision = move || snapshot.get()
        .map(|s| s.revision.to_string()).unwrap_or_else(|| "-".into());
    view! {
        <section class="pet-status">
            <div class="pet-face"
                on:contextmenu=move |event| {
                    event.prevent_default();
                    let (x,y) = (event.client_x(),event.client_y());
                    spawn_local(async move {
                        if let Err(error) = bridge::show_pet_context_menu(x,y).await {
                            web_sys::console::warn_1(&error.into());
                        }
                    });
                }>
                <img class=move || pose().map(PetPose::css_class)
                          .unwrap_or("pose-idle")
                    alt="寵物動畫"
                    src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 96 96'%3E%3Cellipse cx='45' cy='55' rx='32' ry='23' fill='%235c984c'/%3E%3Ccircle cx='71' cy='50' r='15' fill='%239dcc70'/%3E%3Ccircle cx='76' cy='45' r='3' fill='%23252f27'/%3E%3Cpath d='M25 48 Q45 18 65 48 M44 34 L44 75 M28 52 L60 52' fill='none' stroke='%232d5838' stroke-width='4'/%3E%3C/svg%3E"
                />
            </div>
            <strong>{label}</strong>
            <small>"revision "{revision}</small>
        </section>
    }
}
```

<code>apps/desktop_pet/frontend/style.css</code> 加入：

```css
/* 附加至 apps/desktop_pet/frontend/style.css */
.pet-face { display:grid; place-items:center; width:100%; height:56px; }
.pet-face > img { width:80px; height:56px; object-fit:contain;
    display:block; transform-origin:50% 80%; }
.pet-face > img.pose-idle { transform:translateY(0); }
.pet-face > img.pose-blink { transform:scaleY(.88); }
.pet-face > img.pose-interact-a { transform:translateY(-3px) rotate(-5deg); }
.pet-face > img.pose-interact-b { transform:translateY(-7px) rotate(5deg); }
.pet-face > img.pose-sleep-a { transform:scaleY(.85) translateY(5px); opacity:.90; }
.pet-face > img.pose-sleep-b { transform:scaleY(.82) translateY(5px); opacity:.85; }
.pet-life .pet-face { height:56px; overflow:hidden; }
@media (prefers-reduced-motion:reduce) {
  .pet-face > img { transition:none; }
}
```

單一 SVG data URL 隨著每幀 pose 套用不同 CSS transform，DOM 每窗始終只包含一個 <code>img</code> 而不是多文字 glyph；不增加 DOM 寵物分身。既有 <code>snapshot.revision</code>、右鍵原生選單／事件路徑保留。

### Refactor 與驗收邊界

將圖片與狀態標籤分離，<code>PetPose::css_class</code> 將 FSM 映射到有限且可測的六種類別。現有 <code>pet_windows::open</code> 同一 ID 重複出籠只 focus，同時對不同 UUID 分別建立 <code>pet-</code> 標籤；但真實主機必須用至少三個<b>不同 UUID</b> 開窗，個別記錄事件、顯示、回籠、重啟與 200×200 裁切，不能從 Rust unit test 或 capability JSON 推論 GUI 已通過。CSS 調整也要以真實視窗縮放與透明背景核對。

Day25 封存補教 <code>pet-day25-single-image-20261022</code> <b>維持 pending</b>；只有完成三個不同 pet ID 的原生 GUI 與事件、裁切、重啟驗證，才可視證據按 FIFO 結案。Day25 原文／程式／封存證據不回寫。

## ③ Cargo／套件版本

workspace edition 2024、version 0.2.25；frontend <code>leptos 0.8</code> CSR、<code>gloo-timers 0.4</code>、<code>wasm-bindgen 0.2</code>、<code>web-sys 0.3</code>。現有 Tauri 2 配置的版本應以 <code>apps/desktop_pet/Cargo.toml</code> 與 lock 為準。本日不新增 Rust 套件；沒有核實 Cargo.lock 精確解析版本／MSRV，且未執行 GUI 發布建置。

## ④ 驗證命令、預期／實際與 CI

- <code>cargo test -p desktop-pet-frontend day26_each_pose_selects_single_image_class</code>：預期 class 六種互異；<b>未執行</b>。
- <code>cargo check -p desktop-pet-frontend --target wasm32-unknown-unknown</code>：預期 Leptos 生活窗與動態 class 可編譯；<b>未執行</b>。
- <code>cd apps/desktop_pet/frontend && trunk build --release</code>：預期打包可載入 SVG data URL；<b>未執行</b>。
- 真實 Tauri GUI：建立 UUID A／B／C 三寵、三窗同時出籠 → 分別 Sleep/Interact → 確認每窗唯一 img 且互不干擾 → 關 A 後 B/C 保持 → 重啟回復 → 200×200 無裁切；<b>尚未驗收，不可用設定字串測試代替</b>。
- 來源 CI：<code>rust_ai_chat/main</code> <code>aaa9194a966aefae1ef4724f5d2b576b4289c550</code>，本次可核對 [Rust Dashboard Run 39651403341](https://github.com/williechen/rust_ai_chat/actions/runs/39651403341) success，但它並未測試這篇尚未套用的 SVG Green，<b>不是 Pet GUI 的成功證據</b>。
- 機制仍有限制：SVG 視覺動畫尚非真實逐種類 asset 或動作骨架；每窗狀態/選窗競態需要原生測試，未宣稱已實作完整漫遊、多寵物互動或產品驗收。

發布／測試／使用者套用彼此獨立；讀回全文後才可條件式更新 next\_day。
