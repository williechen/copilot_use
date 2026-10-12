use super::*;
#[tokio::test]
async fn snapshot_is_portable_with_wal_commits_and_never_overwrites_existing_data() {
    let directory = std::env::temp_dir().join(format!("chat-portable-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&directory).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let db = directory.join("chat.sqlite3");
    let copy = directory.join("copy.sqlite3");
    let pool = open(&format!("sqlite://{}", db.display())).await.unwrap();
    sqlx::query("INSERT INTO chat_rooms(id,name)VALUES('portable','Portable')")
        .execute(&pool)
        .await
        .unwrap();
    let id = crate::ChatWriter::new(pool.clone())
        .persist("portable", "committed WAL message")
        .await
        .unwrap();
    backup(&pool, &copy).await.unwrap();
    let bytes = std::fs::read(&copy).unwrap();
    assert!(backup(&pool, &copy).await.is_err());
    assert_eq!(std::fs::read(&copy).unwrap(), bytes);
    let moved = directory.join("moved.sqlite3");
    std::fs::rename(&copy, &moved).unwrap();
    let portable = open(&format!("sqlite://{}", moved.display()))
        .await
        .unwrap();
    let content: String = sqlx::query_scalar("SELECT content FROM chat_messages WHERE id=?1")
        .bind(id)
        .fetch_one(&portable)
        .await
        .unwrap();
    assert_eq!(content, "committed WAL message");
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM chat_outbox WHERE event_id=?1")
        .bind(id)
        .fetch_one(&portable)
        .await
        .unwrap();
    assert_eq!(count, 1);
    portable.close().await;
    pool.close().await;
    std::fs::remove_dir_all(directory).unwrap();
}
#[cfg(unix)]
#[tokio::test]
async fn sqlite_rejects_symlink_and_hardlinked_databases() {
    let directory = std::env::temp_dir().join(format!("chat-links-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&directory).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let original = directory.join("original");
    std::fs::write(&original, b"").unwrap();
    let link = directory.join("link");
    std::os::unix::fs::symlink(&original, &link).unwrap();
    assert!(open(&format!("sqlite://{}", link.display())).await.is_err());
    std::fs::remove_file(&link).unwrap();
    std::fs::hard_link(&original, &link).unwrap();
    assert!(open(&format!("sqlite://{}", link.display())).await.is_err());
    std::fs::remove_dir_all(directory).unwrap();
}
