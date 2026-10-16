//! Tauri Adapter：只暴露受型別限制的 Profile／Memory／Review 指令。
mod roaming;
mod updater;
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
async fn create_snapshot(
    state: State<'_, DesktopState>,
) -> Result<day9_pet::backup::Snapshot, String> {
    state
        .store
        .create_snapshot()
        .await
        .map_err(|_| "無法建立 SQLite 與素材快照".into())
}
#[tauri::command]
fn list_snapshots(
    state: State<'_, DesktopState>,
) -> Result<Vec<day9_pet::backup::Snapshot>, String> {
    state
        .store
        .snapshots()
        .map_err(|_| "無法讀取備份清單".into())
}
#[tauri::command]
async fn restore_snapshot(
    app: tauri::AppHandle,
    state: State<'_, DesktopState>,
    id: Uuid,
    confirmed: bool,
) -> Result<day9_pet::backup::Snapshot, String> {
    let before = state
        .store
        .restore_snapshot(id, confirmed)
        .await
        .map_err(|error| {
            if error.to_string() == "snapshot_restored_projection_pending" {
                "資料快照已還原，但本機檔案尚未同步；請檢查資料目錄後重新開啟。".to_owned()
            } else {
                "快照確認、完整性、相容性檢查或還原失敗；目前資料仍保留".to_owned()
            }
        })?;
    let _ = app.emit("active-pet-changed", ());
    Ok(before)
}
#[tauri::command]
async fn interaction(
    state: State<'_, DesktopState>,
    pet: Uuid,
) -> Result<day9_pet::interaction::Interaction, String> {
    state
        .store
        .interaction(pet)
        .await
        .map_err(|_| "無法讀取互動設定".into())
}
#[tauri::command]
async fn set_interaction(
    app: tauri::AppHandle,
    state: State<'_, DesktopState>,
    pet: Uuid,
    preferences: day9_pet::interaction::Interaction,
) -> Result<(), String> {
    state
        .store
        .set_interaction(pet, &preferences)
        .await
        .map_err(|_| "無法保存互動設定".to_owned())?;
    app.emit("active-pet-changed", ())
        .map_err(|_| "無法更新動畫".into())
}
#[tauri::command]
async fn learning_summary(
    state: State<'_, DesktopState>,
    pet: Uuid,
) -> Result<day9_pet::interaction::LearningSummary, String> {
    state
        .store
        .learning_summary(pet)
        .await
        .map_err(|_| "無法讀取複習摘要".into())
}
#[tauri::command]
async fn recall_prompt(
    state: State<'_, DesktopState>,
) -> Result<Option<serde_json::Value>, String> {
    state
        .store
        .recall_prompt()
        .await
        .map_err(|_| "無法讀取回想提醒".into())
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
    day9_pet::application::save_profile(&state.store, id, &profile)
        .await
        .map_err(|error| match error {
            day9_pet::application::ProfileError::ProjectionPending => {
                "設定已保存，但本機檔案同步未完成。請檢查程式資料目錄後重新開啟。".into()
            }
            day9_pet::application::ProfileError::Invalid => "寵物設定不合法".into(),
            day9_pet::application::ProfileError::Unavailable => "無法保存寵物設定".into(),
        })
}
#[tauri::command]
async fn archived_pets(state: State<'_, DesktopState>) -> Result<Vec<PetItem>, String> {
    state
        .store
        .archived_pets()
        .await
        .map_err(|_| "無法讀取封存寵物".into())
}
#[tauri::command]
async fn set_pet_archived(
    app: tauri::AppHandle,
    state: State<'_, DesktopState>,
    pet: Uuid,
    archived: bool,
    confirmed: bool,
) -> Result<serde_json::Value, String> {
    let receipt = state
        .store
        .set_pet_archived(pet, archived, confirmed)
        .await
        .map_err(|_| "無法移除或還原寵物".to_owned())?;
    app.emit("active-pet-changed", serde_json::json!({}))
        .map_err(|_| "無法更新浮動寵物")?;
    Ok(receipt)
}
#[tauri::command]
async fn profile_file(state: State<'_, DesktopState>, pet: Uuid) -> Result<String, String> {
    if pet.is_nil()
        || !state
            .store
            .pets()
            .await
            .map_err(|_| "無法讀取寵物")?
            .iter()
            .any(|p| p.id == pet.to_string())
    {
        return Err("寵物不存在".into());
    }
    Ok(state
        .store
        .database_path()
        .parent()
        .ok_or("設定目錄不存在")?
        .join("pets")
        .join(pet.to_string())
        .join("pet.toml")
        .to_string_lossy()
        .into_owned())
}
#[tauri::command]
async fn import_pet_profile(
    state: State<'_, DesktopState>,
    pet: Uuid,
    confirmed: bool,
) -> Result<(), String> {
    state
        .store
        .import_pet_profile(pet, confirmed)
        .await
        .map_err(|_| "無法匯入此寵物的本機設定檔".into())
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
    app: tauri::AppHandle,
    state: State<'_, DesktopState>,
    pet: Uuid,
) -> Result<serde_json::Value, String> {
    let _permit = state
        .ai_slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| "AI 正忙碌，請稍後重試")?;
    let asset = state
        .store
        .generate_asset(pet)
        .await
        .map_err(|e| e.to_string())?;
    let _ = app.emit("active-pet-changed", pet);
    Ok(asset)
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
async fn scan_duplicate_preview(
    state: State<'_, DesktopState>,
) -> Result<Vec<Vec<serde_json::Value>>, String> {
    let root = state.authorized_root()?;
    let worker = root.clone();
    let groups = tauri::async_runtime::spawn_blocking(move || worker.duplicate_groups())
        .await
        .map_err(|_| "無法完成重複檔預覽")?
        .map_err(|_| "檔案已變更、不安全或超出預覽讀取上限")?;
    let roots = state.organizer.lock().map_err(|_| "無法讀取授權目錄")?;
    if !roots
        .as_ref()
        .is_some_and(|current| std::sync::Arc::ptr_eq(current, &root))
    {
        return Err("目錄授權已變更，請重新預覽".into());
    }
    let mut previews = state.previews.lock().map_err(|_| "無法讀取預覽")?;
    previews.clear();
    Ok(groups.into_iter().enumerate().map(|(index,group)|group.files.into_iter().map(|item|{
        let id=Uuid::new_v4();
        let value=serde_json::json!({"id":id,"name":item.file_name,"size_bytes":item.size_bytes,"duplicate_group":index+1});
        previews.insert(id,item);
        value
    }).collect()).collect())
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
fn list_trash(state: State<'_, DesktopState>) -> Result<Vec<day9_pet::TrashEntry>, String> {
    state
        .authorized_root()?
        .trash()
        .map_err(|_| "無法安全核對回收區".into())
}
#[tauri::command]
async fn restore_file(
    state: State<'_, DesktopState>,
    id: Uuid,
    confirmed: bool,
) -> Result<day9_pet::RestoreReceipt, String> {
    if !confirmed {
        return Err("尚未確認還原".into());
    }
    copilot_telemetry::observe("pet.file.restore", async {
        let root = state.authorized_root()?;
        let entry = root
            .trash()
            .map_err(|_| "無法核對回收區")?
            .into_iter()
            .find(|item| item.operation_id == id)
            .ok_or("項目不存在或身份已改變")?;
        let original = day9_pet::CleanupReceipt {
            operation_id: id,
            quarantined_file: format!("{id}.item"),
            size_bytes: entry.size_bytes,
        };
        state
            .store
            .record_cleanup(&original, &original.event())
            .await
            .map_err(|_| "無法保存原隔離收據")?;
        let receipt = root.confirm_restore(id, true).map_err(|e| e.to_string())?;
        state
            .store
            .record_restoration(&receipt)
            .await
            .map_err(|_| format!("檔案已還原，請復原還原收據：{}", receipt.operation_id))?;
        Ok(receipt)
    })
    .await
}
#[tauri::command]
async fn recover_restore_receipts(
    state: State<'_, DesktopState>,
) -> Result<Vec<day9_pet::RestoreReceipt>, String> {
    let root = state.authorized_root()?;
    let receipts = root
        .restored_receipts()
        .map_err(|_| "無法安全核對還原紀錄")?;
    for receipt in &receipts {
        state
            .store
            .record_restoration(receipt)
            .await
            .map_err(|_| "仍無法保存還原事件")?;
    }
    Ok(receipts)
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
fn gui_acceptance_enabled() -> bool {
    std::env::var("PET_GUI_TEST").as_deref() == Ok("true")
        && (cfg!(debug_assertions) || std::env::args().any(|arg| arg == "--acceptance-gui"))
}
fn validate_acceptance_fixture() -> Result<(), String> {
    if !gui_acceptance_enabled() {
        return Ok(());
    }
    let state =
        std::path::PathBuf::from(std::env::var_os("PET_DATA_DIR").ok_or("missing isolated state")?);
    let report =
        std::path::PathBuf::from(std::env::var_os("PET_GUI_TEST_REPORT").ok_or("missing report")?);
    let root = std::path::PathBuf::from(
        std::env::var_os("PET_AUTHORIZED_ROOT").ok_or("missing fixture root")?,
    );
    let parent = root.parent().ok_or("missing fixture parent")?;
    if state.exists()
        || report.exists()
        || state.parent() != Some(parent)
        || report.parent() != Some(parent)
        || std::fs::symlink_metadata(&root)
            .map_err(|_| "missing fixture root")?
            .file_type()
            .is_symlink()
        || std::fs::read(root.join("sample.txt")).map_err(|_| "missing sample fixture")?
            != b"disposable GUI verification fixture"
        || std::fs::read(root.join("second.txt")).map_err(|_| "missing confirmation fixture")?
            != b"disposable UI confirmation fixture"
    {
        return Err("acceptance requires fresh disposable fixture".into());
    }
    Ok(())
}
#[tauri::command]
fn gui_test_progress(phase: String) -> Result<(), String> {
    if !gui_acceptance_enabled() || phase.len() > 64 {
        return Err("測試入口未啟用".into());
    }
    let path = std::env::var("PET_GUI_TEST_REPORT").map_err(|_| "未設定測試報告位置")?;
    std::fs::write(format!("{path}.progress"), phase).map_err(|_| "無法寫入測試階段".to_owned())
}
#[tauri::command]
fn gui_test_result(app: tauri::AppHandle, success: bool, message: String) -> Result<(), String> {
    if !gui_acceptance_enabled() {
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
        serde_json::json!({"success":success,"message":message,"version":app.package_info().version.to_string(),"source_sha":env!("COPILOT_SOURCE_SHA"),"source_dirty":env!("COPILOT_SOURCE_DIRTY")=="true","release":!cfg!(debug_assertions)}).to_string(),
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

    let context: tauri::Context<tauri::Wry> = tauri::generate_context!();
    if std::env::args().any(|arg| arg == "--version") {
        println!("{}", context.package_info().version);
        return;
    }
    if let Err(error) = validate_acceptance_fixture() {
        eprintln!("{error}");
        std::process::exit(1);
    }
    let _telemetry = copilot_telemetry::init("desktop_pet").expect("無法初始化觀測設定");
    #[cfg(unix)]
    if unsafe { libc::geteuid() } == 0 {
        eprintln!("桌面寵物不得以 root 執行");
        std::process::exit(1);
    }
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(updater::Updates::new())
        .manage(roaming::Roaming::from_environment())
        .on_page_load(|webview, payload| {
            if webview.label() == "main"
                && gui_acceptance_enabled()
                && matches!(payload.event(), tauri::webview::PageLoadEvent::Finished)
            {
                if let Ok(version) = std::env::var("PET_GUI_TEST_UPDATE_VERSION") {
                    let literal = serde_json::to_string(&version).unwrap_or_default();
                    let _ = webview.eval(&format!("window.__PET_TEST_UPDATE_VERSION={literal};"));
                }
                if std::env::var("PET_GUI_TEST_FUNCTIONAL_ONLY").as_deref() == Ok("true") {
                    let _ = webview.eval("window.__PET_TEST_FUNCTIONAL_ONLY=true;");
                }
                if std::env::var("PET_GUI_TEST_AI").as_deref() == Ok("true") {
                    let _ = webview.eval("window.__PET_TEST_LIVE_AI=true;");
                }
                if std::env::var("PET_GUI_TEST_INSTALL_UPDATE").as_deref() == Ok("true") {
                    let _ = webview.eval("window.__PET_TEST_INSTALL_UPDATE=true;");
                }
                if std::env::var("PET_GUI_TEST_UPDATER").as_deref() == Ok("true") {
                    let _ = webview.eval("window.__PET_TEST_UPDATER=true;");
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
            day9_pet::files::prepare_data_directory(&directory)
                .map_err(|_| std::io::Error::other("資料目錄不安全"))?;
            let store =
                tauri::async_runtime::block_on(PetStore::open(&directory.join("pet.sqlite")))?;
            tauri::async_runtime::block_on(store.reconcile_pet_files_startup())
                .map_err(|_| std::io::Error::other("無法同步寵物設定檔"))?;
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
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(roaming::run(handle));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_pets,
            updater::check_local_update,
            updater::verify_local_update,
            updater::install_local_update,
            create_snapshot,
            list_snapshots,
            restore_snapshot,
            interaction,
            set_interaction,
            learning_summary,
            recall_prompt,
            save_profile,
            import_pet_profile,
            profile_file,
            archived_pets,
            set_pet_archived,
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
            scan_duplicate_preview,
            roaming::set_roaming,
            roaming::roaming_status,
            confirm_cleanup,
            recover_cleanup_receipts,
            list_trash,
            restore_file,
            recover_restore_receipts,
            outbox_status,
            requeue_failed_events
        ])
        .run(context)
        .expect("無法啟動桌面寵物");
}
