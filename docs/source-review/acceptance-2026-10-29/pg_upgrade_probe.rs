#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
 let a:Vec<String>=std::env::args().collect();let admin=sqlx::PgPool::connect("postgres://willie@127.0.0.1:55482/postgres").await?;
 for name in ["acceptance_old","acceptance_upgraded","acceptance_bad"] {let exists:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname=$1)").bind(name).fetch_one(&admin).await?;assert!(!exists,"isolated database name already exists: {name}");}
 sqlx::query("CREATE DATABASE acceptance_old").execute(&admin).await?;
 let old=sqlx::PgPool::connect("postgres://willie@127.0.0.1:55482/acceptance_old").await?;
 sqlx::migrate::Migrator::new(std::path::Path::new(&a[1])).await?.run(&old).await?;
 use argon2::PasswordHasher;
 let hash=argon2::Argon2::default().hash_password(b"acceptance-only-password").unwrap().to_string();
 sqlx::query("INSERT INTO uac_user(id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at) VALUES('legacy-admin','admin',$1,'fixture@example.invalid','驗收管理員','active',FALSE,now(),now())").bind(&hash).execute(&old).await?;
 sqlx::query("INSERT INTO product_events(source_app,event_id,schema_version,event_type) VALUES('web_chat','00000000-0000-0000-0000-000000000001',1,'chat.中文 &'),('desktop_pet','00000000-0000-0000-0000-000000000002',1,'pet.sleep')").execute(&old).await?;
 old.close().await;
 let backup=std::process::Command::new("/usr/lib/postgresql/18/bin/pg_dump").args(["-Fc","-f",&a[3],"postgres://willie@127.0.0.1:55482/acceptance_old"]).status()?;assert!(backup.success());
 sqlx::query("CREATE DATABASE acceptance_upgraded TEMPLATE acceptance_old").execute(&admin).await?;sqlx::query("CREATE DATABASE acceptance_bad TEMPLATE acceptance_old").execute(&admin).await?;
 let pool=sqlx::PgPool::connect("postgres://willie@127.0.0.1:55482/acceptance_upgraded").await?;let mig=sqlx::migrate::Migrator::new(std::path::Path::new(&a[2])).await?;mig.run(&pool).await?;
 assert_eq!(hash,sqlx::query_scalar::<_,String>("SELECT user_pd FROM uac_user WHERE id='legacy-admin'").fetch_one(&pool).await?);assert_eq!(2,sqlx::query_scalar::<_,i64>("SELECT count(*) FROM product_events").fetch_one(&pool).await?);mig.run(&pool).await?;
 let n:i64=sqlx::query_scalar("SELECT count(*) FROM _sqlx_migrations WHERE success").fetch_one(&pool).await?;pool.close().await;
 let bad=sqlx::PgPool::connect("postgres://willie@127.0.0.1:55482/acceptance_bad").await?;sqlx::query("UPDATE _sqlx_migrations SET checksum=decode(repeat('00',48),'hex') WHERE version=(SELECT min(version) FROM _sqlx_migrations)").execute(&bad).await?;let err=mig.run(&bad).await.expect_err("modified old migration must be rejected");assert_eq!(2,sqlx::query_scalar::<_,i64>("SELECT count(*) FROM product_events").fetch_one(&bad).await?);bad.close().await;admin.close().await;
 println!("analytics prior-schema populated copy upgrade PASS; password hash unchanged; events=2; migrations={n}; idempotent PASS; backup PASS; checksum drift rejected={err}");Ok(())
}
