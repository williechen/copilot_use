//! SQLite atomic leases and compare-and-swap acknowledgements; immutable source events.
use crate::state::{PetStore, now};
use sqlx::FromRow;
use uuid::Uuid;
#[derive(Debug, FromRow)]
pub struct Lease {
    pub event_id: String,
    pub payload: String,
    pub attempts: i64,
    pub traceparent: Option<String>,
    pub lease_token: String,
}
impl PetStore {
    pub async fn claim_event(&self) -> Result<Option<Lease>, sqlx::Error> {
        sqlx::query_as("UPDATE pet_outbox SET lease_token=?,lease_until=?,attempts=attempts+1 WHERE event_id=(SELECT event_id FROM pet_outbox WHERE delivered=0 AND next_attempt_at<=? AND attempts<10 AND (lease_token IS NULL OR lease_until<=?) ORDER BY rowid LIMIT 1) RETURNING event_id,payload,attempts,traceparent,lease_token")
   .bind(Uuid::new_v4().to_string()).bind(now()+30).bind(now()).bind(now()).fetch_optional(&self.pool).await
    }
    pub async fn ack_event(&self, lease: &Lease) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query("UPDATE pet_outbox SET delivered=1,lease_token=NULL,lease_until=NULL,last_error=NULL WHERE event_id=? AND lease_token=? AND delivered=0 AND lease_until>?")
   .bind(&lease.event_id).bind(&lease.lease_token).bind(now()).execute(&self.pool).await?.rows_affected()==1)
    }
    pub async fn fail_event(
        &self,
        lease: &Lease,
        code: &str,
        permanent: bool,
    ) -> Result<bool, sqlx::Error> {
        let code = match code {
            "invalid_event" => "invalid_event",
            "http_rejected" => "http_rejected",
            "grpc_rejected" => "grpc_rejected",
            "grpc_unavailable" => "grpc_unavailable",
            _ => "transport_or_retryable_http",
        };
        Ok(sqlx::query("UPDATE pet_outbox SET attempts=?,next_attempt_at=?,lease_token=NULL,lease_until=NULL,last_error=? WHERE event_id=? AND lease_token=? AND delivered=0 AND lease_until>?")
   .bind(if permanent {10} else {lease.attempts}).bind(now()+(1_i64<<lease.attempts.min(8))).bind(code).bind(&lease.event_id).bind(&lease.lease_token).bind(now()).execute(&self.pool).await?.rows_affected()==1)
    }
    pub async fn outbox_status(&self) -> Result<serde_json::Value, sqlx::Error> {
        let (delivered,pending,leased,blocked):(i64,i64,i64,i64)=sqlx::query_as("SELECT count(*) FILTER(WHERE delivered=1),count(*) FILTER(WHERE delivered=0 AND attempts<10 AND (lease_token IS NULL OR lease_until<=?)),count(*) FILTER(WHERE delivered=0 AND lease_token IS NOT NULL AND lease_until>?),count(*) FILTER(WHERE delivered=0 AND attempts>=10 AND (lease_token IS NULL OR lease_until<=?)) FROM pet_outbox")
            .bind(now()).bind(now()).bind(now()).fetch_one(&self.pool).await?;
        Ok(
            serde_json::json!({"delivered":delivered,"pending":pending,"leased":leased,"blocked":blocked}),
        )
    }
    pub async fn requeue_blocked_events(&self) -> Result<u64, sqlx::Error> {
        Ok(sqlx::query("UPDATE pet_outbox SET attempts=0,next_attempt_at=0,lease_token=NULL,lease_until=NULL,last_error=NULL WHERE delivered=0 AND attempts>=10 AND (lease_token IS NULL OR lease_until<=?)")
   .bind(now()).execute(&self.pool).await?.rows_affected())
    }
    pub async fn dispatch_once(
        &self,
        endpoint: &str,
        token: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let Some(lease) = self.claim_event().await? else {
            return Ok(false);
        };
        let event: event_contract::ProductEvent =
            match serde_json::from_str::<event_contract::ProductEvent>(&lease.payload) {
                Ok(event)
                    if event.is_valid()
                        && event.source_app == "desktop_pet"
                        && event.event_id.to_string() == lease.event_id =>
                {
                    event
                }
                _ => {
                    self.fail_event(&lease, "invalid_event", true).await?;
                    return Ok(true);
                }
            };
        match analytics_transport::deliver(endpoint, token, &event, lease.traceparent.as_deref())
            .await
        {
            Ok(()) => {
                self.ack_event(&lease).await?;
            }
            Err(status) => {
                let permanent = !analytics_transport::retryable(&status);
                self.fail_event(
                    &lease,
                    if permanent {
                        "grpc_rejected"
                    } else {
                        "grpc_unavailable"
                    },
                    permanent,
                )
                .await?;
            }
        }
        Ok(true)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn expired_lease_is_reclaimed_and_stale_worker_cannot_mutate() {
        let path = std::env::temp_dir().join(format!("pet-lease-{}.sqlite", Uuid::new_v4()));
        let store = PetStore::open(&path).await.unwrap();
        let receipt = crate::CleanupReceipt {
            operation_id: Uuid::new_v4(),
            quarantined_file: "test.item".into(),
            size_bytes: 1,
        };
        store
            .record_cleanup(&receipt, &receipt.event())
            .await
            .unwrap();
        let old = store.claim_event().await.unwrap().unwrap();
        assert!(store.claim_event().await.unwrap().is_none());
        sqlx::query("UPDATE pet_outbox SET lease_until=0")
            .execute(&store.pool)
            .await
            .unwrap();
        let next = store.claim_event().await.unwrap().unwrap();
        assert_eq!(old.event_id, next.event_id);
        assert_ne!(old.lease_token, next.lease_token);
        assert!(!store.ack_event(&old).await.unwrap());
        assert!(!store.fail_event(&old, "http_rejected", true).await.unwrap());
        assert!(
            store
                .fail_event(&next, "http_rejected", true)
                .await
                .unwrap()
        );
        assert!(store.claim_event().await.unwrap().is_none());
        assert_eq!(store.requeue_blocked_events().await.unwrap(), 1);
        let retried = store.claim_event().await.unwrap().unwrap();
        assert!(store.ack_event(&retried).await.unwrap());
        assert!(store.claim_event().await.unwrap().is_none());
        store.pool.close().await;
        std::fs::remove_file(path).unwrap();
    }
}
