#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
 let a:Vec<String>=std::env::args().collect();let pool=sqlx::PgPool::connect("postgres://willie@127.0.0.1:55482/postgres").await?;
 let exists:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname='recheck_20261030_restore')").fetch_one(&pool).await?;assert!(!exists);sqlx::query("CREATE DATABASE recheck_20261030_restore").execute(&pool).await?;pool.close().await;
 let r=std::process::Command::new("/usr/lib/postgresql/18/bin/pg_restore").args(["--no-owner","--dbname=postgres://willie@127.0.0.1:55482/recheck_20261030_restore",&a[1]]).status()?;assert!(r.success());
 let pool=sqlx::PgPool::connect("postgres://willie@127.0.0.1:55482/recheck_20261030_restore").await?;assert_eq!(2,sqlx::query_scalar::<_,i64>("SELECT count(*) FROM _sqlx_migrations").fetch_one(&pool).await?);assert_eq!(2,sqlx::query_scalar::<_,i64>("SELECT count(*) FROM product_events").fetch_one(&pool).await?);
 sqlx::migrate::Migrator::new(std::path::Path::new(&a[2])).await?.run(&pool).await?;assert_eq!(2,sqlx::query_scalar::<_,i64>("SELECT count(*) FROM product_events").fetch_one(&pool).await?);pool.close().await;println!("pg_dump restore old schema PASS; restored prior events=2; forward upgrade after restore PASS");Ok(())
}
