//! Unix descriptor-based Adapter：不跟隨 symlink，先原子捕獲，再驗證身份。
use crate::{CleanupError, CleanupReceipt, FilePreview, modified_nanos, validate_name};
use std::{
    ffi::CString,
    fs::{File, Metadata},
    io::{self, Read, Write},
    os::{
        fd::{AsRawFd, FromRawFd, IntoRawFd},
        unix::fs::{MetadataExt, OpenOptionsExt},
    },
    path::Path,
};
use uuid::Uuid;
pub struct PinnedRoot {
    directory: File,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PreparedOperation {
    operation_id: Uuid,
    preview: FilePreview,
    quarantined_file: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RestoreMarker {
    restore_id: Uuid,
    restoring: bool,
}
fn directory_names(directory: &File) -> Result<Vec<String>, CleanupError> {
    // openat(".") creates an independent directory offset; dup would share it.
    let file = open_at(directory, ".", libc::O_RDONLY | libc::O_DIRECTORY)?;
    let descriptor = file.into_raw_fd();
    let dir = unsafe { libc::fdopendir(descriptor) };
    if dir.is_null() {
        unsafe { libc::close(descriptor) };
        return Err(io::Error::last_os_error().into());
    }
    struct Directory(*mut libc::DIR);
    impl Drop for Directory {
        fn drop(&mut self) {
            unsafe { libc::closedir(self.0) };
        }
    }
    let dir = Directory(dir);
    let mut names = Vec::new();
    for _ in 0..1000 {
        let entry = unsafe { libc::readdir(dir.0) };
        if entry.is_null() {
            break;
        }
        let name = unsafe { std::ffi::CStr::from_ptr((*entry).d_name.as_ptr()) };
        if let Ok(name) = name.to_str() {
            names.push(name.to_owned());
        }
    }
    Ok(names)
}
fn name(value: &str) -> Result<CString, CleanupError> {
    CString::new(value).map_err(|_| CleanupError::InvalidName)
}
fn open_at(directory: &File, value: &str, flags: i32) -> Result<File, CleanupError> {
    let value = name(value)?;
    let fd = unsafe {
        libc::openat(
            directory.as_raw_fd(),
            value.as_ptr(),
            flags | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK,
        )
    };
    if fd < 0 {
        let error = io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ELOOP) {
            return Err(CleanupError::UnsafeItem);
        }
        return Err(error.into());
    }
    Ok(unsafe { File::from_raw_fd(fd) })
}
fn rename(from: &File, source: &str, to: &File, destination: &str) -> Result<(), CleanupError> {
    let source = name(source)?;
    let destination = name(destination)?;
    #[cfg(target_os = "linux")]
    let result = unsafe {
        libc::renameat2(
            from.as_raw_fd(),
            source.as_ptr(),
            to.as_raw_fd(),
            destination.as_ptr(),
            libc::RENAME_NOREPLACE,
        )
    };
    #[cfg(target_os = "macos")]
    let result = unsafe {
        libc::renameatx_np(
            from.as_raw_fd(),
            source.as_ptr(),
            to.as_raw_fd(),
            destination.as_ptr(),
            libc::RENAME_EXCL,
        )
    };
    if result != 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(())
}
fn preview(metadata: Metadata, file_name: &str) -> Result<FilePreview, CleanupError> {
    if !metadata.is_file()
        || metadata.nlink() != 1
        || metadata.uid() != unsafe { libc::geteuid() }
        || metadata.mode() & 0o022 != 0
    {
        return Err(CleanupError::UnsafeItem);
    }
    Ok(FilePreview {
        file_name: file_name.into(),
        size_bytes: metadata.len(),
        last_modified_nanos: modified_nanos(&metadata)?,
        identity: Some([metadata.dev(), metadata.ino(), 0]),
    })
}
impl PinnedRoot {
    fn trash_directory(&self) -> Result<File, CleanupError> {
        let directory = open_at(
            &self.directory,
            ".pet-quarantine",
            libc::O_RDONLY | libc::O_DIRECTORY,
        )?;
        let meta = directory.metadata()?;
        if meta.uid() != unsafe { libc::geteuid() } || meta.mode() & 0o077 != 0 {
            return Err(CleanupError::UnsafeItem);
        }
        Ok(directory)
    }
    fn journal(
        &self,
        directory: &File,
        id: Uuid,
    ) -> Result<(PreparedOperation, Option<Uuid>), CleanupError> {
        if id.is_nil() {
            return Err(CleanupError::InvalidName);
        }
        let mut file = open_at(directory, &format!("{id}.journal"), libc::O_RDONLY)?;
        let meta = file.metadata()?;
        if !meta.is_file()
            || meta.nlink() != 1
            || meta.uid() != unsafe { libc::geteuid() }
            || meta.mode() & 0o077 != 0
            || meta.len() > 8192
        {
            return Err(CleanupError::UnsafeItem);
        }
        let mut bytes = Vec::new();
        (&mut file).take(8193).read_to_end(&mut bytes)?;
        if bytes.len() > 8192 {
            return Err(CleanupError::UnsafeItem);
        }
        let mut lines = bytes.split(|b| *b == b'\n');
        let prepared: PreparedOperation =
            serde_json::from_slice(lines.next().unwrap_or_default()).map_err(io::Error::other)?;
        if prepared.operation_id != id || prepared.quarantined_file != format!("{id}.item") {
            return Err(CleanupError::UnsafeItem);
        }
        validate_name(&prepared.preview.file_name)?;
        let mut restore = None;
        for line in lines {
            if let Ok(marker) = serde_json::from_slice::<RestoreMarker>(line) {
                if !marker.restoring || marker.restore_id.is_nil() {
                    return Err(CleanupError::UnsafeItem);
                }
                if restore.is_some_and(|previous| previous != marker.restore_id) {
                    return Err(CleanupError::UnsafeItem);
                }
                restore = Some(marker.restore_id);
            }
        }
        Ok((prepared, restore))
    }
    fn is_restored(&self, folder: &File, prepared: &PreparedOperation) -> bool {
        matches!(open_at(folder,&prepared.quarantined_file,libc::O_RDONLY),Err(CleanupError::Io(ref e))if e.kind()==io::ErrorKind::NotFound)
            && self
                .preview(&prepared.preview.file_name)
                .is_ok_and(|actual| actual == prepared.preview)
    }
    pub fn trash(&self) -> Result<Vec<crate::TrashEntry>, CleanupError> {
        let folder = match self.trash_directory() {
            Ok(v) => v,
            Err(CleanupError::Io(e)) if e.kind() == io::ErrorKind::NotFound => {
                return Ok(Vec::new());
            }
            Err(e) => return Err(e),
        };
        let mut entries = Vec::new();
        for filename in directory_names(&folder)? {
            let Some(id) = filename
                .strip_suffix(".journal")
                .and_then(|s| Uuid::parse_str(s).ok())
            else {
                continue;
            };
            let Ok((record, restore)) = self.journal(&folder, id) else {
                continue;
            };
            let available = open_at(&folder, &record.quarantined_file, libc::O_RDONLY)
                .and_then(|f| preview(f.metadata()?, &record.preview.file_name))
                .is_ok_and(|p| p == record.preview);
            let restored = restore.is_some() && self.is_restored(&folder, &record);
            if available || restored {
                entries.push(crate::TrashEntry {
                    operation_id: id,
                    file_name: record.preview.file_name,
                    size_bytes: record.preview.size_bytes,
                    restored,
                });
            }
        }
        Ok(entries)
    }
    pub fn restore(&self, id: Uuid) -> Result<crate::RestoreReceipt, CleanupError> {
        let folder = self.trash_directory()?;
        // CLOEXEC descriptors may briefly remain shared with a concurrent fork.
        // Wait a bounded interval for transient contention, never hold a GUI worker indefinitely.
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(500);
        loop {
            if unsafe { libc::flock(folder.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
                break;
            }
            let error = io::Error::last_os_error();
            if error.kind() != io::ErrorKind::WouldBlock || std::time::Instant::now() >= deadline {
                return Err(error.into());
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        let (record, previous) = self.journal(&folder, id)?;
        if let Some(restore_id) = previous
            && self.is_restored(&folder, &record)
        {
            return Ok(crate::RestoreReceipt {
                operation_id: restore_id,
                quarantine_operation_id: id,
                size_bytes: record.preview.size_bytes,
            });
        }
        let captured = open_at(&folder, &record.quarantined_file, libc::O_RDONLY)?;
        if preview(captured.metadata()?, &record.preview.file_name)? != record.preview {
            return Err(CleanupError::Changed);
        }
        let restore_id = previous.unwrap_or_else(Uuid::new_v4);
        let mut journal = open_at(
            &folder,
            &format!("{id}.journal"),
            libc::O_WRONLY | libc::O_APPEND,
        )?;
        let meta = journal.metadata()?;
        if !meta.is_file()
            || meta.nlink() != 1
            || meta.uid() != unsafe { libc::geteuid() }
            || meta.mode() & 0o077 != 0
        {
            return Err(CleanupError::UnsafeItem);
        }
        if previous.is_none() {
            journal.write_all(
                &serde_json::to_vec(&RestoreMarker {
                    restore_id,
                    restoring: true,
                })
                .map_err(io::Error::other)?,
            )?;
            journal.write_all(b"\n")?;
            journal.sync_all()?;
        }
        rename(
            &folder,
            &record.quarantined_file,
            &self.directory,
            &record.preview.file_name,
        )?;
        if !self
            .preview(&record.preview.file_name)
            .is_ok_and(|actual| actual == record.preview)
        {
            if rename(
                &self.directory,
                &record.preview.file_name,
                &folder,
                &record.quarantined_file,
            )
            .is_err()
            {
                return Err(CleanupError::RecoveryRequired(id));
            }
            return Err(CleanupError::Changed);
        }
        self.directory.sync_all()?;
        folder.sync_all()?;
        journal.write_all(b"{\"restored\":true}\n")?;
        journal.sync_all()?;
        Ok(crate::RestoreReceipt {
            operation_id: restore_id,
            quarantine_operation_id: id,
            size_bytes: record.preview.size_bytes,
        })
    }
    pub fn restored_receipts(&self) -> Result<Vec<crate::RestoreReceipt>, CleanupError> {
        let folder = match self.trash_directory() {
            Ok(v) => v,
            Err(CleanupError::Io(e)) if e.kind() == io::ErrorKind::NotFound => {
                return Ok(Vec::new());
            }
            Err(e) => return Err(e),
        };
        let mut receipts = Vec::new();
        for item in self.trash()? {
            if !item.restored {
                continue;
            }
            let (_, Some(restore_id)) = self.journal(&folder, item.operation_id)? else {
                continue;
            };
            receipts.push(crate::RestoreReceipt {
                operation_id: restore_id,
                quarantine_operation_id: item.operation_id,
                size_bytes: item.size_bytes,
            });
        }
        Ok(receipts)
    }

    pub fn scan(&self) -> Result<Vec<FilePreview>, CleanupError> {
        let mut result = Vec::new();
        for name in directory_names(&self.directory)? {
            if let Ok(preview) = self.preview(&name) {
                result.push(preview);
            }
        }
        Ok(result)
    }
    pub fn open(path: &Path) -> Result<Self, CleanupError> {
        let directory = std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(path)?;
        let metadata = directory.metadata()?;
        if metadata.uid() != unsafe { libc::geteuid() } || metadata.mode() & 0o022 != 0 {
            return Err(CleanupError::UnsafeItem);
        }
        Ok(Self { directory })
    }
    pub fn preview(&self, value: &str) -> Result<FilePreview, CleanupError> {
        validate_name(value)?;
        let file = open_at(&self.directory, value, libc::O_RDONLY)?;
        preview(file.metadata()?, value)
    }
    pub(crate) fn sha256(&self, expected: &FilePreview) -> Result<[u8; 32], CleanupError> {
        // 只透過已固定的 root dirfd 開啟；讀取前後均重驗 owner、mode、inode 與 mtime。
        let valid_root = || -> Result<(), CleanupError> {
            let meta = self.directory.metadata()?;
            if !meta.is_dir()
                || meta.uid() != unsafe { libc::geteuid() }
                || meta.mode() & 0o022 != 0
            {
                return Err(CleanupError::UnsafeItem);
            }
            Ok(())
        };
        valid_root()?;
        validate_name(&expected.file_name)?;
        let mut file = open_at(&self.directory, &expected.file_name, libc::O_RDONLY)?;
        let before = file.metadata()?;
        if preview(before.clone(), &expected.file_name)? != *expected {
            return Err(CleanupError::Changed);
        }
        let digest = crate::duplicates::hash_contents(&mut file, expected.size_bytes)?;
        let after = file.metadata()?;
        if preview(after.clone(), &expected.file_name)? != *expected
            || (before.ctime(), before.ctime_nsec()) != (after.ctime(), after.ctime_nsec())
            || self.preview(&expected.file_name)? != *expected
        {
            return Err(CleanupError::Changed);
        }
        valid_root()?;
        Ok(digest)
    }
    pub fn quarantine(&self, expected: &FilePreview) -> Result<CleanupReceipt, CleanupError> {
        validate_name(&expected.file_name)?;
        if self.preview(&expected.file_name)? != *expected {
            return Err(CleanupError::Changed);
        }
        let folder = name(".pet-quarantine")?;
        let result = unsafe { libc::mkdirat(self.directory.as_raw_fd(), folder.as_ptr(), 0o700) };
        if result != 0 && io::Error::last_os_error().raw_os_error() != Some(libc::EEXIST) {
            return Err(io::Error::last_os_error().into());
        }
        let destination = open_at(
            &self.directory,
            ".pet-quarantine",
            libc::O_RDONLY | libc::O_DIRECTORY,
        )?;
        let metadata = destination.metadata()?;
        if metadata.uid() != unsafe { libc::geteuid() } || metadata.mode() & 0o077 != 0 {
            return Err(CleanupError::UnsafeItem);
        }
        let id = Uuid::new_v4();
        let staged = format!("{id}.item");
        let prepared = PreparedOperation {
            operation_id: id,
            preview: expected.clone(),
            quarantined_file: staged.clone(),
        };
        let journal_name = name(&format!("{id}.journal"))?;
        let fd = unsafe {
            libc::openat(
                destination.as_raw_fd(),
                journal_name.as_ptr(),
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                0o600 as libc::c_uint,
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error().into());
        }
        let mut journal = unsafe { File::from_raw_fd(fd) };
        let prepared = serde_json::to_vec(&prepared).map_err(io::Error::other)?;
        journal.write_all(&prepared)?;
        journal.write_all(b"\n")?;
        journal.sync_all()?;
        destination.sync_all()?;
        // 在同一 filesystem 以 dirfd 原子移動。捕獲之後不會再從來源名稱讀檔。
        rename(&self.directory, &expected.file_name, &destination, &staged)?;
        let valid = open_at(&destination, &staged, libc::O_RDONLY)
            .and_then(|file| preview(file.metadata()?, &expected.file_name))
            .is_ok_and(|captured| captured == *expected);
        if !valid {
            // NOREPLACE 回退不會覆蓋來源位置的新檔。回退失敗須保留操作 ID 供復原。
            if rename(&destination, &staged, &self.directory, &expected.file_name).is_err() {
                return Err(CleanupError::RecoveryRequired(id));
            }
            return Err(CleanupError::Changed);
        }
        // durable receipt 只在真實移動與核對身份成功後產生。
        destination.sync_all()?;
        self.directory.sync_all()?;
        journal.write_all(b"{\"completed\":true}\n")?;
        journal.sync_all()?;
        Ok(CleanupReceipt {
            operation_id: id,
            quarantined_file: staged,
            size_bytes: expected.size_bytes,
        })
    }

    /// Recover only an actually captured file with the original identity. A crash
    /// before rename, a torn completion marker or a replaced file never fabricates success.
    pub fn recovered_receipts(&self) -> Result<Vec<CleanupReceipt>, CleanupError> {
        let destination = match open_at(
            &self.directory,
            ".pet-quarantine",
            libc::O_RDONLY | libc::O_DIRECTORY,
        ) {
            Ok(file) => file,
            Err(CleanupError::Io(e)) if e.kind() == io::ErrorKind::NotFound => {
                return Ok(Vec::new());
            }
            Err(e) => return Err(e),
        };
        let metadata = destination.metadata()?;
        if metadata.uid() != unsafe { libc::geteuid() } || metadata.mode() & 0o077 != 0 {
            return Err(CleanupError::UnsafeItem);
        }
        let mut receipts = Vec::new();
        for item in directory_names(&destination)? {
            if !item.ends_with(".journal") {
                continue;
            }
            let mut file = match open_at(&destination, &item, libc::O_RDONLY) {
                Ok(file) => file,
                Err(_) => continue,
            };
            let meta = file.metadata()?;
            if !meta.is_file()
                || meta.nlink() != 1
                || meta.uid() != unsafe { libc::geteuid() }
                || meta.mode() & 0o077 != 0
                || meta.len() > 8192
            {
                continue;
            }
            let mut bytes = Vec::new();
            (&mut file).take(8193).read_to_end(&mut bytes)?;
            if bytes.len() > 8192 {
                continue;
            }
            let first = bytes
                .split(|byte| *byte == b'\n')
                .next()
                .unwrap_or_default();
            let prepared: PreparedOperation = match serde_json::from_slice(first) {
                Ok(record) => record,
                Err(_) => continue,
            };
            if item != format!("{}.journal", prepared.operation_id)
                || prepared.quarantined_file != format!("{}.item", prepared.operation_id)
                || validate_name(&prepared.preview.file_name).is_err()
            {
                continue;
            }
            let captured = open_at(&destination, &prepared.quarantined_file, libc::O_RDONLY)
                .and_then(|file| preview(file.metadata()?, &prepared.preview.file_name));
            if captured.is_ok_and(|file| file == prepared.preview) {
                receipts.push(CleanupReceipt {
                    operation_id: prepared.operation_id,
                    quarantined_file: prepared.quarantined_file,
                    size_bytes: prepared.preview.size_bytes,
                });
            }
        }
        Ok(receipts)
    }
}
