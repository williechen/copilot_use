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
    organizer: std::sync::Mutex<Option<std::sync::Arc<day9_pet::AuthorizedRoot>>>,
    previews: std::sync::Mutex<std::collections::HashMap<Uuid, day9_pet::FilePreview>>,
}
impl DesktopState {
    fn authorized_root(&self) -> Result<std::sync::Arc<day9_pet::AuthorizedRoot>, String> {
        self.organizer
            .lock()
            .map_err(|_| "無法讀取授權目錄")?
            .clone()
            .ok_or_else(|| "尚未設定已授權目錄".into())
    }
    fn confirmed_preview(
        &self,
        id: Uuid,
    ) -> Result<
        (
            std::sync::Arc<day9_pet::AuthorizedRoot>,
            day9_pet::FilePreview,
        ),
        String,
    > {
        let roots = self.organizer.lock().map_err(|_| "無法讀取授權目錄")?;
        let root = roots.clone().ok_or("尚未設定已授權目錄")?;
        let preview = self
            .previews
            .lock()
            .map_err(|_| "無法讀取預覽")?
            .remove(&id)
            .ok_or("預覽已過期，請重新掃描")?;
        Ok((root, preview))
    }
}
#[tauri::command]
fn authorized_directory(state: State<'_, DesktopState>) -> Result<Option<String>, String> {
    Ok(state
        .organizer
        .lock()
        .map_err(|_| "無法讀取授權目錄")?
        .as_ref()
        .map(|root| root.authorized_path().to_string_lossy().into_owned()))
}
#[tauri::command]
fn authorize_directory(
    state: State<'_, DesktopState>,
    path: String,
    confirmed: bool,
) -> Result<String, String> {
    if !confirmed || path.len() > 4096 || !std::path::Path::new(&path).is_absolute() {
        return Err("請明確授權本機絕對路徑".into());
    }
    let root = day9_pet::AuthorizedRoot::new(&path)
        .map_err(|_| "目錄不存在，或不是本人可安全使用的目錄")?;
    let display = root.authorized_path().to_string_lossy().into_owned();
    let mut roots = state.organizer.lock().map_err(|_| "無法更新授權")?;
    state.previews.lock().map_err(|_| "無法清除舊預覽")?.clear();
    *roots = Some(std::sync::Arc::new(root));
    Ok(display)
}
#[tauri::command]
fn revoke_directory(state: State<'_, DesktopState>) -> Result<(), String> {
    let mut roots = state.organizer.lock().map_err(|_| "無法更新授權")?;
    state.previews.lock().map_err(|_| "無法清除舊預覽")?.clear();
    *roots = None;
    Ok(())
}
fn analytics_client() -> Result<(reqwest::Client, String, String), String> {
    let endpoint = std::env::var("ANALYTICS_ENDPOINT").map_err(|_| "尚未設定 Analytics")?;
    let base = endpoint
        .strip_suffix("/api/v1/events")
        .ok_or("Analytics 路徑不合法")?
        .to_owned();
    let token = std::env::var("ANALYTICS_PET_TOKEN").map_err(|_| "尚未設定 Analytics 寵物憑證")?;
    let client = reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|_| "無法建立連線")?;
    Ok((client, base, token))
}
#[tauri::command]
async fn apply_analytics_suggestion(
    state: State<'_, DesktopState>,
    pet: Uuid,
    id: Uuid,
    confirmed: bool,
) -> Result<serde_json::Value, String> {
    if !confirmed {
        return Err("請確認此設定變更".into());
    }
    let (client, base, token) = analytics_client()?;
    let proposal: serde_json::Value = client
        .get(format!("{base}/api/v1/suggestions/{id}"))
        .bearer_auth(&token)
        .send()
        .await
        .map_err(|_| "Analytics 無法連線")?
        .error_for_status()
        .map_err(|_| "建議不存在或未授權")?
        .json()
        .await
        .map_err(|_| "無法讀取建議")?;
    let mut receipt = state
        .store
        .apply_suggestion(pet, id, &proposal, confirmed)
        .await
        .map_err(|_| "建議未核准、已使用或無法保存")?;
    let ack=client.post(format!("{base}/api/v1/suggestions/{id}/receipt")).bearer_auth(token)
        .json(&serde_json::json!({"decision":"applied","receipt_id":receipt["receipt_id"],"value":receipt["value"]}))
        .send().await.is_ok_and(|r|r.status().is_success());
    receipt["analytics_acknowledged"] = serde_json::json!(ack);
    Ok(receipt)
}
#[tauri::command]
async fn rollback_analytics_suggestion(
    state: State<'_, DesktopState>,
    pet: Uuid,
    id: Uuid,
    confirmed: bool,
) -> Result<serde_json::Value, String> {
    if !confirmed {
        return Err("請確認回退此設定".into());
    }
    let (client, base, token) = analytics_client()?;
    let mut receipt = state
        .store
        .rollback_suggestion(pet, id, confirmed)
        .await
        .map_err(|_| "設定已被修改、沒有套用收據或無法回退")?;
    let ack=client.post(format!("{base}/api/v1/suggestions/{id}/receipt")).bearer_auth(token)
        .json(&serde_json::json!({"decision":"rolled_back","receipt_id":receipt["receipt_id"],"value":receipt["value"]}))
        .send().await.is_ok_and(|r|r.status().is_success());
    receipt["analytics_acknowledged"] = serde_json::json!(ack);
    Ok(receipt)
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
    copilot_telemetry::observe("pet.learning.review", async {
        state
            .store
            .review(pet, id, quality)
            .await
            .map_err(|_| "無法保存複習結果".into())
    })
    .await
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
async fn search_memory_tool(
    state: State<'_, DesktopState>,
    pet: Uuid,
    query: String,
) -> Result<serde_json::Value, String> {
    if query.len() > 4096 || query.trim().is_empty() {
        return Err("搜尋詞太長或空白".into());
    }
    let executable = std::env::current_exe().map_err(|_| "無法啟動本機工具")?;
    state
        .store
        .stdio_memory_search(pet, &query, &executable)
        .await
        .map_err(|_| "記憶工具未完成".into())
}
#[tauri::command]
async fn pet_agent(
    state: State<'_, DesktopState>,
    pet: Uuid,
    text: String,
) -> Result<serde_json::Value, String> {
    let _permit = state
        .ai_slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| "AI 正忙碌，請稍後重試")?;
    let executable = std::env::current_exe().map_err(|_| "無法啟動本機 MCP")?;
    copilot_telemetry::observe("pet.agent", state.store.agent_talk(pet, &text, &executable))
        .await
        .map_err(|_| "Agent 未完成或工具未授權".into())
}
fn mcp_once() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{BufRead, Read, Write};
    let database = std::env::var_os("PET_MCP_DATABASE").ok_or("missing MCP database")?;
    let pet: Uuid = std::env::var("PET_MCP_PET")?.parse()?;
    let mut bytes = Vec::new();
    std::io::stdin()
        .lock()
        .take(65537)
        .read_until(b'\n', &mut bytes)?;
    if bytes.len() > 65536 {
        return Err("MCP request too large".into());
    }
    let request: serde_json::Value = serde_json::from_slice(&bytes)?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let response = runtime.block_on(async {
        let store = PetStore::open_readonly(std::path::Path::new(&database)).await?;
        Ok::<_, Box<dyn std::error::Error>>(store.mcp_request(pet, request).await)
    })?;
    std::io::stdout()
        .lock()
        .write_all(format!("{}\n", response).as_bytes())?;
    Ok(())
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
    let roots = state.organizer.lock().map_err(|_| "無法讀取授權目錄")?;
    let root = roots.as_ref().ok_or("尚未設定已授權目錄")?;
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
    copilot_telemetry::observe("pet.file.cleanup", async {
        if !confirmed {
            return Err("操作尚未確認".into());
        }
        let (root, preview) = state.confirmed_preview(id)?;
        let receipt = root
            .confirm_quarantine(&preview, true)
            .map_err(|e| e.to_string())?;
        let event = receipt.event();
        state
            .store
            .record_cleanup(&receipt, &event)
            .await
            .map_err(|_| {
                format!(
                    "檔案已隔離，請復原收據以保存事件；操作 {}",
                    receipt.operation_id
                )
            })?;
        Ok(receipt)
    })
    .await
}
#[tauri::command]
async fn recover_cleanup_receipts(
    state: State<'_, DesktopState>,
) -> Result<Vec<day9_pet::CleanupReceipt>, String> {
    copilot_telemetry::observe("pet.file.recover", async {
        let root = state.authorized_root()?;
        let receipts = root
            .recovered_receipts()
            .map_err(|_| "無法安全核對復原紀錄")?;
        for receipt in &receipts {
            state
                .store
                .record_cleanup(receipt, &receipt.event())
                .await
                .map_err(|_| "收據仍無法保存，請稍後重試")?;
        }
        Ok(receipts)
    })
    .await
}
#[tauri::command]
async fn outbox_status(state: State<'_, DesktopState>) -> Result<serde_json::Value, String> {
    state
        .store
        .outbox_status()
        .await
        .map_err(|_| "無法讀取事件狀態".into())
}
#[tauri::command]
async fn requeue_failed_events(state: State<'_, DesktopState>) -> Result<u64, String> {
    state
        .store
        .requeue_blocked_events()
        .await
        .map_err(|_| "無法重試事件".into())
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
    if std::env::args().any(|arg| arg == "--mcp-once") {
        if mcp_once().is_err() {
            std::process::exit(1);
        }
        return;
    }

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
                if std::env::var("PET_GUI_TEST_ANALYTICS").as_deref() == Ok("true") {
                    let _ = webview.eval("window.__PET_TEST_ANALYTICS=true;");
                    if let Some(id) = std::env::var("PET_GUI_TEST_SUGGESTION")
                        .ok()
                        .and_then(|s| Uuid::parse_str(&s).ok())
                    {
                        let _ = webview.eval(&format!("window.__PET_TEST_SUGGESTION='{}';", id));
                    }
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
                organizer: std::sync::Mutex::new(
                    std::env::var_os("PET_AUTHORIZED_ROOT")
                        .and_then(|path| day9_pet::AuthorizedRoot::new(path).ok())
                        .map(std::sync::Arc::new),
                ),
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
            apply_analytics_suggestion,
            rollback_analytics_suggestion,
            gui_test_result,
            gui_test_progress,
            select_pet,
            active_pet,
            pet_talk,
            pet_agent,
            search_memory_tool,
            generate_asset,
            load_asset,
            authorized_directory,
            authorize_directory,
            revoke_directory,
            scan_file_preview,
            confirm_cleanup,
            recover_cleanup_receipts,
            outbox_status,
            requeue_failed_events
        ])
        .run(tauri::generate_context!())
        .expect("無法啟動桌面寵物");
}
