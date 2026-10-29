use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<String> = std::env::args().collect();
    let kind = &a[1];
    let root = std::path::Path::new(&a[4]);
    std::fs::create_dir_all(root)?;
    let old = root.join(format!("{kind}-old.sqlite"));
    let upgraded = root.join(format!("{kind}-upgraded.sqlite"));
    let bad = root.join(format!("{kind}-bad-checksum.sqlite"));
    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename(&old)
            .create_if_missing(true),
    )
    .await?;
    sqlx::migrate::Migrator::new(std::path::Path::new(&a[2]))
        .await?
        .run(&pool)
        .await?;
    let query = if kind == "chat" {
        sqlx::query("INSERT INTO rooms VALUES('legacy-room','legacy-category','升級前房間','保留說明','2026-10-01','2026-10-01')").execute(&pool).await?;
        sqlx::query("INSERT INTO messages(id,room_id,author_kind,content) VALUES('legacy-message','legacy-room','system','升級前訊息')").execute(&pool).await?;
        "SELECT name || ':' || content FROM rooms JOIN messages ON rooms.id=messages.room_id WHERE rooms.id='legacy-room'"
    } else {
        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('legacy-owner','既有主人')")
            .execute(&pool)
            .await?;
        sqlx::query("INSERT INTO pet_profile(id,name,species,is_default_closure,owner_id) VALUES('legacy-pet','既有寵物','cat',TRUE,'legacy-owner')").execute(&pool).await?;
        "SELECT name || ':' || species || ':' || is_default_closure FROM pet_profile WHERE id='legacy-pet'"
    };
    let before: String = sqlx::query_scalar(query).fetch_one(&pool).await?;
    pool.close().await;
    std::fs::copy(&old, &upgraded)?;
    std::fs::copy(&old, &bad)?;
    let pool = SqlitePool::connect_with(SqliteConnectOptions::new().filename(&upgraded)).await?;
    let migrator = sqlx::migrate::Migrator::new(std::path::Path::new(&a[3])).await?;
    migrator.run(&pool).await?;
    assert_eq!(
        before,
        sqlx::query_scalar::<_, String>(query)
            .fetch_one(&pool)
            .await?
    );
    migrator.run(&pool).await?;
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM _sqlx_migrations WHERE success")
        .fetch_one(&pool)
        .await?;
    pool.close().await;
    let badpool = SqlitePool::connect_with(SqliteConnectOptions::new().filename(&bad)).await?;
    sqlx::query("UPDATE _sqlx_migrations SET checksum=zeroblob(length(checksum)) WHERE version=(SELECT min(version) FROM _sqlx_migrations)").execute(&badpool).await?;
    let err = migrator
        .run(&badpool)
        .await
        .expect_err("checksum drift must fail closed");
    assert_eq!(
        before,
        sqlx::query_scalar::<_, String>(query)
            .fetch_one(&badpool)
            .await?
    );
    badpool.close().await;
    println!(
        "{kind}: populated prior-schema copy upgrade PASS; data={before}; idempotent PASS; migrations={count}; checksum drift rejected={err}"
    );
    Ok(())
}
