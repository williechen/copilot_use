//! 房間列表與各人的管理區；管理 API 仍依 Session／房主身分逐項授權。
use super::*;
#[component]
pub(super) fn RoomLobby() -> impl IntoView {
    let state = session();
    let rooms = RwSignal::new(Vec::<Value>::new());
    let loading = RwSignal::new(true);
    Effect::new(move |_| {
        spawn(async move {
            state.error.set(String::new());
            match api("/api/v1/rooms", "GET", None).await {
                Ok(v) => rooms.set(v.as_array().cloned().unwrap_or_default()),
                Err(e) => state.error.set(e),
            }
            loading.set(false);
        })
    });
    view! {<section id="room-lobby"><h2>"房間列表"</h2><p>"選擇可進入的房間開始聊天；建立與設定自己的房間請到「我的管理」。"</p><A href="/app/manage">"前往我的管理"</A>
    <Show when=move ||loading.get()><p role="status">"正在讀取房間…"</p></Show>
    <Show when=move ||!loading.get()&&rooms.get().is_empty()><p>"目前沒有可進入的房間。你可以在我的管理建立房間，或請房主邀請你。"</p></Show>
    <ul id="room-list">{move ||rooms.get().into_iter().map(|r|{let id=r["id"].as_str().unwrap_or("").to_owned();let name=r["name"].as_str().unwrap_or("").to_owned();view!{<li><A href=format!("/app/chat?room={id}")>{format!("進入：{name}")}</A></li>}}).collect_view()}</ul></section>}.into_any()
}

