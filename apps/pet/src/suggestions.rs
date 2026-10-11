//! Pet-scoped confirmed setting changes and guarded rollback; receipt IDs survive acknowledgement failures.
use crate::state::PetStore;
use serde_json::{Value, json};
use uuid::Uuid;
impl PetStore {
    pub async fn apply_suggestion(
        &self,
        pet: Uuid,
        id: Uuid,
        proposal: &Value,
        confirmed: bool,
    ) -> Result<Value, sqlx::Error> {
        let value = proposal["proposal"]["value"]
            .as_i64()
            .filter(|v| (1..=30).contains(v))
            .ok_or_else(|| sqlx::Error::Protocol("invalid proposal".into()))?;
        if !confirmed
            || proposal["target_app"] != "desktop_pet"
            || proposal["proposal"]["action"] != "pet_review_min_days"
            || !matches!(proposal["status"].as_str(), Some("approved" | "applied"))
        {
            return Err(sqlx::Error::Protocol("proposal not approved".into()));
        }
        let mut tx = self.pool.begin().await?;
        sqlx::query("INSERT INTO pet_policy(pet_id) VALUES(?) ON CONFLICT DO NOTHING")
            .bind(pet.to_string())
            .execute(&mut *tx)
            .await?;
        let (before, revision): (i64, i64) =
            sqlx::query_as("SELECT review_min_days,revision FROM pet_policy WHERE pet_id=?")
                .bind(pet.to_string())
                .fetch_one(&mut *tx)
                .await?;
        let old:Option<(String,bool)>=sqlx::query_as("SELECT apply_receipt,applied FROM pet_suggestion_receipts WHERE proposal_id=? AND pet_id=?").bind(id.to_string()).bind(pet.to_string()).fetch_optional(&mut *tx).await?;
        let receipt = if let Some((receipt, true)) = old {
            receipt
        } else if old.is_some() {
            return Err(sqlx::Error::Protocol("proposal already rolled back".into()));
        } else {
            if proposal["status"] != "approved" {
                return Err(sqlx::Error::Protocol("proposal already used".into()));
            }
            let receipt = Uuid::new_v4().to_string();
            sqlx::query(
                "UPDATE pet_policy SET review_min_days=?,revision=revision+1 WHERE pet_id=?",
            )
            .bind(value)
            .bind(pet.to_string())
            .execute(&mut *tx)
            .await?;
            sqlx::query("INSERT INTO pet_suggestion_receipts(proposal_id,pet_id,before_value,after_value,applied_revision,apply_receipt) VALUES(?,?,?,?,?,?)").bind(id.to_string()).bind(pet.to_string()).bind(before).bind(value).bind(revision+1).bind(&receipt).execute(&mut *tx).await?;
            receipt
        };
        tx.commit().await?;
        Ok(json!({"receipt_id":receipt,"value":value,"applied":true}))
    }
    pub async fn rollback_suggestion(
        &self,
        pet: Uuid,
        id: Uuid,
        confirmed: bool,
    ) -> Result<Value, sqlx::Error> {
        if !confirmed {
            return Err(sqlx::Error::Protocol("not confirmed".into()));
        }
        let mut tx = self.pool.begin().await?;
        let (before,after,expected,applied,old):(i64,i64,i64,bool,Option<String>)=sqlx::query_as("SELECT before_value,after_value,applied_revision,applied,rollback_receipt FROM pet_suggestion_receipts WHERE proposal_id=? AND pet_id=?").bind(id.to_string()).bind(pet.to_string()).fetch_one(&mut *tx).await?;
        let receipt = if !applied {
            old.ok_or_else(|| sqlx::Error::Protocol("missing rollback".into()))?
        } else {
            let current: (i64, i64) =
                sqlx::query_as("SELECT review_min_days,revision FROM pet_policy WHERE pet_id=?")
                    .bind(pet.to_string())
                    .fetch_one(&mut *tx)
                    .await?;
            if current != (after, expected) {
                return Err(sqlx::Error::Protocol(
                    "setting changed after proposal; rollback rejected".into(),
                ));
            }
            let receipt = Uuid::new_v4().to_string();
            sqlx::query(
                "UPDATE pet_policy SET review_min_days=?,revision=revision+1 WHERE pet_id=?",
            )
            .bind(before)
            .bind(pet.to_string())
            .execute(&mut *tx)
            .await?;
            sqlx::query("UPDATE pet_suggestion_receipts SET applied=0,rollback_receipt=? WHERE proposal_id=? AND pet_id=?").bind(&receipt).bind(id.to_string()).bind(pet.to_string()).execute(&mut *tx).await?;
            receipt
        };
        tx.commit().await?;
        Ok(json!({"receipt_id":receipt,"value":before,"rolled_back":true}))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn confirmed_application_is_idempotent_and_rollback_preserves_newer_settings() {
        let path = std::env::temp_dir().join(format!("pet-policy-{}.sqlite", Uuid::new_v4()));
        let store = PetStore::open(&path).await.unwrap();
        let pet = Uuid::new_v4();
        store
            .save_profile(
                pet,
                &crate::profile::PetProfile::parse(include_str!("../../../examples/pet.toml"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let id = Uuid::new_v4();
        let proposal = json!({"target_app":"desktop_pet","status":"approved","proposal":{"action":"pet_review_min_days","value":3}});
        assert!(
            store
                .apply_suggestion(pet, id, &proposal, false)
                .await
                .is_err()
        );
        let first = store
            .apply_suggestion(pet, id, &proposal, true)
            .await
            .unwrap();
        assert_eq!(
            first,
            store
                .apply_suggestion(pet, id, &proposal, true)
                .await
                .unwrap()
        );
        store
            .add_card(pet, "schedule probe", "answer")
            .await
            .unwrap();
        let card = store.cards(pet).await.unwrap().remove(0);
        let reviewed = store
            .review(pet, Uuid::parse_str(&card.id).unwrap(), 5)
            .await
            .unwrap();
        assert_eq!(reviewed.interval_days, 3);
        let restored = store.rollback_suggestion(pet, id, true).await.unwrap();
        assert_eq!(restored["value"], 1);
        let id = Uuid::new_v4();
        store
            .apply_suggestion(pet, id, &proposal, true)
            .await
            .unwrap();
        sqlx::query("UPDATE pet_policy SET revision=revision+1,review_min_days=9")
            .execute(&store.pool)
            .await
            .unwrap();
        assert!(store.rollback_suggestion(pet, id, true).await.is_err());
        store.pool.close().await;
        std::fs::remove_file(path).unwrap();
    }
}
