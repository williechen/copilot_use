//! Tauri 原生視窗 Adapter；純座標規則留在 pet_domain，預設不漫遊。
use day9_pet::{
    interaction::{Action, Speed},
    roaming::{WorkArea, bounded_step},
};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use tauri::{Manager, State};

pub struct Roaming {
    enabled: AtomicBool,
    stalled: AtomicBool,
    moves: AtomicU64,
    menu_open: AtomicBool,
}
impl Roaming {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled: AtomicBool::new(enabled),
            stalled: AtomicBool::new(false),
            moves: AtomicU64::new(0),
            menu_open: AtomicBool::new(false),
        }
    }
}
#[tauri::command]
pub fn set_menu_open(state: State<'_, Roaming>, open: bool) {
    state.menu_open.store(open, Ordering::SeqCst);
}
#[tauri::command]
pub async fn set_roaming(
    state: State<'_, Roaming>,
    desktop: State<'_, super::DesktopState>,
    enabled: bool,
) -> Result<(), String> {
    desktop
        .store
        .set_roaming_enabled(enabled)
        .await
        .map_err(|_| "無法保存巡行設定")?;
    state.stalled.store(false, Ordering::SeqCst);
    state.enabled.store(enabled, Ordering::SeqCst);
    Ok(())
}
#[tauri::command]
pub fn roaming_status(
    app: tauri::AppHandle,
    state: State<'_, Roaming>,
) -> Result<serde_json::Value, String> {
    let pet = app.get_webview_window("pet").ok_or("找不到寵物視窗")?;
    let position = pet.outer_position().map_err(|_| "無法讀取視窗位置")?;
    let size = pet.outer_size().map_err(|_| "無法讀取視窗尺寸")?;
    let monitor = pet
        .current_monitor()
        .map_err(|_| "無法讀取監視器")?
        .ok_or("找不到監視器")?;
    let area = monitor.work_area();
    Ok(
        serde_json::json!({"enabled":state.enabled.load(Ordering::SeqCst),"stalled":state.stalled.load(Ordering::SeqCst),"moves":state.moves.load(Ordering::SeqCst),"x":position.x,"y":position.y,"width":size.width,"height":size.height,"area":{"x":area.position.x,"y":area.position.y,"width":area.size.width,"height":area.size.height}}),
    )
}
pub async fn run(app: tauri::AppHandle) {
    let mut velocity = (2, 1);
    let mut stalled = 0;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        if !app.state::<Roaming>().enabled.load(Ordering::SeqCst)
            || app.state::<Roaming>().menu_open.load(Ordering::SeqCst)
        {
            stalled = 0;
            continue;
        }
        let Some(pet) = app.get_webview_window("pet") else {
            return;
        };
        if !pet.is_visible().unwrap_or(false) {
            continue;
        }
        let store = app.state::<super::DesktopState>().store.clone();
        let Ok(Some(id)) = store.active_pet().await else {
            continue;
        };
        let Ok(id) = uuid::Uuid::parse_str(&id) else {
            continue;
        };
        let Ok(preferences) = store.interaction(id).await else {
            continue;
        };
        if preferences.action != Action::Moving {
            continue;
        }
        let speed = match preferences.speed {
            Speed::Slow => 1,
            Speed::Normal => 2,
            Speed::Fast => 4,
        };
        velocity = (
            if velocity.0 < 0 { -speed } else { speed },
            if velocity.1 < 0 { -1 } else { 1 },
        );
        let (Ok(before), Ok(size), Ok(Some(monitor))) = (
            pet.outer_position(),
            pet.outer_size(),
            pet.current_monitor(),
        ) else {
            continue;
        };
        let area = monitor.work_area();
        let next = bounded_step(
            (before.x, before.y),
            (size.width, size.height),
            WorkArea {
                x: area.position.x,
                y: area.position.y,
                width: area.size.width,
                height: area.size.height,
            },
            velocity,
        );
        velocity = next.velocity;
        if next.position == (before.x, before.y) {
            continue;
        }
        if !app.state::<Roaming>().enabled.load(Ordering::SeqCst)
            || app.state::<Roaming>().menu_open.load(Ordering::SeqCst)
        {
            continue;
        }
        if pet
            .set_position(tauri::PhysicalPosition::new(
                next.position.0,
                next.position.1,
            ))
            .is_err()
        {
            app.state::<Roaming>().stalled.store(true, Ordering::SeqCst);
            app.state::<Roaming>()
                .enabled
                .store(false, Ordering::SeqCst);
            continue;
        }
        tokio::time::sleep(std::time::Duration::from_millis(30)).await;
        if let Ok(after) = pet.outer_position() {
            if after == before {
                stalled += 1;
            } else {
                stalled = 0;
                app.state::<Roaming>().moves.fetch_add(1, Ordering::SeqCst);
            }
            if stalled >= 3 {
                app.state::<Roaming>().stalled.store(true, Ordering::SeqCst);
                app.state::<Roaming>()
                    .enabled
                    .store(false, Ordering::SeqCst);
            }
        }
    }
}