fn audit_time(ms: i64) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms as f64))
            .to_iso_string()
            .as_string()
            .unwrap_or_default()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        ms.to_string()
    }
}
#[component]
pub(super) fn MyManagement() -> impl IntoView {
    let state = session();
    let rooms = RwSignal::new(Vec::<Value>::new());
    let selected = RwSignal::new(String::new());
    let room_name = RwSignal::new(String::new());
    let creating = RwSignal::new(false);
    let new_private = RwSignal::new(false);
    let notice = RwSignal::new(String::new());
    let activity = RwSignal::new(Vec::<Value>::new());
    let members = RwSignal::new(Vec::<Value>::new());
    let member_epoch = RwSignal::new(0_u64);
    let refresh = move || {
        spawn(async move {
            match api("/api/v1/me/rooms", "GET", None).await {
                Ok(v) => rooms.set(v.as_array().cloned().unwrap_or_default()),
                Err(e) => state.error.set(e),
            }
            match api("/api/v1/me/activity", "POST", Some(json!({}))).await {
                Ok(v) => activity.set(v.as_array().cloned().unwrap_or_default()),
                Err(e) => state.error.set(e),
            }
        })
    };
    Effect::new(move |_| {
        state.error.set(String::new());
        refresh();
    });
    Effect::new(move |_| {
        let id = selected.get();
        let _ = member_epoch.get();
        members.set(Vec::new());
        if id.is_empty() {
            return;
        }
        spawn(async move {
            match api(&format!("/api/v1/me/rooms/{id}/members"), "GET", None).await {
                Ok(v) => {
                    if selected.get_untracked() == id {
                        members.set(v.as_array().cloned().unwrap_or_default());
                    }
                }
                Err(e) => state.error.set(e),
            }
        });
    });
    let agent_question = RwSignal::new(String::new());
    let agent_result = RwSignal::new(String::new());
    let allow_tools = RwSignal::new(false);
    let agent_busy = RwSignal::new(false);
    let invitee = RwSignal::new(String::new());
    let create = move || {
        if creating.get_untracked() {
            return;
        }
        let name = room_name.get_untracked().trim().to_owned();
        if name.is_empty() || name.len() > 200 {
            state
                .error
                .set("請填寫房間名稱；名稱太長時請縮短後重試。".into());
            return;
        }
        let id = uuid();
        if id.is_empty() {
            state.error.set("無法建立安全房間 ID".into());
            return;
        }
        state.error.set(String::new());
        creating.set(true);
        spawn(async move {
            match api(
                "/api/v1/rooms",
                "POST",
                Some(json!({"id":id,"name":name,"is_private":new_private.get_untracked()})),
            )
            .await
            {
                Ok(_) => {
                    room_name.set(String::new());
                    // Use the confirmed new room immediately; a failed list refresh must not hide it.
                    rooms
                        .update(|items| items.push(json!({"id":id,"name":name,"can_manage":true,"is_private":new_private.get_untracked()})));
                    selected.set(id);
                    notice.set("房間已建立，可回房間列表進入聊天。".into());
                    refresh();
                }
                Err(e) => state.error.set(format!("建立房間失敗：{e}")),
            }
            creating.set(false);
        });
    };

    view! {<section id="my-management"><h2>"我的管理"</h2><p>"在這裡建立與管理自己的房間；AI 角色與對話設定由系統管理者負責。"</p><A href="/app">"回房間列表"</A><p class="success" role="status">{move ||notice.get()}</p>
           <form id="create-room-form" on:submit=move |e|{e.prevent_default();create();}>
                 <label for="room-name">"新房間名稱"</label><input id="room-name" placeholder="輸入名稱後按 Enter 建立" maxlength="200" prop:value=move ||room_name.get() on:input=move |e|room_name.set(event_target_value(&e))/><label><input type="checkbox" prop:checked=move ||new_private.get() on:change=move |e|new_private.set(event_target_checked(&e))/>"私聊房間（需要邀請）"</label><button id="create-room" type="submit" disabled=move ||creating.get()>{move ||if creating.get(){"建立中…"}else{"建立房間"}}</button></form>
    <h3>"我建立的房間"</h3><ul id="owned-rooms">{move ||rooms.get().into_iter().map(|r|{let id=r["id"].as_str().unwrap_or("").to_owned();let name=r["name"].as_str().unwrap_or("").to_owned();view!{<li><A href=format!("/app/chat?room={id}")>{name}</A>{if r["is_private"]==true{"（私聊）"}else{"（公開）"}}</li>}}).collect_view()}</ul>
    <label>"選擇自己的房間"<select id="managed-room" prop:value=move ||selected.get() on:change=move |e|{state.error.set(String::new());selected.set(event_target_value(&e));}><option value="">"選擇房間"</option>{move ||rooms.get().into_iter().map(|r|view!{<option value=r["id"].as_str().unwrap_or("").to_owned()>{r["name"].as_str().unwrap_or("").to_owned()}</option>}).collect_view()}</select></label>
    <Show when=move ||!selected.get().is_empty()><section id="room-settings"><h3>"房間成員"</h3><p>"公開房間不需邀請；私聊房間需要邀請。"</p>
           <input placeholder="邀請的使用者名稱" on:input=move |e|invitee.set(event_target_value(&e))/><button disabled=move ||selected.get().is_empty()||invitee.get().trim().is_empty() on:click=move |_|{let body=json!({"room_id":selected.get(),"username":invitee.get()});spawn(async move{match api("/api/v1/rooms/invite","POST",Some(body)).await{Ok(_)=>{notice.set("邀請已完成".into());member_epoch.update(|v|*v+=1);},Err(e)=>state.error.set(e)}});}>"邀請加入"</button>

    <ul id="room-members">{move ||members.get().into_iter().map(|m|view!{<li>{format!("{}{}",m["username"].as_str().unwrap_or(""),if m["owner"]==true{"（房主）"}else{""})}</li>}).collect_view()}</ul>
    <TopicControls room=selected/>
    <details><summary>"個人工具檢索"</summary>
           <h3>"工具檢索與 Agent"</h3><textarea placeholder="讓 Agent 搜尋知識庫的問題" maxlength="4096" prop:value=move ||agent_question.get() on:input=move |e|agent_question.set(event_target_value(&e))></textarea>
           <label><input type="checkbox" on:change=move |e|allow_tools.set(event_target_checked(&e))/>"允許唯讀搜尋我的知識庫，將相關資料交給 OpenAI 回答此房間"</label>
           <button disabled=move ||!allow_tools.get() ||agent_busy.get() on:click=move |_| {
               let body=json!({"room_id":selected.get(),"question":agent_question.get()});agent_busy.set(true);agent_result.set("正在使用唯讀知識庫工具…".into());
               spawn(async move {match api("/api/v1/ai/agent","POST",Some(body)).await {
                   Ok(v)=>agent_result.set(format!("{}\n理解問題：{}\n來源：{}\n工具：{}",v["answer"].as_str().unwrap_or(""),v["follow_up_question"].as_str().unwrap_or(""),v["sources"],v["steps"])),Err(e)=>state.error.set(e)
               }agent_busy.set(false);});
           }>"執行 Agent"</button><pre aria-live="polite">{move ||agent_result.get()}</pre>

    </details></section></Show>
    <h3>"我的操作紀錄"</h3><p>"最近100筆自己的紀錄（UTC）。新版啟用前的匿名事件不含帳號。"</p><button on:click=move |_|refresh()>"更新紀錄"</button>
    <ul id="my-activity">{move ||activity.get().into_iter().map(|r|view!{<li>{format!("{} · {} · {}",audit_time(r["occurred_unix_ms"].as_i64().unwrap_or(0)),match r["event_type"].as_str().unwrap_or(""){"login.succeeded"=>"登入成功","login.failed"=>"登入失敗","logout.succeeded"=>"登出","room.created"=>"建立房間",_=>"操作"},r["room_name"].as_str().unwrap_or(""))}</li>}).collect_view()}</ul>
    </section>}.into_any()
}
#[component]
pub(super) fn SystemAdministration() -> impl IntoView {
    let state = session();
    let credential = RwSignal::new(String::new());
    let authenticated = RwSignal::new(false);
    let rooms = RwSignal::new(Vec::<Value>::new());
    let selected = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let suggestion_id = RwSignal::new(String::new());
    let confirm_policy = RwSignal::new(false);
    let policy_result = RwSignal::new(String::new());
    view! {<section id="system-administration"><h2>"系統後台：AI 角色與對話設定"</h2><p>"僅供建置系統的管理者使用。房主與一般成員沒有 AI 設定權限。"</p>
    <form on:submit=move |e|{e.prevent_default();let token=credential.get_untracked();busy.set(true);spawn(async move{match privileged_api(token.clone(),"/api/v1/rooms","GET",None).await{Ok(v)=>{if credential.get_untracked()==token{rooms.set(v.as_array().cloned().unwrap_or_default());authenticated.set(true);state.error.set(String::new());}},Err(_)=>{authenticated.set(false);state.error.set("系統管理憑證不正確或無法連線。".into());}}busy.set(false);});}>
    <label>"系統管理 Token（僅留本頁）"<input id="system-token" type="password" autocomplete="off" prop:value=move ||credential.get() on:input=move |e|{credential.set(event_target_value(&e));authenticated.set(false);selected.set(String::new());rooms.set(Vec::new());}/></label><button type="submit" disabled=move ||busy.get()>"進入系統後台"</button></form>
    <Show when=move ||authenticated.get()><label>"管理 AI 的房間"<select prop:value=move ||selected.get() on:change=move |e|{state.error.set(String::new());selected.set(event_target_value(&e));}><option value="">"選擇房間"</option>{move ||rooms.get().into_iter().map(|r|view!{<option value=r["id"].as_str().unwrap_or("").to_owned()>{r["name"].as_str().unwrap_or("").to_owned()}</option>}).collect_view()}</select></label>
    <Show when=move ||!selected.get().is_empty()>
           <h3>"已審核的設定建議"</h3><p>"在 Analytics 檢視證據並核准後，輸入建議 UUID。只改變目前房間的 AI 歷史訊息數；回退會檢查之後是否有其他變更。"</p>
              <input placeholder="建議 UUID" prop:value=move ||suggestion_id.get() on:input=move |e|suggestion_id.set(event_target_value(&e))/>
              <label><input type="checkbox" on:change=move |e|confirm_policy.set(event_target_checked(&e))/>"確認變更目前房間設定"</label>
              <button disabled=move ||!confirm_policy.get() on:click=move |_|{let path=format!("/api/v1/rooms/{}/suggestions/{}/apply",selected.get(),suggestion_id.get());spawn(async move{match privileged_api(credential.get_untracked(),&path,"POST",Some(json!({"confirmed":true}))).await{Ok(v)=>policy_result.set(v.to_string()),Err(e)=>state.error.set(e)}});}>"套用並回報"</button>
              <button disabled=move ||!confirm_policy.get() on:click=move |_|{let path=format!("/api/v1/rooms/{}/suggestions/{}/rollback",selected.get(),suggestion_id.get());spawn(async move{match privileged_api(credential.get_untracked(),&path,"POST",Some(json!({"confirmed":true}))).await{Ok(v)=>policy_result.set(v.to_string()),Err(e)=>state.error.set(e)}});}>"回退並回報"</button><pre>{move ||policy_result.get()}</pre>

                 <PersonaControls room=selected can_manage=Signal::derive(move ||authenticated.get()) credential=credential/>


    </Show></Show></section>}.into_any()
}
