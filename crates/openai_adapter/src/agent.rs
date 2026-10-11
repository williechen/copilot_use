//! Bounded OpenAI tool loop and strict structured completion. Tool execution belongs to the host.
use crate::{ProviderError, client, configuration};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolStep {
    pub tool: String,
    pub call_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Outcome {
    pub value: Value,
    pub steps: Vec<ToolStep>,
}
pub fn answer_schema() -> Value {
    json!({"type":"object","properties":{"answer":{"type":"string"},"sources":{"type":"array","items":{"type":"string"}},"follow_up_question":{"type":"string"}},"required":["answer","sources","follow_up_question"],"additionalProperties":false})
}
async fn response(body: Value) -> Result<Value, ProviderError> {
    let (key, model) = configuration()?;
    let mut body = body;
    body["model"] = json!(model);
    body["store"] = json!(false);
    let tokens = body["max_output_tokens"].as_u64().unwrap_or(2048);
    if !(64..=2048).contains(&tokens) {
        return Err(ProviderError::Incomplete);
    }
    body["max_output_tokens"] = json!(tokens);
    body["reasoning"] = json!({"effort":"none"});
    body["include"] = json!(["reasoning.encrypted_content"]);
    if body.to_string().len() > 262144 {
        return Err(ProviderError::Incomplete);
    }
    let mut response = client()?
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(key)
        .json(&body)
        .send()
        .await
        .map_err(|_| ProviderError::Transport)?;
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
        if bytes.len() > 1048576 {
            return Err(ProviderError::Incomplete);
        }
    }
    let value: Value = serde_json::from_slice(&bytes).map_err(|_| ProviderError::Incomplete)?;
    if value["status"] != "completed" {
        return Err(ProviderError::Incomplete);
    }
    Ok(value)
}
pub async fn structured(
    input: Value,
    instructions: &str,
    schema: Value,
) -> Result<Value, ProviderError> {
    let value=response(json!({"input":input.to_string(),"instructions":instructions,"text":{"format":{"type":"json_schema","name":"copilot_result","strict":true,"schema":schema}}})).await?;
    let text = output_text(&value)?;
    serde_json::from_str(&text).map_err(|_| ProviderError::Incomplete)
}
fn output_text(value: &Value) -> Result<String, ProviderError> {
    let mut text = String::new();
    for item in value["output"]
        .as_array()
        .ok_or(ProviderError::Incomplete)?
    {
        if item["type"] == "message" {
            for content in item["content"]
                .as_array()
                .ok_or(ProviderError::Incomplete)?
            {
                if content["type"] == "refusal" {
                    return Err(ProviderError::Incomplete);
                }
                if content["type"] == "output_text" {
                    text.push_str(content["text"].as_str().ok_or(ProviderError::Incomplete)?);
                }
            }
        }
    }
    if text.trim().is_empty() || text.len() > 65536 {
        return Err(ProviderError::Incomplete);
    }
    Ok(text)
}
#[derive(Clone, Copy)]
pub struct Limits {
    pub requests: usize,
    pub tool_calls: usize,
    pub output_tokens: u16,
    pub memory: bool,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            requests: 5,
            tool_calls: 4,
            output_tokens: 2048,
            memory: false,
        }
    }
}
pub fn persona_schema() -> Value {
    let mut schema = answer_schema();
    schema["properties"]["memory"] = json!({"type":"string"});
    schema["required"]
        .as_array_mut()
        .unwrap()
        .push(json!("memory"));
    schema
}
pub async fn structured_persona(input: Value, instructions: &str) -> Result<Value, ProviderError> {
    let value=response(json!({"input":input.to_string(),"instructions":instructions,"max_output_tokens":512,"text":{"format":{"type":"json_schema","name":"copilot_persona","strict":true,"schema":persona_schema()}}})).await?;
    let answer: Value =
        serde_json::from_str(&output_text(&value)?).map_err(|_| ProviderError::Incomplete)?;
    validate_persona(&answer)?;
    Ok(answer)
}
fn validate_persona(value: &Value) -> Result<(), ProviderError> {
    if value["memory"].as_str().is_none_or(|v| v.len() > 2048) {
        return Err(ProviderError::Incomplete);
    }
    let mut answer = value.clone();
    answer
        .as_object_mut()
        .ok_or(ProviderError::Incomplete)?
        .remove("memory");
    validate_answer(&answer)
}
pub async fn run<F, Fut>(
    input: Value,
    instructions: &str,
    tool_catalog: Vec<Value>,
    execute: F,
) -> Result<Outcome, ProviderError>
where
    F: FnMut(String, Value) -> Fut,
    Fut: std::future::Future<Output = Result<Value, ProviderError>>,
{
    run_limited(
        input,
        instructions,
        tool_catalog,
        execute,
        Limits::default(),
    )
    .await
}
pub async fn run_limited<F, Fut>(
    input: Value,
    instructions: &str,
    tool_catalog: Vec<Value>,
    mut execute: F,
    limits: Limits,
) -> Result<Outcome, ProviderError>
where
    F: FnMut(String, Value) -> Fut,
    Fut: std::future::Future<Output = Result<Value, ProviderError>>,
{
    if !(2..=5).contains(&limits.requests)
        || limits.tool_calls == 0
        || limits.tool_calls > 4
        || !(64..=2048).contains(&limits.output_tokens)
    {
        return Err(ProviderError::Incomplete);
    }
    if instructions.len() > 32768 || tool_catalog.is_empty() || tool_catalog.len() > 8 {
        return Err(ProviderError::Incomplete);
    }
    let names: Vec<String> = tool_catalog
        .iter()
        .filter_map(|t| t["name"].as_str().map(str::to_owned))
        .collect();
    if names.len() != tool_catalog.len()
        || names.iter().any(|n| {
            n.is_empty()
                || n.len() > 64
                || !n
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-'))
        })
    {
        return Err(ProviderError::Incomplete);
    }
    let tools:Vec<Value>=tool_catalog.iter().map(|tool|json!({"type":"function","name":tool["name"],"description":tool["description"],"parameters":tool["inputSchema"],"strict":true})).collect();
    let mut messages = if let Some(array) = input.as_array() {
        array.clone()
    } else {
        vec![json!({"role":"user","content":input.as_str().ok_or(ProviderError::Incomplete)?})]
    };
    let mut steps = Vec::new();
    for round in 0..limits.requests {
        let choice = if round == 0 {
            json!("required")
        } else if round + 1 == limits.requests {
            json!("none")
        } else {
            json!("auto")
        };
        let schema = if limits.memory {
            persona_schema()
        } else {
            answer_schema()
        };
        let value=response(json!({"max_output_tokens":limits.output_tokens,"input":messages,"instructions":instructions,"tools":tools,"tool_choice":choice,"parallel_tool_calls":false,"text":{"format":{"type":"json_schema","name":"copilot_answer","strict":true,"schema":schema}}})).await?;
        let output = value["output"]
            .as_array()
            .ok_or(ProviderError::Incomplete)?;
        let calls: Vec<Value> = output
            .iter()
            .filter(|v| v["type"] == "function_call")
            .cloned()
            .collect();
        if calls.is_empty() {
            let structured: Value = serde_json::from_str(&output_text(&value)?)
                .map_err(|_| ProviderError::Incomplete)?;
            if limits.memory {
                validate_persona(&structured)?;
            } else {
                validate_answer(&structured)?;
            }
            return Ok(Outcome {
                value: structured,
                steps,
            });
        }
        if calls.len() > 1 || steps.len() >= limits.tool_calls {
            return Err(ProviderError::Incomplete);
        }
        messages.extend(output.clone());
        for call in calls {
            let name = call["name"].as_str().ok_or(ProviderError::Incomplete)?;
            if !names.iter().any(|n| n == name) {
                return Err(ProviderError::Incomplete);
            }
            let id = call["call_id"].as_str().ok_or(ProviderError::Incomplete)?;
            let encoded = call["arguments"]
                .as_str()
                .ok_or(ProviderError::Incomplete)?;
            if encoded.len() > 32768 {
                return Err(ProviderError::Incomplete);
            }
            let args = serde_json::from_str(encoded).map_err(|_| ProviderError::Incomplete)?;
            let result = execute(name.into(), args).await?;
            if result.to_string().len() > 32768 {
                return Err(ProviderError::Incomplete);
            }
            messages.push(
                json!({"type":"function_call_output","call_id":id,"output":result.to_string()}),
            );
            steps.push(ToolStep {
                tool: name.into(),
                call_id: id.into(),
            });
        }
    }
    Err(ProviderError::Incomplete)
}
pub fn validate_answer(value: &Value) -> Result<(), ProviderError> {
    if value.as_object().is_none_or(|v| v.len() != 3)
        || value["answer"]
            .as_str()
            .is_none_or(|v| v.trim().is_empty() || v.len() > 65536)
        || value["follow_up_question"]
            .as_str()
            .is_none_or(|v| v.len() > 4096)
        || value["sources"].as_array().is_none_or(|v| {
            v.len() > 8 || v.iter().any(|id| id.as_str().is_none_or(|v| v.len() > 64))
        })
    {
        return Err(ProviderError::Incomplete);
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partial_or_untyped_answers_are_rejected() {
        assert!(validate_answer(&json!({"answer":"partial"})).is_err());
        assert!(
            validate_answer(&json!({"answer":"", "sources":[],"follow_up_question":"Q"})).is_err()
        );
        assert!(
            validate_answer(&json!({"answer":"answer","sources":[],"follow_up_question":"Q"}))
                .is_ok()
        );
    }
}
