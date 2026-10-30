# Pet Day44｜活動模型與舊代碼升級已在main

## ① 現況、來源與caller

產品唯讀`2c7ff8b861d32ab6d879bf80c9f2dce50de38097`已採用forward第5版、升級example與兩項回歸測試。本輪以主分支原檔重驗112項通過；原候選附件已封存，**不要重複套用**。Day不增加，next_day維持45。

同pet_id／owner_id任一`pet_activity.is_activity=true`就是出籠，不限walk／不限一筆true；全false／無活動才回籠。新建profile與walk／sit兩筆inactive同交易，既有UI隨後明確select／出籠。walk允許漫遊，sit仍出籠但禁止漫遊；同時true時sit優先，持久化walk=false。回籠／Native X停該隻全部活動，不動其他寵物；既有is_default_closure只作啟動預設，沒有新籠態或漫遊欄位。

正式鏈仍為ActivityControls→bridge／index.html invoke→Tauri逐UUID命令→PetStore→activity交易；Native錯誤還原精確活動與選取snapshot。startup只恢復預設可出籠者。漫遊逐步鎖內核對DB／generation，受既有PET_ENABLE_ROAM opt-in、睡覺及拖曳暫停限制。這些主分支程式與舊GUI人工驗收分開，新模型native旅程仍未驗。

## Refactor｜已採用的找回視窗幾何搬層

pet_windows的find_saved_pet呼叫window_position::centered_position，純幾何與native視窗／Runtime責任分開；此重構早已在main，不新增工作量或Day。保留負座標、工作區小於視窗、UUID／A/B等有效斷言，本輪Pet112包含原斷言。完整目前純模組如下：

```rust
use tauri::{PhysicalPosition, PhysicalRect, PhysicalSize};

/// Pure native-window geometry. The caller owns show/focus and roam pause/resume.
pub(crate) fn centered_position(
    area: &PhysicalRect<i32, u32>,
    size: PhysicalSize<u32>,
) -> PhysicalPosition<i32> {
    let x = i64::from(area.position.x) + i64::from(area.size.width.saturating_sub(size.width) / 2);
    let y =
        i64::from(area.position.y) + i64::from(area.size.height.saturating_sub(size.height) / 2);
    PhysicalPosition {
        x: x.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
        y: y.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_work_area_never_places_window_before_its_origin() {
        let area = PhysicalRect {
            position: PhysicalPosition { x: -300, y: 20 },
            size: PhysicalSize {
                width: 100,
                height: 100,
            },
        };
        assert_eq!(
            centered_position(
                &area,
                PhysicalSize {
                    width: 200,
                    height: 300
                }
            ),
            PhysicalPosition { x: -300, y: 20 }
        );
        assert_eq!(
            centered_position(
                &area,
                PhysicalSize {
                    width: 20,
                    height: 20
                }
            ),
            PhysicalPosition { x: -260, y: 60 }
        );
    }
}
```

## ② 已採用的完整Green與回歸程式

第5版只把舊「散步／坐下」轉為walk／sit並正規化sit優先，保留原活動ID／owner／其他活動／預設與字典，不改前四版migration。第4版可能已建立同類型基本列，第5版保留所有列；重複操作使用既有活動，不反覆新增。字典主鍵碰撞仍安全拒絕，沒有自動覆寫。以下展開本次三個完整單元，作為目前main的教學檢閱，無須再套patch。

原候選與main只差migration檔尾換行，checksum仍不同；下列Markdown呈現不能替代原檔bytes。部署／恢復請使用精確Git blob與[checksum清單](../../../source-review/pet-day44-adopted-2026-10-30/migration-checksums.json)，不得手改已部署migration或跳過SQLx checksum。

### `pet_apps/migrations/20261030140000_normalize_activity_codes.sql`

