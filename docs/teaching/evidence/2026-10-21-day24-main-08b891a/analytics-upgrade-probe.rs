use sqlx::migrate::{Migrator,MigrateError};
#[tokio::test]
async fn observes_analytics_old_database_checksum_block_without_deleting_admin(){
 let url=std::env::var("DAY24_UPGRADE_PG_URL").unwrap();let pool=sqlx::PgPool::connect(&url).await.unwrap();
 let path=std::env::var("DAY24_BASELINE_PG").unwrap();Migrator::new(std::path::Path::new(&path)).await.unwrap().run(&pool).await.unwrap();
 stats_server::bootstrap::initialize(&pool,"lesson-only-password".into()).await.unwrap();
 let error=sqlx::migrate!("./migrations").run(&pool).await.unwrap_err();
 println!("observed expected upgrade defect: {error}");assert!(matches!(error,MigrateError::VersionMismatch(20261018073932)));
 let count:i64=sqlx::query_scalar("SELECT count(*) FROM uac_user").fetch_one(&pool).await.unwrap();assert_eq!(count,1);
}
