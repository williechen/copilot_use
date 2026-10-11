window.addEventListener('error',e=>window.__PET_LOAD_ERROR=e.message);window.addEventListener('unhandledrejection',e=>window.__PET_LOAD_ERROR=String(e.reason));
// 僅在明確開啟的 debug GUI 測試中執行，驗證真正 WebKit/WASM 與 Tauri IPC。
(async()=>{
 const assert=(value,message)=>{if(!value)throw Error(message)};
 const limit=Date.now()+15000;
 while(!document.querySelector('.pet')){if(Date.now()>limit)throw Error('Leptos mount timeout: '+document.body.innerHTML.slice(0,800)+' '+window.__PET_LOAD_ERROR);await new Promise(r=>setTimeout(r,50));}
 const box=document.querySelector('.pet').getBoundingClientRect();assert(box.width===200&&box.height===200,'200x200 display');
 const invoke=window.__TAURI__.core.invoke;
 const pet='8df47580-64e1-4335-8e50-6b593051a398';
 const other='a4105c9b-67f8-4c60-8376-181c706de117';
 const memory='b4197c3b-3f50-4dc1-b12f-fd1a7143ec85';
 const profile={'名字':'GUI 驗證','種類':'鳥','喜好':['音樂'],'個性':['活潑'],'興趣':['Rust']};
 await invoke('save_profile',{id:pet,profile});await invoke('save_profile',{id:other,profile});
 const pets=await invoke('list_pets');assert(pets.some(x=>x.id===pet),'profile persistence');
 await invoke('save_memory',{pet,id:memory,content:'安全隔離的測試記憶'});
 assert((await invoke('list_memories',{pet})).some(x=>x.id===memory),'memory saved');
 assert(!(await invoke('list_memories',{pet:other})).some(x=>x.id===memory),'memory isolation');
 await invoke('delete_memory',{pet:other,id:memory});assert((await invoke('list_memories',{pet})).some(x=>x.id===memory),'cross-pet deletion denied');
 await invoke('delete_memory',{pet,id:memory});assert(!(await invoke('list_memories',{pet})).some(x=>x.id===memory),'memory deletion');
 await invoke('add_card',{pet,question:'Rust 的所有權如何避免 double free？',answer:'單一所有者與借用檢查'});
 const cards=await invoke('list_cards',{pet});assert(cards.length>0,'review card saved');
 const reviewed=await invoke('review_card',{pet,id:cards[0].id,quality:5});assert(reviewed.interval_days>0,'spaced review scheduling');
 const knowledge=crypto.randomUUID();await invoke('save_memory',{pet,id:knowledge,content:'Agent 驗收代碼為 ASTRO-771。probe memory owned by this pet.'});
 await invoke('save_memory',{pet:other,id:crypto.randomUUID(),content:'probe other pet hidden code STALE-999'});
 const retrieved=await invoke('search_memory_tool',{pet,query:'probe'});assert(retrieved.hits.some(x=>x.id===knowledge)&&!retrieved.hits.some(x=>x.content.includes('STALE-999')),'actual readonly MCP stdio scope');

 if(window.__PET_TEST_SUGGESTION){
  const id=window.__PET_TEST_SUGGESTION;
  let refused=false;try{await invoke('apply_analytics_suggestion',{pet,id,confirmed:false})}catch(_){refused=true;}assert(refused,'policy explicit confirmation');
  const applied=await invoke('apply_analytics_suggestion',{pet,id,confirmed:true});
  assert(applied.value===3&&applied.analytics_acknowledged,'native policy apply acknowledgement');
  const replay=await invoke('apply_analytics_suggestion',{pet,id,confirmed:true});assert(replay.receipt_id===applied.receipt_id,'native policy idempotence');
  const rolled=await invoke('rollback_analytics_suggestion',{pet,id,confirmed:true});assert(rolled.value===1&&rolled.analytics_acknowledged,'native policy rollback acknowledgement');
 }
 await invoke('gui_test_progress',{phase:'core-complete'});
 if(window.__PET_TEST_LIVE_AI){
  await invoke('gui_test_progress',{phase:'live-ai'});
  let deltas=0;const request=crypto.randomUUID();const unlisten=await window.__TAURI__.event.listen('pet-reply-delta',e=>{if(e.payload.request===request)deltas++});
  const output=await invoke('pet_talk',{pet,request,text:'請用一句繁體中文說明 Rust 的所有權，再問一個理解問題。'});
  unlisten();assert(output.length>0&&deltas>0,'real AI streaming');
  const agent=await invoke('pet_agent',{pet,text:'請使用記憶檢索工具回答 Agent 驗收代碼，引用資料並提出一個理解問題。'});
  assert(agent.answer.includes('ASTRO-771')&&agent.sources.includes(knowledge)&&agent.steps.length>0&&agent.follow_up_question,'real Pet MCP Agent structured answer');

 }
 if(window.__PET_TEST_LIVE_IMAGE){
  await invoke('gui_test_progress',{phase:'live-image'});
  const asset=await invoke('generate_asset',{pet});assert(asset.png_base64.length>0&&asset.width>0&&asset.height>0,'real OpenAI sprite');
  const cached=await invoke('load_asset',{pet});assert(cached.png_base64===asset.png_base64,'sprite cache persistence');
 }
 await invoke('gui_test_progress',{phase:'cleanup'});
 const files=await invoke('scan_file_preview');const sample=files.find(f=>f.name==='sample.txt');assert(sample&&sample.size_bytes>0,'file preview');
 const rootPath=await invoke('authorized_directory');assert(rootPath,'authorized directory');
 await invoke('revoke_directory');
 let revoked=false;try{await invoke('confirm_cleanup',{id:sample.id,confirmed:true})}catch(_){revoked=true;}assert(revoked,'revocation invalidates old previews');
 let unconfirmed=false;try{await invoke('authorize_directory',{path:rootPath,confirmed:false})}catch(_){unconfirmed=true;}assert(unconfirmed,'directory authorization requires confirmation');
 await invoke('authorize_directory',{path:rootPath,confirmed:true});
 const fresh=await invoke('scan_file_preview');sample.id=fresh.find(x=>x.name==='sample.txt').id;
 let refused=false;try{await invoke('confirm_cleanup',{id:sample.id,confirmed:false})}catch(_){refused=true;}assert(refused,'explicit confirmation');
 const receipt=await invoke('confirm_cleanup',{id:sample.id,confirmed:true});assert(receipt.operation_id&&receipt.size_bytes>0,'real cleanup receipt');
 const waitUntil=async(test,label)=>{const until=Date.now()+10000;while(!test()){if(Date.now()>until)throw Error(label);await new Promise(r=>setTimeout(r,25));}};
 document.querySelector('#scan-files').click();
 const section=document.querySelector('#scan-files').closest('section');
 await waitUntil(()=>[...section.querySelectorAll('article')].some(x=>x.textContent.includes('second.txt')),'organizer UI preview');
 const row=[...section.querySelectorAll('article')].find(x=>x.textContent.includes('second.txt'));
 const button=row.querySelector('button');assert(button.disabled,'organizer button requires confirmation');
 const checkbox=row.querySelector('input');checkbox.checked=true;checkbox.dispatchEvent(new Event('change',{bubbles:true}));
 await waitUntil(()=>!button.disabled,'organizer confirmation checkbox');button.click();
 await waitUntil(()=>section.textContent.includes('已隔離；收據'),'organizer UI success receipt');
 const recovered=await invoke('recover_cleanup_receipts');assert(recovered.length===2&&recovered.some(x=>x.operation_id===receipt.operation_id),'durable journal recovery');
 assert((await invoke('recover_cleanup_receipts')).length===2,'repeat recovery idempotence');
 const select=document.querySelector('select');
 // 實際觸發 UI 的選擇與資料載入事件。
 const option=document.createElement('option');option.value=pet;select.appendChild(option);select.value=pet;select.dispatchEvent(new Event('change',{bubbles:true}));
 await new Promise(r=>setTimeout(r,250));
 assert(document.body.textContent.includes('Rust 的所有權'),'Leptos renders native data');

 for(const [field,value]of [['名字','GUI 驗證'],['種類','鳥']]){
  const input=[...document.querySelectorAll('label')].find(x=>x.textContent===field).querySelector('input');input.value=value;input.dispatchEvent(new Event('input',{bubbles:true}));
 }
 const color=[...document.querySelectorAll('label')].find(x=>x.textContent==='顏色').querySelector('input');
 color.value='翠綠色';color.dispatchEvent(new Event('input',{bubbles:true}));
 [...document.querySelectorAll('button')].find(x=>x.textContent==='保存設定').click();
 await waitUntil(()=>document.body.textContent.includes('我的寵物'),'profile UI');
 let savedProfile;const profileDeadline=Date.now()+10000;
 do{savedProfile=JSON.parse((await invoke('list_pets')).find(x=>x.id===pet).profile);if(savedProfile['外觀']?.['顏色']==='翠綠色')break;await new Promise(r=>setTimeout(r,25));}while(Date.now()<profileDeadline);
 assert(savedProfile['外觀']['顏色']==='翠綠色','advanced override UI persisted');
 const memorySection=[...document.querySelectorAll('section')].find(x=>x.querySelector('h2')?.textContent==='可管理的記憶');
 const memoryRow=[...memorySection.querySelectorAll('article')].find(x=>x.textContent.includes('ASTRO-771'));
 memoryRow.querySelector('button').click();const editor=memorySection.querySelector('textarea');editor.value='edited ASTRO-771 test memory';editor.dispatchEvent(new Event('input',{bubbles:true}));
 [...memorySection.querySelectorAll('button')].find(x=>x.textContent==='保存記憶').click();
 let edited;const editDeadline=Date.now()+10000;do{edited=(await invoke('list_memories',{pet})).find(x=>x.id===knowledge);if(edited?.content==='edited ASTRO-771 test memory')break;await new Promise(r=>setTimeout(r,25));}while(Date.now()<editDeadline);
 assert(edited.content==='edited ASTRO-771 test memory','memory edit UI preserves scoped ID');

 if(window.__PET_TEST_ANALYTICS){
  await invoke('gui_test_progress',{phase:'analytics-outbox'});
  const deadline=Date.now()+30000;let outbox;
  do {outbox=await invoke('outbox_status');if(outbox.delivered>=3&&!outbox.pending&&!outbox.leased&&!outbox.blocked)break;await new Promise(r=>setTimeout(r,100));}while(Date.now()<deadline);
  assert(outbox.delivered>=3&&!outbox.pending&&!outbox.leased&&!outbox.blocked,'actual Pet to Analytics delivery');
 }
 await invoke('gui_test_result',{success:true,message:'PASS: WebKit/WASM, 200x200, IPC, profiles, memory isolation/deletion, recall scheduling, advanced overrides, memory editing, opaque-ID confirmed cleanup, organizer GUI, journal recovery, MCP stdio scope and rendered data; AI='+!!window.__PET_TEST_LIVE_AI+'; image='+!!window.__PET_TEST_LIVE_IMAGE+'; analytics='+!!window.__PET_TEST_ANALYTICS});
})().catch(async e=>{await window.__TAURI__.core.invoke('gui_test_result',{success:false,message:String(e?.message||e)})});
