//! gRPC driver shared by SQLite Outboxes; domain crates do not depend on this crate.
use event_contract::ProductEvent;
use std::time::Duration;
use tonic::{Code, Request, Status, transport::Endpoint};
pub mod proto {
    tonic::include_proto!("copilot.analytics.v1");
}
pub const MAX_BATCH: usize = 64;
pub const MAX_MESSAGE_BYTES: usize = 65536;

impl From<&ProductEvent> for proto::ProductEvent {
    fn from(event: &ProductEvent) -> Self {
        Self {
            schema_version: event.schema_version.into(),
            event_id: event.event_id.to_string(),
            source_app: event.source_app.clone(),
            event_type: event.event_type.clone(),
        }
    }
}
impl TryFrom<proto::ProductEvent> for ProductEvent {
    type Error = Status;
    fn try_from(event: proto::ProductEvent) -> Result<Self, Self::Error> {
        let value = Self {
            schema_version: event
                .schema_version
                .try_into()
                .map_err(|_| Status::invalid_argument("invalid_event"))?,
            event_id: uuid::Uuid::parse_str(&event.event_id)
                .map_err(|_| Status::invalid_argument("invalid_event"))?,
            source_app: event.source_app,
            event_type: event.event_type,
        };
        if !value.is_valid() {
            return Err(Status::invalid_argument("invalid_event"));
        }
        Ok(value)
    }
}

/// Configuration remains an HTTP origin or the existing /api/v1/events URL.
pub async fn deliver(
    endpoint: &str,
    token: &str,
    event: &ProductEvent,
    traceparent: Option<&str>,
) -> Result<(), Status> {
    let origin = endpoint.strip_suffix("/api/v1/events").unwrap_or(endpoint);
    let mut endpoint = Endpoint::from_shared(origin.to_owned())
        .map_err(|_| Status::invalid_argument("invalid_endpoint"))?
        .connect_timeout(Duration::from_secs(3))
        .timeout(Duration::from_secs(10));
    if origin.starts_with("https://") {
        endpoint = endpoint
            .tls_config(tonic::transport::ClientTlsConfig::new().with_native_roots())
            .map_err(|_| Status::invalid_argument("invalid_tls_config"))?;
    }
    let channel = endpoint
        .connect()
        .await
        .map_err(|_| Status::unavailable("transport_unavailable"))?;
    let mut client = proto::event_ingestion_client::EventIngestionClient::new(channel)
        .max_decoding_message_size(MAX_MESSAGE_BYTES)
        .max_encoding_message_size(MAX_MESSAGE_BYTES);
    let mut request = Request::new(proto::IngestEventsRequest {
        events: vec![event.into()],
    });
    request.set_timeout(Duration::from_secs(10));
    request.metadata_mut().insert(
        "authorization",
        format!("Bearer {token}")
            .parse()
            .map_err(|_| Status::unauthenticated("invalid_credential"))?,
    );
    if let Some(parent) = traceparent
        && let Ok(value) = parent.parse()
    {
        request.metadata_mut().insert("traceparent", value);
    }
    let results = client.ingest_events(request).await?.into_inner().results;
    let [ack] = results.as_slice() else {
        return Err(Status::unavailable("invalid_ack"));
    };
    if ack.event_id != event.event_id.to_string() || ack.source_app != event.source_app {
        return Err(Status::unavailable("invalid_ack"));
    }
    match proto::Outcome::try_from(ack.outcome) {
        Ok(proto::Outcome::Stored | proto::Outcome::AlreadyExists) => Ok(()),
        Ok(proto::Outcome::InvalidEvent) => Err(Status::invalid_argument("invalid_event")),
        Ok(proto::Outcome::IdentityConflict) => Err(Status::already_exists("identity_conflict")),
        _ => Err(Status::unavailable("invalid_ack")),
    }
}
pub fn retryable(status: &Status) -> bool {
    !matches!(
        status.code(),
        Code::InvalidArgument
            | Code::Unauthenticated
            | Code::PermissionDenied
            | Code::AlreadyExists
            | Code::Unimplemented
    )
}
