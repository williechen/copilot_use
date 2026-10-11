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
 await invoke('gui_test_progress',{phase:'core-complete'});
 if(window.__PET_TEST_LIVE_AI){
  await invoke('gui_test_progress',{phase:'live-ai'});
  let deltas=0;const request=crypto.randomUUID();const unlisten=await window.__TAURI__.event.listen('pet-reply-delta',e=>{if(e.payload.request===request)deltas++});
  const output=await invoke('pet_talk',{pet,request,text:'請用一句繁體中文說明 Rust 的所有權，再問一個理解問題。'});
  unlisten();assert(output.length>0&&deltas>0,'real AI streaming');
 }
 if(window.__PET_TEST_LIVE_IMAGE){
  await invoke('gui_test_progress',{phase:'live-image'});
  const asset=await invoke('generate_asset',{pet});assert(asset.png_base64.length>0&&asset.width>0&&asset.height>0,'real OpenAI sprite');
  const cached=await invoke('load_asset',{pet});assert(cached.png_base64===asset.png_base64,'sprite cache persistence');
 }
 await invoke('gui_test_progress',{phase:'cleanup'});
 const files=await invoke('scan_file_preview');const sample=files.find(f=>f.name==='sample.txt');assert(sample&&sample.size_bytes>0,'file preview');
 let refused=false;try{await invoke('confirm_cleanup',{id:sample.id,confirmed:false})}catch(_){refused=true;}assert(refused,'explicit confirmation');
 const receipt=await invoke('confirm_cleanup',{id:sample.id,confirmed:true});assert(receipt.operation_id&&receipt.size_bytes>0,'real cleanup receipt');
 const select=document.querySelector('select');
 // 實際觸發 UI 的選擇與資料載入事件。
 const option=document.createElement('option');option.value=pet;select.appendChild(option);select.value=pet;select.dispatchEvent(new Event('change',{bubbles:true}));
 await new Promise(r=>setTimeout(r,250));
 assert(document.body.textContent.includes('Rust 的所有權'),'Leptos renders native data');
 await invoke('gui_test_result',{success:true,message:'PASS: WebKit/WASM, 200x200, IPC, profiles, memory isolation/deletion, recall scheduling, opaque-ID confirmed cleanup and rendered data; AI='+!!window.__PET_TEST_LIVE_AI+'; image='+!!window.__PET_TEST_LIVE_IMAGE});
})().catch(async e=>{await window.__TAURI__.core.invoke('gui_test_result',{success:false,message:String(e?.message||e)})});
