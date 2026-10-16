//! Local signed channel only. The UI explicitly checks, verifies and confirms installation.
use base64::Engine;
use serde_json::{Value, json};
use std::{sync::Mutex, time::Duration};
use tauri::{Emitter, State};
use tauri_plugin_updater::{Update, UpdaterExt};
use uuid::Uuid;
struct Pending {
    id: Uuid,
    update: Update,
    key: String,
    bytes: Option<Vec<u8>>,
}
pub struct Updates {
    pending: Mutex<Option<Pending>>,
    slots: tokio::sync::Semaphore,
}
impl Updates {
    pub fn new() -> Self {
        Self {
            pending: Mutex::new(None),
            slots: tokio::sync::Semaphore::new(1),
        }
    }
}
fn local(url: &url::Url) -> bool {
    matches!(url.scheme(), "http" | "https")
        && url.username().is_empty()
        && url.password().is_none()
        && url.fragment().is_none()
        && url.query().is_none()
        && match url.host() {
            Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
            Some(url::Host::Ipv6(ip)) => ip.is_loopback(),
            _ => false,
        }
}
pub(crate) fn verify(
    bytes: &[u8],
    signature: &str,
    key: &str,
    version: &str,
) -> Result<(), String> {
    let decode = |s: &str| {
        base64::engine::general_purpose::STANDARD
            .decode(s)
            .ok()
            .and_then(|b| String::from_utf8(b).ok())
            .ok_or("簽章格式不合法".to_owned())
    };
    let key = minisign_verify::PublicKey::decode(&decode(key)?).map_err(|_| "公鑰格式不合法")?;
    let signature =
        minisign_verify::Signature::decode(&decode(signature)?).map_err(|_| "簽章格式不合法")?;
    key.verify(bytes, &signature, true)
        .map_err(|_| "更新檔簽章驗證失敗")?;
    let signed = signature
        .trusted_comment()
        .split('\t')
        .find_map(|field| field.strip_prefix("version:"))
        .ok_or("簽章未包含版本")?;
    let parse =
        |s: &str| semver::Version::parse(s.trim_start_matches('v')).map_err(|_| "版本不合法");
    if parse(signed)? != parse(version)? {
        return Err("簽章版本與公告版本不同".into());
    }
    Ok(())
}
#[tauri::command]
pub async fn check_local_update(
    app: tauri::AppHandle,
    state: State<'_, Updates>,
) -> Result<Value, String> {
    let _permit = state.slots.try_acquire().map_err(|_| "更新工作執行中")?;
    *state.pending.lock().map_err(|_| "無法讀取更新狀態")? = None;
    let endpoint = url::Url::parse(
        &std::env::var("PET_UPDATE_ENDPOINT").map_err(|_| "尚未設定本地更新 channel")?,
    )
    .map_err(|_| "更新網址不合法")?;
    if !local(&endpoint) {
        return Err("更新 channel 必須是 numeric loopback 網址".into());
    }
    let key = std::env::var("PET_UPDATE_PUBLIC_KEY").map_err(|_| "尚未設定可信更新公鑰")?;
    if key.is_empty() || key.len() > 4096 {
        return Err("可信公鑰不合法".into());
    }
    let builder = app
        .updater_builder()
        .endpoints(vec![endpoint.clone()])
        .map_err(|_| "更新 channel 不合法")?
        .pubkey(&key)
        .no_proxy()
        .clear_headers()
        .timeout(Duration::from_secs(20))
        .configure_client(|builder| builder.redirect(updater_http::redirect::Policy::none()));
    #[cfg(windows)]
    let builder = {
        // NSIS /D is last and points only to this running application's directory.
        // A manifest cannot select an arbitrary installation destination.
        let executable = std::env::current_exe().map_err(|_| "無法確認目前安裝目錄")?;
        let mut destination = std::ffi::OsString::from("/D=");
        destination.push(executable.parent().ok_or("無法確認目前安裝目錄")?);
        builder
            .clear_installer_args()
            .installer_arg("/S")
            .installer_arg(destination)
    };
    let updater = builder.build().map_err(|_| "此安裝格式不支援自動更新")?;
    let Some(update) = updater.check().await.map_err(|_| "無法檢查本地更新")? else {
        return Ok(json!({"status":"up_to_date"}));
    };
    if !local(&update.download_url) || update.download_url.origin() != endpoint.origin() {
        return Err("更新檔必須位於同一本地 channel".into());
    }
    let id = Uuid::new_v4();
    let result = json!({"status":"available","id":id,"version":update.version,"current_version":update.current_version});
    *state.pending.lock().map_err(|_| "無法保存更新狀態")? = Some(Pending {
        id,
        update,
        key,
        bytes: None,
    });
    Ok(result)
}
#[tauri::command]
pub async fn verify_local_update(state: State<'_, Updates>, id: Uuid) -> Result<Value, String> {
    let _permit = state.slots.try_acquire().map_err(|_| "更新工作執行中")?;
    let (update, key) = {
        let pending = state.pending.lock().map_err(|_| "無法讀取更新狀態")?;
        let pending = pending
            .as_ref()
            .filter(|p| p.id == id)
            .ok_or("更新檢查已過期")?;
        (pending.update.clone(), pending.key.clone())
    };
    let mut response = updater_http::Client::builder()
        .no_proxy()
        .redirect(updater_http::redirect::Policy::none())
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|_| "無法建立更新連線")?
        .get(update.download_url.clone())
        .send()
        .await
        .map_err(|_| "無法下載更新")?
        .error_for_status()
        .map_err(|_| "無法下載更新")?;
    const MAX: usize = 256 * 1024 * 1024;
    if response.content_length().is_some_and(|n| n > MAX as u64) {
        return Err("更新檔超過256 MiB限制".into());
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| "下載未完成")? {
        if bytes.len().saturating_add(chunk.len()) > MAX {
            return Err("更新檔超過256 MiB限制".into());
        }
        bytes.extend_from_slice(&chunk);
    }
    verify(&bytes, &update.signature, &key, &update.version)?;
    let size = bytes.len();
    let mut pending = state.pending.lock().map_err(|_| "無法保存更新")?;
    pending
        .as_mut()
        .filter(|p| p.id == id)
        .ok_or("更新已過期")?
        .bytes = Some(bytes);
    Ok(json!({"id":id,"version":update.version,"status":"signature_verified","bytes":size}))
}
#[tauri::command]
pub async fn install_local_update(
    app: tauri::AppHandle,
    state: State<'_, Updates>,
    desktop: State<'_, super::DesktopState>,
    id: Uuid,
    confirmed: bool,
) -> Result<Value, String> {
    if !confirmed {
        return Err("請確認安裝已驗證的更新".into());
    }
    let _permit = state.slots.try_acquire().map_err(|_| "更新工作執行中")?;
    let mut pending = state
        .pending
        .lock()
        .map_err(|_| "無法讀取更新")?
        .take()
        .filter(|p| p.id == id && p.bytes.is_some())
        .ok_or("更新尚未驗證或已過期")?;
    let before = desktop
        .store
        .create_snapshot()
        .await
        .map_err(|_| "更新前備份失敗；未安裝")?;
    let version = pending.update.version.clone();
    let bytes = pending.bytes.take().ok_or("更新尚未驗證")?;
    verify(&bytes, &pending.update.signature, &pending.key, &version)?;
    #[cfg(windows)]
    if super::gui_acceptance_enabled()
        && std::env::var("PET_GUI_TEST_INSTALL_UPDATE").as_deref() == Ok("true")
    {
        let path = std::env::var("PET_GUI_TEST_REPORT").map_err(|_| "missing isolated report")?;
        // Windows updater exits after spawning NSIS. This is pending, never success;
        // the external verifier must prove installed bytes and cold version first.
        std::fs::write(path,json!({"success":false,"update_pending":true,"prerequisites_passed":true,"before_snapshot":before.id,"version":app.package_info().version.to_string(),"source_sha":env!("COPILOT_SOURCE_SHA"),"source_dirty":env!("COPILOT_SOURCE_DIRTY")=="true","release":!cfg!(debug_assertions)}).to_string()).map_err(|_| "無法保存待驗證的更新收據")?;
    }
    let update = pending.update.restart_after_install(false);
    tokio::task::spawn_blocking(move || update.install(bytes))
        .await
        .map_err(|_| "安裝工作失敗")?
        .map_err(|_| "安裝更新失敗；資料備份仍保留")?;
    let _ = app.emit_to("main", "local-update-installed", json!({"version":version}));
    Ok(json!({"status":"installed_restart_required","version":version,"before_snapshot":before.id}))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_numeric_loopback_urls_are_accepted() {
        for url in ["http://127.0.0.1:123/manifest", "https://[::1]/manifest"] {
            assert!(local(&url::Url::parse(url).unwrap()));
        }
        for url in [
            "https://example.com",
            "http://localhost",
            "http://127.0.0.1/?token=x",
            "http://user@127.0.0.1/",
        ] {
            assert!(!local(&url::Url::parse(url).unwrap()));
        }
        assert!(verify(b"synthetic", "bad", "bad", "0.1.1").is_err());
    }
}
