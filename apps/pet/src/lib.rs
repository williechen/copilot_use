//! Day 9：先預覽、再明確確認，僅操作受控目錄內頂層一般檔案。
//! 重要：本教學只演示隔離 quarantine 的最小 vertical slice；正式跨平台安全
//! 清理仍須使用 openat2/O_NOFOLLOW/目錄 fd 等原子路徑約束防護並通過 GUI E2E。
use event_contract::ProductEvent;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::{Path, PathBuf}};
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
    #[error("IO: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FilePreview {
    pub file_name: String,
    pub size_bytes: u64,
    pub last_modified_nanos: u128,
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
    root: PathBuf,
}

fn validate_name(name: &str) -> Result<(), CleanupError> {
    if name.is_empty() || name.starts_with('.') || name == "." || name == ".."
        || !name.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_'))
    { return Err(CleanupError::InvalidName); }
    Ok(())
}

fn modified_nanos(meta: &fs::Metadata) -> Result<u128, CleanupError> {
    Ok(meta.modified()?.duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| io::Error::other("mtime predates epoch"))?.as_nanos())
}

impl AuthorizedRoot {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, CleanupError> {
        let root = fs::canonicalize(path)?;
        if !root.is_dir() { return Err(CleanupError::InvalidName); }
        Ok(Self { root })
    }

    pub fn preview(&self, name: &str) -> Result<FilePreview, CleanupError> {
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
        })
    }

    /// 必須由 UI 明確要求確認；只把一般檔案移至同一個授權根的 quarantine。
    /// 注意：metadata check 與 rename 非原子；本實作尚不符合高風險敵對目錄場景。
    pub fn confirm_quarantine(&self, preview: &FilePreview, confirmed: bool) -> Result<CleanupReceipt, CleanupError> {
        if !confirmed { return Err(CleanupError::InvalidName); }
        let now = self.preview(&preview.file_name)?;
        if &now != preview { return Err(CleanupError::Changed); }

        let quarantine = self.root.join(".pet-quarantine");
        fs::create_dir_all(&quarantine)?;
        if !fs::symlink_metadata(&quarantine)?.file_type().is_dir() {
            return Err(CleanupError::UnsafeItem);
        }
        let operation_id = Uuid::new_v4();
        let quarantine_name = format!("{}-{}", operation_id, preview.file_name);
        fs::rename(self.root.join(&preview.file_name), quarantine.join(&quarantine_name))?;
        Ok(CleanupReceipt {
            operation_id,
            quarantined_file: quarantine_name,
            size_bytes: preview.size_bytes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_is_readonly_confirmation_is_required() {
        let dir = std::env::temp_dir().join(format!("pet-day9-{}", Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("example.txt"), b"sample").unwrap();
        let root = AuthorizedRoot::new(&dir).unwrap();
        let preview = root.preview("example.txt").unwrap();
        assert!(dir.join("example.txt").exists());
        assert!(root.confirm_quarantine(&preview, false).is_err());
        assert!(dir.join("example.txt").exists());
        let receipt = root.confirm_quarantine(&preview, true).unwrap();
        assert!(!dir.join("example.txt").exists());
        assert!(dir.join(".pet-quarantine").join(&receipt.quarantined_file).exists());
        assert_eq!(receipt.event().source_app, "desktop_pet");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn reject_relative_escape_and_changed_file() {
        let dir = std::env::temp_dir().join(format!("pet-day9-{}", Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("a.txt"), b"1").unwrap();
        let root = AuthorizedRoot::new(&dir).unwrap();
        assert!(root.preview("../secret").is_err());
        let preview = root.preview("a.txt").unwrap();
        fs::write(dir.join("a.txt"), b"changed").unwrap();
        assert!(matches!(root.confirm_quarantine(&preview, true), Err(CleanupError::Changed)));
        fs::remove_dir_all(&dir).unwrap();
    }
}
