//! PostgreSQL Adapter: UUID remains a domain value; the column may be UUID or VARCHAR(40).
use event_contract::ProductEvent;
use serde::Serialize;
use sqlx::{FromRow, PgPool, Row};
#[derive(Debug, Serialize, FromRow, PartialEq, Eq)]
pub struct EventSummary {
    pub source_app: String,
    pub event_type: String,
    pub total: i64,
}

#[derive(Clone)]
pub struct EventStore {
    pool: PgPool,
}

impl EventStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// 真正持久化成功才回 2xx；同一來源/事件的重送不再增加計數。
    pub async fn insert_once(&self, event: &ProductEvent) -> Result<bool, sqlx::Error> {
        use sha2::{Digest, Sha256};
        let key =
            Sha256::digest(format!("{}:{}", event.source_app, event.event_id).as_bytes()).to_vec();
        let kind = Sha256::digest(event.event_type.as_bytes()).to_vec();
        let mut tx = self.pool.begin().await?;
        let inserted=sqlx::query("INSERT INTO event_dedup_keys(key_hash,type_hash) VALUES ($1,$2) ON CONFLICT DO NOTHING").bind(&key).bind(&kind).execute(&mut *tx).await?;
        if inserted.rows_affected() == 0 {
            let previous: Vec<u8> =
                sqlx::query_scalar("SELECT type_hash FROM event_dedup_keys WHERE key_hash=$1")
                    .bind(key)
                    .fetch_one(&mut *tx)
                    .await?;
            if previous != kind {
                return Err(sqlx::Error::Protocol("event_identity_conflict".into()));
            }
            tx.commit().await?;
            return Ok(false);
        }
        sqlx::query("INSERT INTO product_events(source_app,event_id,schema_version,event_type) VALUES ($1,$2,$3,$4)")
            .bind(&event.source_app).bind(event.event_id).bind(i16::try_from(event.schema_version).unwrap_or(-1)).bind(&event.event_type).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(true)
    }

    pub async fn health(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
    pub async fn lookup(
        &self,
        source: &str,
        id: uuid::Uuid,
    ) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar("SELECT event_type FROM product_events WHERE source_app=$1 AND event_id::uuid=$2 AND NOT archived")
            .bind(source).bind(id).fetch_optional(&self.pool).await
    }
    pub async fn summary(&self) -> Result<Vec<EventSummary>, sqlx::Error> {
        sqlx::query_as::<_, EventSummary>(
            "SELECT source_app, event_type, count(*)::bigint AS total
             FROM product_events WHERE NOT archived
             GROUP BY source_app, event_type ORDER BY source_app, event_type",
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn total(&self) -> Result<i64, sqlx::Error> {
        let row = sqlx::query("SELECT count(*) AS total FROM product_events WHERE NOT archived")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.get("total"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[sqlx::test(migrations = "./migrations")]
    async fn uuid_domain_event_roundtrips_through_native_and_varchar_storage(pool: PgPool) {
        let store = EventStore::new(pool.clone());
        let original = ProductEvent::chat_message_persisted(uuid::Uuid::new_v4());
        assert!(store.insert_once(&original).await.unwrap());
        assert_eq!(
            store
                .lookup(&original.source_app, original.event_id)
                .await
                .unwrap()
                .as_deref(),
            Some("message.persisted")
        );
        sqlx::query("ALTER TABLE product_events ALTER COLUMN event_id TYPE VARCHAR(40) USING event_id::text").execute(&pool).await.unwrap();
        let stored: String = sqlx::query_scalar("SELECT event_id FROM product_events")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(uuid::Uuid::parse_str(&stored).unwrap(), original.event_id);
        assert!(!store.insert_once(&original).await.unwrap());
        let mut text_event = ProductEvent::pet_cleanup_succeeded(uuid::Uuid::new_v4());
        let (a, b) = tokio::join!(
            store.insert_once(&text_event),
            store.insert_once(&text_event)
        );
        assert!(a.unwrap() ^ b.unwrap());
        assert_eq!(
            store
                .lookup(&text_event.source_app, text_event.event_id)
                .await
                .unwrap()
                .as_deref(),
            Some("file.cleanup.succeeded")
        );
        text_event.event_type = "question.answered".into();
        assert!(matches!(
            store.insert_once(&text_event).await,
            Err(sqlx::Error::Protocol(_))
        ));
        sqlx::query(
            "ALTER TABLE product_events ALTER COLUMN event_id TYPE UUID USING event_id::uuid",
        )
        .execute(&pool)
        .await
        .unwrap();
        assert_eq!(
            store
                .lookup(&original.source_app, original.event_id)
                .await
                .unwrap()
                .as_deref(),
            Some("message.persisted")
        );
        assert_eq!(store.total().await.unwrap(), 2);
    }
}
