#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
 let a:Vec<String>=std::env::args().collect();let pool=sqlx::SqlitePool::connect_with(sqlx::sqlite::SqliteConnectOptions::new().filename(&a[1])).await?;let migrator=sqlx::migrate::Migrator::new(std::path::Path::new(&a[2])).await?;migrator.run(&pool).await?;migrator.run(&pool).await?;pool.close().await;println!("existing local SQLite backup migration and idempotency PASS");Ok(())
}
