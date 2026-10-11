//! Leptos CSR GUI：多寵物五欄設定、可管理記憶、Active Recall 與間隔複習。
use leptos::prelude::*;
use serde_json::{Value, json};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=["window","__TAURI__","core"],js_name=invoke)]
    fn invoke(command: &str, args: JsValue) -> js_sys::Promise;
    #[wasm_bindgen(js_namespace=["window","crypto"],js_name=randomUUID)]
    fn random_uuid() -> String;
    #[wasm_bindgen(js_namespace=["window","__TAURI__","window"],js_name=getCurrentWindow)]
    fn current_window() -> JsValue;
    #[wasm_bindgen(js_namespace=["window","__TAURI__","event"],js_name=listen)]
    fn listen(event: &str, handler: &js_sys::Function) -> js_sys::Promise;
}
async fn call(command: &str, args: Value) -> Result<Value, String> {
    let value = js_sys::JSON::parse(&args.to_string()).map_err(|_| "參數格式錯誤".to_owned())?;
    let result = wasm_bindgen_futures::JsFuture::from(invoke(command, value))
        .await
        .map_err(|e| e.as_string().unwrap_or_else(|| "操作失敗".into()))?;
    let text = js_sys::JSON::stringify(&result)
        .map_err(|_| "回覆格式錯誤".to_owned())?
        .as_string()
        .unwrap_or("null".into());
    serde_json::from_str(&text).map_err(|_| "回覆格式錯誤".into())
}
fn uuid() -> String {
    random_uuid()
}
fn strings(s: String) -> Vec<String> {
    s.split(',')
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(str::to_owned)
        .collect()
}
#[component]
fn Settings() -> impl IntoView {
    let sprite = RwSignal::new(String::new());
    let prompt = RwSignal::new(String::new());
    let reply = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let request = RwSignal::new(String::new());
    leptos::task::spawn_local(async move {
        let handler = Closure::<dyn FnMut(JsValue)>::new(move |value: JsValue| {
            if let Ok(raw) = js_sys::JSON::stringify(&value)
                && let Some(raw) = raw.as_string()
                && let Ok(event) = serde_json::from_str::<Value>(&raw)
                && event["payload"]["request"].as_str() == Some(request.get_untracked().as_str())
            {
                let delta = event["payload"]["text"].as_str().unwrap_or("").to_owned();
                reply.update(|text| text.push_str(&delta));
            }
        });
        let _ = wasm_bindgen_futures::JsFuture::from(listen(
            "pet-reply-delta",
            handler.as_ref().unchecked_ref(),
        ))
        .await;
        handler.forget();
    });
    let pets = RwSignal::new(Vec::<Value>::new());
    let pet = RwSignal::new(String::new());
    let name = RwSignal::new(String::new());
    let species = RwSignal::new(String::new());
    let likes = RwSignal::new(String::new());
    let personality = RwSignal::new(String::new());
    let interests = RwSignal::new(String::new());
    let memories = RwSignal::new(Vec::<Value>::new());
    let cards = RwSignal::new(Vec::<Value>::new());
    let memory = RwSignal::new(String::new());
    let question = RwSignal::new(String::new());
    let answer = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());
    let refresh = move || {
        leptos::task::spawn_local(async move {
            match call("list_pets", json!({})).await {
                Ok(v) => pets.set(v.as_array().cloned().unwrap_or_default()),
                Err(e) => error.set(e),
            }
        });
    };
    refresh();
    let load = move |id: String| {
        pet.set(id.clone());
        sprite.set(String::new());
        if let Some(item) = pets.get().iter().find(|x| x["id"] == id) {
            if let Ok(p) = serde_json::from_str::<Value>(item["profile"].as_str().unwrap_or("{}")) {
                name.set(p["名字"].as_str().unwrap_or("").into());
                species.set(p["種類"].as_str().unwrap_or("").into());
                let join = |key: &str| {
                    p[key]
                        .as_array()
                        .map(|a| {
                            a.iter()
                                .filter_map(Value::as_str)
                                .collect::<Vec<_>>()
                                .join(",")
                        })
                        .unwrap_or_default()
                };
                likes.set(join("喜好"));
                personality.set(join("個性"));
                interests.set(join("興趣"));
            }
        }
        leptos::task::spawn_local(async move {
            let _ = call("select_pet", json!({"pet":id})).await;
            if let Ok(asset) = call("load_asset", json!({"pet":id})).await
                && let Some(png) = asset["png_base64"].as_str()
            {
                sprite.set(format!("url(data:image/png;base64,{png})"));
            }
            match call("list_memories", json!({"pet":id})).await {
                Ok(v) => memories.set(v.as_array().cloned().unwrap_or_default()),
                Err(e) => error.set(e),
            }
            match call("list_cards", json!({"pet":id})).await {
                Ok(v) => cards.set(v.as_array().cloned().unwrap_or_default()),
                Err(e) => error.set(e),
            }
        });
    };
    view! {<main><h1>"桌面 AI 寵物"</h1><p class="error" role="alert">{move ||error.get()}</p>
       <section><h2>"我的寵物"</h2><select on:change=move |e|load(event_target_value(&e))><option value="">"選擇寵物"</option>{move ||pets.get().into_iter().map(|p|{let id=p["id"].as_str().unwrap_or("").to_owned();let profile:Value=serde_json::from_str(p["profile"].as_str().unwrap_or("{}")).unwrap_or_default();view!{<option value=id>{profile["名字"].as_str().unwrap_or("寵物").to_owned()}</option>}}).collect_view()}</select>
       <button on:click=move |_|{pet.set(uuid());name.set(String::new());species.set(String::new());likes.set(String::new());personality.set(String::new());interests.set(String::new());memories.set(Vec::new());cards.set(Vec::new());}>"新增寵物"</button>
       <div class="pet" class:animated=move ||!sprite.get().is_empty() style:background-image=move ||sprite.get()><Show when=move ||sprite.get().is_empty()><span>"🐾"</span></Show></div>
    <small>{move ||if sprite.get().is_empty() {"200×200 顯示區；尚未生成素材"}else{"200×200 角色動畫；素材已快取"}}</small>
    <button disabled=move ||busy.get() on:click=move |_|{let pid=pet.get();busy.set(true);leptos::task::spawn_local(async move{match call("generate_asset",json!({"pet":pid})).await{Ok(v)=>{if let Some(png)=v["png_base64"].as_str(){sprite.set(format!("url(data:image/png;base64,{png})"));}},Err(e)=>error.set(e)}busy.set(false);});}>"生成角色動畫"</button>
       <label>"名字"<input prop:value=move ||name.get() on:input=move |e|name.set(event_target_value(&e))/></label>
       <label>"種類"<input prop:value=move ||species.get() on:input=move |e|species.set(event_target_value(&e))/></label>
       <label>"喜好（逗號分隔）"<input prop:value=move ||likes.get() on:input=move |e|likes.set(event_target_value(&e))/></label>
       <label>"個性（逗號分隔）"<input prop:value=move ||personality.get() on:input=move |e|personality.set(event_target_value(&e))/></label>
       <label>"興趣（逗號分隔）"<input prop:value=move ||interests.get() on:input=move |e|interests.set(event_target_value(&e))/></label>
       <button on:click=move |_|{let id=pet.get();let profile=json!({"名字":name.get(),"種類":species.get(),"喜好":strings(likes.get()),"個性":strings(personality.get()),"興趣":strings(interests.get())});leptos::task::spawn_local(async move{match call("save_profile",json!({"id":id,"profile":profile})).await{Ok(_)=>refresh(),Err(e)=>error.set(e)}});}>"保存設定"</button></section>
       <section><h2>"AI 對話與教學"</h2><p>"回答會使用此寵物的設定、記憶與對話歷史並傳送至 OpenAI；可先修改或刪除不想使用的記憶。"</p><textarea prop:value=move ||prompt.get() on:input=move |e|prompt.set(event_target_value(&e)) placeholder="輸入問題或想學習的主題"></textarea><button disabled=move ||busy.get() on:click=move |_|{let pid=pet.get();let text=prompt.get();let rid=uuid();request.set(rid.clone());reply.set(String::new());busy.set(true);leptos::task::spawn_local(async move{match call("pet_talk",json!({"pet":pid,"request":rid,"text":text})).await{Ok(v)=>reply.set(v.as_str().unwrap_or("").to_owned()),Err(e)=>error.set(e)}busy.set(false);});}>"提問"</button><pre aria-live="polite">{move ||reply.get()}</pre></section>
    <section><h2>"可管理的記憶"</h2><textarea placeholder="新增記憶" prop:value=move ||memory.get() on:input=move |e|memory.set(event_target_value(&e))></textarea><button on:click=move |_|{let id=pet.get();let text=memory.get();leptos::task::spawn_local(async move{match call("save_memory",json!({"pet":id,"id":uuid(),"content":text})).await{Ok(_)=>{memory.set(String::new());load(id)},Err(e)=>error.set(e)}});}>"保存記憶"</button>
       {move ||memories.get().into_iter().map(|m|{let id=m["id"].as_str().unwrap_or("").to_owned();view!{<article>{m["content"].as_str().unwrap_or("").to_owned()}<button on:click=move |_|{let mid=id.clone();let pid=pet.get();leptos::task::spawn_local(async move{match call("delete_memory",json!({"pet":pid,"id":mid})).await{Ok(_)=>load(pid),Err(e)=>error.set(e)}});}>"刪除"</button></article>}}).collect_view()}</section>
       <section><h2>"主動回想與間隔複習"</h2><input placeholder="題目" on:input=move |e|question.set(event_target_value(&e))/><input placeholder="答案" on:input=move |e|answer.set(event_target_value(&e))/><button on:click=move |_|{let pid=pet.get();let q=question.get();let a=answer.get();leptos::task::spawn_local(async move{match call("add_card",json!({"pet":pid,"question":q,"answer":a})).await{Ok(_)=>load(pid),Err(e)=>error.set(e)}});}>"新增複習"</button>
       {move ||cards.get().into_iter().map(|c|{let id=c["id"].as_str().unwrap_or("").to_owned();view!{<article><p>{c["question"].as_str().unwrap_or("").to_owned()}</p><details><summary>"顯示答案"</summary>{c["answer"].as_str().unwrap_or("").to_owned()}</details><small>{format!("間隔 {} 天；下次複習 Unix 時間 {}",c["interval_days"],c["due_at"])}</small><button on:click=move |_|{let cid=id.clone();let pid=pet.get();leptos::task::spawn_local(async move{match call("review_card",json!({"pet":pid,"id":cid,"quality":5})).await{Ok(_)=>load(pid),Err(e)=>error.set(e)}});}>"成功回想"</button></article>}}).collect_view()}</section>
       </main>}
}
#[component]
fn PetOnly() -> impl IntoView {
    let sprite = RwSignal::new(String::new());
    let refresh = move || {
        leptos::task::spawn_local(async move {
            if let Ok(pet) = call("active_pet", json!({})).await
                && let Some(id) = pet.as_str()
                && let Ok(asset) = call("load_asset", json!({"pet":id})).await
                && let Some(png) = asset["png_base64"].as_str()
            {
                sprite.set(format!("url(data:image/png;base64,{png})"));
            }
        })
    };
    refresh();
    leptos::task::spawn_local(async move {
        let handler = Closure::<dyn FnMut(JsValue)>::new(move |_| refresh());
        let _ = wasm_bindgen_futures::JsFuture::from(listen(
            "active-pet-changed",
            handler.as_ref().unchecked_ref(),
        ))
        .await;
        handler.forget();
    });
    view! {<div class="floating" data-tauri-drag-region=""><div class="pet" class:animated=move ||!sprite.get().is_empty() style:background-image=move ||sprite.get()><Show when=move ||sprite.get().is_empty()><span>"🐾"</span></Show></div></div>}
}
#[component]
fn App() -> impl IntoView {
    if js_sys::Reflect::get(&current_window(), &"label".into())
        .ok()
        .and_then(|v| v.as_string())
        .as_deref()
        == Some("pet")
    {
        view! {<PetOnly/>}.into_any()
    } else {
        view! {<Settings/>}.into_any()
    }
}
fn main() {
    leptos::mount::mount_to_body(App);
}
