//! 先預覽、再明確確認；Linux 使用 dirfd 原子捕獲與身份重驗證。
//! 尚未驗證的其他平台 fail closed，不執行危險移動。
pub mod agent;
pub mod ai;
pub mod backup;
pub mod interaction;
pub mod outbox;
pub mod profile;
pub mod state;
pub mod suggestions;
#[cfg(any(target_os = "linux", target_os = "macos"))]
mod unix_fs;
#[cfg(windows)]
mod windows_fs;
use event_contract::ProductEvent;
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use thiserror::Error;
use uuid::Uuid;
#[cfg(windows)]
use windows_fs as unix_fs;

#[derive(Debug, Error)]
pub enum CleanupError {
    #[error("invalid or unauthorized file name")]
    InvalidName,
    #[error("refuse symbolic link or non-regular file")]
    UnsafeItem,
    #[error("item changed after preview")]
    Changed,
    #[error("operation requires recovery: {0}")]
    RecoveryRequired(Uuid),
    #[error("IO: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FilePreview {
    pub file_name: String,
    pub size_bytes: u64,
    pub last_modified_nanos: u128,
    #[serde(default)]
    pub identity: Option<[u64; 3]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CleanupReceipt {
    pub operation_id: Uuid,
    pub quarantined_file: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrashEntry {
    pub operation_id: Uuid,
    pub file_name: String,
    pub size_bytes: u64,
    pub restored: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RestoreReceipt {
    pub operation_id: Uuid,
    pub quarantine_operation_id: Uuid,
    pub size_bytes: u64,
}
impl RestoreReceipt {
    pub fn event(&self) -> ProductEvent {
        ProductEvent {
            schema_version: 1,
            event_id: self.operation_id,
            source_app: "desktop_pet".into(),
            event_type: "file.restore.succeeded".into(),
        }
    }
}
impl CleanupReceipt {
    /// 真實 rename 成功後才建立產品事件；預覽不產生成功事件。
    pub fn event(&self) -> ProductEvent {
        ProductEvent::pet_cleanup_succeeded(self.operation_id)
    }
}

pub struct AuthorizedRoot {
    #[cfg_attr(
        any(target_os = "linux", target_os = "macos", target_os = "windows"),
        allow(dead_code)
    )]
    root: PathBuf,
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    pinned: unix_fs::PinnedRoot,
}

fn validate_name(name: &str) -> Result<(), CleanupError> {
    if name.is_empty()
        || name.len() > 255
        || name.starts_with('.')
        || name.ends_with(' ')
        || name.ends_with('.')
        || name
            .chars()
            .any(|c| c.is_control() || matches!(c, '/' | '\\' | ':'))
    {
        return Err(CleanupError::InvalidName);
    }
    Ok(())
}

fn modified_nanos(meta: &fs::Metadata) -> Result<u128, CleanupError> {
    Ok(meta
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| io::Error::other("mtime predates epoch"))?
        .as_nanos())
}

impl AuthorizedRoot {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, CleanupError> {
        #[cfg(target_os = "windows")]
        match path.as_ref().components().next() {
            Some(std::path::Component::Prefix(prefix))
                if matches!(
                    prefix.kind(),
                    std::path::Prefix::Disk(_) | std::path::Prefix::VerbatimDisk(_)
                ) => {}
            _ => return Err(CleanupError::InvalidName),
        }
        let root = fs::canonicalize(path)?;
        if !root.is_dir() {
            return Err(CleanupError::InvalidName);
        }
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        let pinned = unix_fs::PinnedRoot::open(&root)?;
        Ok(Self {
            root,
            #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
            pinned,
        })
    }

    pub fn authorized_path(&self) -> &Path {
        &self.root
    }

