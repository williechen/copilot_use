//! OpenAI Responses SSE Adapter；只有完整成功的回答會持久化與產生 Outbox。
use super::{StateData, authorized, realtime::RoomEvent};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{
        IntoResponse, Response,
        sse::{Event, KeepAlive, Sse},
    },
};
use day9_chat::ChatWriter;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{convert::Infallible, time::Duration};
use tokio_stream::wrappers::ReceiverStream;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReplyRequest {
    room_id: String,
}

// SSE 可以在任何 UTF-8 byte 或 CRLF 分界切開；整個事件完成後才解碼。
#[derive(Default)]
struct Decoder {
    pending: Vec<u8>,
}
impl Decoder {
    fn push(&mut self, bytes: &[u8]) -> Result<Vec<Value>, ()> {
        self.pending.extend_from_slice(bytes);
        if self.pending.len() > 262144 {
            return Err(());
        }
        let mut events = Vec::new();
        loop {
            let lf = self
                .pending
                .windows(2)
                .position(|w| w == b"\n\n")
                .map(|p| (p, 2));
            let crlf = self
                .pending
                .windows(4)
                .position(|w| w == b"\r\n\r\n")
                .map(|p| (p, 4));
            let boundary = match (lf, crlf) {
                (Some(a), Some(b)) => Some(if a.0 < b.0 { a } else { b }),
                (a, b) => a.or(b),
            };
            let Some((end, width)) = boundary else {
                break;
            };
            let text = std::str::from_utf8(&self.pending[..end]).map_err(|_| ())?;
            let data = text
                .lines()
                .filter_map(|line| line.strip_prefix("data:").map(|v| v.trim_start()))
                .collect::<Vec<_>>()
                .join("\n");
            if !data.is_empty() && data != "[DONE]" {
                events.push(serde_json::from_str(&data).map_err(|_| ())?);
            }
            self.pending.drain(..end + width);
        }
        Ok(events)
    }
}

pub(super) async fn reply(
    State(state): State<StateData>,
    headers: HeaderMap,
    Json(input): Json<ReplyRequest>,
) -> Result<Response, StatusCode> {
    if !authorized(&headers, &state.api_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if input.room_id.is_empty() || input.room_id.len() > 40 {
        return Err(StatusCode::BAD_REQUEST);
    }
    let key = std::env::var("OPENAI_API_KEY").map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let model = std::env::var("OPENAI_CHAT_MODEL").map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    if key.is_empty() || model.is_empty() {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }
    let permit = state
        .ai_slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| StatusCode::TOO_MANY_REQUESTS)?;
    let rows:Vec<(String,String)>=sqlx::query_as("SELECT author_kind,content FROM (SELECT author_kind,content,created_at,id FROM chat_messages WHERE room_id=$1 ORDER BY created_at DESC,id DESC LIMIT 30) h ORDER BY created_at,id")
        .bind(&input.room_id).fetch_all(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
    if rows.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let messages: Vec<Value> = rows
        .into_iter()
        .map(|(kind, text)| json!({"role":if kind=="ai" {"assistant"}else{"user"},"content":text}))
        .collect();
    if serde_json::to_vec(&messages)
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .len()
        > 131072
    {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(90))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let response=client.post("https://api.openai.com/v1/responses").bearer_auth(key)
        .json(&json!({"model":model,"input":messages,"instructions":"請以繁體中文回答；若資訊不足請直接說明。","store":false,"stream":true,"max_output_tokens":1024}))
        .send().await.map_err(|_|StatusCode::BAD_GATEWAY)?;
    if !response.status().is_success() {
        return Err(StatusCode::BAD_GATEWAY);
    }
    let (sender, receiver) = tokio::sync::mpsc::channel(32);
    tokio::spawn(async move {
        let _permit = permit;
        if produce(response, &sender, &state, &input.room_id, &model)
            .await
            .is_err()
        {
            let _ = sender
                .send(Ok(Event::default()
                    .event("error")
                    .data("回答未完成，未保存")))
                .await;
        }
    });
    Ok(Sse::new(ReceiverStream::new(receiver))
        .keep_alive(KeepAlive::default())
        .into_response())
}
async fn produce(
    mut response: reqwest::Response,
    sender: &tokio::sync::mpsc::Sender<Result<Event, Infallible>>,
    state: &StateData,
    room_id: &str,
    model: &str,
) -> Result<(), ()> {
    let mut decoder = Decoder::default();
    let mut output = String::new();
    loop {
        let chunk = tokio::select! { _=sender.closed()=>return Err(()), value=response.chunk()=>value.map_err(|_|())? };
        let Some(chunk) = chunk else {
            return Err(());
        };
        for event in decoder.push(&chunk)? {
            match event.get("type").and_then(Value::as_str) {
                Some("response.output_text.delta") => {
                    let delta = event.get("delta").and_then(Value::as_str).ok_or(())?;
                    output.push_str(delta);
                    if output.len() > 65536 {
                        return Err(());
                    }
                    sender
                        .send(Ok(Event::default()
                            .event("delta")
                            .json_data(json!({"text":delta}))
                            .map_err(|_| ())?))
                        .await
                        .map_err(|_| ())?;
                }
                Some("response.completed") => {
                    if output.trim().is_empty() {
                        return Err(());
                    }
                    let id = ChatWriter::new(state.pool.clone())
                        .persist_as(room_id, &output, "ai", Some(model))
                        .await
                        .map_err(|_| ())?;
                    let _ = state.events.send(RoomEvent {
                        room_id: room_id.into(),
                        payload: json!({"type":"message","id":id,"content":output}),
                    });
                    sender
                        .send(Ok(Event::default()
                            .event("done")
                            .json_data(json!({"id":id}))
                            .map_err(|_| ())?))
                        .await
                        .map_err(|_| ())?;
                    return Ok(());
                }
                Some("response.failed" | "response.incomplete" | "error") => return Err(()),
                _ => {}
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn utf8_and_crlf_can_split_at_every_byte() {
        let text = "event: response.output_text.delta\r\ndata: {\"type\":\"response.output_text.delta\",\"delta\":\"你好\"}\r\n\r\n";
        let mut parser = Decoder::default();
        let mut values = Vec::new();
        for byte in text.as_bytes() {
            values.extend(parser.push(&[*byte]).unwrap());
        }
        assert_eq!(values.len(), 1);
        assert_eq!(values[0]["delta"], "你好");
    }
    #[test]
    fn incomplete_and_oversized_frames_do_not_succeed() {
        let mut parser = Decoder::default();
        assert!(parser.push(b"data: {").unwrap().is_empty());
        assert!(parser.push(&vec![b'x'; 262145]).is_err());
    }
}
