//! Browser Adapter：同來源 Session HTTP、WS 事件與 SSE。
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use serde_json::{Value, json};
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{MessageEvent, WebSocket};
pub async fn api(path: &str, method: &str, body: Option<Value>) -> Result<Value, String> {
    let builder = gloo_net::http::RequestBuilder::new(path)
        .method(method.try_into().map_err(|_| "方法不合法")?)
        .header("X-Copilot-CSRF", "1")
        .header("Content-Type", "application/json");
    let response = if let Some(body) = body {
        builder
            .body(body.to_string())
            .map_err(|_| "無法建立請求")?
            .send()
            .await
    } else {
        builder.send().await
    }
    .map_err(|_| "連線失敗，請重試")?;
    if !response.ok() {
        return Err(format!("操作失敗：{}", response.status()));
    }
    if response.status() == 204
        || response.status() == 201 && path == "/api/v1/rooms"
        || response.status() == 201 && path == "/api/v1/auth/register"
    {
        return Ok(Value::Null);
    }
    response.json().await.map_err(|_| "回覆格式不合法".into())
}
struct SocketGuard {
    socket: WebSocket,
    _open: Closure<dyn FnMut()>,
    _message: Closure<dyn FnMut(MessageEvent)>,
}
impl Drop for SocketGuard {
    fn drop(&mut self) {
        self.socket.set_onopen(None);
        self.socket.set_onmessage(None);
        let _ = self.socket.close();
    }
}
pub fn observe_room(
    room: RwSignal<String>,
    messages: RwSignal<Vec<Value>>,
    error: RwSignal<String>,
) {
    Effect::new(move |_| {
        let id = room.get();
        if id.is_empty() {
            return;
        }
        let location = web_sys::window().unwrap().location();
        let host = location.host().unwrap_or_default();
        let scheme = if location.protocol().as_deref() == Ok("https:") {
            "wss"
        } else {
            "ws"
        };
        let Ok(socket) = WebSocket::new(&format!("{scheme}://{host}/api/v1/ws/{id}")) else {
            error.set("無法開啟即時連線".into());
            return;
        };
        let s = socket.clone();
        let open = Closure::<dyn FnMut()>::new(move || {
            let _ = s.send_with_str(&json!({"token":""}).to_string());
        });
        socket.set_onopen(Some(open.as_ref().unchecked_ref()));
        let current = id.clone();
        let message = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
            if let Some(text) = event.data().as_string()
                && let Ok(value) = serde_json::from_str::<Value>(&text)
            {
                match value["type"].as_str() {
                    Some("message") => messages.update(|items| {
                        if !items.iter().any(|m| m["id"] == value["id"]) {
                            items.push(value.clone());
                        }
                    }),
                    Some("ready" | "resync_required") => {
                        let path = format!("/api/v1/messages?room_id={current}");
                        super::spawn(async move {
                            match api(&path, "GET", None).await {
                                Ok(v) => messages.set(v.as_array().cloned().unwrap_or_default()),
                                Err(e) => error.set(e),
                            }
                        });
                    }
                    Some("error") => error.set("即時連線驗證失敗".into()),
                    _ => {}
                }
            }
        });
        socket.set_onmessage(Some(message.as_ref().unchecked_ref()));
        let guard = SendWrapper::new(SocketGuard {
            socket,
            _open: open,
            _message: message,
        });
        on_cleanup(move || drop(guard));
    });
}
pub async fn ai_stream(body: Value, output: RwSignal<String>) -> Result<(), String> {
    // Fetch/SSE Reader 透過瀏覽器標準 API 保持 incremental rendering。
    let options = js_sys::Object::new();
    js_sys::Reflect::set(&options, &"method".into(), &"POST".into()).map_err(|_| "請求錯誤")?;
    let headers =
        js_sys::JSON::parse(r#"{"Content-Type":"application/json","X-Copilot-CSRF":"1"}"#)
            .map_err(|_| "請求錯誤")?;
    js_sys::Reflect::set(&options, &"headers".into(), &headers).map_err(|_| "請求錯誤")?;
    js_sys::Reflect::set(&options, &"body".into(), &body.to_string().into())
        .map_err(|_| "請求錯誤")?;
    let fetch: js_sys::Function =
        js_sys::Reflect::get(&web_sys::window().unwrap(), &"fetch".into())
            .map_err(|_| "無法建立連線")?
            .unchecked_into();
    let promise: js_sys::Promise = fetch
        .call2(
            &web_sys::window().unwrap(),
            &"/api/v1/ai/reply".into(),
            &options,
        )
        .map_err(|_| "無法建立連線")?
        .unchecked_into();
    let response = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|_| "AI 連線失敗")?;
    if !js_sys::Reflect::get(&response, &"ok".into())
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return Err("AI 尚未設定或请求失敗".into());
    }
    let body = js_sys::Reflect::get(&response, &"body".into()).map_err(|_| "無法讀取串流")?;
    let reader: js_sys::Function = js_sys::Reflect::get(&body, &"getReader".into())
        .map_err(|_| "無法讀取串流")?
        .unchecked_into();
    let reader = reader.call0(&body).map_err(|_| "無法讀取串流")?;
    let read: js_sys::Function = js_sys::Reflect::get(&reader, &"read".into())
        .map_err(|_| "無法讀取串流")?
        .unchecked_into();
    let mut pending = Vec::<u8>::new();
    let mut completed = false;
    loop {
        let result = wasm_bindgen_futures::JsFuture::from(
            read.call0(&reader)
                .map_err(|_| "串流失敗")?
                .unchecked_into::<js_sys::Promise>(),
        )
        .await
        .map_err(|_| "串流失敗")?;
        if js_sys::Reflect::get(&result, &"done".into())
            .ok()
            .and_then(|v| v.as_bool())
            == Some(true)
        {
            break;
        }
        let bytes = js_sys::Uint8Array::new(
            &js_sys::Reflect::get(&result, &"value".into()).map_err(|_| "串流失敗")?,
        )
        .to_vec();
        pending.extend(bytes);
        if pending.len() > 262144 {
            return Err("串流事件過大".into());
        }
        while let Some(end) = pending.windows(2).position(|w| w == b"\n\n") {
            let frame = std::str::from_utf8(&pending[..end]).map_err(|_| "串流格式錯誤")?;
            let event = frame
                .lines()
                .find_map(|l| l.strip_prefix("event:").map(str::trim));
            let data = frame
                .lines()
                .filter_map(|l| l.strip_prefix("data:").map(str::trim))
                .collect::<Vec<_>>()
                .join("\n");
            match event {
                Some("delta") => {
                    let value: Value = serde_json::from_str(&data).map_err(|_| "串流格式錯誤")?;
                    output.update(|text| text.push_str(value["text"].as_str().unwrap_or("")));
                }
                Some("done") => completed = true,
                Some("error") => return Err(data),
                _ => {}
            }
            pending.drain(..end + 2);
        }
    }
    if !completed {
        return Err("AI 回答未完成，未保存".into());
    }
    Ok(())
}