    pub fn scan(&self) -> Result<Vec<FilePreview>, CleanupError> {
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        {
            self.pinned.scan()
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            let mut result = Vec::new();
            for entry in fs::read_dir(&self.root)?.take(1000) {
                let entry = entry?;
                if let Some(name) = entry.file_name().to_str()
                    && let Ok(p) = self.preview(name)
                {
                    result.push(p);
                }
            }
            Ok(result)
        }
    }
    pub fn preview(&self, name: &str) -> Result<FilePreview, CleanupError> {
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        {
            self.pinned.preview(name)
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            validate_name(name)?;
            let file = self.root.join(name);
            let meta = fs::symlink_metadata(&file)?;
            if !meta.file_type().is_file() || meta.file_type().is_symlink() {
                return Err(CleanupError::UnsafeItem);
            }
            Ok(FilePreview {
                file_name: name.to_string(),
                size_bytes: meta.len(),
                last_modified_nanos: modified_nanos(&meta)?,
                identity: None,
            })
        }
    }

    /// 必須由 UI 明確要求確認；只把一般檔案移至同一個授權根的 quarantine。
    /// Linux 平台以固定目錄 descriptor、NOREPLACE 與捕獲後 inode 驗證完成操作。
    pub fn confirm_quarantine(
        &self,
        preview: &FilePreview,
        confirmed: bool,
    ) -> Result<CleanupReceipt, CleanupError> {
        if !confirmed {
            return Err(CleanupError::InvalidName);
        }
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        {
            self.pinned.quarantine(preview)
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            let _ = preview;
            Err(CleanupError::Io(io::Error::new(
                io::ErrorKind::Unsupported,
                "此平台尚未驗證安全移動 Adapter",
            )))
        }
    }
    pub fn trash(&self) -> Result<Vec<TrashEntry>, CleanupError> {
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        {
            self.pinned.trash()
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            Err(io::Error::new(io::ErrorKind::Unsupported, "安全回收區 Adapter 尚未驗證").into())
        }
    }
    pub fn confirm_restore(
        &self,
        id: Uuid,
        confirmed: bool,
    ) -> Result<RestoreReceipt, CleanupError> {
        if !confirmed {
            return Err(CleanupError::InvalidName);
        }
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        {
            self.pinned.restore(id)
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            let _ = id;
            Err(io::Error::new(io::ErrorKind::Unsupported, "安全還原 Adapter 尚未驗證").into())
        }
    }
    pub fn restored_receipts(&self) -> Result<Vec<RestoreReceipt>, CleanupError> {
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        {
            self.pinned.restored_receipts()
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            Err(io::Error::new(io::ErrorKind::Unsupported, "安全還原復原 Adapter 尚未驗證").into())
        }
    }
    pub fn recovered_receipts(&self) -> Result<Vec<CleanupReceipt>, CleanupError> {
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        {
            self.pinned.recovered_receipts()
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            Err(CleanupError::Io(io::Error::new(
                io::ErrorKind::Unsupported,
                "安全復原 Adapter 尚未驗證",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn private_directory(path: impl AsRef<Path>) {
        fs::create_dir_all(&path).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
        }
    }
    fn private_fixture(path: impl AsRef<Path>, contents: &[u8]) {
        fs::write(&path, contents).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(path, fs::Permissions::from_mode(0o600)).unwrap();
        }
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn preview_is_readonly_confirmation_is_required() {
        let dir = std::env::temp_dir().join(format!("pet-day9-{}", Uuid::new_v4()));
        private_directory(&dir);
        private_fixture(dir.join("example.txt"), b"sample");
        let root = AuthorizedRoot::new(&dir).unwrap();
        let preview = root.preview("example.txt").unwrap();
        assert!(dir.join("example.txt").exists());
        assert!(root.confirm_quarantine(&preview, false).is_err());
        assert!(dir.join("example.txt").exists());
        let receipt = root.confirm_quarantine(&preview, true).unwrap();
        assert!(!dir.join("example.txt").exists());
        assert!(
            dir.join(".pet-quarantine")
                .join(&receipt.quarantined_file)
                .exists()
        );
        assert_eq!(receipt.event().source_app, "desktop_pet");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn reject_relative_escape_and_changed_file() {
        let dir = std::env::temp_dir().join(format!("pet-day9-{}", Uuid::new_v4()));
        private_directory(&dir);
        private_fixture(dir.join("a.txt"), b"1");
        let root = AuthorizedRoot::new(&dir).unwrap();
        assert!(root.preview("../secret").is_err());
        let preview = root.preview("a.txt").unwrap();
        fs::write(dir.join("a.txt"), b"changed").unwrap();
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        assert!(matches!(
            root.confirm_quarantine(&preview, true),
            Err(CleanupError::Changed)
        ));
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        assert!(
            matches!(root.confirm_quarantine(&preview,true), Err(CleanupError::Io(e)) if e.kind()==io::ErrorKind::Unsupported)
        );
        assert_eq!(fs::read(dir.join("a.txt")).unwrap(), b"changed");
        drop(root);
        fs::remove_dir_all(&dir).unwrap();
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    #[test]
    fn unverified_platform_cannot_move_file() {
        let dir = std::env::temp_dir().join(format!("pet-unsupported-{}", Uuid::new_v4()));
        private_directory(&dir);
        private_fixture(dir.join("a.txt"), b"original");
        let root = AuthorizedRoot::new(&dir).unwrap();
        let item = root.preview("a.txt").unwrap();
        assert!(root.confirm_quarantine(&item, true).is_err());
        assert_eq!(fs::read(dir.join("a.txt")).unwrap(), b"original");
        fs::remove_dir_all(dir).unwrap();
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    #[ignore = "Only invoked by the parent with its own disposable fixture"]
    fn restore_process_child() {
        let path = PathBuf::from(std::env::var("COPILOT_RESTORE_TEST_ROOT").unwrap());
        let id = Uuid::parse_str(&std::env::var("COPILOT_RESTORE_TEST_ID").unwrap()).unwrap();
        let root = AuthorizedRoot::new(&path).unwrap();
        fs::write(path.join(format!("ready-{}", std::process::id())), b"ready").unwrap();
        let limit = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !path.join("go").exists() {
            assert!(std::time::Instant::now() < limit);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        match root.confirm_restore(id, true) {
            Ok(_) => {}
            Err(CleanupError::Io(e)) if e.kind() == io::ErrorKind::WouldBlock => {}
            result => panic!("unexpected child restore result {result:?}"),
        }
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn two_processes_restore_with_one_durable_receipt() {
        let path = std::env::temp_dir().join(format!("pet-restore-process-{}", Uuid::new_v4()));
        private_directory(&path);
        private_fixture(path.join("a.txt"), b"original");
        let root = AuthorizedRoot::new(&path).unwrap();
        let receipt = root
            .confirm_quarantine(&root.preview("a.txt").unwrap(), true)
            .unwrap();
        let spawn = || {
            std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "tests::restore_process_child", "--ignored"])
                .env_clear()
                .env("COPILOT_RESTORE_TEST_ROOT", &path)
                .env("COPILOT_RESTORE_TEST_ID", receipt.operation_id.to_string())
                .spawn()
                .unwrap()
        };
        let mut first = spawn();
        let mut second = spawn();
        let limit = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !(path.join(format!("ready-{}", first.id())).exists()
            && path.join(format!("ready-{}", second.id())).exists())
        {
            assert!(std::time::Instant::now() < limit);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        fs::write(path.join("go"), b"go").unwrap();
        assert!(first.wait().unwrap().success());
        assert!(second.wait().unwrap().success());
        assert_eq!(root.restored_receipts().unwrap().len(), 1);
        assert_eq!(fs::read(path.join("a.txt")).unwrap(), b"original");
        fs::remove_dir_all(path).unwrap();
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn restore_is_confirmed_idempotent_and_does_not_replace_a_new_file() {
        let dir = std::env::temp_dir().join(format!("pet-restore-{}", Uuid::new_v4()));
        private_directory(&dir);
        private_fixture(dir.join("a.txt"), b"original");
        let root = AuthorizedRoot::new(&dir).unwrap();
        let saved = root
            .confirm_quarantine(&root.preview("a.txt").unwrap(), true)
            .unwrap();
        assert!(!root.trash().unwrap()[0].restored);
        assert!(root.confirm_restore(saved.operation_id, false).is_err());
        private_fixture(dir.join("a.txt"), b"replacement");
        assert!(root.confirm_restore(saved.operation_id, true).is_err());
        assert_eq!(fs::read(dir.join("a.txt")).unwrap(), b"replacement");
        fs::remove_file(dir.join("a.txt")).unwrap();
        let restored = root.confirm_restore(saved.operation_id, true).unwrap();
        assert_eq!(fs::read(dir.join("a.txt")).unwrap(), b"original");
        assert_eq!(
            root.confirm_restore(saved.operation_id, true).unwrap(),
            restored
        );
        assert!(root.trash().unwrap()[0].restored);
        drop(root);
        let reopened = AuthorizedRoot::new(&dir).unwrap();
        assert_eq!(reopened.restored_receipts().unwrap(), vec![restored]);
        fs::remove_dir_all(dir).unwrap();
    }
    #[cfg(unix)]
    #[test]
    fn reject_source_and_quarantine_symlinks_without_touching_target() {
        use std::os::unix::fs::symlink;
        let dir = std::env::temp_dir().join(format!("pet-symlink-{}", Uuid::new_v4()));
        let outside = std::env::temp_dir().join(format!("pet-outside-{}", Uuid::new_v4()));
        private_directory(&dir);
        fs::create_dir(&outside).unwrap();
        fs::write(outside.join("private.txt"), b"private").unwrap();
        let root = AuthorizedRoot::new(&dir).unwrap();
        symlink(outside.join("private.txt"), dir.join("link.txt")).unwrap();
        assert!(root.preview("link.txt").is_err());
        private_fixture(dir.join("a.txt"), b"1");
        let preview = root.preview("a.txt").unwrap();
        symlink(&outside, dir.join(".pet-quarantine")).unwrap();
        assert!(root.confirm_quarantine(&preview, true).is_err());
        assert!(dir.join("a.txt").exists());
        assert_eq!(fs::read(outside.join("private.txt")).unwrap(), b"private");
        fs::remove_dir_all(dir).unwrap();
        fs::remove_dir_all(outside).unwrap();
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn replacing_authorized_root_path_cannot_redirect_operation() {
        let dir = std::env::temp_dir().join(format!("pet-pin-{}", Uuid::new_v4()));
        let moved = dir.with_extension("moved");
        private_directory(&dir);
        private_fixture(dir.join("a.txt"), b"original");
        let root = AuthorizedRoot::new(&dir).unwrap();
        let preview = root.preview("a.txt").unwrap();
        fs::rename(&dir, &moved).unwrap();
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("a.txt"), b"unrelated").unwrap();
        let receipt = root.confirm_quarantine(&preview, true).unwrap();
        assert_eq!(fs::read(dir.join("a.txt")).unwrap(), b"unrelated");
        assert_eq!(
            fs::read(moved.join(".pet-quarantine").join(receipt.quarantined_file)).unwrap(),
            b"original"
        );
        fs::remove_dir_all(dir).unwrap();
        fs::remove_dir_all(moved).unwrap();
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn journal_recovers_after_restart_but_never_for_replaced_file() {
        let dir = std::env::temp_dir().join(format!("pet-journal-{}", Uuid::new_v4()));
        private_directory(&dir);
        private_fixture(dir.join("a.txt"), b"original");
        let root = AuthorizedRoot::new(&dir).unwrap();
        assert_eq!(root.scan().unwrap().len(), 1);
        assert_eq!(root.scan().unwrap().len(), 1);
        let receipt = root
            .confirm_quarantine(&root.preview("a.txt").unwrap(), true)
            .unwrap();
        drop(root);
        let root = AuthorizedRoot::new(&dir).unwrap();
        assert_eq!(root.recovered_receipts().unwrap(), vec![receipt.clone()]);
        let captured = dir.join(".pet-quarantine").join(receipt.quarantined_file);
        fs::remove_file(&captured).unwrap();
        private_fixture(captured, b"unrelated");
        assert!(root.recovered_receipts().unwrap().is_empty());
        fs::remove_dir_all(dir).unwrap();
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn externally_writable_root_is_not_authorized() {
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("pet-public-{}", Uuid::new_v4()));
        private_directory(&dir);
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o777)).unwrap();
        assert!(matches!(
            AuthorizedRoot::new(&dir),
            Err(CleanupError::UnsafeItem)
        ));
        fs::remove_dir_all(dir).unwrap();
    }
}
