//! 先預覽、再明確確認；Linux 使用 dirfd 原子捕獲與身份重驗證。
//! 尚未驗證的其他平台 fail closed，不執行危險移動。
pub mod agent;
pub mod ai;
pub mod outbox;
pub mod profile;
pub mod state;
pub mod suggestions;
#[cfg(any(target_os = "linux", target_os = "macos"))]
mod unix_fs;
use event_contract::ProductEvent;
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use thiserror::Error;
use uuid::Uuid;

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

impl CleanupReceipt {
    /// 真實 rename 成功後才建立產品事件；預覽不產生成功事件。
    pub fn event(&self) -> ProductEvent {
        ProductEvent::pet_cleanup_succeeded(self.operation_id)
    }
}

pub struct AuthorizedRoot {
    #[cfg_attr(any(target_os = "linux", target_os = "macos"), allow(dead_code))]
    root: PathBuf,
    #[cfg(any(target_os = "linux", target_os = "macos"))]
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
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let pinned = unix_fs::PinnedRoot::open(&root)?;
        Ok(Self {
            root,
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            pinned,
        })
    }

    pub fn authorized_path(&self) -> &Path {
        &self.root
    }

    pub fn scan(&self) -> Result<Vec<FilePreview>, CleanupError> {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            self.pinned.scan()
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
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
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            self.pinned.preview(name)
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
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
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            self.pinned.quarantine(preview)
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            let _ = preview;
            Err(CleanupError::Io(io::Error::new(
                io::ErrorKind::Unsupported,
                "此平台尚未驗證安全移動 Adapter",
            )))
        }
    }
    pub fn recovered_receipts(&self) -> Result<Vec<CleanupReceipt>, CleanupError> {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            self.pinned.recovered_receipts()
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
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
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        assert!(matches!(
            root.confirm_quarantine(&preview, true),
            Err(CleanupError::Changed)
        ));
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        assert!(
            matches!(root.confirm_quarantine(&preview,true), Err(CleanupError::Io(e)) if e.kind()==io::ErrorKind::Unsupported)
        );
        assert_eq!(fs::read(dir.join("a.txt")).unwrap(), b"changed");
        fs::remove_dir_all(&dir).unwrap();
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
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