[精確主分支原檔](https://github.com/williechen/rust_ai_ring/blob/2c7ff8b861d32ab6d879bf80c9f2dce50de38097/pet_apps/migrations/20261030140000_normalize_activity_codes.sql)

```sql
-- Preserve all activity IDs, owner links, dictionary rows and deployed migration checksums.
UPDATE pet_activity SET activity_type=CASE activity_type WHEN '散步' THEN 'walk' WHEN '坐下' THEN 'sit' END
WHERE activity_type IN ('散步','坐下');
UPDATE pet_activity AS w SET is_activity=0
WHERE w.activity_type='walk' AND w.is_activity=1
AND EXISTS(SELECT 1 FROM pet_activity s WHERE s.pet_id=w.pet_id AND s.owner_id=w.owner_id AND s.activity_type='sit' AND s.is_activity=1);
```

### `pet_apps/pet_server/examples/day44_upgrade.rs`

[精確主分支原檔](https://github.com/williechen/rust_ai_ring/blob/2c7ff8b861d32ab6d879bf80c9f2dce50de38097/pet_apps/pet_server/examples/day44_upgrade.rs)

```rust
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename(&args[1])
            .create_if_missing(true),
    )
    .await?;
    if args.get(4).is_some_and(|s| s == "seed") {
        sqlx::migrate::Migrator::new(std::path::Path::new(&args[2]))
            .await?
            .run(&pool)
            .await?;
        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','驗收主人')")
            .execute(&pool)
            .await?;
        sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id,is_default_closure) VALUES('old-out','Old Out','tortoise','local-owner',0),('old-in','Old In','tortoise','local-owner',1),('old-sit','Old Sit','tortoise','local-owner',0)").execute(&pool).await?;
        sqlx::query("INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity) VALUES('legacy-walk','old-sit','local-owner','散步',1),('legacy-sit','old-sit','local-owner','坐下',1),('legacy-play','old-sit','local-owner','玩耍',1)").execute(&pool).await?;
    }
    let profile_count: i64 = sqlx::query_scalar("SELECT count(*) FROM pet_profile")
        .fetch_one(&pool)
        .await?;
    let schema: Vec<(String, String)> =
        sqlx::query_as("SELECT name,sql FROM sqlite_master WHERE type='table' ORDER BY name")
            .fetch_all(&pool)
            .await?;
    let before: Vec<(String, Option<i64>)> =
        sqlx::query_as("SELECT id,is_default_closure FROM pet_profile ORDER BY id")
            .fetch_all(&pool)
            .await?;
    let m = sqlx::migrate::Migrator::new(std::path::Path::new(&args[3])).await?;
    if args.get(4).is_some_and(|s| s == "tamper") {
        sqlx::query("UPDATE _sqlx_migrations SET checksum=zeroblob(length(checksum)) WHERE version=(SELECT min(version) FROM _sqlx_migrations)").execute(&pool).await?;
        assert!(m.run(&pool).await.is_err());
        assert_eq!(
            before,
            sqlx::query_as::<_, (String, Option<i64>)>(
                "SELECT id,is_default_closure FROM pet_profile ORDER BY id"
            )
            .fetch_all(&pool)
            .await?
        );
        println!("checksum drift rejected and profile preferences preserved");
        return Ok(());
    }
    m.run(&pool).await?;
    m.run(&pool).await?;
    assert_eq!(
        schema,
        sqlx::query_as::<_, (String, String)>(
            "SELECT name,sql FROM sqlite_master WHERE type='table' ORDER BY name"
        )
        .fetch_all(&pool)
        .await?
    );
    assert_eq!(
        before,
        sqlx::query_as::<_, (String, Option<i64>)>(
            "SELECT id,is_default_closure FROM pet_profile ORDER BY id"
        )
        .fetch_all(&pool)
        .await?
    );
    let missing:i64=sqlx::query_scalar("SELECT count(*) FROM pet_profile p WHERE NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=p.id AND a.owner_id=p.owner_id AND a.activity_type='walk') OR NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=p.id AND a.owner_id=p.owner_id AND a.activity_type='sit')").fetch_one(&pool).await?;
    assert_eq!(missing, 0);
    let conflicts:i64=sqlx::query_scalar("SELECT count(*) FROM pet_activity w WHERE w.activity_type='walk' AND w.is_activity=1 AND EXISTS(SELECT 1 FROM pet_activity s WHERE s.pet_id=w.pet_id AND s.owner_id=w.owner_id AND s.activity_type='sit' AND s.is_activity=1)").fetch_one(&pool).await?;
    assert_eq!(conflicts, 0);
    if args.get(4).is_some_and(|s| s == "seed") {
        let out = pet_persistence::activity::status(&pool, "old-out").await?;
        assert!(out.outside && out.roaming);
        let caged = pet_persistence::activity::status(&pool, "old-in").await?;
        assert!(!caged.outside);
        let sit = pet_persistence::activity::status(&pool, "old-sit").await?;
        assert!(sit.outside && !sit.roaming);
        let play: i64 =
            sqlx::query_scalar("SELECT is_activity FROM pet_activity WHERE id='legacy-play'")
                .fetch_one(&pool)
                .await?;
        assert_eq!(play, 1);
    }
    println!(
        "upgrade/idempotence passed; profiles={profile_count}; profile defaults unchanged; schema/columns unchanged; two basic activities present; sit priority preserved"
    );
    pool.close().await;
    Ok(())
}
```

### `pet_apps/pet_server/tests/day44_activity_migration.rs`

[精確主分支原檔](https://github.com/williechen/rust_ai_ring/blob/2c7ff8b861d32ab6d879bf80c9f2dce50de38097/pet_apps/pet_server/tests/day44_activity_migration.rs)

```rust
use sqlx::{SqlitePool, migrate::Migrator, sqlite::SqlitePoolOptions};

async fn old_database() -> (SqlitePool, Migrator) {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    let all = sqlx::migrate!("../migrations");
    let old = Migrator::with_migrations(
        all.iter()
            .filter(|m| m.version < 20261030060000)
            .cloned()
            .collect(),
    );
    old.run(&pool).await.unwrap();
    sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','Owner')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id,is_default_closure) VALUES('a','A','tortoise','local-owner',0)").execute(&pool).await.unwrap();
    (pool, all)
}

#[tokio::test]
async fn old_display_names_migrate_to_codes_with_sit_priority_and_preserve_dictionary() {
    let (pool, all) = old_database().await;
    sqlx::query("INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity) VALUES('w','a','local-owner','散步',1),('s','a','local-owner','坐下',1),('p','a','local-owner','play',1)").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO codemap(id,category,code,value) VALUES('custom-walk','activity','walk','自訂散步')").execute(&pool).await.unwrap();
    all.run(&pool)
        .await
        .expect("new migration must run on SQLite");
    all.run(&pool).await.unwrap();
    let activities: Vec<(String, String, i64)> =
        sqlx::query_as("SELECT id,activity_type,is_activity FROM pet_activity WHERE id IN ('w','s','p') ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(
        activities,
        vec![
            ("p".into(), "play".into(), 1),
            ("s".into(), "sit".into(), 1),
            ("w".into(), "walk".into(), 0)
        ]
    );
    let custom: String = sqlx::query_scalar("SELECT value FROM codemap WHERE id='custom-walk'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(custom, "自訂散步");
    let dictionary_count: i64 = sqlx::query_scalar("SELECT count(*) FROM codemap")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(dictionary_count, 3); // existing row plus main's two initial dictionary rows, none deleted
    let activity_count: i64 = sqlx::query_scalar("SELECT count(*) FROM pet_activity")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(activity_count, 5); // three original IDs plus the two basic rows created by deployed migration 4
    let timestamp: Option<String> =
        sqlx::query_scalar("SELECT created_at FROM codemap WHERE code='sit'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(timestamp.is_some_and(|s| !s.is_empty()));
    let status = pet_persistence::activity::status(&pool, "a").await.unwrap();
    assert!(status.outside && !status.roaming && !status.default_caged);
    pool.close().await;
}

#[tokio::test]
async fn dictionary_id_collision_rolls_back_every_activity_change() {
    let (pool, all) = old_database().await;
    sqlx::query("INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity) VALUES('w','a','local-owner','散步',1)").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO codemap(id,category,code,value) VALUES('initial-sit','other','keep','untouched')").execute(&pool).await.unwrap();
    assert!(all.run(&pool).await.is_err());
    let activities: Vec<(String, String, i64)> =
        sqlx::query_as("SELECT id,activity_type,is_activity FROM pet_activity")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(activities, vec![("w".into(), "散步".into(), 1)]);
    let dictionary: Vec<(String, String)> = sqlx::query_as("SELECT code,value FROM codemap")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(dictionary, vec![("keep".into(), "untouched".into())]);
    let versions: i64 = sqlx::query_scalar("SELECT count(*) FROM _sqlx_migrations WHERE success=1")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(versions, 3);
    pool.close().await;
}
```

## ③ 版本與相依

四workspace manifest／lock未變；本輪rustc／cargo／Node實際版本見[原始命令](../../../source-review/pet-day44-adopted-2026-10-30/commands.json)。沒有升級依賴，未宣稱完整官方最新版本或跨平台MSRV Gate完成。

## ④ Gate2實測更新（2026-10-31）

本輪2c7ff8b已完成前端release與原生debug內嵌資產建置，真Tauri／WebKit／私有Xvfb功能25項通過：三隻、位移／坐下、DB失敗保留、Native X、控制台隱藏重開、窄窗、睡覺／醒來及兩次真程序重啟與預設資格。沒有mock IPC或修改產品。既有同SHA112項、Node12／WASM與備份副本證據仍沿原次執行，不重複計功。

[本輪完整證據與截圖](../../../source-review/gate2-2026-10-31/README.md) · [25步原始紀錄](../../../source-review/gate2-2026-10-31/pet-gui-checks.json) · [免重編實機腳本](../../../source-review/gate2-2026-10-31/manual-pet.sh)

實際桌面右鍵／拖曳／縮放／多螢幕及真OS開窗失敗仍待驗；已準備獨立手動DB與凍結原生產物，預檢不等於人工通過。整體Gate2另受Chat排除名單與Analytics授權缺口／實際舊PG缺備份阻擋，不能用此25步當完整驗收。next_day45、FIFO、last_full_comparison不變。
