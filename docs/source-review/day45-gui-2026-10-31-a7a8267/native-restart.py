from pathlib import Path
prefix=Path('work/day45-gui/native-extra-body.py').read_text().split('main=None')[0]
exec(prefix)
main=None;handles={}
for h in req('GET','/window/handles'):
 req('POST','/window',{'handle':h});pid=js('return new URLSearchParams(location.search).get("pet_id")')
 if not pid:main=h
 else:handles[pid]=h
ids=dict(db('SELECT name,id FROM pet_profile'));a=ids['Native Day45 A'];b=ids['Native Day45 B']
req('POST','/window',{'handle':handles[b]});state=asyncjs('const done=arguments[arguments.length-1];window.getPetState().then(done).catch(e=>done({error:String(e)}));');save('native OS context menu routed Sleep to B',{'snapshot':state});assert state['state']=='sleeping',state
req('POST','/window',{'handle':handles[a]});state_a=asyncjs('const done=arguments[arguments.length-1];window.getPetState().then(done);');assert state_a['state']=='idle',state_a;save('native menu action preserves other pet state')
# Drag is verified independently with actual OS XTest events; W3C pointer synthesis did not move the OS window.
if Path('work/day45-gui/native-drag.json').exists():save('native drag separately verified',json.loads(Path('work/day45-gui/native-drag.json').read_text()))
else:save('native drag unresolved; independent restart acceptance continues')
# Set startup behavior through actual row UI, then terminate and relaunch the native executable.
req('POST','/window',{'handle':main})
def rowclick(name,text):
 e=req('POST','/element',{'using':'xpath','value':'//li[contains(.,"'+name+'")]//button[normalize-space()="'+text+'"]'})['element-6066-11e4-a52e-4f735466cecf'];req('POST','/element/'+e+'/click',{})
rowclick('Native Day45 B','找回寵物');req('POST','/window',{'handle':handles[b]});recovered=req('GET','/window/rect');assert recovered['x']>=0 and recovered['y']>=0,recovered;save('native find-pet UI recovers offscreen dragged window',{'rect':recovered});req('POST','/window',{'handle':main})
rowclick('Native Day45 B','啟動時預設出籠');rowclick('Native Day45 A','啟動時預設留籠');save('native UI persists independent startup defaults',{'defaults':db('SELECT name,is_default_closure FROM pet_profile')})
req('DELETE','')
import urllib.request
value={'capabilities':{'alwaysMatch':{'tauri:options':{'application':'/home/willie/Documents/Codex/2026-10-27/referenced-chatgpt-conversation-this-is-an/work/cargo-target/debug/pet-server'}}}}
r=urllib.request.urlopen(urllib.request.Request('http://127.0.0.1:4444/session',data=json.dumps(value).encode(),headers={'Content-Type':'application/json'}),timeout=35);new=json.load(r);(out/'tauri-session.json').write_text(json.dumps(new));session=new['value']['sessionId'];base='http://127.0.0.1:4444/session/'+session
wait(lambda:len(req('GET','/window/handles'))==2,30);restored=[]
for h in req('GET','/window/handles'):
 req('POST','/window',{'handle':h});pid=js('return new URLSearchParams(location.search).get("pet_id")')
 if pid:restored.append(pid);assert pid==b;assert req('GET','/window/rect')['width']==200
assert restored==[b],restored;save('real process restart restores B and leaves A caged',{'restored':restored,'db':db('SELECT pet_id,is_activity FROM pet_activity')});shot('pet-native-restarted.png')
(out/'native-restart-results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n')
