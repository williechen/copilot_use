#![recursion_limit = "256"]
//! Leptos SSR/Hydration Router：登入、巢狀受保護頁面、聊天室與知識庫。
use leptos::prelude::*;
use leptos_router::{
    components::{A, Outlet, ParentRoute, ProtectedRoute, Route, Router, Routes},
    hooks::use_navigate,
    path,
};
use serde_json::{Value, json};
mod management;
#[derive(Clone, Copy)]
struct Session {
    signed_in: RwSignal<bool>,
    error: RwSignal<String>,
}
#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(target_arch = "wasm32")]
use browser::api;
#[cfg(not(target_arch = "wasm32"))]
async fn api(_: &str, _: &str, _: Option<Value>) -> Result<Value, String> {
    Err("此操作需要瀏覽器".into())
}
async fn privileged_api(
    token: String,
    path: &str,
    method: &str,
    body: Option<Value>,
) -> Result<Value, String> {
    #[cfg(target_arch = "wasm32")]
    {
        browser::api_with_token(&token, path, method, body).await
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (token, path, method, body);
        Err("此操作需要瀏覽器".into())
    }
}
fn session() -> Session {
    use_context().expect("Session context")
}
fn spawn(task: impl std::future::Future<Output = ()> + 'static) {
    leptos::task::spawn_local(task);
}
#[component]
pub fn App(signed_in: bool) -> impl IntoView {
    let state = Session {
        signed_in: RwSignal::new(signed_in),
        error: RwSignal::new(String::new()),
    };
    provide_context(state);
    view! {<Router><main><h1>"Rust AI Chat"</h1><nav><A href="/">"首頁"</A><a rel="external" href=move ||if state.signed_in.get(){"/app/manage"}else{"/login"}>{move ||if state.signed_in.get(){"管理"}else{"登入"}}</a></nav><p class="error" role="alert">{move ||state.error.get()}</p>
    <Routes fallback=||view!{<p>"找不到頁面"</p>}>
    <Route path=path!("/") view=||view!{<management::RoomLobby public=true/>}/>
    <Route path=path!("/enter") view=RoomEntry/>
    <Route path=path!("/registration-complete") view=RegistrationComplete/>
    <Route path=path!("/login") view=Login/>
    <Route path=path!("/admin") view=management::SystemAdministration/>
    <ParentRoute path=path!("/app") view=Layout>
    <ProtectedRoute path=path!("") view=||view!{<management::RoomLobby public=false/>} condition=move ||Some(state.signed_in.get()) redirect_path=||"/login"/>
    <ProtectedRoute path=path!("chat") view=Chat condition=move ||Some(state.signed_in.get()) redirect_path=||"/login"/>
    <ProtectedRoute path=path!("manage") view=management::MyManagement condition=move ||Some(state.signed_in.get()) redirect_path=||"/login"/>
    <ProtectedRoute path=path!("knowledge") view=Knowledge condition=move ||Some(state.signed_in.get()) redirect_path=||"/login"/>
    </ParentRoute></Routes></main></Router>}
}
// 只接受房間識別，不接受任意回跳網址，避免外部導向及路徑注入。
fn valid_room_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 40
        && id
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
}
fn login_destination(room: Option<&str>) -> String {
    match room.filter(|id| valid_room_id(id)) {
        Some(id) => format!("/enter?room={id}"),
        None => "/app/manage".into(),
    }
}
#[component]
fn RegistrationComplete() -> impl IntoView {
    view! {<section id="registration-complete"><h2>"註冊完成"</h2><p>"目前已登出。請回首頁重新選擇聊天室，或由首頁登入個人頁。"</p><A href="/">"回首頁"</A></section>}
}
#[component]
fn RoomEntry() -> impl IntoView {
    let state = session();
    let params = leptos_router::hooks::use_query_map();
    let navigate = use_navigate();
    let error = RwSignal::new(String::new());
    Effect::new(move |_| {
        let id = params.with(|p| p.get("room")).unwrap_or_default();
        if !valid_room_id(&id) {
            error.set("房間識別不合法，請回首頁重新選房。".into());
            return;
        }
        let navigate = navigate.clone();
        if !state.signed_in.get_untracked() {
            navigate(&format!("/login?room={id}"), Default::default());
            return;
        }
        error.set(String::new());
        spawn(async move {
            // 進房前由伺服器檢查 Session 與房間權限。
            let result = api(&format!("/api/v1/rooms/{id}/join"), "POST", Some(json!({}))).await;
            if params.with_untracked(|p| p.get("room")).as_deref() != Some(id.as_str()) {
                return;
            }
            match result {
                Ok(_) => navigate(&format!("/app/chat?room={id}"), Default::default()),
                Err(e) if e == "登入已失效，請重新登入。" => {
                    state.signed_in.set(false);
                    navigate(&format!("/login?room={id}"), Default::default());
                }
                Err(e) => error.set(format!("無法進入房間：{e}")),
            }
        });
    });
    view! {<section><h2>"進入聊天室"</h2><p role="status">{move ||if error.get().is_empty(){"正在核對房間進入權限…".into()}else{error.get()}}</p><A href="/">"回首頁"</A></section>}
}
#[component]
fn Layout() -> impl IntoView {
    let state = session();
    view! {<section><nav><A href="/app">"房間列表"</A><A href="/app/manage">"我的管理"</A><A href="/app/knowledge">"知識庫"</A></nav><button on:click=move |_|spawn(async move{match api("/api/v1/auth/logout","POST",Some(json!({}))).await{Ok(_)=>state.signed_in.set(false),Err(e)=>state.error.set(e)}})>"登出"</button><Outlet/></section>}
}
fn credential_error(username: &str, password: &str) -> Option<&'static str> {
    if username.len() < 3 || username.len() > 100 {
        Some("請填寫使用者名稱，長度須為 3–100 字元。")
    } else if !username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        Some("使用者名稱只能包含文字、數字、底線（_）或連字號（-）。")
    } else if password.len() < 12 || password.len() > 128 {
        Some("密碼長度不符，請使用至少 12 字元、最多 128 字元的密碼。")
    } else {
        None
    }
}
fn auth_input_values(username: RwSignal<String>, password: RwSignal<String>) -> (String, String) {
    // Read the form controls as well as signals, including values filled by the browser.
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        let value = |id: &str| {
            web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id(id))
                .and_then(|e| e.dyn_into::<web_sys::HtmlInputElement>().ok())
                .map(|input| input.value())
        };
        (
            value("username")
                .unwrap_or_else(|| username.get_untracked())
                .trim()
                .to_owned(),
            value("password").unwrap_or_else(|| password.get_untracked()),
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        (
            username.get_untracked().trim().to_owned(),
            password.get_untracked(),
        )
    }
}
#[cfg(target_arch = "wasm32")]
fn api_failure(path: &str, status: u16) -> String {
    if matches!(path, "/api/v1/auth/login" | "/api/v1/auth/register") {
        match status {
            400 => "帳號或密碼格式不符：使用者名稱須為 3–100 字元，只能包含文字、數字、_ 或 -；密碼至少 12 字元。".into(),
            401 => "帳號或密碼不正確。首次使用請先按「註冊」，成功後再登入。".into(),
            403 => "頁面驗證未通過，請重新整理後再試。".into(),
            409 => "此使用者名稱已被註冊，請登入或使用另一個名稱。".into(),
            429 => "嘗試次數過多，請稍候一分鐘再試。".into(),
            500..=599 => "服務暫時無法完成登入或註冊，請稍後再試。".into(),
            _ => format!("操作失敗：{status}"),
        }
    } else {
        match status {
            401 => "登入已失效，請重新登入。".into(),
            403 => "你沒有進入或操作此房間的權限。".into(),
            _ => format!("操作失敗：{status}"),
        }
    }
}
#[component]
fn Login() -> impl IntoView {
    let state = session();
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let params = leptos_router::hooks::use_query_map();
    let notice = RwSignal::new(
        if params
            .with_untracked(|p| p.get("account_deleted"))
            .as_deref()
            == Some("1")
        {
            "帳號已刪除，尚未交接的房間已封存。".to_owned()
        } else {
            String::new()
        },
    );
    let navigate = use_navigate();
    let register_navigate = navigate.clone();
    let login = move || {
        if busy.get_untracked() {
            return;
        }
        let (name, secret) = auth_input_values(username, password);
        state.error.set(String::new());
        notice.set(String::new());
        if let Some(error) = credential_error(&name, &secret) {
            state.error.set(error.into());
            return;
        }
        username.set(name.clone());
        let credentials = json!({"username":name,"password":secret});
        let navigate = navigate.clone();
        busy.set(true);
        spawn(async move {
            match api("/api/v1/auth/login", "POST", Some(credentials)).await {
                Ok(_) => {
                    password.set(String::new());
                    state.error.set(String::new());
                    state.signed_in.set(true);
                    navigate(
                        &login_destination(params.with_untracked(|p| p.get("room")).as_deref()),
                        Default::default(),
                    );
                }
                Err(e) => state.error.set(e),
            }
            busy.set(false)
        });
    };
    view! {<section id="account-access"><h2>"登入帳號"</h2>
    <p>"首次使用請先填寫帳號與密碼，按「註冊」建立帳號；註冊完成後會進入獨立訊息頁，請返回首頁重新選房或登入個人頁。"</p>
    <form id="auth-form" on:submit=move |event|{event.prevent_default();login();}>
    <label>"使用者名稱"<input id="username" name="username" required autocomplete="username" prop:value=move ||username.get() on:input=move |e|username.set(event_target_value(&e)) on:change=move |e|username.set(event_target_value(&e))/></label>
    <small>"3–100 字元，可使用文字、數字、_ 或 -。"</small>
    <label>"密碼（至少 12 字元）"<input id="password" name="password" type="password" required autocomplete="current-password" prop:value=move ||password.get() on:input=move |e|password.set(event_target_value(&e)) on:change=move |e|password.set(event_target_value(&e))/></label>
    <button id="login" type="submit" disabled=move ||busy.get()>"登入"</button>
    <button id="register" type="button" disabled=move ||busy.get() on:click=move |_|{
        if busy.get_untracked(){return;}
        let (name,secret)=auth_input_values(username,password);
        state.error.set(String::new());notice.set(String::new());
        if let Some(error)=credential_error(&name,&secret){state.error.set(error.into());return;}
        username.set(name.clone());
        let credentials=json!({"username":name,"password":secret});busy.set(true);
        let navigate=register_navigate.clone();
        spawn(async move{match api("/api/v1/auth/register","POST",Some(credentials)).await{
            Ok(_)=>match api("/api/v1/auth/logout","POST",Some(json!({}))).await {
                Ok(_)=>{password.set(String::new());state.signed_in.set(false);navigate("/registration-complete",leptos_router::NavigateOptions{replace:true,..Default::default()});},
                Err(e) if e == "登入已失效，請重新登入。" => {password.set(String::new());state.signed_in.set(false);navigate("/registration-complete",leptos_router::NavigateOptions{replace:true,..Default::default()});},
                Err(e)=>state.error.set(format!("註冊完成，但無法確認登出：{e}")),
            },Err(e)=>state.error.set(e)
        }busy.set(false)});
    }>"註冊"</button>
    <p class="success" role="status">{move ||notice.get()}</p></form></section>}
}
#[component]
fn Chat() -> impl IntoView {
    let state = session();
    let query_params = leptos_router::hooks::use_query_map();
    let rooms = RwSignal::new(Vec::<Value>::new());
    let selected = RwSignal::new(String::new());
    let messages = RwSignal::new(Vec::<Value>::new());
    let draft = RwSignal::new(String::new());
    let ai_output = RwSignal::new(String::new());
    let rag = RwSignal::new(false);
    let room_loading = RwSignal::new(false);
    let load = move |id: String| {
        state.error.set(String::new());
        selected.set(id.clone());
        messages.set(Vec::new());
        room_loading.set(!id.is_empty());
        if id.is_empty() {
            return;
        }
        let query = format!("/api/v1/messages?room_id={id}");
        spawn(async move {
            let result =
                match api(&format!("/api/v1/rooms/{id}/join"), "POST", Some(json!({}))).await {
                    Ok(_) => api(&query, "GET", None).await,
                    Err(e) => Err(e),
                };
            if selected.get_untracked() != id {
                return;
            }
            match result {
                Ok(v) => messages.set(v.as_array().cloned().unwrap_or_default()),
                Err(e) => state.error.set(format!("讀取房間訊息失敗：{e}")),
            }
            room_loading.set(false);
        });
    };
    let refresh = move || {
        spawn(async move {
            match api("/api/v1/rooms", "GET", None).await {
                Ok(v) => {
                    let available = v.as_array().cloned().unwrap_or_default();
                    let first = available
                        .first()
                        .and_then(|r| r["id"].as_str())
                        .unwrap_or("")
                        .to_owned();
                    let requested = query_params.with_untracked(|q| q.get("room"));
                    let target = requested.unwrap_or(first);
                    if !target.is_empty()
                        && !available.iter().any(|r| r["id"].as_str() == Some(&target))
                    {
                        state
                            .error
                            .set("房間不可進入，請回房間列表選擇可用房間。".into());
                        return;
                    }
                    rooms.set(available);
                    if selected.get_untracked().is_empty() && !target.is_empty() {
                        load(target);
                    }
                }
                Err(e) => state.error.set(format!("讀取房間清單失敗：{e}")),
            }
        })
    };
    Effect::new(move |_| refresh());
    let pending = RwSignal::new(Vec::<Value>::new());
    let presence = RwSignal::new(Vec::<Value>::new());
    let typing = RwSignal::new(String::new());
    let typing_signal = RwSignal::new(0_u32);
    #[cfg(target_arch = "wasm32")]
    {
        browser::observe_room(
            selected,
            messages,
            state.error,
            presence,
            typing,
            typing_signal,
        );
        browser::delivery_queue(selected, pending, messages, state.error);
    }

    let send = move || {
        let room = selected.get_untracked();
        let content = draft.get_untracked();
        if room.is_empty() || content.trim().is_empty() {
            state.error.set("請先選擇房間，再輸入訊息".into());
            return;
        }
        if pending.get_untracked().len() >= 32 {
            state.error.set("待送出佇列已滿（32 筆）".into());
            return;
        }
        let id = uuid();
        if id.is_empty() {
            state.error.set("無法建立安全訊息 ID".into());
            return;
        }
        state.error.set(String::new());
        pending.update(|items| items.push(json!({"id":id,"room_id":room,"content":content})));
        draft.set(String::new());
    };
    view! {<section><h2>"聊天室"</h2>
        <p id="room-status" role="status">{move ||if room_loading.get(){"正在讀取房間訊息…".to_owned()}else if selected.get().is_empty(){"請回房間列表選擇可用房間。".to_owned()}else{let name=rooms.get().iter().find(|r|r["id"]==selected.get()).and_then(|r|r["name"].as_str()).unwrap_or("").to_owned();format!("目前房間：{name}")}}</p>
        <p id="presence" role="status">{move ||format!("在線連線：{}",presence.get().len())}</p><p id="typing" aria-live="polite">{move ||typing.get()}</p>
        <ul id="messages" aria-live="polite">{move ||messages.get().into_iter().map(|m|view!{<li>{m["content"].as_str().unwrap_or("").to_owned()}<small>{m["id"].as_str().unwrap_or("").to_owned()}</small></li>}).collect_view()}</ul>
        <button on:click=move |_| {
            if messages.get_untracked().len()>=1000 {state.error.set("此頁最多保留 1000 筆歷史訊息".into());return;}
            let room=selected.get();let oldest=messages.get().first().and_then(|m|m["id"].as_str()).unwrap_or("").to_owned();
            if room.is_empty() ||oldest.is_empty(){return;}
            spawn(async move{match api(&format!("/api/v1/messages?room_id={room}&before={oldest}"),"GET",None).await {
                Ok(v)=>{if selected.get_untracked()==room {let mut older=v.as_array().cloned().unwrap_or_default();messages.update(|current|{older.extend(current.iter().filter(|m|!older.iter().any(|o|o["id"]==m["id"])).cloned().collect::<Vec<_>>());if older.len()>1000 {older.drain(..older.len()-1000);}
                *current=older;});}},Err(e)=>state.error.set(e)
            }});
        }>"載入較舊訊息"</button>
        <form id="message-form" on:submit=move |e|{e.prevent_default();send();}>
        <label for="message">"訊息"</label><input id="message" maxlength="65536" placeholder="輸入訊息後按 Enter 送出" disabled=move ||selected.get().is_empty() prop:value=move ||draft.get() on:input=move |e|{draft.set(event_target_value(&e));if !draft.get_untracked().is_empty(){typing_signal.update(|v|*v=v.wrapping_add(1));}}/><button id="send" type="submit" disabled=move ||selected.get().is_empty()||draft.get().trim().is_empty()>"送出"</button></form>
        <p role="status">{move ||format!("待送出：{} 筆（此頁關閉前保留）",pending.get().len())}</p>
        <button on:click=move |_|pending.update(|_|{})>"重試待送出"</button>
        <button on:click=move |_|pending.set(Vec::new())>"取消待送出"</button>
        <p>"取消佇列不會撤回已開始的送出請求。"</p>
        <label><input type="checkbox" on:change=move |e|rag.set(event_target_checked(&e))/>"使用我的知識庫"</label><button id="ai" disabled=move ||selected.get().is_empty() on:click=move |_|{let body=json!({"room_id":selected.get(),"rag":rag.get()});ai_output.set(String::new());
        #[cfg(target_arch="wasm32")] spawn(async move{if let Err(e)=browser::ai_stream(body,ai_output).await{state.error.set(e)}});
        #[cfg(not(target_arch="wasm32"))] let _=body;
        }>"請 AI 回覆"</button><pre id="ai-reply" aria-live="polite">{move ||ai_output.get()}</pre>
    </section>}
}
#[component]
fn TopicControls(room: RwSignal<String>) -> impl IntoView {
    let enabled = RwSignal::new(false);
    let known = RwSignal::new(false);
    let epoch = RwSignal::new(0_u64);
    let pending = RwSignal::new(0_i64);
    let busy = RwSignal::new(false);
    let error = RwSignal::new(String::new());
    let refresh = move || {
        let current = room.get();
        let version = epoch.get_untracked();
        if current.is_empty() {
            return;
        }
        spawn(async move {
            match api(
                &format!("/api/v1/rooms/{current}/topic-consent"),
                "GET",
                None,
            )
            .await
            {
                Ok(v) => {
                    if room.get_untracked() == current && epoch.get_untracked() == version {
                        known.set(true);
                        enabled.set(v["enabled"] == true);
                        pending.set(v["withdrawal_pending"].as_i64().unwrap_or(0));
                        error.set(String::new());
                    }
                }
                Err(e) => {
                    if room.get_untracked() == current && epoch.get_untracked() == version {
                        error.set(e);
                    }
                }
            }
        });
    };
    Effect::new(move |_| {
        let current = room.get();
        epoch.update(|v| *v = v.wrapping_add(1));
        enabled.set(false);
        known.set(false);
        busy.set(false);
        pending.set(0);
        error.set(String::new());
        if !current.is_empty() {
            refresh();
        }
    });
    view! {<Show when=move ||!room.get().is_empty()><section id="topic-consent"><h3>"匿名技術詞彙統計"</h3><p>"啟用後，僅你在此房間的後續訊息中的固定技術詞會以匿名計數送至 Product Analytics。含憑證或聯絡資訊的訊息略過。停用會透過可靠佇列封存既有詞彙統計。"</p>
    <label><input id="allow-topic-counting" type="checkbox" prop:checked=move ||enabled.get() disabled=move ||busy.get()||!known.get() on:change=move |e|{let value=event_target_checked(&e);let current=room.get();epoch.update(|v|*v=v.wrapping_add(1));let version=epoch.get_untracked();busy.set(true);spawn(async move{match api(&format!("/api/v1/rooms/{current}/topic-consent"),"POST",Some(json!({"enabled":value}))).await{Ok(v)=>{if room.get_untracked()==current&&epoch.get_untracked()==version{known.set(true);enabled.set(v["enabled"]==true);pending.set(v["withdrawal_pending"].as_i64().unwrap_or(0));error.set(String::new());}},Err(e)=>{if room.get_untracked()==current&&epoch.get_untracked()==version{enabled.set(enabled.get_untracked());error.set(e);}}}if epoch.get_untracked()==version{busy.set(false);}});}/>"同意此房間的技術詞彙計數"</label>
    <p role="status">{move ||if !known.get(){"正在讀取此房間的設定".to_owned()}else if enabled.get(){"已啟用：固定詞表，最多每則32次".to_owned()}else if pending.get()>0{format!("已停用；{} 筆既有詞彙事件待封存確認",pending.get())}else{"已停用；沒有待封存的詞彙事件".to_owned()}}</p><button disabled=move ||busy.get() on:click=move |_|refresh()>"查看封存進度"</button><p role="alert">{move ||error.get()}</p></section></Show>}
}
#[component]
fn PersonaControls(
    room: RwSignal<String>,
    can_manage: Signal<bool>,
    credential: RwSignal<String>,
) -> impl IntoView {
    let personas = RwSignal::new(Vec::<Value>::new());
    let id = RwSignal::new(String::new());
    let name = RwSignal::new(String::new());
    let instructions = RwSignal::new(String::new());
    let memory = RwSignal::new(String::new());
    let cooldown = RwSignal::new(2_i32);
    let enabled = RwSignal::new(true);
    let topic = RwSignal::new(String::new());
    let turns = RwSignal::new(2_i32);
    let budget = RwSignal::new(4096_i32);
    let tools = RwSignal::new(false);
    let confirmed = RwSignal::new(false);
    let busy = RwSignal::new(false);
    let run_id = RwSignal::new(String::new());
    let active_room = RwSignal::new(String::new());
    let result = RwSignal::new(String::new());
    let error = session().error;
    let refresh = move || {
        let current = room.get();
        if current.is_empty() || !can_manage.get() {
            return;
        }
        spawn(async move {
            match privileged_api(
                credential.get_untracked(),
                &format!("/api/v1/rooms/{current}/personas"),
                "GET",
                None,
            )
            .await
            {
                Ok(v) => {
                    if room.get_untracked() == current {
                        personas.set(v.as_array().cloned().unwrap_or_default());
                    }
                }
                Err(e) => error.set(e),
            }
        });
    };
    Effect::new(move |_| {
        let _ = room.get();
        let _ = can_manage.get();
        personas.set(Vec::new());
        id.set(String::new());
        name.set(String::new());
        instructions.set(String::new());
        memory.set(String::new());
        topic.set(String::new());
        confirmed.set(false);
        if !busy.get_untracked() {
            run_id.set(String::new());
        }
        result.set(String::new());
        refresh();
    });
    view! {<Show when=move ||can_manage.get()><section><h3>"Persona 與 AI 對 AI（系統管理者功能）"</h3><p>"最多八個角色，輪流發言。執行會把此房間近期訊息、角色設定與記憶交給 OpenAI；每次模型請求最多512輸出 tokens，工具模式每輪最多兩次模型請求。預算包含失敗的嘗試。"</p>
    <select on:change=move |e|{let selected=event_target_value(&e);if let Some(p)=personas.get().iter().find(|p|p["id"]==selected){id.set(selected);name.set(p["name"].as_str().unwrap_or("").into());instructions.set(p["instructions"].as_str().unwrap_or("").into());memory.set(p["memory"].as_str().unwrap_or("").into());cooldown.set(p["cooldown_seconds"].as_i64().unwrap_or(2)as i32);enabled.set(p["enabled"]==true);}}><option value="">"選擇角色"</option>{move ||personas.get().into_iter().map(|p|view!{<option value=p["id"].as_str().unwrap_or("").to_owned()>{p["name"].as_str().unwrap_or("").to_owned()}</option>}).collect_view()}</select>
    <button on:click=move |_|{id.set(uuid());name.set(String::new());instructions.set(String::new());memory.set(String::new());enabled.set(true);}>"新增角色"</button>
    <label>"名字"<input maxlength="80" prop:value=move ||name.get() on:input=move |e|name.set(event_target_value(&e))/></label>
    <label>"角色設定"<textarea maxlength="4096" prop:value=move ||instructions.get() on:input=move |e|instructions.set(event_target_value(&e))></textarea></label>
    <label>"可編輯／清空的角色記憶"<textarea maxlength="2048" prop:value=move ||memory.get() on:input=move |e|memory.set(event_target_value(&e))></textarea></label>
    <label>"同一角色冷卻秒數"<input type="number" min="1" max="30" prop:value=move ||cooldown.get() on:input=move |e|cooldown.set(event_target_value(&e).parse().unwrap_or(2))/></label>
    <label><input type="checkbox" prop:checked=move ||enabled.get() on:change=move |e|enabled.set(event_target_checked(&e))/>"允許此角色參與"</label>
    <button on:click=move |_|{let current=room.get();let pid=if id.get().is_empty(){uuid()}else{id.get()};id.set(pid.clone());let body=json!({"id":pid,"name":name.get(),"instructions":instructions.get(),"memory":memory.get(),"cooldown_seconds":cooldown.get(),"enabled":enabled.get()});spawn(async move{match privileged_api(credential.get_untracked(),&format!("/api/v1/rooms/{current}/personas"),"POST",Some(body)).await{Ok(_)=>refresh(),Err(e)=>error.set(e)}});}>"保存角色"</button>
    <h4>"有界對話"</h4><textarea maxlength="4096" placeholder="共同討論的主題" prop:value=move ||topic.get() on:input=move |e|topic.set(event_target_value(&e))></textarea>
    <label>"輪數（1–8）"<input type="number" min="1" max="8" prop:value=move ||turns.get() on:input=move |e|turns.set(event_target_value(&e).parse().unwrap_or(2))/></label>
    <label>"輸出 token 上限"<input type="number" min="512" max="16384" prop:value=move ||budget.get() on:input=move |e|budget.set(event_target_value(&e).parse().unwrap_or(4096))/></label>
    <label><input type="checkbox" on:change=move |e|tools.set(event_target_checked(&e))/>"允許唯讀 MCP 知識庫工具，並把檢索資料交給模型"</label>
    <label><input type="checkbox" on:change=move |e|confirmed.set(event_target_checked(&e))/>"確認使用以上房間資料與請求預算"</label>
    <button disabled=move ||!confirmed.get()||busy.get() on:click=move |_|{let current=room.get();let rid=uuid();active_room.set(current.clone());run_id.set(rid.clone());let body=json!({"id":rid,"topic":topic.get(),"max_turns":turns.get(),"max_output_budget":budget.get(),"tools":tools.get(),"confirmed":true});busy.set(true);result.set("執行中；已完成輪次會保存，最多180秒".into());spawn(async move{match privileged_api(credential.get_untracked(),&format!("/api/v1/rooms/{current}/persona-runs"),"POST",Some(body)).await{Ok(v)=>{if room.get_untracked()==current{result.set(v.to_string());refresh();}},Err(e)=>{if room.get_untracked()==current {result.set("執行未完成；請查看錯誤或執行收據".into());}error.set(e)}}busy.set(false);});}>"開始對話"</button>
    <button disabled=move ||!busy.get() on:click=move |_|{let current=active_room.get();let rid=run_id.get();spawn(async move{match privileged_api(credential.get_untracked(),&format!("/api/v1/rooms/{current}/persona-runs/{rid}/cancel"),"POST",Some(json!({"confirmed":true}))).await{Ok(_)=>result.set("已請求取消；已完成輪次仍保留".into()),Err(e)=>error.set(e)}});}>"取消剩餘輪次"</button>
    <button disabled=move ||run_id.get().is_empty() on:click=move |_|{let current=active_room.get();let rid=run_id.get();spawn(async move{match privileged_api(credential.get_untracked(),&format!("/api/v1/rooms/{current}/persona-runs/{rid}"),"GET",None).await{Ok(v)=>result.set(v.to_string()),Err(e)=>error.set(e)}});}>"查看執行收據"</button><pre aria-live="polite">{move ||result.get()}</pre>
    </section></Show>}.into_any()
}
#[component]
fn Knowledge() -> impl IntoView {
    let state = session();
    let title = RwSignal::new(String::new());
    let content = RwSignal::new(String::new());
    let documents = RwSignal::new(Vec::<Value>::new());
    let refresh = move || {
        spawn(async move {
            match api("/api/v1/knowledge", "GET", None).await {
                Ok(v) => documents.set(v.as_array().cloned().unwrap_or_default()),
                Err(e) => state.error.set(e),
            }
        })
    };
    Effect::new(move |_| refresh());
    view! {<section><h2>"我的知識庫"</h2><p>"加入的資料將傳送至 OpenAI 產生檢索向量；只加入你已授權使用的資料。"</p><input placeholder="標題" maxlength="200" on:input=move |e|title.set(event_target_value(&e))/><textarea maxlength="32768" placeholder="參考資料" on:input=move |e|content.set(event_target_value(&e))></textarea><button on:click=move |_|{let body=json!({"title":title.get(),"content":content.get()});spawn(async move{match api("/api/v1/knowledge","POST",Some(body)).await{Ok(_)=>refresh(),Err(e)=>state.error.set(e)}});}>"加入知識庫"</button>
    <ul>{move ||documents.get().into_iter().map(|d|{let id=d["id"].as_str().unwrap_or("").to_owned();view!{<li>{d["title"].as_str().unwrap_or("").to_owned()}<button on:click=move |_|{let path=format!("/api/v1/knowledge/{id}");spawn(async move{match api(&path,"DELETE",None).await{Ok(_)=>refresh(),Err(e)=>state.error.set(e)}});}>"刪除"</button></li>}}).collect_view()}</ul></section>}
}
#[cfg(target_arch = "wasm32")]
fn uuid() -> String {
    js_sys::Reflect::get(&web_sys::window().unwrap(), &"crypto".into())
        .and_then(|crypto| {
            js_sys::Reflect::get(&crypto, &"randomUUID".into()).and_then(|f| {
                use wasm_bindgen::JsCast;
                f.unchecked_into::<js_sys::Function>().call0(&crypto)
            })
        })
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_default()
}
#[cfg(not(target_arch = "wasm32"))]
fn uuid() -> String {
    String::new()
}
pub fn shell(signed_in: bool) -> impl IntoView {
    view! {<!DOCTYPE html><html lang="zh-Hant"><head><meta charset="utf-8"/><meta name="viewport" content="width=device-width,initial-scale=1"/><meta name="copilot-signed-in" content=if signed_in{"true"}else{"false"}/><title>"Rust AI Chat"</title><link rel="stylesheet" href="/pkg/chat.css"/></head><body><App signed_in/><script type="module">"import init from '/pkg/chat_ui.js';await init({module_or_path:'/pkg/chat_ui_bg.wasm'});"</script></body></html>}
}
#[cfg(feature = "hydrate")]
pub fn hydrate() {
    #[cfg(target_arch = "wasm32")]
    {
        let signed_in = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| {
                d.query_selector("meta[name='copilot-signed-in']")
                    .ok()
                    .flatten()
            })
            .and_then(|e| e.get_attribute("content"))
            .as_deref()
            == Some("true");
        leptos::mount::hydrate_body(move || view! {<App signed_in/>});
    }
}

