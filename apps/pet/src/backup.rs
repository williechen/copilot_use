//! App-owned SQLite snapshots include generated image assets. Restore user data transactionally,
//! preserving current event delivery, filesystem receipts and pets absent from the snapshot.
use crate::state::PetStore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{io::Read, path::PathBuf};
use uuid::Uuid;
type Error = Box<dyn std::error::Error + Send + Sync>;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub id: Uuid,
    pub sha256: String,
    pub created_at: i64,
    pub bytes: u64,
}
impl PetStore {
    fn backups(&self) -> Result<PathBuf, Error> {
        Ok(self
            .database_path()
            .parent()
            .ok_or("missing data directory")?
            .join("backups"))
    }
    pub async fn create_snapshot(&self) -> Result<Snapshot, Error> {
        let root = self.backups()?;
        std::fs::create_dir_all(&root)?;
        if std::fs::symlink_metadata(&root)?.file_type().is_symlink() {
            return Err("unsafe backup directory".into());
        }
        let id = Uuid::new_v4();
        let directory = root.join(id.to_string());
        std::fs::create_dir(&directory)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700))?;
        }
        let path = directory.join("snapshot.sqlite");
        // SQLite's online VACUUM snapshot is consistent while the application keeps running.
        sqlx::query("VACUUM main INTO ?")
            .bind(path.to_str().ok_or("invalid snapshot path")?)
            .execute(&self.pool)
            .await?;
        let bytes = std::fs::read(&path)?;
        let snapshot = Snapshot {
            id,
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            created_at: crate::state::now(),
            bytes: bytes.len() as u64,
        };
        std::fs::write(
            directory.join("manifest.json"),
            serde_json::to_vec(&snapshot)?,
        )?;
        std::fs::File::open(&path)?.sync_all()?;
        std::fs::File::open(directory.join("manifest.json"))?.sync_all()?;
        #[cfg(unix)]
        std::fs::File::open(directory)?.sync_all()?;
        Ok(snapshot)
    }
    pub fn snapshots(&self) -> Result<Vec<Snapshot>, Error> {
        let root = self.backups()?;
        if !root.exists() {
            return Ok(vec![]);
        }
        let mut result = Vec::new();
        for item in std::fs::read_dir(root)? {
            let item = item?;
            if !item.file_type()?.is_dir() {
                continue;
            }
            let Ok(id) = Uuid::parse_str(&item.file_name().to_string_lossy()) else {
                continue;
            };
            let file = item.path().join("manifest.json");
            if std::fs::symlink_metadata(&file)?.file_type().is_symlink() {
                continue;
            }
            let bytes = std::fs::read(file)?;
            if bytes.len() > 4096 {
                continue;
            }
            let entry: Snapshot = serde_json::from_slice(&bytes)?;
            if entry.id == id {
                result.push(entry)
            }
        }
        result.sort_by_key(|s| std::cmp::Reverse(s.created_at));
        result.truncate(100);
        Ok(result)
    }
    pub async fn restore_snapshot(&self, id: Uuid, confirmed: bool) -> Result<Snapshot, Error> {
        if !confirmed {
            return Err("restore requires confirmation".into());
        }
        let directory = self.backups()?.join(id.to_string());
        if std::fs::symlink_metadata(&directory)?
            .file_type()
            .is_symlink()
        {
            return Err("unsafe snapshot directory".into());
        }
        let manifest = directory.join("manifest.json");
        if std::fs::symlink_metadata(&manifest)?
            .file_type()
            .is_symlink()
        {
            return Err("unsafe manifest".into());
        }
        let entry: Snapshot = serde_json::from_slice(&std::fs::read(manifest)?)?;
        if entry.id != id || entry.bytes > 268435456 {
            return Err("invalid snapshot".into());
        }
        let path = directory.join("snapshot.sqlite");
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
        }
        let file = options.open(&path)?;
        if !file.metadata()?.is_file() {
            return Err("unsafe snapshot".into());
        }
        let mut bytes = Vec::new();
        file.take(268435457).read_to_end(&mut bytes)?;
        if bytes.len() as u64 != entry.bytes
            || format!("{:x}", Sha256::digest(&bytes)) != entry.sha256
        {
            return Err("snapshot checksum mismatch".into());
        }
        // This checkpoint is kept even when restore validation/transaction fails.
        let before = self.create_snapshot().await?;
        let temporary = self
            .backups()?
            .join(format!("restore-{}.sqlite", Uuid::new_v4()));
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        {
            use std::io::Write;
            let mut file = options.open(&temporary)?;
            file.write_all(&bytes)?;
            file.sync_all()?;
        }
        let restored = self.restore_verified(&temporary).await;
        let _ = std::fs::remove_file(temporary);
        restored?;
        Ok(before)
    }
    async fn restore_verified(&self, path: &std::path::Path) -> Result<(), Error> {
        let mut connection = self.pool.acquire().await?;
        sqlx::query("ATTACH DATABASE ? AS snapshot")
            .bind(path.to_str().ok_or("invalid snapshot path")?)
            .execute(&mut *connection)
            .await?;
        let result=async {
            let integrity:String=sqlx::query_scalar("PRAGMA snapshot.integrity_check").fetch_one(&mut *connection).await?;if integrity!="ok"{return Err("snapshot integrity check failed".into())}
            let compatible:bool=sqlx::query_scalar("SELECT NOT EXISTS(SELECT version,checksum FROM main._sqlx_migrations EXCEPT SELECT version,checksum FROM snapshot._sqlx_migrations) AND NOT EXISTS(SELECT version,checksum FROM snapshot._sqlx_migrations EXCEPT SELECT version,checksum FROM main._sqlx_migrations)").fetch_one(&mut *connection).await?;
            if !compatible{return Err("snapshot schema differs from current application".into())}
            sqlx::query("BEGIN IMMEDIATE").execute(&mut *connection).await?;
            let copied=async {
                sqlx::query("INSERT INTO main.pets SELECT * FROM snapshot.pets WHERE true ON CONFLICT(id)DO UPDATE SET profile=excluded.profile").execute(&mut *connection).await?;
                for (delete,copy) in [
 ("DELETE FROM main.memories WHERE pet_id IN(SELECT id FROM snapshot.pets)","INSERT INTO main.memories SELECT * FROM snapshot.memories"),
 ("DELETE FROM main.cards WHERE pet_id IN(SELECT id FROM snapshot.pets)","INSERT INTO main.cards SELECT * FROM snapshot.cards"),
 ("DELETE FROM main.conversations WHERE pet_id IN(SELECT id FROM snapshot.pets)","INSERT INTO main.conversations SELECT * FROM snapshot.conversations"),
 ("DELETE FROM main.assets WHERE pet_id IN(SELECT id FROM snapshot.pets)","INSERT INTO main.assets SELECT * FROM snapshot.assets")
] {
 sqlx::query(delete).execute(&mut *connection).await?;
 sqlx::query(copy).execute(&mut *connection).await?;
}
                sqlx::query("INSERT INTO main.settings SELECT * FROM snapshot.settings WHERE true ON CONFLICT(key)DO UPDATE SET value=excluded.value").execute(&mut *connection).await?;
                Ok::<(),sqlx::Error>(())
            }.await;
            match copied {Ok(())=>{sqlx::query("COMMIT").execute(&mut *connection).await?;},Err(e)=>{let _=sqlx::query("ROLLBACK").execute(&mut *connection).await;return Err(e.into());}}
            Ok::<(),Error>(())
        }.await;
        let detach = sqlx::query("DETACH DATABASE snapshot")
            .execute(&mut *connection)
            .await;
        result?;
        detach?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn snapshots_restore_profiles_memories_and_assets_without_losing_delivery_or_new_pets() {
        let dir = std::env::temp_dir().join(format!("pet-backup-{}", Uuid::new_v4()));
        std::fs::create_dir(&dir).unwrap();
        let store = PetStore::open(&dir.join("pet.sqlite")).await.unwrap();
        let pet = Uuid::new_v4();
        let profile =
            serde_json::from_str(r#"{"名字":"原始","種類":"鳥","喜好":[],"個性":[],"興趣":[]}"#)
                .unwrap();
        store.save_profile(pet, &profile).await.unwrap();
        let memory = Uuid::new_v4();
        store.save_memory(pet, memory, "before").await.unwrap();
        store.add_card(pet, "Q", "A").await.unwrap();
        sqlx::query("INSERT INTO assets VALUES(?,?,200,200,1,0)")
            .bind(pet.to_string())
            .bind("base64-fixture")
            .execute(&store.pool)
            .await
            .unwrap();
        let backup = store.create_snapshot().await.unwrap();
        assert_eq!(store.snapshots().unwrap().len(), 1);
        store.save_memory(pet, memory, "after").await.unwrap();
        let new_pet = Uuid::new_v4();
        store.save_profile(new_pet, &profile).await.unwrap();
        sqlx::query("INSERT INTO pet_outbox(event_id,payload)VALUES('new-event','{}')")
            .execute(&store.pool)
            .await
            .unwrap();
        assert!(store.restore_snapshot(backup.id, false).await.is_err());
        let undo = store.restore_snapshot(backup.id, true).await.unwrap();
        assert_eq!(store.memories(pet).await.unwrap()[0].content, "before");
        assert_eq!(store.pets().await.unwrap().len(), 2);
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM pet_outbox")
                .fetch_one(&store.pool)
                .await
                .unwrap(),
            1
        );
        assert_eq!(
            sqlx::query_scalar::<_, String>("SELECT png_base64 FROM assets")
                .fetch_one(&store.pool)
                .await
                .unwrap(),
            "base64-fixture"
        );
        store.restore_snapshot(undo.id, true).await.unwrap();
        assert_eq!(store.memories(pet).await.unwrap()[0].content, "after");
        std::fs::write(
            dir.join("backups")
                .join(backup.id.to_string())
                .join("snapshot.sqlite"),
            b"tampered",
        )
        .unwrap();
        assert!(store.restore_snapshot(backup.id, true).await.is_err());
        assert_eq!(store.memories(pet).await.unwrap()[0].content, "after");
        store.pool.close().await;
        std::fs::remove_dir_all(dir).unwrap();
    }
}
