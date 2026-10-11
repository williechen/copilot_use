//! Pet 獨立 SQLite Adapter；Profile、記憶、複習與事件不共享 Chat/Analytics DB。
use crate::profile::PetProfile;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow, SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

pub fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
#[derive(Clone)]
pub struct PetStore {
    pub pool: SqlitePool,
}
#[derive(Serialize, Deserialize, FromRow)]
pub struct PetItem {
    pub id: String,
    pub profile: String,
}
#[derive(Serialize, Deserialize, FromRow)]
pub struct Memory {
    pub id: String,
    pub pet_id: String,
    pub content: String,
    pub created_at: i64,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Card {
    pub id: String,
    pub pet_id: String,
    pub question: String,
    pub answer: String,
    pub repetitions: i64,
    pub interval_days: i64,
    pub ease: f64,
    pub due_at: i64,
}
impl Card {
    pub fn review(&mut self, quality: u8, at: i64) -> Result<(), &'static str> {
        if quality > 5 {
            return Err("評分必須介於 0 到 5");
        }
        if quality < 3 {
            self.repetitions = 0;
            self.interval_days = 1;
        } else {
            self.interval_days = match self.repetitions {
                0 => 1,
                1 => 6,
                _ => ((self.interval_days as f64) * self.ease)
                    .round()
                    .clamp(1.0, 3650.0) as i64,
            };
            self.repetitions += 1;
        }
        let delta = 5.0 - quality as f64;
        self.ease = (self.ease + 0.1 - delta * (0.08 + delta * 0.02)).clamp(1.3, 3.0);
        self.due_at = at + self.interval_days * 86400;
        Ok(())
    }
}
impl PetStore {
    pub async fn open(path: &Path) -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .foreign_keys(true);
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }
    pub async fn select_pet(&self, pet: Uuid) -> Result<(), sqlx::Error> {
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pets WHERE id=?)")
            .bind(pet.to_string())
            .fetch_one(&self.pool)
            .await?;
        if !exists {
            return Err(sqlx::Error::RowNotFound);
        }
        sqlx::query("INSERT INTO settings(key,value) VALUES ('active_pet',?) ON CONFLICT(key) DO UPDATE SET value=excluded.value").bind(pet.to_string()).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn active_pet(&self) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar("SELECT value FROM settings WHERE key='active_pet'")
            .fetch_optional(&self.pool)
            .await
    }
    pub async fn pets(&self) -> Result<Vec<PetItem>, sqlx::Error> {
        sqlx::query_as("SELECT id,profile FROM pets ORDER BY id")
            .fetch_all(&self.pool)
            .await
    }
    pub async fn save_profile(
        &self,
        id: Uuid,
        profile: &PetProfile,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        profile.validate()?;
        let text = serde_json::to_string(profile)?;
        sqlx::query("INSERT INTO pets(id,profile) VALUES (?,?) ON CONFLICT(id) DO UPDATE SET profile=excluded.profile").bind(id.to_string()).bind(text).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn memories(&self, id: Uuid) -> Result<Vec<Memory>, sqlx::Error> {
        sqlx::query_as("SELECT id,pet_id,content,created_at FROM memories WHERE pet_id=? ORDER BY created_at DESC,id LIMIT 100").bind(id.to_string()).fetch_all(&self.pool).await
    }
    pub async fn save_memory(&self, pet: Uuid, id: Uuid, content: &str) -> Result<(), sqlx::Error> {
        if content.trim().is_empty() || content.len() > 4096 {
            return Err(sqlx::Error::Protocol("記憶內容不合法".into()));
        }
        sqlx::query("INSERT INTO memories(id,pet_id,content,created_at) VALUES (?,?,?,?) ON CONFLICT(id) DO UPDATE SET content=excluded.content WHERE memories.pet_id=excluded.pet_id")
            .bind(id.to_string()).bind(pet.to_string()).bind(content).bind(now()).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn delete_memory(&self, pet: Uuid, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM memories WHERE pet_id=? AND id=?")
            .bind(pet.to_string())
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn cards(&self, pet: Uuid) -> Result<Vec<Card>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM cards WHERE pet_id=? ORDER BY due_at,id LIMIT 100")
            .bind(pet.to_string())
            .fetch_all(&self.pool)
            .await
    }
    pub async fn add_card(
        &self,
        pet: Uuid,
        question: &str,
        answer: &str,
    ) -> Result<(), sqlx::Error> {
        if [question, answer]
            .iter()
            .any(|s| s.trim().is_empty() || s.len() > 4096)
        {
            return Err(sqlx::Error::Protocol("複習題目或答案不合法".into()));
        }
        sqlx::query("INSERT INTO cards(id,pet_id,question,answer,due_at) VALUES (?,?,?,?,?)")
            .bind(Uuid::new_v4().to_string())
            .bind(pet.to_string())
            .bind(question)
            .bind(answer)
            .bind(now())
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn record_cleanup(
        &self,
        receipt: &crate::CleanupReceipt,
        event: &event_contract::ProductEvent,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("INSERT INTO cleanup_receipts(operation_id,receipt,created_at) VALUES (?,?,?)")
            .bind(receipt.operation_id.to_string())
            .bind(serde_json::to_string(receipt).map_err(|e| sqlx::Error::Protocol(e.to_string()))?)
            .bind(now())
            .execute(&mut *tx)
            .await?;
        sqlx::query("INSERT INTO pet_outbox(event_id,payload) VALUES (?,?)")
            .bind(event.event_id.to_string())
            .bind(serde_json::to_string(event).map_err(|e| sqlx::Error::Protocol(e.to_string()))?)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
    pub async fn review(&self, pet: Uuid, id: Uuid, quality: u8) -> Result<Card, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let mut card: Card = sqlx::query_as("SELECT * FROM cards WHERE pet_id=? AND id=?")
            .bind(pet.to_string())
            .bind(id.to_string())
            .fetch_one(&mut *tx)
            .await?;
        card.review(quality, now())
            .map_err(|e| sqlx::Error::Protocol(e.into()))?;
        sqlx::query("UPDATE cards SET repetitions=?,interval_days=?,ease=?,due_at=? WHERE pet_id=? AND id=?")
            .bind(card.repetitions).bind(card.interval_days).bind(card.ease).bind(card.due_at).bind(pet.to_string()).bind(id.to_string()).execute(&mut *tx).await?;
        let event = event_contract::ProductEvent {
            schema_version: 1,
            event_id: Uuid::new_v4(),
            source_app: "desktop_pet".into(),
            event_type: "learning.review.completed".into(),
        };
        sqlx::query("INSERT INTO pet_outbox(event_id,payload) VALUES (?,?)")
            .bind(event.event_id.to_string())
            .bind(serde_json::to_string(&event).map_err(|e| sqlx::Error::Protocol(e.to_string()))?)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(card)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn failed_recall_resets_schedule_and_ease_is_bounded() {
        let mut card = Card {
            id: Uuid::new_v4().to_string(),
            pet_id: Uuid::new_v4().to_string(),
            question: "Q".into(),
            answer: "A".into(),
            repetitions: 2,
            interval_days: 6,
            ease: 2.5,
            due_at: 0,
        };
        card.review(5, 100).unwrap();
        assert_eq!(card.interval_days, 15);
        for _ in 0..20 {
            card.review(0, 100).unwrap();
        }
        assert_eq!(card.repetitions, 0);
        assert_eq!(card.interval_days, 1);
        assert!(card.ease >= 1.3);
        assert!(card.review(6, 100).is_err());
    }
    #[tokio::test]
    async fn pet_memory_isolation_and_restart_persistence() {
        let path = std::env::temp_dir().join(format!("pet-state-{}.sqlite", Uuid::new_v4()));
        let store = PetStore::open(&path).await.unwrap();
        let pet = Uuid::new_v4();
        let other = Uuid::new_v4();
        let profile = PetProfile::parse(include_str!("../../../examples/pet.toml")).unwrap();
        store.save_profile(pet, &profile).await.unwrap();
        store.save_profile(other, &profile).await.unwrap();
        let id = Uuid::new_v4();
        store.save_memory(pet, id, "學習 Rust").await.unwrap();
        store.delete_memory(other, id).await.unwrap();
        assert!(store.memories(other).await.unwrap().is_empty());
        assert_eq!(store.memories(pet).await.unwrap().len(), 1);
        store.pool.close().await;
        let next = PetStore::open(&path).await.unwrap();
        assert_eq!(next.memories(pet).await.unwrap().len(), 1);
        next.delete_memory(pet, id).await.unwrap();
        assert!(next.memories(pet).await.unwrap().is_empty());
        next.pool.close().await;
        std::fs::remove_file(path).unwrap();
    }
}
