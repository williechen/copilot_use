//! 透過專用唯讀憑證查詢 Chat 私有操作稽核；不混入聚合事件或 AI 資料。
use super::{AnalyticsState, authorized};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use event_contract::audit::{AuditQuery, AuditRecord};
#[derive(Clone)]
pub struct ChatAuditSource {
    client: reqwest::Client,
    endpoint: reqwest::Url,
    token: String,
}
impl ChatAuditSource {
    pub fn new(
        endpoint: &str,
        token: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let url = reqwest::Url::parse(endpoint)?;
        if url.scheme() != "http"
            || !matches!(
                url.host_str(),
                Some("127.0.0.1" | "127.0.0.2" | "localhost")
            )
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || url.path() != "/api/v1/audit/activity"
            || token.len() < 16
        {
            return Err("Chat 稽核來源須為本機專用查詢入口與至少16字元獨立憑證".into());
        }
        Ok(Self {
            client: reqwest::Client::builder()
                .no_proxy()
                .redirect(reqwest::redirect::Policy::none())
                .timeout(std::time::Duration::from_secs(3))
                .build()?,
            endpoint: url,
            token: token.into(),
        })
    }
    pub fn from_env() -> Result<Option<Self>, Box<dyn std::error::Error + Send + Sync>> {
        match (
            std::env::var("CHAT_AUDIT_ENDPOINT"),
            std::env::var("CHAT_AUDIT_TOKEN"),
        ) {
            (Ok(endpoint), Ok(token)) => Ok(Some(Self::new(&endpoint, &token)?)),
            (Err(_), Err(_)) => Ok(None),
            _ => Err("CHAT_AUDIT_ENDPOINT 與 CHAT_AUDIT_TOKEN 必須一起設定".into()),
        }
    }
}
pub(super) async fn query(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Json(input): Json<AuditQuery>,
) -> Result<Json<Vec<AuditRecord>>, StatusCode> {
    if !authorized(&headers, &state.admin_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if !input.is_valid() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let source = state.audit_source.ok_or(StatusCode::SERVICE_UNAVAILABLE)?;
    let mut response = source
        .client
        .post(source.endpoint)
        .bearer_auth(source.token)
        .json(&input)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    if response.status() != reqwest::StatusCode::OK {
        return Err(StatusCode::BAD_GATEWAY);
    }
    let mut body = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?
    {
        if body.len() + chunk.len() > 131072 {
            return Err(StatusCode::BAD_GATEWAY);
        }
        body.extend_from_slice(&chunk);
    }
    let rows: Vec<AuditRecord> =
        serde_json::from_slice(&body).map_err(|_| StatusCode::BAD_GATEWAY)?;
    if rows.len() > 100 {
        return Err(StatusCode::BAD_GATEWAY);
    }
    Ok(Json(rows))
}
