const fs = require('node:fs');
const vm = require('node:vm');
const assert = require('node:assert/strict');
const html = fs.readFileSync(process.argv[2] || 'apps/desktop_pet/frontend/index.html','utf8');
const calls=[];
const rows=[{id:'saved-1',name:'小橘',species:'tortoise'}];
const window={__TAURI__:{core:{invoke:async (command,args)=>{calls.push({command,args});return command==='list_pets'?rows:'小橘（saved-1）';}},event:{listen:()=>{}}}};
vm.runInNewContext(html.match(/<script>([\s\S]*?)<\/script>/)[1],{window,URLSearchParams});
(async()=>{assert.equal(await window.createPet(' 小橘 '),'小橘（saved-1）');assert.deepEqual(await window.listPets(),rows);assert.equal(calls[0].command,'create_pet');assert.equal(calls[0].args.name,' 小橘 ');assert.equal(calls[1].command,'list_pets');assert.equal(window.validateNewPetName,undefined);console.log('PASS: real HTML bridge invokes create_pet(name) and list_pets; mock IPC only, not native GUI');})().catch(e=>{console.error(e);process.exitCode=1;});
