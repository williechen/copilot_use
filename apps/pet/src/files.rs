//! App-owned per-pet configuration and asset projection. SQLite remains authoritative;
//! startup and explicit refresh repair incomplete projections without changing backups.
use crate::{
    profile::{AssetManifest, PetProfile},
    state::PetStore,
};
use base64::Engine;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use uuid::Uuid;
type Error = Box<dyn std::error::Error + Send + Sync>;

fn private_directory(path: &Path) -> Result<(), Error> {
    match fs::create_dir(path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(error) => return Err(error.into()),
    }
    // Another process may create the same projection root concurrently. Always
    // validate the resulting directory; never accept a symlink or another owner.
    let meta = fs::symlink_metadata(path)?;
    if !meta.is_dir() || meta.file_type().is_symlink() {
        return Err("unsafe pet directory".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        if fs::metadata(path)?.uid() != unsafe { libc::geteuid() } {
            return Err("untrusted pet directory".into());
        }
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        let file = fs::OpenOptions::new()
            .access_mode(0x00020000 | 0x00040000)
            .custom_flags(0x02000000 | 0x00200000)
            .open(path)?;
        crate::windows_fs::make_private(&file)?;
    }
    Ok(())
}
pub fn prepare_data_directory(path: &Path) -> Result<(), Error> {
    private_directory(path)
}
fn options() -> fs::OpenOptions {
    let mut options = fs::OpenOptions::new();
    options.read(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        options
            .access_mode(0x80000000 | 0x40000000 | 0x00040000 | 0x00020000)
            .custom_flags(0x00200000);
    }
    options
}
async fn lock(root: &Path) -> Result<fs::File, Error> {
    let file = options()
        .create(true)
        .truncate(false)
        .open(root.join(".projection.lock"))?;
    if !file.metadata()?.is_file() {
        return Err("unsafe projection lock".into());
    }
    let deadline = Instant::now() + Duration::from_millis(500);
    loop {
        match file.try_lock() {
            Ok(()) => return Ok(file),
            Err(fs::TryLockError::WouldBlock) if Instant::now() < deadline => {
                tokio::time::sleep(Duration::from_millis(5)).await
            }
            Err(error) => return Err(error.into()),
        }
    }
}
fn atomic(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    if let Ok(meta) = fs::symlink_metadata(path)
        && (!meta.is_file() || meta.file_type().is_symlink())
    {
        return Err("unsafe pet projection".into());
    }
    let parent = path.parent().ok_or("missing parent")?;
    let temporary = parent.join(format!(".{}.tmp", Uuid::new_v4()));
    let result = (|| -> Result<(), Error> {
        let mut file = options().create_new(true).open(&temporary)?;
        #[cfg(windows)]
        crate::windows_fs::make_private(&file)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&temporary, path)?;
        #[cfg(unix)]
        fs::File::open(parent)?.sync_all()?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}
pub fn read_profile(root: &Path, pet: Uuid) -> Result<PetProfile, Error> {
    if pet.is_nil() {
        return Err("invalid pet identity".into());
    }
    for directory in [root.to_path_buf(), root.join(pet.to_string())] {
        let meta = fs::symlink_metadata(directory)?;
        if !meta.is_dir() || meta.file_type().is_symlink() {
            return Err("unsafe profile directory".into());
        }
    }
    let path = root.join(pet.to_string()).join("pet.toml");
    let meta = fs::symlink_metadata(&path)?;
    if !meta.is_file() || meta.file_type().is_symlink() || meta.len() > 32768 {
        return Err("unsafe profile file".into());
    }
    let mut text = String::new();
    fs::File::open(path)?
        .take(32769)
        .read_to_string(&mut text)?;
    Ok(PetProfile::parse(&text)?)
}
impl PetStore {
    fn files_root(&self) -> Result<PathBuf, Error> {
        let root = self
            .database_path()
            .parent()
            .ok_or("missing data directory")?
            .join("pets");
        private_directory(&root)?;
        Ok(root)
    }
    /// A UUID-derived path only; this never grants FileOrganizer authorization.
    pub async fn sync_pet_files(&self, pet: Uuid) -> Result<(), Error> {
        self.sync_pet_files_mode(pet, false).await
    }
    async fn sync_pet_files_mode(&self, pet: Uuid, preserve_manual: bool) -> Result<(), Error> {
        if pet.is_nil() {
            return Err("invalid pet identity".into());
        }
        // Fair async serialization for all clones of this store prevents an eight
        // pet burst from racing directory ACL updates or timing out on our own lock.
        // The file lock below still coordinates independently opened stores/processes.
        let _queue = self.projection_queue.lock().await;
        let root = self.files_root()?;
        let _lock = lock(&root).await?;
        let profile: String = sqlx::query_scalar("SELECT profile FROM pets WHERE id=?")
            .bind(pet.to_string())
            .fetch_one(&self.pool)
            .await?;
        let parsed: PetProfile = serde_json::from_str(&profile)?;
        parsed.validate()?;
        let directory = root.join(pet.to_string());
        private_directory(&directory)?;
        let profile_hash = format!("{:x}", Sha256::digest(profile.as_bytes()));
        let mut keep_manual = false;
        if preserve_manual {
            let manifest = directory.join("profile.manifest.json");
            let path = directory.join("pet.toml");
            if fs::symlink_metadata(&manifest)
                .is_ok_and(|m| m.is_file() && !m.file_type().is_symlink() && m.len() <= 4096)
                && fs::symlink_metadata(&path)
                    .is_ok_and(|m| m.is_file() && !m.file_type().is_symlink() && m.len() <= 32768)
            {
                let metadata: serde_json::Value = serde_json::from_slice(&fs::read(manifest)?)?;
                if metadata["pet_id"] == pet.to_string() && metadata["profile_hash"] == profile_hash
                {
                    // The canonical revision is unchanged: preserve even invalid manual drafts.
                    keep_manual = true;
                }
            }
        }
        if !keep_manual {
            atomic(&directory.join("pet.toml"), parsed.user_toml()?.as_bytes())?;
        }

        atomic(
            &directory.join("profile.manifest.json"),
            &serde_json::to_vec(
                &json!({"schema_version":1,"pet_id":pet,"profile_hash":profile_hash}),
            )?,
        )?;
        let assets = directory.join("assets");
        private_directory(&assets)?;
        let row: Option<(String, i64, i64, i64, String)> = sqlx::query_as(
            "SELECT png_base64,width,height,version,profile_hash FROM assets WHERE pet_id=?",
        )
        .bind(pet.to_string())
        .fetch_optional(&self.pool)
        .await?;
        if let Some((png, width, height, version, hash)) = row.filter(|r| r.4 == profile_hash) {
            if version < 1 || width < 1 || height < 1 {
                return Err("invalid asset manifest".into());
            }
            let bytes = base64::engine::general_purpose::STANDARD.decode(png)?;
            if bytes.len() > 16777216 || !bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
                return Err("invalid generated PNG".into());
            }
            let generated = assets.join("generated");
            private_directory(&generated)?;
            let version_dir = generated.join(version.to_string());
            private_directory(&version_dir)?;
            atomic(&version_dir.join("sheet.png"), &bytes)?;
            let manifest = AssetManifest::new(pet, u32::try_from(width)?, u32::try_from(height)?)
                .generated(
                    u32::try_from(version)?,
                    hash,
                    format!("{:x}", Sha256::digest(&bytes)),
                );
            atomic(
                &assets.join("manifest.json"),
                &serde_json::to_vec(&manifest)?,
            )?;
        } else {
            atomic(
                &assets.join("manifest.json"),
                &serde_json::to_vec(
                    &json!({"schema_version":1,"pet_id":pet,"profile_hash":profile_hash,"status":"not_generated_for_current_profile"}),
                )?,
            )?;
        }
        Ok(())
    }
    pub async fn reconcile_pet_files_startup(&self) -> Result<(), Error> {
        for item in self.pets().await? {
            self.sync_pet_files_mode(Uuid::parse_str(&item.id)?, true)
                .await?;
        }
        Ok(())
    }
    pub async fn sync_all_pet_files(&self) -> Result<(), Error> {
        for item in self.pets().await? {
            self.sync_pet_files(Uuid::parse_str(&item.id)?).await?;
        }
        Ok(())
    }
    pub async fn import_pet_profile(&self, pet: Uuid, confirmed: bool) -> Result<(), Error> {
        if !confirmed || pet.is_nil() {
            return Err("profile import requires confirmation".into());
        }
        let root = self.files_root()?;
        let profile = read_profile(&root, pet)?;
        self.save_profile(pet, &profile).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn concurrent_first_projection_creation_preserves_each_pet() {
        let directory = tempfile::tempdir().unwrap();
        let store = PetStore::open(&directory.path().join("pet.sqlite"))
            .await
            .unwrap();
        let barrier = std::sync::Arc::new(tokio::sync::Barrier::new(9));
        let mut handles = Vec::new();
        for index in 0..8 {
            let store = store.clone();
            let barrier = barrier.clone();
            handles.push(tokio::spawn(async move{
                let pet=Uuid::new_v4();
                let profile:PetProfile=serde_json::from_value(json!({"名字":format!("Concurrent {index}"),"種類":"鳥","喜好":[],"個性":[],"興趣":[]})).unwrap();
                barrier.wait().await;
                if let Err(error)=store.save_profile(pet,&profile).await {
                    let retry=store.sync_pet_files(pet).await;
                    panic!("synthetic concurrent projection failed: {error}; actual retry: {retry:?}");
                }
                pet
            }));
        }
        barrier.wait().await;
        for handle in handles {
            let pet = handle.await.unwrap();
            assert!(
                directory
                    .path()
                    .join("pets")
                    .join(pet.to_string())
                    .join("pet.toml")
                    .is_file()
            );
        }
        store.pool.close().await;
    }
    #[tokio::test]
    async fn actual_per_pet_files_are_isolated_and_confirmed_import_updates_only_selected_pet() {
        let directory = std::env::temp_dir().join(format!("pet-files-{}", Uuid::new_v4()));
        fs::create_dir(&directory).unwrap();
        let store = PetStore::open(&directory.join("pet.sqlite")).await.unwrap();
        let profile: PetProfile = serde_json::from_value(
            json!({"名字":"原始","種類":"鳥","喜好":[],"個性":[],"興趣":[]}),
        )
        .unwrap();
        let first = Uuid::new_v4();
        let second = Uuid::new_v4();
        store.save_profile(first, &profile).await.unwrap();
        store.save_profile(second, &profile).await.unwrap();
        let first_path = directory
            .join("pets")
            .join(first.to_string())
            .join("pet.toml");
        let second_path = directory
            .join("pets")
            .join(second.to_string())
            .join("pet.toml");
        let original = fs::read(&second_path).unwrap();
        let parsed: toml::Value =
            toml::from_str(&fs::read_to_string(&first_path).unwrap()).unwrap();
        assert_eq!(parsed.as_table().unwrap().len(), 5);
        let mut changed = profile.clone();
        changed.name = "檔案匯入".into();
        fs::write(&first_path, toml::to_string(&changed).unwrap()).unwrap();
        store.reconcile_pet_files_startup().await.unwrap();
        assert!(
            fs::read_to_string(&first_path)
                .unwrap()
                .contains("檔案匯入"),
            "startup preserves manual drafts before confirmation"
        );
        assert!(store.import_pet_profile(first, false).await.is_err());
        store.import_pet_profile(first, true).await.unwrap();
        assert_eq!(fs::read(&second_path).unwrap(), original);
        assert!(
            store
                .pets()
                .await
                .unwrap()
                .iter()
                .find(|p| p.id == first.to_string())
                .unwrap()
                .profile
                .contains("檔案匯入")
        );
        let manifest: serde_json::Value = serde_json::from_slice(
            &fs::read(first_path.parent().unwrap().join("profile.manifest.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(manifest["pet_id"], first.to_string());
        store.pool.close().await;
        fs::remove_dir_all(directory).unwrap();
    }
    #[tokio::test]
    async fn gui_optional_empty_overrides_roundtrip_through_toml() {
        let directory = std::env::temp_dir().join(format!("pet-files-optional-{}", Uuid::new_v4()));
        fs::create_dir(&directory).unwrap();
        let store = PetStore::open(&directory.join("pet.sqlite")).await.unwrap();
        let pet = Uuid::new_v4();
        let profile:PetProfile=serde_json::from_value(json!({"名字":"GUI 驗證","種類":"鳥","喜好":[],"個性":[],"興趣":[],"外觀":{"顏色":"翠綠色","風格":null,"品種":null},"動畫":{"動作":null,"速度":null}})).unwrap();
        store.save_profile(pet, &profile).await.unwrap();
        store.import_pet_profile(pet, true).await.unwrap();
        store.pool.close().await;
        fs::remove_dir_all(directory).unwrap();
    }
    #[cfg(unix)]
    #[tokio::test]
    async fn unsafe_projection_never_writes_outside_and_can_be_repaired_from_canonical_database() {
        let directory = std::env::temp_dir().join(format!("pet-files-fault-{}", Uuid::new_v4()));
        fs::create_dir(&directory).unwrap();
        let store = PetStore::open(&directory.join("pet.sqlite")).await.unwrap();
        let mut profile: PetProfile = serde_json::from_value(
            json!({"名字":"原始","種類":"鳥","喜好":[],"個性":[],"興趣":[]}),
        )
        .unwrap();
        let pet = Uuid::new_v4();
        store.save_profile(pet, &profile).await.unwrap();
        let path = directory
            .join("pets")
            .join(pet.to_string())
            .join("pet.toml");
        let outside = directory.join("outside.txt");
        fs::write(&outside, b"untouched").unwrap();
        fs::remove_file(&path).unwrap();
        std::os::unix::fs::symlink(&outside, &path).unwrap();
        profile.name = "canonical saved".into();
        assert!(store.save_profile(pet, &profile).await.is_err());
        assert_eq!(fs::read(&outside).unwrap(), b"untouched");
        fs::remove_file(&path).unwrap();
        store.sync_pet_files(pet).await.unwrap();
        assert!(
            fs::read_to_string(path)
                .unwrap()
                .contains("canonical saved")
        );
        store.pool.close().await;
        fs::remove_dir_all(directory).unwrap();
    }
}
