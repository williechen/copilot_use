//! Recoverable per-pet removal preserves other pets, memory and generated assets.
use crate::state::{PetItem, PetStore, now};
use serde_json::{Value, json};
use uuid::Uuid;
type Error = Box<dyn std::error::Error + Send + Sync>;
impl PetStore {
    pub async fn archived_pets(&self) -> Result<Vec<PetItem>, sqlx::Error> {
        sqlx::query_as("SELECT id,profile FROM pets WHERE archived_at IS NOT NULL ORDER BY id")
            .fetch_all(&self.pool)
            .await
    }
    pub async fn set_pet_archived(
        &self,
        pet: Uuid,
        archived: bool,
        confirmed: bool,
    ) -> Result<Value, Error> {
        if !confirmed || pet.is_nil() {
            return Err("pet removal/restore requires confirmation".into());
        }
        let mut transaction = self.pool.begin().await?;
        if !sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM pets WHERE id=?)")
            .bind(pet.to_string())
            .fetch_one(&mut *transaction)
            .await?
        {
            return Err("pet not found".into());
        }
        let changed = if archived {
            sqlx::query("UPDATE pets SET archived_at=? WHERE id=? AND archived_at IS NULL")
                .bind(now())
                .bind(pet.to_string())
                .execute(&mut *transaction)
                .await?
        } else {
            sqlx::query("UPDATE pets SET archived_at=NULL WHERE id=? AND archived_at IS NOT NULL")
                .bind(pet.to_string())
                .execute(&mut *transaction)
                .await?
        }
        .rows_affected()
            == 1;
        if changed {
            if archived {
                sqlx::query("DELETE FROM settings WHERE key='active_pet' AND value=?")
                    .bind(pet.to_string())
                    .execute(&mut *transaction)
                    .await?;
            }
            let event = event_contract::ProductEvent {
                schema_version: 1,
                event_id: Uuid::new_v4(),
                source_app: "desktop_pet".into(),
                event_type: if archived {
                    "pet.archived"
                } else {
                    "pet.restored"
                }
                .into(),
            };
            sqlx::query("INSERT INTO pet_outbox(event_id,payload,traceparent)VALUES(?,?,?)")
                .bind(event.event_id.to_string())
                .bind(serde_json::to_string(&event)?)
                .bind(copilot_telemetry::outgoing_headers().get("traceparent"))
                .execute(&mut *transaction)
                .await?;
        }
        transaction.commit().await?;
        Ok(json!({"pet_id":pet,"archived":archived,"changed":changed}))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn removal_is_confirmed_recoverable_idempotent_and_keeps_other_pet_and_memory() {
        let directory = std::env::temp_dir().join(format!("pet-registry-{}", Uuid::new_v4()));
        std::fs::create_dir(&directory).unwrap();
        let store = PetStore::open(&directory.join("pet.sqlite")).await.unwrap();
        let profile = serde_json::from_value(
            json!({"名字":"原始","種類":"鳥","喜好":[],"個性":[],"興趣":[]}),
        )
        .unwrap();
        let pet = Uuid::new_v4();
        let other = Uuid::new_v4();
        store.save_profile(pet, &profile).await.unwrap();
        store.save_profile(other, &profile).await.unwrap();
        store.select_pet(pet).await.unwrap();
        store
            .save_memory(pet, Uuid::new_v4(), "retained")
            .await
            .unwrap();
        assert!(store.set_pet_archived(pet, true, false).await.is_err());
        assert!(store.set_pet_archived(pet, true, true).await.unwrap()["changed"] == true);
        assert!(store.set_pet_archived(pet, true, true).await.unwrap()["changed"] == false);
        assert!(store.active_pet().await.unwrap().is_none());
        assert_eq!(store.pets().await.unwrap()[0].id, other.to_string());
        assert_eq!(store.archived_pets().await.unwrap().len(), 1);
        assert!(store.select_pet(pet).await.is_err());
        assert!(store.save_profile(pet, &profile).await.is_err());
        assert!(store.set_pet_archived(pet, false, true).await.unwrap()["changed"] == true);
        assert_eq!(store.pets().await.unwrap().len(), 2);
        assert_eq!(store.memories(pet).await.unwrap()[0].content, "retained");
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM pet_outbox")
                .fetch_one(&store.pool)
                .await
                .unwrap(),
            2
        );
        store.pool.close().await;
        std::fs::remove_dir_all(directory).unwrap();
    }
}
