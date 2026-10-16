window.addEventListener('error',e=>window.__PET_LOAD_ERROR=e.message);window.addEventListener('unhandledrejection',e=>window.__PET_LOAD_ERROR=String(e.reason));
// 只在明確啟用且通過全新 fixture 檢查的 GUI 驗收中執行。
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
 if(!window.__PET_TEST_FUNCTIONAL_ONLY){const retrieved=await invoke('search_memory_tool',{pet,query:'probe'});assert(retrieved.hits.some(x=>x.id===knowledge)&&!retrieved.hits.some(x=>x.content.includes('STALE-999')),'actual readonly MCP stdio scope');}

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
  await invoke('save_profile',{id:pet,profile:{...profile,'名字':'已修改角色'}});
  assert((await invoke('load_asset',{pet}))===null,'changed profile cannot render stale generated sprite');
  await invoke('save_profile',{id:pet,profile});
  assert((await invoke('load_asset',{pet})).png_base64===asset.png_base64,'matching original profile restores cached sprite');
 }
 // 明確啟用原生漫遊，驗證真實 pet 視窗移動與工作區邊界後立即關閉。
 await invoke('select_pet',{pet});
 await invoke('set_interaction',{pet,preferences:{action:'moving',speed:'fast',proactive:false}});
 await invoke('set_roaming',{enabled:true});
 const roamBefore=await invoke('roaming_status');let roamAfter=roamBefore;
 const roamDeadline=Date.now()+10000;
 while(roamAfter.moves<roamBefore.moves+3){if(Date.now()>roamDeadline)throw Error('native window roaming stalled');await new Promise(r=>setTimeout(r,100));roamAfter=await invoke('roaming_status');}
 assert(roamAfter.x>=roamAfter.area.x&&roamAfter.y>=roamAfter.area.y&&roamAfter.x+roamAfter.width<=roamAfter.area.x+roamAfter.area.width&&roamAfter.y+roamAfter.height<=roamAfter.area.y+roamAfter.area.height,'native roaming stays in monitor work area');
 await invoke('set_roaming',{enabled:false});
 await new Promise(r=>setTimeout(r,200));const stopped=await invoke('roaming_status');await new Promise(r=>setTimeout(r,300));const still=await invoke('roaming_status');
 assert(!still.enabled&&stopped.x===still.x&&stopped.y===still.y,'native roaming stops on explicit disable');
 await invoke('set_interaction',{pet,preferences:{action:'idle',speed:'normal',proactive:false}});
 const duplicateGroups=await invoke('scan_duplicate_preview');
 assert(duplicateGroups.some(g=>g.some(f=>f.name==='sample.txt')&&g.some(f=>f.name==='sample-copy.txt')),'actual SHA256 duplicate IPC grouping');
 document.querySelector('#scan-duplicates').click();
 const duplicateDeadline=Date.now()+10000;
 while(!document.body.textContent.includes('SHA256 重複群組：1')){if(Date.now()>duplicateDeadline)throw Error('duplicate grouping UI');await new Promise(r=>setTimeout(r,50));}
 const duplicateRows=[...document.querySelector('#scan-duplicates').closest('section').querySelectorAll('article')].filter(row=>row.textContent.includes('sample'));
 assert(duplicateRows.length===2&&duplicateRows.every(row=>row.querySelector('button')?.disabled),'duplicate UI preserves per-file confirmation');
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
 const waitUntil=async(test,label)=>{const until=Date.now()+10000;while(!test()){if(Date.now()>until)throw Error(label+'; UI: '+(document.querySelector('[role="alert"]')?.textContent||''));await new Promise(r=>setTimeout(r,25));}};
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
 let restoreDenied=false;try{await invoke('restore_file',{id:receipt.operation_id,confirmed:false})}catch(_){restoreDenied=true;}assert(restoreDenied,'restore explicit confirmation');
 document.querySelector('#load-trash').click();const trash=document.querySelector('#trash-browser');
 await waitUntil(()=>[...trash.querySelectorAll('article')].some(x=>x.textContent.includes('sample.txt')),'trash UI listing');
 const trashRow=[...trash.querySelectorAll('article')].find(x=>x.textContent.includes('sample.txt'));const restoreButton=trashRow.querySelector('button');assert(restoreButton.disabled,'restore UI confirmation');
 const restoreBox=trashRow.querySelector('input');restoreBox.checked=true;restoreBox.dispatchEvent(new Event('change',{bubbles:true}));await waitUntil(()=>!restoreButton.disabled,'restore checkbox');restoreButton.click();
 await waitUntil(()=>trash.textContent.includes('已還原；收據'),'restore UI receipt');
 const restores=await invoke('recover_restore_receipts');assert(restores.length===1,'restore recovery');
 const retry=await invoke('restore_file',{id:receipt.operation_id,confirmed:true});assert(retry.operation_id===restores[0].operation_id,'restore stable retry receipt');
 assert((await invoke('recover_restore_receipts')).length===1,'restore recovery idempotence');
 assert((await invoke('scan_file_preview')).some(x=>x.name==='sample.txt'),'restored file available');

 const select=document.querySelector('select');
 // 實際觸發 UI 的選擇與資料載入事件。
 const option=document.createElement('option');option.value=pet;select.appendChild(option);select.value=pet;select.dispatchEvent(new Event('change',{bubbles:true}));
 await waitUntil(()=>document.body.textContent.includes('Rust 的所有權'),'Leptos renders native data');
 if(window.__PET_TEST_LIVE_IMAGE){await waitUntil(()=>document.querySelector('.sprite'),'generated sprite rendered');const current=await invoke('load_asset',{pet});const frame=document.querySelector('.sprite').getBoundingClientRect();assert(frame.width<=200&&frame.height<=200&&Math.abs(frame.width/frame.height-current.width/current.height)<0.01,'actual sprite frame preserves original aspect inside 200x200');}


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
 let deniedImport=false;try{await invoke('import_pet_profile',{pet,confirmed:false})}catch(_){deniedImport=true}assert(deniedImport,'profile import requires confirmation');await invoke('import_pet_profile',{pet,confirmed:true});
 const importButton=document.querySelector('#import-pet-profile');assert(importButton.disabled,'profile import UI confirmation');
 const importConfirm=document.querySelector('#confirm-profile-import');importConfirm.checked=true;importConfirm.dispatchEvent(new Event('change',{bubbles:true}));await waitUntil(()=>!importButton.disabled,'profile import checkbox');importButton.click();
 await waitUntil(()=>document.querySelector('#import-pet-profile').disabled,'actual profile import completed');assert(document.querySelector('#profile-location').textContent.includes(pet),'selected pet configuration path rendered');

 const memorySection=[...document.querySelectorAll('section')].find(x=>x.querySelector('h2')?.textContent==='可管理的記憶');
 await waitUntil(()=>[...memorySection.querySelectorAll('article')].some(x=>x.textContent.includes('ASTRO-771')),'memory UI load after profile save');
 const memoryRow=[...memorySection.querySelectorAll('article')].find(x=>x.textContent.includes('ASTRO-771'));
 memoryRow.querySelector('button').click();const editor=memorySection.querySelector('textarea');editor.value='edited ASTRO-771 test memory';editor.dispatchEvent(new Event('input',{bubbles:true}));
 [...memorySection.querySelectorAll('button')].find(x=>x.textContent==='保存記憶').click();
 let edited;const editDeadline=Date.now()+10000;do{edited=(await invoke('list_memories',{pet})).find(x=>x.id===knowledge);if(edited?.content==='edited ASTRO-771 test memory')break;await new Promise(r=>setTimeout(r,25));}while(Date.now()<editDeadline);
 assert(edited.content==='edited ASTRO-771 test memory','memory edit UI preserves scoped ID');

 const actionControl=document.querySelector('#pet-action');await waitUntil(()=>!actionControl.disabled,'selected interaction settings loaded');actionControl.value='learning';actionControl.dispatchEvent(new Event('change',{bubbles:true}));
 const speedControl=document.querySelector('#pet-speed');await waitUntil(()=>!speedControl.disabled,'action auto-save completes');speedControl.value='fast';speedControl.dispatchEvent(new Event('change',{bubbles:true}));
 const interactionDeadline=Date.now()+10000;let preferences;
 do{preferences=await invoke('interaction',{pet});if(preferences.action==='learning'&&preferences.speed==='fast')break;await new Promise(r=>setTimeout(r,25));}while(Date.now()<interactionDeadline);
 assert(preferences.action==='learning'&&preferences.speed==='fast'&&!preferences.proactive,'animation action/speed UI persisted with proactive off: '+JSON.stringify(preferences));
 const beforeControls=(await invoke('list_pets')).map(p=>p.id).sort();
 const controls=await invoke('gui_test_window_controls');assert(controls.right_click&&controls.second_instance_exited&&controls.existing_controls_visible&&controls.window_count===2,'right-click and repeated launch reuse existing windows');
 assert(JSON.stringify(beforeControls)===JSON.stringify((await invoke('list_pets')).map(p=>p.id).sort()),'opening controls never creates another pet');
 // 操作真正控制介面；整段使用同一程序，不按保存、不重新啟動。
 async function preference(field,value){
  const control=document.querySelector('#pet-'+field);
  await waitUntil(()=>!control.disabled,'interaction control ready');
  control.value=value;control.dispatchEvent(new Event('change',{bubbles:true}));
  const deadline=Date.now()+10000;
  while((await invoke('interaction',{pet}))[field]!==value){if(Date.now()>deadline)throw Error('control auto-save '+field);await new Promise(r=>setTimeout(r,30));}
 }
 async function roamingToggle(enabled){
  const control=document.querySelector('#pet-roaming');
  await waitUntil(()=>!control.disabled,'roaming control ready');
  control.checked=enabled;control.dispatchEvent(new Event('change',{bubbles:true}));
  const deadline=Date.now()+10000;
  while((await invoke('roaming_status')).enabled!==enabled){if(Date.now()>deadline)throw Error('roaming checkbox did not apply');await new Promise(r=>setTimeout(r,30));}
 }
 async function moved(){const before=await invoke('roaming_status');const deadline=Date.now()+10000;
  while((await invoke('roaming_status')).moves<before.moves+3){if(Date.now()>deadline)throw Error('UI native roaming did not move');await new Promise(r=>setTimeout(r,100));}}
 async function stationary(){await new Promise(r=>setTimeout(r,250));const before=await invoke('roaming_status');await new Promise(r=>setTimeout(r,400));const after=await invoke('roaming_status');assert(before.x===after.x&&before.y===after.y,'UI stops native window immediately');}
 await preference('action','moving');await preference('speed','normal');
 await roamingToggle(true);await moved();
 await preference('action','idle');await stationary();
 assert((await invoke('roaming_status')).enabled,'idle keeps explicit roaming preference');
 await preference('action','moving');await moved();
 await roamingToggle(false);await stationary();
 await roamingToggle(true);await moved();
 await roamingToggle(false);await preference('action','learning');
 assert((await invoke('learning_summary',{pet})).total>0,'actual learning summary');
 assert(await invoke('recall_prompt')===null,'proactivity requires opt in');
 const backups=document.querySelector('#pet-backups');document.querySelector('#create-snapshot').click();
 await waitUntil(()=>backups.textContent.includes('備份完成'),'backup UI completion');const snapshots=await invoke('list_snapshots');assert(snapshots.length===1&&snapshots[0].sha256.length===64,'snapshot manifest');
 await invoke('save_memory',{pet,id:knowledge,content:'changed after snapshot'});
 let deniedBackup=false;try{await invoke('restore_snapshot',{id:snapshots[0].id,confirmed:false})}catch(_){deniedBackup=true;}assert(deniedBackup,'snapshot restore explicit confirmation');
 const backupSelect=backups.querySelector('select');backupSelect.value=snapshots[0].id;backupSelect.dispatchEvent(new Event('change',{bubbles:true}));
 const restoreSnapshotButton=backups.querySelector('#restore-snapshot');assert(restoreSnapshotButton.disabled,'snapshot UI confirmation');
 const backupConfirm=backups.querySelector('input');backupConfirm.checked=true;backupConfirm.dispatchEvent(new Event('change',{bubbles:true}));await waitUntil(()=>!restoreSnapshotButton.disabled,'snapshot confirm checkbox');restoreSnapshotButton.click();
 await waitUntil(()=>backups.textContent.includes('還原完成'),'snapshot restore UI completion');assert((await invoke('list_memories',{pet})).find(x=>x.id===knowledge).content==='edited ASTRO-771 test memory','SQLite memory restored');assert((await invoke('list_snapshots')).length===2,'pre-restore safety snapshot retained');
 if(window.__PET_TEST_ANALYTICS){
  await invoke('gui_test_progress',{phase:'analytics-outbox'});
  const deadline=Date.now()+30000;let outbox;
  do {outbox=await invoke('outbox_status');if(outbox.delivered>=4&&!outbox.pending&&!outbox.leased&&!outbox.blocked)break;await new Promise(r=>setTimeout(r,100));}while(Date.now()<deadline);
  assert(outbox.delivered>=4&&!outbox.pending&&!outbox.leased&&!outbox.blocked,'actual Pet to Analytics delivery');
 }
 // Wait for the actual UI timer, shared between the settings and floating windows.
 await invoke('add_card',{pet,question:'主動回想測試題：Rust 所有權',answer:'只有一個擁有者'});
 const proactiveControl=document.querySelector('#pet-proactive');proactiveControl.checked=true;proactiveControl.dispatchEvent(new Event('change',{bubbles:true}));
 const proactiveDeadline=Date.now()+70000;
 while(!document.querySelector('.recall-prompt')?.textContent.includes('主動回想測試題')){if(Date.now()>proactiveDeadline)throw Error('actual proactive recall timer did not display');await new Promise(r=>setTimeout(r,100));}
 assert(await invoke('recall_prompt')===null,'displayed reminder is rate limited across windows');
 document.querySelector('.recall-prompt button').click();

 const registry=document.querySelector('#pet-archive');const archivePet=document.querySelector('#archive-pet');assert(archivePet.disabled,'pet removal UI confirmation');
 let removalDenied=false;try{await invoke('set_pet_archived',{pet,archived:true,confirmed:false})}catch(_){removalDenied=true}assert(removalDenied,'pet removal IPC confirmation');
 const removeConfirm=document.querySelector('#confirm-pet-archive');removeConfirm.checked=true;removeConfirm.dispatchEvent(new Event('change',{bubbles:true}));await waitUntil(()=>!archivePet.disabled,'pet removal checkbox');archivePet.click();
 await waitUntil(()=>registry.querySelector(`article[data-pet="${pet}"]`),'removed pet appears in recoverable archive');
 assert(!(await invoke('list_pets')).some(x=>x.id===pet)&&(await invoke('list_pets')).some(x=>x.id===other),'actual pet removal preserves other pet');
 const archivedRow=registry.querySelector(`article[data-pet="${pet}"]`);const restorePet=archivedRow.querySelector('button');assert(restorePet.disabled,'pet restore UI confirmation');const restorePetConfirm=archivedRow.querySelector('input');restorePetConfirm.checked=true;restorePetConfirm.dispatchEvent(new Event('change',{bubbles:true}));await waitUntil(()=>!restorePet.disabled,'pet restore checkbox');restorePet.click();
 await waitUntil(()=>registry.textContent.includes('已還原寵物與原有資料'),'actual pet restore receipt');assert((await invoke('list_memories',{pet})).some(x=>x.id===knowledge),'restored pet retains saved memory');
 assert((await invoke('list_pets')).length===2,'restored pet registry');
 if(window.__PET_TEST_UPDATER){
  const available=await invoke('check_local_update');assert(available.status==='available'&&available.version===window.__PET_TEST_UPDATE_VERSION,'local signed update discovered');
  let unconfirmedInstall=false;try{await invoke('install_local_update',{id:available.id,confirmed:false})}catch(_){unconfirmedInstall=true}assert(unconfirmedInstall,'updater installation requires confirmation');
  const verified=await invoke('verify_local_update',{id:available.id});assert(verified.status==='signature_verified'&&verified.bytes>0,'actual updater signature download verified');
  if(window.__PET_TEST_INSTALL_UPDATE){
   const installed=await invoke('install_local_update',{id:available.id,confirmed:true});
   assert(installed.status==='installed_restart_required'&&installed.version===window.__PET_TEST_UPDATE_VERSION&&installed.before_snapshot,'actual signed package installed after SQLite snapshot');
  }else{
  const wrongVersion=await invoke('check_local_update');let versionDenied=false;try{await invoke('verify_local_update',{id:wrongVersion.id})}catch(_){versionDenied=true}assert(versionDenied,'signed version mismatch rejected');
  const tampered=await invoke('check_local_update');let tamperDenied=false;try{await invoke('verify_local_update',{id:tampered.id})}catch(_){tamperDenied=true}assert(tamperDenied,'modified updater package rejected');
  let remoteDenied=false;try{await invoke('check_local_update')}catch(_){remoteDenied=true}assert(remoteDenied,'updater remote URL rejected before download');
  }
 }
 await invoke('gui_test_result',{success:true,message:'PASS: WebKit/WASM, 200x200, IPC, profiles, memory isolation/deletion, recall scheduling, advanced overrides, memory editing, opaque-ID confirmed cleanup, organizer GUI, journal/restore recovery, Trash UI, animation choices, rendered proactive reminder and cooldown, learning summary, SQLite/assets snapshot restore, per-pet TOML import and recoverable registry, right-click controls and single-instance reactivation without duplicate pets, native roaming and live UI toggle/action/speed auto-save without restart, SHA256 duplicate preview and per-file confirmation; MCP='+(window.__PET_TEST_FUNCTIONAL_ONLY?'excluded':'local protocol checks')+'; AI='+!!window.__PET_TEST_LIVE_AI+'; image='+!!window.__PET_TEST_LIVE_IMAGE+'; analytics='+!!window.__PET_TEST_ANALYTICS+'; updater_signature='+!!window.__PET_TEST_UPDATER});
})().catch(async e=>{await window.__TAURI__.core.invoke('gui_test_result',{success:false,message:String(e?.message||e)})});
