from pathlib import Path
import subprocess,os
b=Path.cwd();src=Path('/home/willie/Documents/rust_ai_ring');d=b/'work/day44-recheck-fixed'
subprocess.run(['git','clone','--no-hardlinks',str(src),str(d)],check=True,stdout=subprocess.DEVNULL)
for w in ['crates','chat_apps','pet_apps','analytics_apps']:
 (d/w/'target').symlink_to(b/'work/latest'/w/'target',target_is_directory=True)
 for p in (d/w).rglob('*.rs'):
  if 'target' not in p.parts:os.utime(p,None)
test=r'''use sqlx::{SqlitePool, migrate::Migrator, sqlite::SqlitePoolOptions};

async fn old_database() -> (SqlitePool, Migrator) {
    let pool = SqlitePoolOptions::new().max_connections(1).connect("sqlite::memory:").await.unwrap();
    let all = sqlx::migrate!("../migrations");
    let old = Migrator::with_migrations(all.iter().filter(|m| m.version < 20261030060000).cloned().collect());
    old.run(&pool).await.unwrap();
    sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','Owner')").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id,is_default_closure) VALUES('a','A','tortoise','local-owner',0)").execute(&pool).await.unwrap();
    (pool, all)
}

#[tokio::test]
async fn old_display_names_migrate_to_codes_with_sit_priority_and_preserve_dictionary() {
    let (pool, all) = old_database().await;
    sqlx::query("INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity) VALUES('w','a','local-owner','散步',1),('s','a','local-owner','坐下',1),('p','a','local-owner','play',1)").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO codemap(id,category,code,value) VALUES('custom-walk','activity','walk','自訂散步')").execute(&pool).await.unwrap();
    all.run(&pool).await.expect("new migration must run on SQLite");
    all.run(&pool).await.unwrap();
    let activities: Vec<(String,String,i64)> = sqlx::query_as("SELECT id,activity_type,is_activity FROM pet_activity ORDER BY id").fetch_all(&pool).await.unwrap();
    assert_eq!(activities, vec![("p".into(),"play".into(),1),("s".into(),"sit".into(),1),("w".into(),"walk".into(),0)]);
    let dictionary: Vec<(String,String)> = sqlx::query_as("SELECT code,value FROM codemap WHERE category='activity' ORDER BY code").fetch_all(&pool).await.unwrap();
    assert_eq!(dictionary, vec![("sit".into(),"坐下".into()),("walk".into(),"自訂散步".into())]);
    let timestamp: Option<String> = sqlx::query_scalar("SELECT created_at FROM codemap WHERE code='sit'").fetch_one(&pool).await.unwrap();
    assert!(timestamp.is_some_and(|s| !s.is_empty()));
    let status = pet_persistence::activity::status(&pool,"a").await.unwrap();
    assert!(status.outside && !status.roaming && !status.default_caged);
    pool.close().await;
}

#[tokio::test]
async fn dictionary_id_collision_rolls_back_every_activity_change() {
    let (pool, all) = old_database().await;
    sqlx::query("INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity) VALUES('w','a','local-owner','散步',1)").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO codemap(id,category,code,value) VALUES('initial-sit','other','keep','untouched')").execute(&pool).await.unwrap();
    assert!(all.run(&pool).await.is_err());
    let activities: Vec<(String,String,i64)> = sqlx::query_as("SELECT id,activity_type,is_activity FROM pet_activity").fetch_all(&pool).await.unwrap();
    assert_eq!(activities,vec![("w".into(),"散步".into(),1)]);
    let dictionary:Vec<(String,String)> = sqlx::query_as("SELECT code,value FROM codemap").fetch_all(&pool).await.unwrap();
    assert_eq!(dictionary,vec![("keep".into(),"untouched".into())]);
    let versions:i64=sqlx::query_scalar("SELECT count(*) FROM _sqlx_migrations WHERE success=1").fetch_one(&pool).await.unwrap();
    assert_eq!(versions,3);
    pool.close().await;
}
'''
(d/'pet_apps/pet_server/tests/day44_activity_migration.rs').write_text(test)
print('candidate clone and identical migration regression tests prepared; product untouched')
