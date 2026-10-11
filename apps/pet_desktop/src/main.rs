//! Tauri Adapter：只暴露受型別限制的 Profile／Memory／Review 指令。
use day9_pet::{
    profile::PetProfile,
    state::{Card, Memory, PetItem, PetStore},
};
use tauri::{Emitter, Manager, State};
use uuid::Uuid;
struct DesktopState {
    store: PetStore,
    ai_slots: std::sync::Arc<tokio::sync::Semaphore>,
    organizer: Option<day9_pet::AuthorizedRoot>,
    previews: std::sync::Mutex<std::collections::HashMap<Uuid, day9_pet::FilePreview>>,
}
#[tauri::command]
async fn list_pets(state: State<'_, DesktopState>) -> Result<Vec<PetItem>, String> {
    state.store.pets().await.map_err(|_| "無法讀取寵物".into())
}
#[tauri::command]
async fn save_profile(
    state: State<'_, DesktopState>,
    id: Uuid,
    profile: PetProfile,
) -> Result<(), String> {
    state
        .store
        .save_profile(id, &profile)
        .await
        .map_err(|_| "寵物設定不合法或無法保存".into())
}
#[tauri::command]
async fn list_memories(state: State<'_, DesktopState>, pet: Uuid) -> Result<Vec<Memory>, String> {
    state
        .store
        .memories(pet)
        .await
        .map_err(|_| "無法讀取記憶".into())
}
#[tauri::command]
async fn save_memory(
    state: State<'_, DesktopState>,
    pet: Uuid,
    id: Uuid,
    content: String,
) -> Result<(), String> {
    state
        .store
        .save_memory(pet, id, &content)
        .await
        .map_err(|_| "無法保存記憶".into())
}
#[tauri::command]
async fn delete_memory(state: State<'_, DesktopState>, pet: Uuid, id: Uuid) -> Result<(), String> {
    state
        .store
        .delete_memory(pet, id)
        .await
        .map_err(|_| "無法刪除記憶".into())
}
#[tauri::command]
async fn list_cards(state: State<'_, DesktopState>, pet: Uuid) -> Result<Vec<Card>, String> {
    state
        .store
        .cards(pet)
        .await
        .map_err(|_| "無法讀取複習項目".into())
}
#[tauri::command]
async fn add_card(
    state: State<'_, DesktopState>,
    pet: Uuid,
    question: String,
    answer: String,
) -> Result<(), String> {
    state
        .store
        .add_card(pet, &question, &answer)
        .await
        .map_err(|_| "無法保存複習題目".into())
}
#[tauri::command]
async fn review_card(
    state: State<'_, DesktopState>,
    pet: Uuid,
    id: Uuid,
    quality: u8,
) -> Result<Card, String> {
    state
        .store
        .review(pet, id, quality)
        .await
        .map_err(|_| "無法保存複習結果".into())
}
#[tauri::command]
async fn pet_talk(
    app: tauri::AppHandle,
    state: State<'_, DesktopState>,
    pet: Uuid,
    request: Uuid,
    text: String,
) -> Result<String, String> {
    let _permit = state
        .ai_slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| "AI 正忙碌，請稍後重試")?;
    copilot_telemetry::observe(
        "pet.talk",
        state.store.talk(pet, &text, move |delta| {
            let _ = app.emit(
                "pet-reply-delta",
                serde_json::json!({"request":request,"text":delta}),
            );
        }),
    )
    .await
    .map_err(|e| e.to_string())
}
#[tauri::command]
async fn generate_asset(
    state: State<'_, DesktopState>,
    pet: Uuid,
) -> Result<serde_json::Value, String> {
    let _permit = state
        .ai_slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| "AI 正忙碌，請稍後重試")?;
    state
        .store
        .generate_asset(pet)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
