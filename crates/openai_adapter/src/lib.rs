//! OpenAI HTTPS Adapter 與 bounded SSE 解碼，與 UI/DB Adapter 分離。
use serde_json::Value;
// SSE 可以在任何 UTF-8 byte 或 CRLF 分界切開；整個事件完成後才解碼。
#[derive(Default)]
pub struct Decoder {
    pending: Vec<u8>,
}
impl Decoder {
    pub fn push(&mut self, bytes: &[u8]) -> Result<Vec<Value>, ProviderError> {
        self.pending.extend_from_slice(bytes);
        if self.pending.len() > 262144 {
            return Err(ProviderError::Incomplete);
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
            let text =
                std::str::from_utf8(&self.pending[..end]).map_err(|_| ProviderError::Incomplete)?;
            let data = text
                .lines()
                .filter_map(|line| line.strip_prefix("data:").map(|v| v.trim_start()))
                .collect::<Vec<_>>()
                .join("\n");
            if !data.is_empty() && data != "[DONE]" {
                events.push(serde_json::from_str(&data).map_err(|_| ProviderError::Incomplete)?);
            }
            self.pending.drain(..end + width);
        }
        Ok(events)
    }
}

use base64::Engine;
use serde_json::json;
use std::time::Duration;
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("OpenAI 設定未完成")]
    Configuration,
    #[error("OpenAI 請求失敗")]
    Transport,
    #[error("OpenAI 回應狀態：{0}")]
    Status(u16),
    #[error("OpenAI 回應未完成或格式不合法")]
    Incomplete,
}
fn client() -> Result<reqwest::Client, ProviderError> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(360))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| ProviderError::Transport)
}
fn configuration() -> Result<(String, String), ProviderError> {
    let key = std::env::var("OPENAI_API_KEY").map_err(|_| ProviderError::Configuration)?;
    let model = std::env::var("OPENAI_CHAT_MODEL").map_err(|_| ProviderError::Configuration)?;
    if key.is_empty() || model.is_empty() {
        return Err(ProviderError::Configuration);
    }
    Ok((key, model))
}
pub async fn stream_text(
    input: Value,
    instructions: &str,
    mut delta: impl FnMut(&str) + Send,
) -> Result<String, ProviderError> {
    let (key, model) = configuration()?;
    if input.to_string().len() > 131072 || instructions.len() > 32768 {
        return Err(ProviderError::Incomplete);
    }
    let mut response=client()?.post("https://api.openai.com/v1/responses").bearer_auth(key)
  .json(&json!({"model":model,"input":input,"instructions":instructions,"stream":true,"store":false,"max_output_tokens":2048}))
  .send().await.map_err(|_|ProviderError::Transport)?;
    if !response.status().is_success() {
        return Err(ProviderError::Status(response.status().as_u16()));
    }
    let mut decoder = Decoder::default();
    let mut output = String::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| ProviderError::Transport)?
    {
        for event in decoder
            .push(&chunk)
            .map_err(|_| ProviderError::Incomplete)?
        {
            match event["type"].as_str() {
                Some("response.output_text.delta") => {
                    let text = event["delta"].as_str().ok_or(ProviderError::Incomplete)?;
                    output.push_str(text);
                    if output.len() > 65536 {
                        return Err(ProviderError::Incomplete);
                    }
                    delta(text);
                }
                Some("response.completed") if !output.trim().is_empty() => return Ok(output),
                Some("response.failed" | "response.incomplete" | "error") => {
                    return Err(ProviderError::Incomplete);
                }
                _ => {}
            }
        }
    }
    Err(ProviderError::Incomplete)
}
/// 由指定的文字模型呼叫 OpenAI 生圖工具，回傳經格式/尺寸限制的 4×4 PNG sheet。
pub async fn generate_sprite(prompt: &str) -> Result<(String, u32, u32), ProviderError> {
    let (key, model) = configuration()?;
    if prompt.len() > 32768 {
        return Err(ProviderError::Incomplete);
    }
    let mut response=client()?.post("https://api.openai.com/v1/responses").bearer_auth(key)
 .json(&json!({"model":model,"input":prompt,"store":false,"reasoning":{"effort":"none"},"tool_choice":{"type":"image_generation"},"tools":[{"type":"image_generation","model":"gpt-image-2.5-sunburst","output_format":"png","background":"transparent"}]}))
 .send().await.map_err(|_|ProviderError::Transport)?;
    if !response.status().is_success() {
        return Err(ProviderError::Status(response.status().as_u16()));
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| ProviderError::Transport)?
    {
        bytes.extend_from_slice(&chunk);
        if bytes.len() > 16777216 {
            return Err(ProviderError::Incomplete);
        }
    }
    let value: Value = serde_json::from_slice(&bytes).map_err(|_| ProviderError::Incomplete)?;
    if value["status"] != "completed" {
        return Err(ProviderError::Incomplete);
    }
    let data = value["output"]
        .as_array()
        .and_then(|items| items.iter().find(|x| x["type"] == "image_generation_call"))
        .and_then(|x| x["result"].as_str())
        .ok_or(ProviderError::Incomplete)?;
    let png = base64::engine::general_purpose::STANDARD
        .decode(data)
        .map_err(|_| ProviderError::Incomplete)?;
    if png.len() < 24 || &png[..8] != b"\x89PNG\r\n\x1a\n" {
        return Err(ProviderError::Incomplete);
    }
    let width = u32::from_be_bytes(
        png[16..20]
            .try_into()
            .map_err(|_| ProviderError::Incomplete)?,
    );
    let height = u32::from_be_bytes(
        png[20..24]
            .try_into()
            .map_err(|_| ProviderError::Incomplete)?,
    );
    if width == 0 || height == 0 {
        return Err(ProviderError::Incomplete);
    }
    Ok((data.into(), width, height))
}

pub async fn embedding(text: &str) -> Result<Vec<f32>, ProviderError> {
    if text.trim().is_empty() || text.len() > 32768 {
        return Err(ProviderError::Incomplete);
    }
    let (key, _) = configuration()?;
    let model =
        std::env::var("OPENAI_EMBEDDING_MODEL").unwrap_or_else(|_| "text-embedding-3-small".into());
    let response = client()?
        .post("https://api.openai.com/v1/embeddings")
        .bearer_auth(key)
        .json(&json!({"model":model,"input":text,"dimensions":1536,"encoding_format":"float"}))
        .send()
        .await
        .map_err(|_| ProviderError::Transport)?;
    if !response.status().is_success() {
        return Err(ProviderError::Status(response.status().as_u16()));
    }
    let value: Value = response
        .json()
        .await
        .map_err(|_| ProviderError::Incomplete)?;
    let vector: Vec<f32> = serde_json::from_value(value["data"][0]["embedding"].clone())
        .map_err(|_| ProviderError::Incomplete)?;
    if vector.len() != 1536 || vector.iter().any(|x| !x.is_finite()) {
        return Err(ProviderError::Incomplete);
    }
    Ok(vector)
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
