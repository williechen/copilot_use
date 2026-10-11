//! Leptos SSR/Hydration Router：登入、巢狀受保護頁面、聊天室與知識庫。
use leptos::prelude::*;
use leptos_router::{
    components::{A, Outlet, ParentRoute, ProtectedRoute, Route, Router, Routes},
    hooks::use_navigate,
    path,
};
use serde_json::{Value, json};
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
    view! {<Router><main><h1>"Rust AI Chat"</h1><nav><A href="/">"首頁"</A><A href="/login">"登入"</A><A href="/app">"聊天室"</A><A href="/app/knowledge">"知識庫"</A></nav><p class="error" role="alert">{move ||state.error.get()}</p>
    <Routes fallback=||view!{<p>"找不到頁面"</p>}>
    <Route path=path!("/") view=||view!{<p>"多人、多主題、持久訊息與 AI 對話"</p><A href="/app">"進入聊天室"</A>}/>
    <Route path=path!("/login") view=Login/>
    <ParentRoute path=path!("/app") view=Layout>
    <ProtectedRoute path=path!("") view=Chat condition=move ||Some(state.signed_in.get()) redirect_path=||"/login"/>
    <ProtectedRoute path=path!("knowledge") view=Knowledge condition=move ||Some(state.signed_in.get()) redirect_path=||"/login"/>
    </ParentRoute></Routes></main></Router>}
}
#[component]
fn Layout() -> impl IntoView {
    let state = session();
    view! {<section><button on:click=move |_|spawn(async move{match api("/api/v1/auth/logout","POST",Some(json!({}))).await{Ok(_)=>state.signed_in.set(false),Err(e)=>state.error.set(e)}})>"登出"</button><Outlet/></section>}
}
#[component]
fn Login() -> impl IntoView {
    let state = session();
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let navigate = use_navigate();
    let login = move |_| {
        let credentials = json!({"username":username.get(),"password":password.get()});
        let navigate = navigate.clone();
        busy.set(true);
        spawn(async move {
            match api("/api/v1/auth/login", "POST", Some(credentials)).await {
                Ok(_) => {
                    password.set(String::new());
                    state.signed_in.set(true);
                    navigate("/app", Default::default());
                }
                Err(e) => state.error.set(e),
            }
            busy.set(false)
        });
    };
    view! {<section><h2>"登入帳號"</h2><label>"使用者名稱"<input id="username" minlength="3" maxlength="100" autocomplete="username" prop:value=move ||username.get() on:input=move |e|username.set(event_target_value(&e))/></label><label>"密碼（至少 12 字元）"<input id="password" type="password" minlength="12" maxlength="128" autocomplete="current-password" prop:value=move ||password.get() on:input=move |e|password.set(event_target_value(&e))/></label><button id="login" disabled=move ||busy.get() on:click=login>"登入"</button>
    <button id="register" disabled=move ||busy.get() on:click=move |_|{let credentials=json!({"username":username.get(),"password":password.get()});busy.set(true);spawn(async move{match api("/api/v1/auth/register","POST",Some(credentials)).await{Ok(_)=>state.error.set("註冊成功，請登入".into()),Err(e)=>state.error.set(e)}busy.set(false)});}>"註冊"</button></section>}
}
#[component]
fn Chat() -> impl IntoView {
    let state = session();
    let rooms = RwSignal::new(Vec::<Value>::new());
    let selected = RwSignal::new(String::new());
    let messages = RwSignal::new(Vec::<Value>::new());
    let draft = RwSignal::new(String::new());
    let room_name = RwSignal::new(String::new());
    let ai_output = RwSignal::new(String::new());
    let rag = RwSignal::new(false);
    let agent_question = RwSignal::new(String::new());
    let agent_result = RwSignal::new(String::new());
    let allow_tools = RwSignal::new(false);
    let agent_busy = RwSignal::new(false);
    let suggestion_id = RwSignal::new(String::new());
    let confirm_policy = RwSignal::new(false);
    let policy_result = RwSignal::new(String::new());
    let invitee = RwSignal::new(String::new());
    let refresh = move || {
        spawn(async move {
            match api("/api/v1/rooms", "GET", None).await {
                Ok(v) => rooms.set(v.as_array().cloned().unwrap_or_default()),
                Err(e) => state.error.set(e),
            }
        })
    };
    Effect::new(move |_| refresh());
    let load = move |id: String| {
        selected.set(id.clone());
        messages.set(Vec::new());
        let query = format!("/api/v1/messages?room_id={id}");
        spawn(async move {
            match api(&query, "GET", None).await {
                Ok(v) => {
                    if selected.get_untracked() == id {
                        messages.set(v.as_array().cloned().unwrap_or_default());
                    }
                }
                Err(e) => state.error.set(e),
            }
        });
    };
    let pending = RwSignal::new(Vec::<Value>::new());
    #[cfg(target_arch = "wasm32")]
    {
        browser::observe_room(selected, messages, state.error);
        browser::delivery_queue(selected, pending, messages, state.error);
    }

    view! {<section><h2>"聊天室"</h2><input id="room-name" placeholder="新房間名稱" maxlength="200" prop:value=move ||room_name.get() on:input=move |e|room_name.set(event_target_value(&e))/><button id="create-room" on:click=move |_|{let id=uuid();let name=room_name.get();spawn(async move{match api("/api/v1/rooms","POST",Some(json!({"id":id,"name":name}))).await{Ok(_)=>{room_name.set(String::new());if let Ok(v)=api("/api/v1/rooms","GET",None).await {rooms.set(v.as_array().cloned().unwrap_or_default());}load(id)},Err(e)=>state.error.set(e)}});}>"建立私人房間"</button>
        <select id="rooms" prop:value=move ||selected.get() on:change=move |e|load(event_target_value(&e))><option value="">"選擇房間"</option>{move ||rooms.get().into_iter().map(|r|view!{<option value=r["id"].as_str().unwrap_or("").to_owned()>{r["name"].as_str().unwrap_or("").to_owned()}</option>}).collect_view()}</select>
        <input placeholder="邀請的使用者名稱" on:input=move |e|invitee.set(event_target_value(&e))/><button on:click=move |_|{let body=json!({"room_id":selected.get(),"username":invitee.get()});spawn(async move{match api("/api/v1/rooms/invite","POST",Some(body)).await{Ok(_)=>state.error.set("邀請已完成".into()),Err(e)=>state.error.set(e)}});}>"邀請加入"</button>
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
        <input id="message" maxlength="65536" placeholder="輸入訊息" prop:value=move ||draft.get() on:input=move |e|draft.set(event_target_value(&e))/><button id="send" on:click=move |_| {
            let room=selected.get();let content=draft.get();
            if room.is_empty() || content.trim().is_empty(){state.error.set("請選擇房間並輸入訊息".into());return;}
            if pending.get_untracked().len()>=32{state.error.set("待送出佇列已滿（32 筆）".into());return;}
            let id=uuid();if id.is_empty(){state.error.set("無法建立安全訊息 ID".into());return;}
            pending.update(|items|items.push(json!({"id":id,"room_id":room,"content":content})));
            draft.set(String::new());
        }>"送出"</button>
        <p role="status">{move ||format!("待送出：{} 筆（此頁關閉前保留）",pending.get().len())}</p>
        <button on:click=move |_|pending.update(|_|{})>"重試待送出"</button>
        <button on:click=move |_|pending.set(Vec::new())>"取消待送出"</button>
        <p>"取消佇列不會撤回已開始的送出請求。"</p>
        <label><input type="checkbox" on:change=move |e|rag.set(event_target_checked(&e))/>"使用我的知識庫"</label><button id="ai" on:click=move |_|{let body=json!({"room_id":selected.get(),"rag":rag.get()});ai_output.set(String::new());
        #[cfg(target_arch="wasm32")] spawn(async move{if let Err(e)=browser::ai_stream(body,ai_output).await{state.error.set(e)}});
        #[cfg(not(target_arch="wasm32"))] let _=body;
        }>"請 AI 回覆"</button><pre id="ai-reply" aria-live="polite">{move ||ai_output.get()}</pre>
        <h3>"工具檢索與 Agent"</h3><textarea placeholder="讓 Agent 搜尋知識庫的問題" maxlength="4096" prop:value=move ||agent_question.get() on:input=move |e|agent_question.set(event_target_value(&e))></textarea>
        <label><input type="checkbox" on:change=move |e|allow_tools.set(event_target_checked(&e))/>"允許唯讀搜尋我的知識庫，將相關資料交給 OpenAI 回答此房間"</label>
        <button disabled=move ||!allow_tools.get() ||agent_busy.get() on:click=move |_| {
            let body=json!({"room_id":selected.get(),"question":agent_question.get()});agent_busy.set(true);agent_result.set("正在使用唯讀知識庫工具…".into());
            spawn(async move {match api("/api/v1/ai/agent","POST",Some(body)).await {
                Ok(v)=>agent_result.set(format!("{}\n理解問題：{}\n來源：{}\n工具：{}",v["answer"].as_str().unwrap_or(""),v["follow_up_question"].as_str().unwrap_or(""),v["sources"],v["steps"])),Err(e)=>state.error.set(e)
            }agent_busy.set(false);});
        }>"執行 Agent"</button><pre aria-live="polite">{move ||agent_result.get()}</pre>
        <h3>"已審核的設定建議"</h3><p>"在 Analytics 檢視證據並核准後，輸入建議 UUID。只改變目前房間的 AI 歷史訊息數；回退會檢查之後是否有其他變更。"</p>
        <input placeholder="建議 UUID" prop:value=move ||suggestion_id.get() on:input=move |e|suggestion_id.set(event_target_value(&e))/>
        <label><input type="checkbox" on:change=move |e|confirm_policy.set(event_target_checked(&e))/>"確認變更目前房間設定"</label>
        <button disabled=move ||!confirm_policy.get() on:click=move |_|{let path=format!("/api/v1/rooms/{}/suggestions/{}/apply",selected.get(),suggestion_id.get());spawn(async move{match api(&path,"POST",Some(json!({"confirmed":true}))).await{Ok(v)=>policy_result.set(v.to_string()),Err(e)=>state.error.set(e)}});}>"套用並回報"</button>
        <button disabled=move ||!confirm_policy.get() on:click=move |_|{let path=format!("/api/v1/rooms/{}/suggestions/{}/rollback",selected.get(),suggestion_id.get());spawn(async move{match api(&path,"POST",Some(json!({"confirmed":true}))).await{Ok(v)=>policy_result.set(v.to_string()),Err(e)=>state.error.set(e)}});}>"回退並回報"</button><pre>{move ||policy_result.get()}</pre>

    </section>}
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
