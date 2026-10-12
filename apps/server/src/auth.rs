//! Password/Session/房間 ACL Adapter；Session Cookie HttpOnly、Strict，DB 只保存 token hash。
use super::{StateData, authorized};
use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{PasswordHash, SaltString},
};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use uuid::Uuid;
#[derive(Clone, Copy)]
pub(super) enum Principal {
    Admin,
    User(Uuid),
}
impl Principal {
    pub fn user_id(self) -> Option<Uuid> {
        match self {
            Self::Admin => None,
            Self::User(id) => Some(id),
        }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Credentials {
    username: String,
    password: String,
}
fn csrf(headers: &HeaderMap) -> Result<(), StatusCode> {
    if headers.get("x-copilot-csrf").and_then(|v| v.to_str().ok()) != Some("1") {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(())
}
pub(super) async fn principal(
    state: &StateData,
    headers: &HeaderMap,
    write: bool,
) -> Result<Principal, StatusCode> {
    if authorized(headers, &state.api_token) {
        return Ok(Principal::Admin);
    }
    let token = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split(';')
                .find_map(|v| v.trim().strip_prefix("copilot_session="))
        })
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if token.len() != 64 || !token.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if write {
        csrf(headers)?;
    }
    let user: Option<Uuid> = sqlx::query_scalar(
        "SELECT user_id FROM chat_sessions WHERE token_hash=?1 AND expires_at>unixepoch('subsec')",
    )
    .bind(Sha256::digest(token.as_bytes()).to_vec())
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    user.map(Principal::User).ok_or(StatusCode::UNAUTHORIZED)
}
pub(super) async fn room_allowed(
    state: &StateData,
    principal: Principal,
    room: &str,
) -> Result<(), StatusCode> {
    if matches!(principal, Principal::Admin) {
        return Ok(());
    }
    let allowed: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM chat_room_members WHERE room_id=?1 AND user_id=?2)",
    )
    .bind(room)
    .bind(principal.user_id())
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    if allowed {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
fn validate(input: &Credentials) -> Result<(), StatusCode> {
    if input.username.len() < 3
        || input.username.len() > 100
        || !input
            .username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        || input.password.len() < 12
        || input.password.len() > 128
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(())
}
// Shared SQLite counters bound sequential abuse across independent Chat processes.
async fn throttle(state: &StateData, operation: &str, username: &str) -> Result<(), StatusCode> {
    let mut tx = state
        .pool
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    sqlx::query("DELETE FROM chat_auth_throttle WHERE key IN(SELECT key FROM chat_auth_throttle WHERE expires_at<=unixepoch('subsec') LIMIT 1000)").execute(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    for (key, limit) in [
        (format!("{operation}:global"), 120),
        (
            format!("{operation}:{:x}", Sha256::digest(username.as_bytes())),
            8,
        ),
    ] {
        let count:i32=sqlx::query_scalar("INSERT INTO chat_auth_throttle(key,attempts,expires_at)VALUES(?1,1,unixepoch('subsec')+60)ON CONFLICT(key)DO UPDATE SET attempts=CASE WHEN chat_auth_throttle.expires_at<=unixepoch('subsec')THEN 1 ELSE chat_auth_throttle.attempts+1 END,expires_at=CASE WHEN chat_auth_throttle.expires_at<=unixepoch('subsec')THEN unixepoch('subsec')+60 ELSE chat_auth_throttle.expires_at END RETURNING attempts").bind(key).fetch_one(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
        if count > limit {
            tx.commit()
                .await
                .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }
    tx.commit()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(())
}
pub(super) async fn register(
    State(state): State<StateData>,
    headers: HeaderMap,
    Json(input): Json<Credentials>,
) -> Result<StatusCode, StatusCode> {
    csrf(&headers)?;
    validate(&input)?;
    let _permit = state
        .auth_slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| StatusCode::TOO_MANY_REQUESTS)?;
    throttle(&state, "register", &input.username).await?;
    let password = input.password;
    let hash = tokio::task::spawn_blocking(move || {
        let salt = SaltString::encode_b64(Uuid::new_v4().as_bytes()).map_err(|_| ())?;
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|_| ())
    })
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let result = sqlx::query("INSERT INTO chat_users(id,username,password_hash) VALUES (?1,?2,?3)")
        .bind(Uuid::new_v4())
        .bind(input.username)
        .bind(hash)
        .execute(&state.pool)
        .await;
    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e)
            if e.as_database_error()
                .is_some_and(|d| d.is_unique_violation()) =>
        {
            Err(StatusCode::CONFLICT)
        }
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}
pub(super) async fn login(
    State(state): State<StateData>,
    headers: HeaderMap,
    Json(input): Json<Credentials>,
) -> Result<impl IntoResponse, StatusCode> {
    csrf(&headers)?;
    validate(&input)?;
    let _permit = state
        .auth_slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| StatusCode::TOO_MANY_REQUESTS)?;
    throttle(&state, "login", &input.username).await?;
    let row: Option<(Uuid, String)> =
        sqlx::query_as("SELECT id,password_hash FROM chat_users WHERE username=?1")
            .bind(input.username)
            .fetch_optional(&state.pool)
            .await
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let Some((id, hash)) = row else {
        failed_login(&state.pool).await?;
        return Err(StatusCode::UNAUTHORIZED);
    };
    let verified = tokio::task::spawn_blocking(move || {
        PasswordHash::new(&hash).is_ok_and(|hash| {
            Argon2::default()
                .verify_password(input.password.as_bytes(), &hash)
                .is_ok()
        })
    })
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if !verified {
        failed_login(&state.pool).await?;
        return Err(StatusCode::UNAUTHORIZED);
    }
    let token = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    let mut tx = state
        .pool
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    sqlx::query("DELETE FROM chat_sessions WHERE token_hash IN(SELECT token_hash FROM chat_sessions WHERE expires_at<=unixepoch('subsec') LIMIT 1000)").execute(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    sqlx::query("INSERT INTO chat_sessions(token_hash,user_id,expires_at) VALUES (?1,?2,unixepoch('subsec')+86400)").bind(Sha256::digest(token.as_bytes()).to_vec()).bind(id).execute(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    auth_event(&mut tx, "login.succeeded").await?;
    tx.commit()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let secure = if std::env::var("COOKIE_SECURE").as_deref() == Ok("true") {
        "; Secure"
    } else {
        ""
    };
    Ok((
        [(
            header::SET_COOKIE,
            format!(
                "copilot_session={token}; HttpOnly; SameSite=Strict; Path=/; Max-Age=86400{secure}"
            ),
        )],
        Json(serde_json::json!({"user_id":id})),
    ))
}
pub(super) async fn logout(
    State(state): State<StateData>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, StatusCode> {
    let _ = principal(&state, &headers, true).await?;
    if let Some(token) = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split(';')
                .find_map(|v| v.trim().strip_prefix("copilot_session="))
        })
    {
        let mut tx = state
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
        let result = sqlx::query("DELETE FROM chat_sessions WHERE token_hash=?1")
            .bind(Sha256::digest(token.as_bytes()).to_vec())
            .execute(&mut *tx)
            .await
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
        if result.rows_affected() == 1 {
            auth_event(&mut tx, "logout.succeeded").await?;
        }
        tx.commit()
            .await
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    }
    Ok((
        [(
            header::SET_COOKIE,
            "copilot_session=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0",
        )],
        StatusCode::NO_CONTENT,
    ))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Invitation {
    room_id: String,
    username: String,
}
pub(super) async fn invite(
    State(state): State<StateData>,
    headers: HeaderMap,
    Json(input): Json<Invitation>,
) -> Result<StatusCode, StatusCode> {
    let actor = principal(&state, &headers, true).await?;
    if let Principal::User(id) = actor {
        let owner: Option<Uuid> = sqlx::query_scalar("SELECT owner_id FROM chat_rooms WHERE id=?1")
            .bind(&input.room_id)
            .fetch_optional(&state.pool)
            .await
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
            .flatten();
        if owner != Some(id) {
            return Err(StatusCode::FORBIDDEN);
        }
    }
    let user: Uuid = sqlx::query_scalar("SELECT id FROM chat_users WHERE username=?1")
        .bind(input.username)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .ok_or(StatusCode::NOT_FOUND)?;
    sqlx::query(
        "INSERT INTO chat_room_members(room_id,user_id) VALUES (?1,?2) ON CONFLICT DO NOTHING",
    )
    .bind(input.room_id)
    .bind(user)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn failed_login(pool: &sqlx::SqlitePool) -> Result<(), StatusCode> {
    let mut tx = pool
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    auth_event(&mut tx, "login.failed").await?;
    tx.commit()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(())
}
async fn auth_event(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    kind: &str,
) -> Result<(), StatusCode> {
    sqlx::query("INSERT INTO chat_outbox(event_id,schema_version,source_app,event_type,traceparent)VALUES(?1,1,'web_chat',?2,?3)")
        .bind(Uuid::new_v4()).bind(kind).bind(copilot_telemetry::outgoing_headers().get("traceparent"))
        .execute(&mut **tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(())
}
