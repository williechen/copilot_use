//! Browser Adapter：同來源 Session HTTP、WS 事件與 SSE。
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use serde_json::{Value, json};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{CloseEvent, MessageEvent, WebSocket};
pub async fn api(path: &str, method: &str, body: Option<Value>) -> Result<Value, String> {
    request(path, method, body, None).await
}
pub async fn api_with_token(
    token: &str,
    path: &str,
    method: &str,
    body: Option<Value>,
) -> Result<Value, String> {
    request(path, method, body, Some(token)).await
}
async fn request(
    path: &str,
    method: &str,
    body: Option<Value>,
    token: Option<&str>,
) -> Result<Value, String> {
    let mut builder = gloo_net::http::RequestBuilder::new(path)
        .method(method.try_into().map_err(|_| "方法不合法")?)
        .header("X-Copilot-CSRF", "1")
        .header("Content-Type", "application/json");
    if let Some(token) = token {
        builder = builder.header("Authorization", &format!("Bearer {token}"));
    }
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
        return Err(super::api_failure(path, response.status()));
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
    active: Rc<Cell<bool>>,
    socket: WebSocket,
    _open: Closure<dyn FnMut()>,
    _message: Closure<dyn FnMut(MessageEvent)>,
    _close: Closure<dyn FnMut(CloseEvent)>,
    retry: Rc<RefCell<Option<gloo_timers::callback::Timeout>>>,
    _presence_poll: gloo_timers::callback::Interval,
    typing_timer: Rc<RefCell<Option<gloo_timers::callback::Timeout>>>,
}
impl Drop for SocketGuard {
    fn drop(&mut self) {
        self.active.set(false);
        self.socket.set_onopen(None);
        self.socket.set_onmessage(None);
        self.socket.set_onclose(None);
        self.retry.borrow_mut().take();
        self.typing_timer.borrow_mut().take();
        let _ = self.socket.close();
    }
}
pub fn observe_room(
    room: RwSignal<String>,
    messages: RwSignal<Vec<Value>>,
    error: RwSignal<String>,
    presence: RwSignal<Vec<Value>>,
    typing: RwSignal<String>,
    typing_signal: RwSignal<u32>,
) {
    let outgoing = Rc::new(RefCell::new(None::<WebSocket>));
    let sender = outgoing.clone();
    let last_typing = Rc::new(Cell::new(0_f64));
    Effect::new(move |_| {
        if typing_signal.get() == 0 {
            return;
        }
        let now = js_sys::Date::now();
        if now - last_typing.get() < 500.0 {
            return;
        }
        last_typing.set(now);
        if let Some(socket) = sender.borrow().as_ref()
            && socket.ready_state() == WebSocket::OPEN
        {
            let _ = socket.send_with_str(r#"{"type":"typing"}"#);
        }
    });
    let generation = RwSignal::new(0_u32);
    let attempts = Rc::new(Cell::new(0_u32));
    Effect::new(move |_| {
        let _ = generation.get();
        let id = room.get();
        presence.set(Vec::new());
        typing.set(String::new());
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
        *outgoing.borrow_mut() = Some(socket.clone());
        let s = socket.clone();
        let open = Closure::<dyn FnMut()>::new(move || {
            let _ = s.send_with_str(&json!({"token":""}).to_string());
        });
        socket.set_onopen(Some(open.as_ref().unchecked_ref()));
        let current = id.clone();
        let active = Rc::new(Cell::new(true));
        let listening = active.clone();
        let failures = attempts.clone();
        let typing_timer = Rc::new(RefCell::new(None));
        let timer = typing_timer.clone();
        let own = Rc::new(RefCell::new(String::new()));
        let message = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
            if !listening.get() || room.get_untracked() != current {
                return;
            }
            if let Some(text) = event.data().as_string()
                && let Ok(value) = serde_json::from_str::<Value>(&text)
            {
                match value["type"].as_str() {
                    Some("typing") => {
                        if value["connection_id"].as_str() != Some(own.borrow().as_str()) {
                            typing.set("有人正在輸入…".into());
                            let listening = listening.clone();
                            *timer.borrow_mut() =
                                Some(gloo_timers::callback::Timeout::new(2000, move || {
                                    if listening.get() {
                                        typing.set(String::new());
                                    }
                                }));
                        }
                    }
                    Some("presence") => {
                        presence.update(|items| {
                            let id = value["connection_id"].clone();
                            if value["online"] == true {
                                if !items.contains(&id) {
                                    items.push(id);
                                }
                            } else {
                                items.retain(|v| v != &id);
                            }
                        });
                    }
                    Some("message") => messages.update(|items| {
                        if !items.iter().any(|m| m["id"] == value["id"]) {
                            items.push(value.clone());
                            if items.len() > 200 {
                                items.remove(0);
                            }
                        }
                    }),
                    Some("ready" | "resync_required") => {
                        if value["type"] == "ready" {
                            *own.borrow_mut() =
                                value["connection_id"].as_str().unwrap_or("").into();
                            presence.set(value["presence"].as_array().cloned().unwrap_or_default());
                        }
                        failures.set(0);
                        if ["即時連線中斷，正在重新連線", "連線失敗，請重試"]
                            .contains(&error.get_untracked().as_str())
                        {
                            error.set(String::new());
                        }
                        let current = current.clone();
                        let listening = listening.clone();
                        let path = format!("/api/v1/messages?room_id={current}");
                        super::spawn(async move {
                            match api(&path, "GET", None).await {
                                Ok(v) => {
                                    if listening.get() && room.get_untracked() == current {
                                        messages.set(v.as_array().cloned().unwrap_or_default());
                                    }
                                }
                                Err(e) => {
                                    if listening.get() {
                                        error.set(e);
                                    }
                                }
                            }
                        });
                    }
                    Some("error") => error.set(
                        if value["code"] == "connection_quota" {
                            "連線數已達上限，請關閉其他連線後重試"
                        } else {
                            "即時連線驗證失敗"
                        }
                        .into(),
                    ),
                    _ => {}
                }
            }
        });
        socket.set_onmessage(Some(message.as_ref().unchecked_ref()));
        let retry = Rc::new(RefCell::new(None));
        let pending = retry.clone();
        let failures = attempts.clone();
        let close = Closure::<dyn FnMut(CloseEvent)>::new(move |event: CloseEvent| {
            if event.code() == 1008 || failures.get() >= 8 {
                error.set("即時連線已停止；重新登入或重新選擇房間".into());
                return;
            }
            let delay = 500 * 2_u32.pow(failures.get().min(5));
            failures.set(failures.get() + 1);
            error.set("即時連線中斷，正在重新連線".into());
            *pending.borrow_mut() = Some(gloo_timers::callback::Timeout::new(delay, move || {
                generation.update(|v| *v = v.wrapping_add(1));
            }));
        });
        socket.set_onclose(Some(close.as_ref().unchecked_ref()));

        let current = id.clone();
        let live = active.clone();
        let poll = gloo_timers::callback::Interval::new(5000, move || {
            let current = current.clone();
            let live = live.clone();
            super::spawn(async move {
                if let Ok(v) = api(&format!("/api/v1/rooms/{current}/presence"), "GET", None).await
                    && live.get()
                    && room.get_untracked() == current
                {
                    presence.set(v.as_array().cloned().unwrap_or_default());
                }
            });
        });
        let guard = SendWrapper::new(SocketGuard {
            active,
            socket,
            _open: open,
            _message: message,
            _close: close,
            retry,
            _presence_poll: poll,
            typing_timer,
        });
        on_cleanup(move || drop(guard));
    });
}
/// Bounded in-memory queue: retries keep the exact same UUID and payload.
/// Page reload intentionally does not retain private message content in browser storage.
pub fn delivery_queue(
    room: RwSignal<String>,
    pending: RwSignal<Vec<Value>>,
    messages: RwSignal<Vec<Value>>,
    error: RwSignal<String>,
) {
    let active = Rc::new(Cell::new(true));
    let running = Rc::new(Cell::new(false));
    let cleanup = SendWrapper::new(active.clone());
    on_cleanup(move || cleanup.set(false));
    Effect::new(move |_| {
        let entries = pending.get();
        if entries.is_empty() || running.replace(true) {
            return;
        }
        let active = active.clone();
        let running = running.clone();
        super::spawn(async move {
            let mut attempts = 0_u32;
            while active.get() {
                let Some(body) = pending.get_untracked().first().cloned() else {
                    break;
                };
                match api("/api/v1/messages", "POST", Some(body.clone())).await {
                    Ok(_) => {
                        if !active.get() {
                            break;
                        }
                        pending.update(|queue| queue.retain(|entry| entry["id"] != body["id"]));
                        if room.get_untracked() == body["room_id"].as_str().unwrap_or("") {
                            messages.update(|items| {
                                if !items.iter().any(|entry| entry["id"] == body["id"]) {
                                    items.push(body.clone());
                                }
                                if items.len() > 200 {
                                    items.remove(0);
                                }
                            });
                        }
                        if error.get_untracked() == "連線失敗，請重試" {
                            error.set(String::new());
                        }
                        attempts = 0;
                    }
                    Err(e) => {
                        if !active.get() {
                            break;
                        }
                        error.set(e.clone());
                        if !(e == "連線失敗，請重試"
                            || e.ends_with("500")
                            || e.ends_with("502")
                            || e.ends_with("503")
                            || e.ends_with("504"))
                            || attempts >= 8
                        {
                            break;
                        }
                        let delay = 500 * 2_u32.pow(attempts.min(5));
                        attempts += 1;
                        gloo_timers::future::TimeoutFuture::new(delay).await;
                    }
                }
            }
            running.set(false);
        });
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
