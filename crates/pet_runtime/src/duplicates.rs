use crate::FilePreview;
#[derive(Debug, serde::Serialize)]
pub struct DuplicateGroup {
    pub size_bytes: u64,
    pub files: Vec<FilePreview>,
}
// 每次預覽最多讀 256 MiB、每檔 64 MiB；大檔要求改用較小的明確授權目錄。
pub(crate) fn hash_contents(
    file: &mut std::fs::File,
    expected: u64,
) -> Result<[u8; 32], crate::CleanupError> {
    use sha2::{Digest, Sha256};
    use std::io::Read;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 65536];
    let mut total = 0u64;
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        total = total
            .checked_add(count as u64)
            .ok_or(crate::CleanupError::Changed)?;
        if total > expected || total > 64 * 1024 * 1024 {
            return Err(crate::CleanupError::Changed);
        }
        hash.update(&buffer[..count]);
    }
    if total != expected {
        return Err(crate::CleanupError::Changed);
    }
    Ok(hash.finalize().into())
}
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
pub(crate) fn group(
    root: &crate::unix_fs::PinnedRoot,
) -> Result<Vec<DuplicateGroup>, crate::CleanupError> {
    use std::collections::BTreeMap;
    let mut sizes: BTreeMap<u64, Vec<FilePreview>> = BTreeMap::new();
    for item in root.scan()? {
        sizes.entry(item.size_bytes).or_default().push(item);
    }
    let mut hashes: BTreeMap<(u64, [u8; 32]), Vec<FilePreview>> = BTreeMap::new();
    let mut budget = 256u64 * 1024 * 1024;
    for (size, files) in sizes {
        if files.len() < 2 {
            continue;
        }
        for file in files {
            if size > 64 * 1024 * 1024 || size > budget {
                return Err(std::io::Error::other("重複檔預覽超過讀取上限").into());
            }
            budget -= size;
            let digest = root.sha256(&file)?;
            hashes.entry((size, digest)).or_default().push(file);
        }
    }
    Ok(hashes
        .into_iter()
        .filter_map(|((size, _), mut files)| {
            if files.len() < 2 {
                return None;
            }
            files.sort_by(|a, b| a.file_name.cmp(&b.file_name));
            Some(DuplicateGroup {
                size_bytes: size,
                files,
            })
        })
        .collect())
}
#[cfg(test)]
mod tests {
    use crate::AuthorizedRoot;
    #[test]
    fn duplicates_group_same_bytes_but_not_same_size_or_symlinks_without_modifying_files() {
        let dir = tempfile::tempdir().unwrap();
        for (name, bytes) in [
            ("one", "same"),
            ("two", "same"),
            ("different", "xxxx"),
            ("empty-a", ""),
            ("empty-b", ""),
        ] {
            std::fs::write(dir.path().join(name), bytes).unwrap();
        }
        #[cfg(unix)]
        std::os::unix::fs::symlink(dir.path().join("one"), dir.path().join("link")).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
            for name in ["one", "two", "different", "empty-a", "empty-b"] {
                std::fs::set_permissions(
                    dir.path().join(name),
                    std::fs::Permissions::from_mode(0o600),
                )
                .unwrap();
            }
        }
        let root = AuthorizedRoot::new(dir.path()).unwrap();
        let groups = root.duplicate_groups().unwrap();
        assert_eq!(groups.len(), 2);
        let nonempty = groups.iter().find(|g| g.size_bytes == 4).unwrap();
        assert_eq!(
            nonempty
                .files
                .iter()
                .map(|f| f.file_name.as_str())
                .collect::<Vec<_>>(),
            vec!["one", "two"]
        );
        assert_eq!(std::fs::read(dir.path().join("one")).unwrap(), b"same");
        assert!(!dir.path().join(".pet-quarantine").exists());
    }
}

#[cfg(all(test, unix))]
mod changed_tests {
    use crate::{AuthorizedRoot, CleanupError};
    use std::os::unix::fs::PermissionsExt;
    #[test]
    fn duplicate_hash_revalidates_preview_file_and_authorized_root() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        let path = dir.path().join("file");
        std::fs::write(&path, "same").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        let root = AuthorizedRoot::new(dir.path()).unwrap();
        let old = root.preview("file").unwrap();
        std::fs::write(&path, "changed").unwrap();
        assert!(matches!(
            root.pinned.sha256(&old),
            Err(CleanupError::Changed)
        ));
        let current = root.preview("file").unwrap();
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o777)).unwrap();
        assert!(root.pinned.sha256(&current).is_err());
    }
}
