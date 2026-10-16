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
    #[wasm_bindgen(js_namespace=["window","__TAURI__","event"],js_name=emit)]
    fn emit_event(event: &str, payload: JsValue) -> js_sys::Promise;
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
fn fit_sprite(width: f64, height: f64) -> (f64, f64) {
    if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 {
        return (200.0, 200.0);
    }
    let scale = 200.0 / width.max(height);
    (width * scale, height * scale)
}
fn animation_y(value: Value, height: f64) -> String {
    let row = match value["action"].as_str().unwrap_or("idle") {
        "moving" => 1.0,
        "happy" => 2.0,
        "learning" => 3.0,
        _ => 0.0,
    };
    format!("{}px", -height * row)
}
#[component]
fn SpriteView(
    image: RwSignal<String>,
    animation: RwSignal<Value>,
    shape: RwSignal<(f64, f64)>,
) -> impl IntoView {
    view! {<div class="pet"><Show when=move ||!image.get().is_empty() fallback=||view!{<span>"🐾"</span>}><div class="sprite animated" style=move ||{let(w,h)=shape.get();format!("width:{w}px;height:{h}px;--frame-w:{w}px;--frame-h:{h}px;background-image:{};background-position-y:{};animation-duration:{}",image.get(),animation_y(animation.get(),h),animation_duration(animation.get()))}></div></Show></div>}
}
fn animation_duration(value: Value) -> String {
    match value["speed"].as_str().unwrap_or("normal") {
        "slow" => "2s",
        "fast" => "0.5s",
        _ => "1s",
    }
    .into()
}
#[component]
fn RecallPrompt() -> impl IntoView {
    let question = RwSignal::new(String::new());
    leptos::task::spawn_local(async move {
        let handler = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            if let Ok(raw) = js_sys::JSON::stringify(&v)
                && let Some(raw) = raw.as_string()
                && let Ok(value) = serde_json::from_str::<Value>(&raw)
                && let Some(q) = value["payload"]["question"].as_str()
            {
                question.set(q.into());
            }
        });
        let _ = wasm_bindgen_futures::JsFuture::from(listen(
            "recall-prompt",
            handler.as_ref().unchecked_ref(),
        ))
        .await;
        handler.forget();
    });
    let poll = move || {
        leptos::task::spawn_local(async move {
            if let Ok(v) = call("recall_prompt", json!({})).await {
                if let Some(q) = v["question"].as_str() {
                    question.set(q.into());
                    if let Ok(payload) = js_sys::JSON::parse(&v.to_string()) {
                        let _ = wasm_bindgen_futures::JsFuture::from(emit_event(
                            "recall-prompt",
                            payload,
                        ))
                        .await;
                    }
                }
            }
        })
    };
    let timer = leptos::leptos_dom::helpers::set_interval_with_handle(
        poll,
        std::time::Duration::from_secs(60),
    )
    .ok();
    on_cleanup(move || {
        if let Some(timer) = timer {
            timer.clear()
        }
    });
    view! {<Show when=move ||!question.get().is_empty()><aside class="recall-prompt" aria-live="polite"><small>"試著回想："{move ||question.get()}</small><button on:click=move |_|question.set(String::new())>"關閉"</button></aside></Show>}
}
#[component]
fn BackupControls() -> impl IntoView {
    let entries = RwSignal::new(Vec::<Value>::new());
    let selected = RwSignal::new(String::new());
    let confirmed = RwSignal::new(false);
    let status = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let refresh = move || {
        leptos::task::spawn_local(async move {
            match call("list_snapshots", json!({})).await {
                Ok(v) => entries.set(v.as_array().cloned().unwrap_or_default()),
                Err(e) => status.set(e),
            }
        })
    };
    view! {<div id="pet-backups"><h3>"本地備份與還原"</h3><p>"快照包含角色、記憶、複習、對話及生成素材。還原會先備份目前狀態，再恢復快照中的寵物；保留新增寵物、事件投遞與檔案操作收據。"</p>
    <button id="create-snapshot" disabled=move ||busy.get() on:click=move |_|{busy.set(true);leptos::task::spawn_local(async move {match call("create_snapshot",json!({})).await{Ok(v)=>{status.set(format!("備份完成 {}",v["id"]));refresh();},Err(e)=>status.set(e)}busy.set(false);});}>"建立快照"</button>
    <button id="list-snapshots" on:click=move |_|refresh()>"讀取快照"</button>
    <select prop:value=move ||selected.get() on:change=move |e|{selected.set(event_target_value(&e));confirmed.set(false);}>
    <option value="">"選擇快照"</option>{move ||entries.get().into_iter().map(|v|view!{<option value=v["id"].as_str().unwrap_or("").to_owned()>{format!("{} — {} bytes",v["id"],v["bytes"])}</option>}).collect_view()}</select>
    <label><input type="checkbox" prop:checked=move ||confirmed.get() on:change=move |e|confirmed.set(event_target_checked(&e))/>"確認恢復快照中的資料"</label>
    <button id="restore-snapshot" disabled=move ||busy.get()||!confirmed.get()||selected.get().is_empty() on:click=move |_|{let id=selected.get();busy.set(true);leptos::task::spawn_local(async move{match call("restore_snapshot",json!({"id":id,"confirmed":true})).await{Ok(v)=>{status.set(format!("還原完成；操作前快照 {}。請重新選擇寵物讀取資料。",v["id"]));refresh();},Err(e)=>status.set(e)}busy.set(false);});}>"還原快照"</button><pre aria-live="polite">{move ||status.get()}</pre></div>}
}
#[component]
fn UpdateControls() -> impl IntoView {
    let id = RwSignal::new(String::new());
    let verified = RwSignal::new(false);
    let confirmed = RwSignal::new(false);
    let busy = RwSignal::new(false);
    let status = RwSignal::new(String::new());
    view! {<div id="local-updater"><h3>"本地簽章更新"</h3><p>"檢查已設定的本地 channel，下載後驗證簽章與版本，再由你確認安裝。安裝前會建立資料快照。"</p>
    <button id="check-update" disabled=move ||busy.get() on:click=move |_|{busy.set(true);id.set(String::new());verified.set(false);confirmed.set(false);leptos::task::spawn_local(async move{match call("check_local_update",json!({})).await{Ok(v)=>{id.set(v["id"].as_str().unwrap_or("").into());status.set(v.to_string());},Err(e)=>status.set(e)}busy.set(false);});}>"檢查本地更新"</button>
    <button id="verify-update" disabled=move ||busy.get()||id.get().is_empty() on:click=move |_|{let current=id.get();busy.set(true);leptos::task::spawn_local(async move{match call("verify_local_update",json!({"id":current})).await{Ok(v)=>{verified.set(true);status.set(v.to_string());},Err(e)=>{verified.set(false);status.set(e)}}busy.set(false);});}>"下載並驗證"</button>
    <label><input type="checkbox" prop:checked=move ||confirmed.get() on:change=move |e|confirmed.set(event_target_checked(&e))/>"確認安裝已驗證的版本"</label>
    <button id="install-update" disabled=move ||busy.get()||!verified.get()||!confirmed.get() on:click=move |_|{let current=id.get();busy.set(true);leptos::task::spawn_local(async move{match call("install_local_update",json!({"id":current,"confirmed":true})).await{Ok(v)=>status.set(v.to_string()),Err(e)=>status.set(e)}busy.set(false);});}>"備份資料並安裝"</button><pre aria-live="polite">{move ||status.get()}</pre></div>}
}
#[component]
fn Settings() -> impl IntoView {
    let confirm_import = RwSignal::new(false);
    let profile_location = RwSignal::new(String::new());
    let sprite = RwSignal::new(String::new());
    let shape = RwSignal::new((200.0_f64, 200.0_f64));
    let animation = RwSignal::new(json!({"action":"idle","speed":"normal","proactive":false}));
    let roaming = RwSignal::new(false);
    let interaction_loading = RwSignal::new(false);
    let load_generation = RwSignal::new(0_u64);
    leptos::task::spawn_local(async move {
        if let Ok(v) = call("roaming_status", json!({})).await {
            roaming.set(v["enabled"].as_bool().unwrap_or(false));
        }
    });
    let prompt = RwSignal::new(String::new());
    let reply = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let allow_tools = RwSignal::new(false);
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
    let color = RwSignal::new(String::new());
    let style = RwSignal::new(String::new());
    let breed = RwSignal::new(String::new());
    let actions = RwSignal::new(String::new());
    let speed = RwSignal::new(String::new());
    let memory_edit_id = RwSignal::new(String::new());
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
    let load = move |id: String| {
        load_generation.update(|v| *v = v.wrapping_add(1));
        let generation = load_generation.get_untracked();
        interaction_loading.set(true);
        pet.set(id.clone());
        memories.set(Vec::new());
        cards.set(Vec::new());
        memory.set(String::new());
        memory_edit_id.set(String::new());
        request.set(uuid());
        reply.set(String::new());
        prompt.set(String::new());
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
                color.set(p["外觀"]["顏色"].as_str().unwrap_or("").into());
                style.set(p["外觀"]["風格"].as_str().unwrap_or("").into());
                breed.set(p["外觀"]["品種"].as_str().unwrap_or("").into());
                speed.set(p["動畫"]["速度"].as_str().unwrap_or("").into());
                actions.set(
                    p["動畫"]["動作"]
                        .as_array()
                        .map(|a| {
                            a.iter()
                                .filter_map(Value::as_str)
                                .collect::<Vec<_>>()
                                .join(",")
                        })
                        .unwrap_or_default(),
                );
            }
        }
        leptos::task::spawn_local(async move {
            let _ = call("select_pet", json!({"pet":id})).await;
            if pet.get_untracked() != id || load_generation.get_untracked() != generation {
                return;
            }
            if let Ok(asset) = call("load_asset", json!({"pet":id})).await
                && let Some(png) = asset["png_base64"].as_str()
            {
                if pet.get_untracked() != id || load_generation.get_untracked() != generation {
                    return;
                }
                shape.set(fit_sprite(
                    asset["width"].as_f64().unwrap_or(200.0),
                    asset["height"].as_f64().unwrap_or(200.0),
                ));
                sprite.set(format!("url(data:image/png;base64,{png})"));
            }
            if pet.get_untracked() != id || load_generation.get_untracked() != generation {
                return;
            }
            if let Ok(v) = call("interaction", json!({"pet":id})).await {
                if pet.get_untracked() == id && load_generation.get_untracked() == generation {
                    animation.set(v)
                }
            }
            if load_generation.get_untracked() == generation {
                interaction_loading.set(false);
            }
            match call("list_memories", json!({"pet":id})).await {
                Ok(v)
                    if pet.get_untracked() == id
                        && load_generation.get_untracked() == generation =>
                {
                    memories.set(v.as_array().cloned().unwrap_or_default())
                }
                Ok(_) => {}
                Err(e)
                    if pet.get_untracked() == id
                        && load_generation.get_untracked() == generation =>
                {
                    error.set(e)
                }
                Err(_) => {}
            }
            if let Ok(value) = call("profile_file", json!({"pet":id})).await
                && pet.get_untracked() == id
                && load_generation.get_untracked() == generation
            {
                profile_location.set(value.as_str().unwrap_or("").to_owned());
            }
            match call("list_cards", json!({"pet":id})).await {
                Ok(v)
                    if pet.get_untracked() == id
                        && load_generation.get_untracked() == generation =>
                {
                    cards.set(v.as_array().cloned().unwrap_or_default())
                }
                Ok(_) => {}
                Err(e)
                    if pet.get_untracked() == id
                        && load_generation.get_untracked() == generation =>
                {
                    error.set(e)
                }
                Err(_) => {}
            }
        });
    };
    leptos::task::spawn_local(async move {
        if let Ok(v) = call("list_pets", json!({})).await {
            pets.set(v.as_array().cloned().unwrap_or_default());
            if pet.get_untracked().is_empty()
                && let Ok(active) = call("active_pet", json!({})).await
                && let Some(id) = active.as_str()
            {
                load(id.to_owned());
            }
        }
    });
    let interaction_busy = RwSignal::new(false);
    let roaming_busy = RwSignal::new(false);
    let apply_interaction = move |field: &'static str, value: Value| {
        if interaction_busy.get_untracked()
            || interaction_loading.get_untracked()
            || pet.get_untracked().is_empty()
        {
            return;
        }
        let id = pet.get_untracked();
        let mut preferences = animation.get_untracked();
        preferences[field] = value;
        interaction_busy.set(true);
        leptos::task::spawn_local(async move {
            match call(
                "set_interaction",
                json!({"pet":id,"preferences":preferences}),
            )
            .await
            {
                Ok(_) if pet.get_untracked() == id => {
                    animation.set(preferences);
                    error.set(String::new());
                }
                Err(e) => error.set(e),
                _ => {}
            }
            interaction_busy.set(false);
        });
    };
    view! {<main><h1>"桌面 AI 寵物"</h1><p class="error" role="alert">{move ||error.get()}</p>
       <section><h2>"我的寵物"</h2><select id="pet-selector" prop:value=move ||pet.get() on:change=move |e|load(event_target_value(&e))><option value="">"選擇寵物"</option>{move ||pets.get().into_iter().map(|p|{let id=p["id"].as_str().unwrap_or("").to_owned();let profile:Value=serde_json::from_str(p["profile"].as_str().unwrap_or("{}")).unwrap_or_default();view!{<option value=id>{profile["名字"].as_str().unwrap_or("寵物").to_owned()}</option>}}).collect_view()}</select>
       <button on:click=move |_|{pet.set(uuid());name.set(String::new());species.set(String::new());likes.set(String::new());personality.set(String::new());interests.set(String::new());color.set(String::new());style.set(String::new());breed.set(String::new());actions.set(String::new());speed.set(String::new());memory.set(String::new());memory_edit_id.set(String::new());memories.set(Vec::new());cards.set(Vec::new());}>"新增寵物"</button>
       <SpriteView image=sprite animation=animation shape=shape/>
    <small>{move ||if sprite.get().is_empty() {"200×200 顯示區；目前設定尚無可用素材"}else{"200×200 角色動畫；素材已快取"}}</small>
    <label>"目前動作"<select id="pet-action" disabled=move ||interaction_busy.get()||interaction_loading.get()||pet.get().is_empty() prop:value=move ||animation.get()["action"].as_str().unwrap_or("idle").to_owned() on:change=move |e|apply_interaction("action",json!(event_target_value(&e)))><option value="idle">"待機"</option><option value="moving">"移動"</option><option value="happy">"開心"</option><option value="learning">"學習"</option></select></label>
    <label>"播放速度"<select id="pet-speed" disabled=move ||interaction_busy.get()||interaction_loading.get()||pet.get().is_empty() prop:value=move ||animation.get()["speed"].as_str().unwrap_or("normal").to_owned() on:change=move |e|apply_interaction("speed",json!(event_target_value(&e)))><option value="slow">"慢"</option><option value="normal">"正常"</option><option value="fast">"快"</option></select></label>
    <label><input id="pet-proactive" type="checkbox" disabled=move ||interaction_busy.get()||interaction_loading.get()||pet.get().is_empty() prop:checked=move ||animation.get()["proactive"]==true on:change=move |e|apply_interaction("proactive",json!(event_target_checked(&e)))/>"主動顯示本地到期複習題（每小時最多一次）"</label>
    <label><input id="pet-roaming" type="checkbox" disabled=move ||roaming_busy.get() prop:checked=move ||roaming.get() on:change=move |e|{let enabled=event_target_checked(&e);roaming_busy.set(true);leptos::task::spawn_local(async move{match call("set_roaming",json!({"enabled":enabled})).await{Ok(_)=>{roaming.set(enabled);error.set(String::new());},Err(e)=>error.set(e)}roaming_busy.set(false);});}/>"啟用原生視窗漫遊（動作須選移動）"</label>
    <p>"動作、速度與巡行開關即時生效並自動保存，不需重新啟動。"</p>
    <button id="save-interaction" disabled=move ||interaction_busy.get()||interaction_loading.get()||pet.get().is_empty() on:click=move |_|{let pid=pet.get();let preferences=animation.get();leptos::task::spawn_local(async move{if let Err(e)=call("set_interaction",json!({"pet":pid,"preferences":preferences})).await {error.set(e)}});}>"保存互動設定"</button>
    <RecallPrompt/>
    <PetArchive pet=pet changed=Callback::new(move |id:Option<String>|{refresh();if let Some(id)=id{load(id)}else{pet.set(uuid());sprite.set(String::new());name.set(String::new());memories.set(Vec::new());cards.set(Vec::new());}})/>
    <BackupControls/>
    <UpdateControls/>
    <button disabled=move ||busy.get() on:click=move |_|{let pid=pet.get();busy.set(true);leptos::task::spawn_local(async move{match call("generate_asset",json!({"pet":pid})).await{Ok(v)=>{if pet.get_untracked()==pid && let Some(png)=v["png_base64"].as_str(){shape.set(fit_sprite(v["width"].as_f64().unwrap_or(200.0),v["height"].as_f64().unwrap_or(200.0)));sprite.set(format!("url(data:image/png;base64,{png})"));}},Err(e)=>error.set(e)}busy.set(false);});}>"生成角色動畫"</button>
       <label>"名字"<input prop:value=move ||name.get() on:input=move |e|name.set(event_target_value(&e))/></label>
       <label>"種類"<input prop:value=move ||species.get() on:input=move |e|species.set(event_target_value(&e))/></label>
       <label>"喜好（逗號分隔）"<input prop:value=move ||likes.get() on:input=move |e|likes.set(event_target_value(&e))/></label>
       <label>"個性（逗號分隔）"<input prop:value=move ||personality.get() on:input=move |e|personality.set(event_target_value(&e))/></label>
       <label>"興趣（逗號分隔）"<input prop:value=move ||interests.get() on:input=move |e|interests.set(event_target_value(&e))/></label>
       <details><summary>"選用外觀與動畫覆寫"</summary><p>"留白由五項設定決定。生成新素材時才套用，不會自動發送生圖請求。"</p>
        <label>"顏色"<input maxlength="256" prop:value=move ||color.get() on:input=move |e|color.set(event_target_value(&e))/></label>
        <label>"風格"<input maxlength="256" prop:value=move ||style.get() on:input=move |e|style.set(event_target_value(&e))/></label>
        <label>"品種"<input maxlength="256" prop:value=move ||breed.get() on:input=move |e|breed.set(event_target_value(&e))/></label>
        <label>"動作（逗號分隔）"<input prop:value=move ||actions.get() on:input=move |e|actions.set(event_target_value(&e))/></label>
        <label>"速度描述"<input maxlength="256" prop:value=move ||speed.get() on:input=move |e|speed.set(event_target_value(&e))/></label></details>
       <button on:click=move |_|{let id=pet.get();let optional=|s:String|if s.trim().is_empty(){Value::Null}else{json!(s)};let profile=json!({"名字":name.get(),"種類":species.get(),"喜好":strings(likes.get()),"個性":strings(personality.get()),"興趣":strings(interests.get()),"外觀":{"顏色":optional(color.get()),"風格":optional(style.get()),"品種":optional(breed.get())},"動畫":{"動作":if actions.get().trim().is_empty(){Value::Null}else{json!(strings(actions.get()))},"速度":optional(speed.get())}});leptos::task::spawn_local(async move{match call("save_profile",json!({"id":id,"profile":profile})).await{Ok(_)=>{refresh();load(id)},Err(e)=>error.set(e)}});}>"保存設定"</button></section>
       <section><h2>"本機設定檔"</h2><p>"每隻寵物均有獨立資料夾與 pet.toml。手動編輯五項設定後，可匯入目前選擇的寵物；不會更動其他寵物。"</p>
       <p id="profile-location">{move ||profile_location.get()}</p><label><input id="confirm-profile-import" type="checkbox" on:change=move |e|confirm_import.set(event_target_checked(&e))/>"確認使用目前寵物的本機設定檔"</label>
       <button id="import-pet-profile" disabled=move ||!confirm_import.get() on:click=move |_|{let id=pet.get();leptos::task::spawn_local(async move{match call("import_pet_profile",json!({"pet":id,"confirmed":true})).await{Ok(_)=>{refresh();load(id);confirm_import.set(false)},Err(e)=>error.set(e)}});}>"匯入 pet.toml"</button></section>
       <section><h2>"AI 對話與教學"</h2><p>"回答會使用此寵物的設定、記憶與對話歷史並傳送至 OpenAI；可先修改或刪除不想使用的記憶。"</p><textarea prop:value=move ||prompt.get() on:input=move |e|prompt.set(event_target_value(&e)) placeholder="輸入問題或想學習的主題"></textarea><button disabled=move ||busy.get() on:click=move |_|{let pid=pet.get();let text=prompt.get();let rid=uuid();request.set(rid.clone());reply.set(String::new());busy.set(true);leptos::task::spawn_local(async move{match call("pet_talk",json!({"pet":pid,"request":rid,"text":text})).await{Ok(v)=>{if pet.get_untracked()==pid && request.get_untracked()==rid {reply.set(v.as_str().unwrap_or("").to_owned());}},Err(e)=>{if pet.get_untracked()==pid {error.set(e);}}}busy.set(false);});}>"提問"</button><pre aria-live="polite">{move ||reply.get()}</pre>
    <label><input type="checkbox" on:change=move |e|allow_tools.set(event_target_checked(&e))/>"允許唯讀記憶檢索工具，將相關記憶交給 OpenAI 回答"</label>
    <button disabled=move ||busy.get() ||!allow_tools.get() on:click=move |_| {
        let pid=pet.get();let text=prompt.get();request.set(uuid());busy.set(true);reply.set("正在用本機記憶檢索工具…".into());
        leptos::task::spawn_local(async move{match call("pet_agent",json!({"pet":pid,"text":text})).await {
            Ok(v)=>{if pet.get_untracked()==pid {reply.set(format!("{}\n理解問題：{}\n來源：{}\n工具：{}",v["answer"].as_str().unwrap_or(""),v["follow_up_question"].as_str().unwrap_or(""),v["sources"],v["steps"]));}},Err(e)=>{if pet.get_untracked()==pid {error.set(e);}}
        }busy.set(false);});
    }>"執行記憶 Agent"</button></section>
    <section><h2>"可管理的記憶"</h2><textarea placeholder="新增記憶" prop:value=move ||memory.get() on:input=move |e|memory.set(event_target_value(&e))></textarea><button on:click=move |_|{let id=pet.get();let text=memory.get();let mid=if memory_edit_id.get().is_empty(){uuid()}else{memory_edit_id.get()};leptos::task::spawn_local(async move{match call("save_memory",json!({"pet":id,"id":mid,"content":text})).await{Ok(_)=>{memory.set(String::new());if pet.get_untracked()==id {load(id)}},Err(e)=>error.set(e)}});}>"保存記憶"</button><button on:click=move |_|{memory_edit_id.set(String::new());memory.set(String::new());}>"新增／取消編輯"</button>
       {move ||memories.get().into_iter().map(|m|{let id=m["id"].as_str().unwrap_or("").to_owned();let edit_id=id.clone();let edit_text=m["content"].as_str().unwrap_or("").to_owned();view!{<article>{m["content"].as_str().unwrap_or("").to_owned()}<button on:click=move |_|{memory_edit_id.set(edit_id.clone());memory.set(edit_text.clone());}>"編輯"</button><button on:click=move |_|{let mid=id.clone();let pid=pet.get();leptos::task::spawn_local(async move{match call("delete_memory",json!({"pet":pid,"id":mid})).await{Ok(_)=>{if pet.get_untracked()==pid {load(pid)}},Err(e)=>error.set(e)}});}>"刪除"</button></article>}}).collect_view()}</section>
       <section><h2>"主動回想與間隔複習"</h2><input placeholder="題目" on:input=move |e|question.set(event_target_value(&e))/><input placeholder="答案" on:input=move |e|answer.set(event_target_value(&e))/><button on:click=move |_|{let pid=pet.get();let q=question.get();let a=answer.get();leptos::task::spawn_local(async move{match call("add_card",json!({"pet":pid,"question":q,"answer":a})).await{Ok(_)=>{if pet.get_untracked()==pid {load(pid)}},Err(e)=>error.set(e)}});}>"新增複習"</button>
       {move ||cards.get().into_iter().map(|c|{let id=c["id"].as_str().unwrap_or("").to_owned();let failed=id.clone();let difficult=id.clone();view!{<article><p>{c["question"].as_str().unwrap_or("").to_owned()}</p><details><summary>"顯示答案"</summary>{c["answer"].as_str().unwrap_or("").to_owned()}</details><small>{format!("間隔 {} 天；下次複習 Unix 時間 {}",c["interval_days"],c["due_at"])}</small><button on:click=move |_|{let cid=id.clone();let pid=pet.get();leptos::task::spawn_local(async move{match call("review_card",json!({"pet":pid,"id":cid,"quality":5})).await{Ok(_)=>{if pet.get_untracked()==pid {load(pid)}},Err(e)=>error.set(e)}});}>"成功回想"</button>
        <button on:click=move |_|{let cid=difficult.clone();let pid=pet.get();leptos::task::spawn_local(async move{match call("review_card",json!({"pet":pid,"id":cid,"quality":3})).await{Ok(_)=>{if pet.get_untracked()==pid {load(pid)}},Err(e)=>error.set(e)}});}>"想起來但困難"</button>
        <button on:click=move |_|{let cid=failed.clone();let pid=pet.get();leptos::task::spawn_local(async move{match call("review_card",json!({"pet":pid,"id":cid,"quality":0})).await{Ok(_)=>{if pet.get_untracked()==pid {load(pid)}},Err(e)=>error.set(e)}});}>"忘記了，重新練習"</button></article>}}).collect_view()}</section>
       <SuggestionPolicy pet=pet/>
       <FileOrganizer/>
       </main>}
}
#[component]
fn SuggestionPolicy(pet: RwSignal<String>) -> impl IntoView {
    let id = RwSignal::new(String::new());
    let confirmed = RwSignal::new(false);
    let result = RwSignal::new(String::new());
    view! {<section><h2>"已審核的設定建議"</h2><p>"先在 Analytics 檢查證據並核准。套用只調整目前寵物成功回想的最短複習間隔；回退會檢查是否有之後的變更。"</p>
    <input placeholder="建議 UUID" prop:value=move ||id.get() on:input=move |e|id.set(event_target_value(&e))/>
    <label><input type="checkbox" on:change=move |e|confirmed.set(event_target_checked(&e))/>"確認變更目前寵物設定"</label>
    <button disabled=move ||!confirmed.get() on:click=move |_|{let args=json!({"pet":pet.get(),"id":id.get(),"confirmed":true});leptos::task::spawn_local(async move{match call("apply_analytics_suggestion",args).await{Ok(v)=>result.set(v.to_string()),Err(e)=>result.set(e)}});}>"套用並回報"</button>
    <button disabled=move ||!confirmed.get() on:click=move |_|{let args=json!({"pet":pet.get(),"id":id.get(),"confirmed":true});leptos::task::spawn_local(async move{match call("rollback_analytics_suggestion",args).await{Ok(v)=>result.set(v.to_string()),Err(e)=>result.set(e)}});}>"回退並回報"</button><pre aria-live="polite">{move ||result.get()}</pre></section>}
}
#[component]
fn FileOrganizer() -> impl IntoView {
    let root_path = RwSignal::new(String::new());
    let authorized = RwSignal::new(false);
    let files = RwSignal::new(Vec::<Value>::new());
    let status = RwSignal::new(String::new());
    leptos::task::spawn_local(async move {
        if let Ok(v) = call("authorized_directory", json!({})).await {
            if let Some(path) = v.as_str() {
                root_path.set(path.into());
            }
        }
    });
    let refresh = move |notice: bool| {
        leptos::task::spawn_local(async move {
            match call("scan_file_preview", json!({})).await {
                Ok(v) => {
                    files.set(v.as_array().cloned().unwrap_or_default());
                    if notice {
                        status.set("預覽完成，尚未移動檔案".into());
                    }
                }
                Err(e) => status.set(e),
            }
        })
    };
    view! {<section><h2>"檔案整理"</h2><p>"只處理你設定的授權目錄。每個檔案先預覽並確認，移到隔離區。"</p>
    <label>"本機目錄的絕對路徑"<input id="authorized-root" maxlength="4096" prop:value=move ||root_path.get() on:input=move |e|{root_path.set(event_target_value(&e));authorized.set(false);}/></label>
    <label><input id="authorize-root-confirm" type="checkbox" on:change=move |e|authorized.set(event_target_checked(&e))/>"我授權預覽與整理此目錄"</label>
    <button disabled=move ||!authorized.get() on:click=move |_|{let path=root_path.get();leptos::task::spawn_local(async move{
        match call("authorize_directory",json!({"path":path,"confirmed":true})).await {Ok(v)=>{root_path.set(v.as_str().unwrap_or("").into());files.set(Vec::new());status.set("目錄已授權；請先預覽檔案".into());},Err(e)=>status.set(e)}
    });}>"使用此目錄"</button>
    <button on:click=move |_|leptos::task::spawn_local(async move{match call("revoke_directory",json!({})).await{Ok(_)=>{files.set(Vec::new());authorized.set(false);status.set("已撤銷目錄授權".into());},Err(e)=>status.set(e)}})>"撤銷目錄授權"</button>
    <button id="scan-duplicates" on:click=move |_|leptos::task::spawn_local(async move{match call("scan_duplicate_preview",json!({})).await{
        Ok(v)=>{let groups=v.as_array().cloned().unwrap_or_default();let count=groups.len();files.set(groups.into_iter().flat_map(|group|group.as_array().cloned().unwrap_or_default()).collect());status.set(format!("SHA256 重複群組：{count}；尚未移動，需逐檔確認"));},Err(e)=>status.set(e)
    }})>"SHA256 重複檔預覽"</button>
    <button id="scan-files" on:click=move |_|refresh(true)>"預覽授權目錄"</button>
    <button on:click=move |_|leptos::task::spawn_local(async move {
        match call("outbox_status",json!({})).await {
            Ok(v)=>status.set(format!("事件：已送達 {}、待送出 {}、投遞中 {}、需要重試 {}",v["delivered"],v["pending"],v["leased"],v["blocked"])),Err(e)=>status.set(e),
        }
    })>"查看事件投遞"</button>
    <button on:click=move |_|leptos::task::spawn_local(async move {
        match call("requeue_failed_events",json!({})).await {Ok(v)=>status.set(format!("已排入 {} 筆失敗事件重試",v)),Err(e)=>status.set(e)}
    })>"重試失敗事件"</button>
    <button id="recover-receipts" on:click=move |_|leptos::task::spawn_local(async move {
        match call("recover_cleanup_receipts",json!({})).await {
            Ok(v)=>status.set(format!("已核對 {} 筆收據與事件",v.as_array().map_or(0,Vec::len))),
            Err(e)=>status.set(e),
        }
    })>"復原收據與事件"</button><p role="status">{move ||status.get()}</p>
    <TrashBrowser/>
    {move ||files.get().into_iter().map(|item| {
        let id=item["id"].as_str().unwrap_or("").to_owned();
        let confirmed=RwSignal::new(false);let working=RwSignal::new(false);
        view!{<article><small>{item["duplicate_group"].as_u64().map(|n|format!("SHA256 重複群組 {n}")).unwrap_or_default()}</small><p>{item["name"].as_str().unwrap_or("").to_owned()}{format!("（{} bytes）",item["size_bytes"])}</p>
        <label><input type="checkbox" on:change=move |e|confirmed.set(event_target_checked(&e))/>"我確認移到隔離區"</label>
        <button disabled=move ||!confirmed.get() ||working.get() on:click=move |_| {
            working.set(true);let id=id.clone();
            leptos::task::spawn_local(async move {
                match call("confirm_cleanup",json!({"id":id,"confirmed":true})).await {
                    Ok(v)=>{status.set(format!("已隔離；收據 {}",v["operation_id"].as_str().unwrap_or("")));refresh(false);}
                    Err(e)=>status.set(e),
                }
                working.set(false);
            });
        }>"確認隔離"</button></article>}
    }).collect_view()}</section>}
}
#[component]
fn TrashBrowser() -> impl IntoView {
    let items = RwSignal::new(Vec::<Value>::new());
    let status = RwSignal::new(String::new());
    let refresh = move || {
        leptos::task::spawn_local(async move {
            match call("list_trash", json!({})).await {
                Ok(v) => items.set(v.as_array().cloned().unwrap_or_default()),
                Err(e) => status.set(e),
            }
        })
    };
    view! {<div id="trash-browser"><h3>"本程式回收區"</h3><p>"檔案保留在授權目錄內的私人回收區。還原只回到原名稱；有同名檔案時拒絕覆寫。撤銷授權後不能還原。"</p>
    <button id="load-trash" on:click=move |_|refresh()>"查看回收區"</button>
    <button on:click=move |_|leptos::task::spawn_local(async move{match call("recover_restore_receipts",json!({})).await{Ok(v)=>status.set(format!("已復原 {} 筆還原事件",v.as_array().map_or(0,Vec::len))),Err(e)=>status.set(e)}})>"復原還原收據"</button><p role="status">{move ||status.get()}</p>
    {move ||items.get().into_iter().map(|item|{
        let id=item["operation_id"].as_str().unwrap_or("").to_owned();let restored=item["restored"].as_bool().unwrap_or(false);let confirmed=RwSignal::new(false);let busy=RwSignal::new(false);
        view!{<article><p>{item["file_name"].as_str().unwrap_or("").to_owned()}{format!("（{} bytes）",item["size_bytes"])}</p><small>{if restored{"已還原"}else{"在回收區"}}</small>
        <label><input type="checkbox" disabled=restored on:change=move |e|confirmed.set(event_target_checked(&e))/>"確認還原至原名稱"</label>
        <button disabled=move ||restored||!confirmed.get()||busy.get() on:click=move |_|{let id=id.clone();busy.set(true);leptos::task::spawn_local(async move{match call("restore_file",json!({"id":id,"confirmed":true})).await{Ok(v)=>{status.set(format!("已還原；收據 {}",v["operation_id"]));refresh();},Err(e)=>status.set(e)}busy.set(false);});}>"還原檔案"</button></article>}
    }).collect_view()}</div>}
}
#[component]
fn PetArchive(pet: RwSignal<String>, changed: Callback<Option<String>>) -> impl IntoView {
    let confirmed = RwSignal::new(false);
    let items = RwSignal::new(Vec::<Value>::new());
    let status = RwSignal::new(String::new());
    let refresh = move || {
        leptos::task::spawn_local(async move {
            match call("archived_pets", json!({})).await {
                Ok(v) => items.set(v.as_array().cloned().unwrap_or_default()),
                Err(e) => status.set(e),
            }
        })
    };
    refresh();
    view! {<section id="pet-archive"><h2>"移除與還原寵物"</h2><p>"移除後不再顯示或使用此寵物回答。設定、記憶與素材保留在封存區，其他寵物不受影響，可隨時還原。"</p>
    <label><input id="confirm-pet-archive" type="checkbox" on:change=move |e|confirmed.set(event_target_checked(&e))/>"確認將目前寵物移到封存區"</label>
    <button id="archive-pet" disabled=move ||!confirmed.get() on:click=move |_|{let id=pet.get();leptos::task::spawn_local(async move{match call("set_pet_archived",json!({"pet":id,"archived":true,"confirmed":true})).await{Ok(_)=>{status.set("已移除，可從封存區還原".into());changed.run(None);refresh();},Err(e)=>status.set(e)}});}>"移除目前寵物"</button>
    <button id="load-archived-pets" on:click=move |_|refresh()>"查看封存區"</button><p role="status">{move ||status.get()}</p>
    {move ||items.get().into_iter().map(|item|{let id=item["id"].as_str().unwrap_or("").to_owned();let profile:Value=serde_json::from_str(item["profile"].as_str().unwrap_or("{}")).unwrap_or_default();let restore_confirm=RwSignal::new(false);let attribute_id=id.clone();
    view!{<article data-pet=attribute_id><p>{profile["名字"].as_str().unwrap_or("寵物").to_owned()}</p><label><input type="checkbox" on:change=move |e|restore_confirm.set(event_target_checked(&e))/>"確認還原此寵物"</label>
    <button disabled=move ||!restore_confirm.get() on:click=move |_|{let id=id.clone();leptos::task::spawn_local(async move{match call("set_pet_archived",json!({"pet":id,"archived":false,"confirmed":true})).await{Ok(_)=>{status.set("已還原寵物與原有資料".into());changed.run(Some(id));refresh();},Err(e)=>status.set(e)}});}>"還原寵物"</button></article>}}).collect_view()}</section>}
}
#[component]
fn PetOnly() -> impl IntoView {
    let menu_open = RwSignal::new(false);
    let menu_error = RwSignal::new(String::new());
    let sprite = RwSignal::new(String::new());
    let shape = RwSignal::new((200.0_f64, 200.0_f64));
    let animation = RwSignal::new(json!({"action":"idle","speed":"normal","proactive":false}));
    let generation = RwSignal::new(0_u32);
    let refresh = move || {
        generation.update(|v| *v = v.wrapping_add(1));
        let version = generation.get_untracked();
        sprite.set(String::new());
        leptos::task::spawn_local(async move {
            if let Ok(pet) = call("active_pet", json!({})).await
                && let Some(id) = pet.as_str()
            {
                if generation.get_untracked() != version {
                    return;
                }
                if let Ok(asset) = call("load_asset", json!({"pet":id})).await
                    && let Some(png) = asset["png_base64"].as_str()
                    && generation.get_untracked() == version
                {
                    shape.set(fit_sprite(
                        asset["width"].as_f64().unwrap_or(200.0),
                        asset["height"].as_f64().unwrap_or(200.0),
                    ));
                    sprite.set(format!("url(data:image/png;base64,{png})"));
                }
                if let Ok(v) = call("interaction", json!({"pet":id})).await {
                    if generation.get_untracked() == version {
                        animation.set(v)
                    }
                }
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
    view! {<div class="floating" data-tauri-drag-region="" on:contextmenu=move |e|{e.prevent_default();e.stop_propagation();leptos::task::spawn_local(async move{match call("set_menu_open",json!({"open":true})).await{Ok(_)=>menu_open.set(true),Err(e)=>menu_error.set(e)}});}>
    <SpriteView image=sprite animation=animation shape=shape/><RecallPrompt/>
    <Show when=move ||menu_open.get()><nav id="pet-context-menu" aria-label="寵物控制選單">
    <button id="open-controls" on:click=move |_|{leptos::task::spawn_local(async move{match call("show_controls",json!({})).await{Ok(_)=>{let _=call("set_menu_open",json!({"open":false})).await;menu_open.set(false);},Err(e)=>menu_error.set(e)}});}>"開啟控制面板"</button>
    <button on:click=move |_|{leptos::task::spawn_local(async move{let _=call("set_menu_open",json!({"open":false})).await;menu_open.set(false);});}>"關閉選單"</button>
    <button id="quit-pet" on:click=move |_|{leptos::task::spawn_local(async move{let _=call("quit_pet",json!({})).await;});}>"結束程式"</button>
    <span role="alert">{move ||menu_error.get()}</span></nav></Show>
    </div>}
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nonsquare_sheets_keep_aspect_inside_fixed_viewport_and_invalid_dimensions_fall_back() {
        assert_eq!(fit_sprite(2048.0, 1536.0), (200.0, 150.0));
        assert_eq!(fit_sprite(1024.0, 2048.0), (100.0, 200.0));
        assert_eq!(fit_sprite(0.0, 0.0), (200.0, 200.0));
        assert_eq!(fit_sprite(f64::INFINITY, 200.0), (200.0, 200.0));
        assert_eq!(animation_y(json!({"action":"learning"}), 150.0), "-450px");
    }
}