async fn load_asset(
    state: State<'_, DesktopState>,
    pet: Uuid,
) -> Result<serde_json::Value, String> {
    state
        .store
        .asset(pet)
        .await
        .map_err(|_| "無法讀取角色素材".into())
}
#[tauri::command]
fn scan_file_preview(state: State<'_, DesktopState>) -> Result<Vec<serde_json::Value>, String> {
    let root = state.organizer.as_ref().ok_or("尚未設定已授權目錄")?;
    let items = root.scan().map_err(|_| "无法掃描已授權目錄")?;
    let mut previews = state.previews.lock().map_err(|_| "無法讀取預覽")?;
    previews.clear();
    let mut result = Vec::new();
    for item in items {
        let id = Uuid::new_v4();
        result
            .push(serde_json::json!({"id":id,"name":item.file_name,"size_bytes":item.size_bytes}));
        previews.insert(id, item);
    }
    Ok(result)
}
#[tauri::command]
async fn confirm_cleanup(
    state: State<'_, DesktopState>,
    id: Uuid,
    confirmed: bool,
) -> Result<day9_pet::CleanupReceipt, String> {
    if !confirmed {
        return Err("操作尚未確認".into());
    }
    let preview = state
        .previews
        .lock()
        .map_err(|_| "無法讀取預覽")?
        .remove(&id)
        .ok_or("預覽已過期，請重新掃描")?;
    let root = state.organizer.as_ref().ok_or("尚未設定已授權目錄")?;
    let receipt = root
        .confirm_quarantine(&preview, true)
        .map_err(|e| e.to_string())?;
    let event = receipt.event();
    state
        .store
        .record_cleanup(&receipt, &event)
        .await
        .map_err(|_| "檔案已隔離，但事件保存失敗；請保留操作收據")?;
    Ok(receipt)
}
#[tauri::command]
async fn select_pet(
    app: tauri::AppHandle,
    state: State<'_, DesktopState>,
    pet: Uuid,
) -> Result<(), String> {
    state
        .store
        .select_pet(pet)
        .await
        .map_err(|_| "無法選擇寵物")?;
    app.emit("active-pet-changed", pet)
        .map_err(|_| "無法更新桌面角色")?;
    Ok(())
}
#[tauri::command]
async fn active_pet(state: State<'_, DesktopState>) -> Result<Option<String>, String> {
    state
        .store
        .active_pet()
        .await
        .map_err(|_| "無法讀取目前寵物".into())
}
#[tauri::command]
fn gui_test_progress(phase: String) -> Result<(), String> {
    if !cfg!(debug_assertions)
        || std::env::var("PET_GUI_TEST").as_deref() != Ok("true")
        || phase.len() > 64
    {
        return Err("測試入口未啟用".into());
    }
    let path = std::env::var("PET_GUI_TEST_REPORT").map_err(|_| "未設定測試報告位置")?;
    std::fs::write(format!("{path}.progress"), phase).map_err(|_| "無法寫入測試階段".to_owned())
}
#[tauri::command]
fn gui_test_result(app: tauri::AppHandle, success: bool, message: String) -> Result<(), String> {
    if !cfg!(debug_assertions) || std::env::var("PET_GUI_TEST").as_deref() != Ok("true") {
        return Err("測試入口未啟用".into());
    }
    if success {
        let pet = app.get_webview_window("pet").ok_or("浮動寵物視窗不存在")?;
        let size = pet.inner_size().map_err(|_| "無法取得寵物視窗尺寸")?;
        let scale = pet.scale_factor().map_err(|_| "無法取得顯示比例")?;
        if (size.width as f64 / scale - 200.0).abs() > 1.0
            || (size.height as f64 / scale - 200.0).abs() > 1.0
        {
            return Err(format!(
                "浮動寵物視窗尺寸不是 200×200：physical={}×{}, scale={}",
                size.width, size.height, scale
            ));
        }
    }
    let path = std::env::var("PET_GUI_TEST_REPORT").map_err(|_| "未設定測試報告位置")?;
    std::fs::write(
        path,
        serde_json::json!({"success":success,"message":message}).to_string(),
    )
    .map_err(|_| "無法寫入測試報告")?;
    app.exit(if success { 0 } else { 1 });
    Ok(())
}
fn main() {
    let _telemetry = copilot_telemetry::init("desktop_pet").expect("無法初始化觀測設定");
    #[cfg(unix)]
    if unsafe { libc::geteuid() } == 0 {
        eprintln!("桌面寵物不得以 root 執行");
        std::process::exit(1);
    }
    tauri::Builder::default()
        .on_page_load(|webview, payload| {
            if webview.label() == "main"
                && cfg!(debug_assertions)
                && std::env::var("PET_GUI_TEST").as_deref() == Ok("true")
                && matches!(payload.event(), tauri::webview::PageLoadEvent::Finished)
            {
                if std::env::var("PET_GUI_TEST_AI").as_deref() == Ok("true") {
                    let _ = webview.eval("window.__PET_TEST_LIVE_AI=true;");
                }
                if std::env::var("PET_GUI_TEST_IMAGE").as_deref() == Ok("true") {
                    let _ = webview.eval("window.__PET_TEST_LIVE_IMAGE=true;");
                }
                let _ = webview.eval(include_str!("../../../scripts/gui/pet-self-test.js"));
            }
        })
        .setup(|app| {
            #[cfg(target_os = "linux")]
            if let Some(pet) = app.get_webview_window("pet") {
                use gtk::prelude::*;
                pet.gtk_window()?.set_titlebar(None::<&gtk::Widget>);
                pet.set_size(tauri::LogicalSize::new(200.0, 200.0))?;
            }

            let directory = std::env::var_os("PET_DATA_DIR")
                .map(std::path::PathBuf::from)
                .unwrap_or(app.path().app_data_dir()?);
            std::fs::create_dir_all(&directory)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700))?;
            }
            let store =
                tauri::async_runtime::block_on(PetStore::open(&directory.join("pet.sqlite")))?;
            if let (Ok(endpoint), Ok(token)) = (
                std::env::var("ANALYTICS_ENDPOINT"),
                std::env::var("ANALYTICS_PET_TOKEN"),
            ) {
                if token.len() < 16
                    || !(endpoint.starts_with("https://")
                        || endpoint.starts_with("http://127.0.0.1:")
                        || endpoint.starts_with("http://localhost:"))
                {
                    return Err("統計服務設定不合法".into());
                }
                let worker = store.clone();
                tauri::async_runtime::spawn(async move {
                    loop {
                        let _ = worker.dispatch_once(&endpoint, &token).await;
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    }
                });
            }
            app.manage(DesktopState {
                store,
                ai_slots: std::sync::Arc::new(tokio::sync::Semaphore::new(2)),
                organizer: std::env::var_os("PET_AUTHORIZED_ROOT")
                    .map(day9_pet::AuthorizedRoot::new)
                    .transpose()?,
                previews: std::sync::Mutex::new(std::collections::HashMap::new()),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_pets,
            save_profile,
            list_memories,
            save_memory,
            delete_memory,
            list_cards,
            add_card,
            review_card,
            gui_test_result,
            gui_test_progress,
            select_pet,
            active_pet,
            pet_talk,
            generate_asset,
            load_asset,
            scan_file_preview,
            confirm_cleanup
        ])
        .run(tauri::generate_context!())
        .expect("無法啟動桌面寵物");
}
