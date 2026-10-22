use sqlx::migrate::{Migrator,MigrateError};
#[tokio::test]
async fn observes_pet_old_database_checksum_block_without_deleting_profile(){
 let pool=sqlx::sqlite::SqlitePoolOptions::new().max_connections(1).connect("sqlite::memory:").await.unwrap();
 let path=std::env::var("DAY24_BASELINE_PET").unwrap();Migrator::new(std::path::Path::new(&path)).await.unwrap().run(&pool).await.unwrap();
 desktop_pet::pet_store::PetStore{pool:pool.clone()}.create("升級必須保留").await.unwrap();
 let error=sqlx::migrate!("./migrations").run(&pool).await.unwrap_err();
 println!("observed expected upgrade defect: {error}");assert!(matches!(error,MigrateError::VersionMismatch(20261018095626)));
 let count:i64=sqlx::query_scalar("SELECT count(*) FROM pet_profile").fetch_one(&pool).await.unwrap();assert_eq!(count,1);
}
