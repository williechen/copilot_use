//! Persisted animation and opt-in local recall prompts; no provider call or file action.
use crate::state::{PetStore, now};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Interaction {
    pub action: Action,
    pub speed: Speed,
    pub proactive: bool,
}
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    #[default]
    Idle,
    Moving,
    Happy,
    Learning,
}
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Speed {
    Slow,
    #[default]
    Normal,
    Fast,
}
#[derive(Serialize)]
pub struct LearningSummary {
    pub total: i64,
    pub due: i64,
    pub needs_practice: i64,
    pub established: i64,
}
impl PetStore {
    pub async fn roaming_enabled(&self) -> Result<bool, sqlx::Error> {
        let value: Option<String> =
            sqlx::query_scalar("SELECT value FROM settings WHERE key='window_roaming'")
                .fetch_optional(&self.pool)
                .await?;
        Ok(value.as_deref() == Some("true"))
    }
    pub async fn set_roaming_enabled(&self, enabled: bool) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO settings(key,value)VALUES('window_roaming',?)ON CONFLICT(key)DO UPDATE SET value=excluded.value")
            .bind(if enabled { "true" } else { "false" }).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn interaction(&self, pet: Uuid) -> Result<Interaction, sqlx::Error> {
        let value: Option<String> = sqlx::query_scalar("SELECT value FROM settings WHERE key=?")
            .bind(format!("interaction:{pet}"))
            .fetch_optional(&self.pool)
            .await?;
        value
            .map(|s| {
                serde_json::from_str(&s)
                    .map_err(|_| sqlx::Error::Protocol("invalid interaction settings".into()))
            })
            .transpose()
            .map(|s| s.unwrap_or_default())
    }
    pub async fn set_interaction(
        &self,
        pet: Uuid,
        preferences: &Interaction,
    ) -> Result<(), sqlx::Error> {
        if !sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM pets WHERE id=? AND archived_at IS NULL)",
        )
        .bind(pet.to_string())
        .fetch_one(&self.pool)
        .await?
        {
            return Err(sqlx::Error::RowNotFound);
        }
        sqlx::query("INSERT INTO settings(key,value)VALUES(?,?)ON CONFLICT(key)DO UPDATE SET value=excluded.value").bind(format!("interaction:{pet}")).bind(serde_json::to_string(preferences).map_err(|e|sqlx::Error::Protocol(e.to_string()))?).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn learning_summary(&self, pet: Uuid) -> Result<LearningSummary, sqlx::Error> {
        let (total,due,needs_practice,established):(i64,i64,i64,i64)=sqlx::query_as("SELECT count(*),COALESCE(sum(due_at<=?),0),COALESCE(sum(ease<2.3 OR repetitions=0),0),COALESCE(sum(repetitions>=2),0) FROM cards WHERE pet_id=?").bind(now()).bind(pet.to_string()).fetch_one(&self.pool).await?;
        Ok(LearningSummary {
            total,
            due,
            needs_practice,
            established,
        })
    }
    /// Atomic one-hour cooldown across windows/processes; only current active pet, explicitly opted in.
    pub async fn recall_prompt(&self) -> Result<Option<serde_json::Value>, sqlx::Error> {
        let Some(id) = self.active_pet().await? else {
            return Ok(None);
        };
        let pet =
            Uuid::parse_str(&id).map_err(|_| sqlx::Error::Protocol("invalid active pet".into()))?;
        if !self.interaction(pet).await?.proactive {
            return Ok(None);
        }
        let mut tx = self.pool.begin().await?;
        let card: Option<(String, String)> = sqlx::query_as(
            "SELECT id,question FROM cards WHERE pet_id=? AND due_at<=? ORDER BY due_at,id LIMIT 1",
        )
        .bind(&id)
        .bind(now())
        .fetch_optional(&mut *tx)
        .await?;
        let Some((card, question)) = card else {
            return Ok(None);
        };
        let acquired=sqlx::query("INSERT INTO settings(key,value) VALUES(?,?)ON CONFLICT(key)DO UPDATE SET value=excluded.value WHERE CAST(settings.value AS INTEGER)<=?").bind(format!("recall_after:{pet}")).bind((now()+3600).to_string()).bind(now()).execute(&mut *tx).await?;
        if acquired.rows_affected() != 1 {
            return Ok(None);
        }
        tx.commit().await?;
        Ok(Some(
            serde_json::json!({"pet":id,"card":card,"question":question.chars().take(160).collect::<String>()}),
        ))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn roaming_control_persists_both_enable_and_disable_across_reopen() {
        let directory = std::env::temp_dir().join(format!("pet-roaming-{}", Uuid::new_v4()));
        std::fs::create_dir(&directory).unwrap();
        let path = directory.join("pet.db");
        let store = PetStore::open(&path).await.unwrap();
        assert!(!store.roaming_enabled().await.unwrap());
        store.set_roaming_enabled(true).await.unwrap();
        assert!(store.roaming_enabled().await.unwrap());
        store.pool.close().await;
        let reopened = PetStore::open(&path).await.unwrap();
        assert!(reopened.roaming_enabled().await.unwrap());
        reopened.set_roaming_enabled(false).await.unwrap();
        reopened.pool.close().await;
        let reopened = PetStore::open(&path).await.unwrap();
        assert!(!reopened.roaming_enabled().await.unwrap());
        reopened.pool.close().await;
        std::fs::remove_dir_all(directory).unwrap();
    }
    #[tokio::test]
    async fn local_reminders_require_opt_in_and_are_scoped_durable_and_rate_limited() {
        let directory = std::env::temp_dir().join(format!("pet-interaction-{}", Uuid::new_v4()));
        std::fs::create_dir(&directory).unwrap();
        let path = directory.join("pet.db");
        let store = PetStore::open(&path).await.unwrap();
        let pet = Uuid::new_v4();
        let profile =
            serde_json::from_str(r#"{"名字":"測試","種類":"鳥","喜好":[],"個性":[],"興趣":[]}"#)
                .unwrap();
        store.save_profile(pet, &profile).await.unwrap();
        store.select_pet(pet).await.unwrap();
        store
            .add_card(pet, "回想所有權", "唯一擁有者")
            .await
            .unwrap();
        assert!(store.recall_prompt().await.unwrap().is_none());
        let prefs = Interaction {
            action: Action::Learning,
            speed: Speed::Fast,
            proactive: true,
        };
        store.set_interaction(pet, &prefs).await.unwrap();
        assert_eq!(store.interaction(pet).await.unwrap(), prefs);
        assert_eq!(store.learning_summary(pet).await.unwrap().needs_practice, 1);
        assert_eq!(
            store.recall_prompt().await.unwrap().unwrap()["pet"],
            pet.to_string()
        );
        assert!(store.recall_prompt().await.unwrap().is_none());
        store.pool.close().await;
        let reopened = PetStore::open(&path).await.unwrap();
        assert_eq!(reopened.interaction(pet).await.unwrap(), prefs);
        assert!(reopened.recall_prompt().await.unwrap().is_none());
        reopened.pool.close().await;
        std::fs::remove_dir_all(directory).unwrap();
        assert!(
            serde_json::from_str::<Interaction>(
                r#"{"action":"arbitrary","speed":"fast","proactive":true}"#
            )
            .is_err()
        );
    }
}