#[cfg(test)]
mod entry_tests {
    use super::*;
    #[cfg(feature = "ssr")]
    fn render(path: &str, signed_in: bool) -> String {
        let owner = Owner::new();
        owner.with(|| {
            provide_context(leptos_router::location::RequestUrl::new(path));
            view! {<App signed_in/>}.to_html()
        })
    }
    #[cfg(feature = "ssr")]
    #[test]
    fn homepage_and_registration_completion_have_distinct_entry_surfaces() {
        let guest = render("/", false);
        assert!(guest.contains("room-lobby"));
        assert!(guest.contains(r#"href="/login""#));
        let member = render("/", true);
        assert!(member.contains("管理"));
        assert!(member.contains(r#"href="/app/manage""#));
        let completed = render("/registration-complete", false);
        assert!(completed.contains(r#"id="registration-complete""#));
        assert!(completed.contains("目前已登出"));
        assert!(!completed.contains(r#"id="auth-form""#));
        assert!(!completed.contains(r#"id="room-lobby""#));
    }
    #[test]
    fn login_preserves_only_a_valid_room_target() {
        assert_eq!(
            login_destination(Some("room_123-abc")),
            "/enter?room=room_123-abc"
        );
        for target in [
            "",
            "https://example.com",
            "//example.com",
            "room&admin=1",
            "../admin",
        ] {
            assert_eq!(login_destination(Some(target)), "/app/manage");
        }
        assert_eq!(login_destination(Some(&"x".repeat(41))), "/app/manage");
        assert_eq!(login_destination(None), "/app/manage");
    }
}
