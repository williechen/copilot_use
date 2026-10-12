//! File-backed SQLite Adapter. WAL is local-host only; portable backups contain committed data.
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use std::{path::Path, str::FromStr, time::Duration};

type Error = Box<dyn std::error::Error + Send + Sync>;
pub async fn open(url: &str) -> Result<SqlitePool, Error> {
    if !url.starts_with("sqlite:") || url.contains(":memory:") {
        return Err("Chat requires a file-backed SQLite database".into());
    }
    let options = SqliteConnectOptions::from_str(url)?;
    let file = options.get_filename();
    if let Some(parent) = file.parent().filter(|p| !p.as_os_str().is_empty()) {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700))?;
            }
        }
        if std::fs::symlink_metadata(parent)?.file_type().is_symlink() {
            return Err("database directory must not be a symlink".into());
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::{MetadataExt, PermissionsExt};
            let metadata = std::fs::metadata(parent)?;
            if metadata.uid() != unsafe { libc::getuid() }
                || metadata.permissions().mode() & 0o022 != 0
            {
                return Err(
                    "database directory must be owned and not writable by other users".into(),
                );
            }
        }
    }
    if file.exists() {
        let metadata = std::fs::symlink_metadata(file)?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err("database must be a regular private file".into());
        }
    } else {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        options.open(file)?;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        // getuid has no pointers or memory side effects.
        if std::fs::metadata(file)?.uid() != unsafe { libc::getuid() }
            || std::fs::metadata(file)?.nlink() != 1
        {
            return Err("database owner or hard-link mismatch".into());
        }
        std::fs::set_permissions(file, std::fs::Permissions::from_mode(0o600))?;
    }
    for suffix in ["-wal", "-shm"] {
        let sidecar = std::path::PathBuf::from(format!("{}{suffix}", file.display()));
        if let Ok(metadata) = std::fs::symlink_metadata(sidecar)
            && (!metadata.is_file() || metadata.file_type().is_symlink())
        {
            return Err("SQLite sidecar must not be a symlink".into());
        }
    }
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(35))
        .connect_with(
            options
                .create_if_missing(false)
                .foreign_keys(true)
                .journal_mode(SqliteJournalMode::Wal)
                .synchronous(SqliteSynchronous::Full)
                .busy_timeout(Duration::from_secs(30)),
        )
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
/// SQLite VACUUM INTO is a consistent online snapshot including WAL commits; never copy a live main file alone.
pub async fn backup(pool: &SqlitePool, destination: &Path) -> Result<(), Error> {
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options.open(destination)?;
    sqlx::query("VACUUM INTO ?1")
        .bind(destination.to_str().ok_or("non-UTF8 backup path")?)
        .execute(pool)
        .await?;
    file.sync_all()?;
    let check = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(
            SqliteConnectOptions::new()
                .filename(destination)
                .read_only(true),
        )
        .await?;
    let integrity: String = sqlx::query_scalar("PRAGMA integrity_check")
        .fetch_one(&check)
        .await?;
    let violations = sqlx::query("PRAGMA foreign_key_check")
        .fetch_all(&check)
        .await?;
    check.close().await;
    if integrity != "ok" || !violations.is_empty() {
        return Err("backup integrity check failed".into());
    }
    Ok(())
}

#[cfg(test)]
#[path = "database/tests.rs"]
mod tests;
