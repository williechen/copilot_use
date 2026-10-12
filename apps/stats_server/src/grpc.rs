//! gRPC only adapts authentication/Protobuf; application and durable dedup remain shared.
use crate::{
    AnalyticsState, EventStore,
    application::{Analytics, RepositoryError},
};
use analytics_transport::{
    MAX_BATCH, MAX_MESSAGE_BYTES,
    proto::{
        self,
        event_ingestion_server::{EventIngestion, EventIngestionServer},
    },
};
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct Ingestion(pub AnalyticsState);
#[tonic::async_trait]
impl EventIngestion for Ingestion {
    async fn ingest_events(
        &self,
        request: Request<proto::IngestEventsRequest>,
    ) -> Result<Response<proto::IngestEventsResponse>, Status> {
        let credential = request
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));
        let events = &request.get_ref().events;
        if events.is_empty() || events.len() > MAX_BATCH {
            return Err(Status::invalid_argument("batch_size"));
        }
        // Authenticate the entire batch before any write; source credentials cannot impersonate.
        for event in events {
            let expected = match event.source_app.as_str() {
                "web_chat" => &self.0.chat_token,
                "desktop_pet" => &self.0.pet_token,
                _ => return Err(Status::invalid_argument("invalid_source")),
            };
            if expected.is_empty() || credential != Some(expected.as_ref()) {
                return Err(Status::unauthenticated("source_credential"));
            }
        }
        let app = Analytics::new(EventStore::new(self.0.pool.clone()));
        let mut results = Vec::with_capacity(events.len());
        for wire in request.into_inner().events {
            let event_id = wire.event_id.clone();
            let source_app = wire.source_app.clone();
            let outcome = match event_contract::ProductEvent::try_from(wire) {
                Err(_) => proto::Outcome::InvalidEvent,
                Ok(event) => match app.receive(&event).await {
                    Ok(true) => proto::Outcome::Stored,
                    Ok(false) => proto::Outcome::AlreadyExists,
                    Err(RepositoryError::IdentityConflict) => proto::Outcome::IdentityConflict,
                    Err(RepositoryError::InvalidEvent) => proto::Outcome::InvalidEvent,
                    // Earlier successful events may be committed; retrying the batch is idempotent.
                    Err(RepositoryError::Unavailable) => {
                        return Err(Status::unavailable("storage_unavailable"));
                    }
                },
            };
            results.push(proto::EventResult {
                event_id,
                source_app,
                outcome: outcome.into(),
            });
        }
        Ok(Response::new(proto::IngestEventsResponse { results }))
    }
}
pub fn routes(state: AnalyticsState) -> axum::Router {
    tonic::service::Routes::new(
        EventIngestionServer::new(Ingestion(state))
            .max_decoding_message_size(MAX_MESSAGE_BYTES)
            .max_encoding_message_size(MAX_MESSAGE_BYTES),
    )
    .into_axum_router()
}

#[cfg(test)]
mod tests;
