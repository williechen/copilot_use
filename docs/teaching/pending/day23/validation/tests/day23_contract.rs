use axum::{body::Body, http::{Request,StatusCode}};
use tower::ServiceExt;
#[tokio::test]
async fn day23_fresh_signup_contract() {
 let pool=sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
 persistence::migrate(&pool).await.unwrap();
 let response=server::signup::routes(pool.clone()).oneshot(Request::post("/signup").header("origin","http://127.0.0.1:3000").header("content-type","application/x-www-form-urlencoded").body(Body::from("username=day23learner&email=day23%40example.test&display_name=Day23&password=lesson-only-password")).unwrap()).await.unwrap();
 assert_eq!(response.status(),StatusCode::SEE_OTHER);
 let (hash,sex,birthday):(String,Option<String>,Option<String>)=sqlx::query_as("SELECT user_pd,user_sexy,user_birthday FROM mbr_user").fetch_one(&pool).await.unwrap();
 assert_eq!((sex,birthday),(None,None));
 use argon2::{Argon2,PasswordHash,PasswordVerifier};
 Argon2::default().verify_password(b"lesson-only-password",&PasswordHash::new(&hash).unwrap()).unwrap();
}
#[tokio::test]
async fn day23_old_schema_returns_500() {
 let pool=sqlx::sqlite::SqlitePoolOptions::new().max_connections(1).connect("sqlite::memory:").await.unwrap();
 sqlx::raw_sql(include_str!("fixtures/old-auth.sql")).execute(&pool).await.unwrap();
 let r=server::signup::routes(pool.clone()).oneshot(Request::post("/signup").header("origin","http://127.0.0.1:3000").header("content-type","application/x-www-form-urlencoded").body(Body::from("username=day23learner&email=day23%40example.test&display_name=Day23&password=lesson-only-password")).unwrap()).await.unwrap();
 assert_eq!(r.status(),StatusCode::INTERNAL_SERVER_ERROR);
 let count:i64=sqlx::query_scalar("SELECT count(*) FROM mbr_user").fetch_one(&pool).await.unwrap();assert_eq!(count,0);
}
