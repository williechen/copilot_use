import pathlib,json,re,hashlib,subprocess,tomllib,html,collections
root=pathlib.Path(__file__).resolve().parent
docs=root/'copilot_use';src=root/'rust_ai_chat';out=root/'audit';out.mkdir(exist_ok=True)
sha=subprocess.check_output(['git','rev-parse','HEAD'],cwd=src,text=True).strip()
docsha=subprocess.check_output(['git','rev-parse','HEAD'],cwd=docs,text=True).strip()
catalog=json.loads((docs/'docs/teaching/history/ARTICLES.json').read_text())
symbols=json.loads((root/'source-symbols.json').read_text())
tests=[]
for p in (root/'logs').glob('*.log'):
 if p.name in ['workspace-tests.log','chat-http-console.log']:continue
 for name,status in re.findall(r'^test ([\w:]+) \.\.\. (ok|FAILED|ignored)',p.read_text(errors='replace'),re.M):
  tests.append(dict(name=name,status=status,log=p.name))
packages={}
for p in src.rglob('Cargo.toml'):
 d=tomllib.loads(p.read_text())
 if 'package' in d:packages[d['package']['name']]=str(p.parent.relative_to(src))
def flat(s):return html.unescape(s).replace('\\_','_')
def audit(path):
 p=docs/path;b=p.read_bytes();t=b.decode();clean=flat(t)
 definitions=list(dict.fromkeys(re.findall(r'\b(?:fn|struct|enum|trait)\s+(\w+)',clean)))
 testnames=list(dict.fromkeys(re.findall(r'#\[(?:tokio::|sqlx::)?test[^\]]*\]\s*(?:async\s+)?fn\s+(\w+)',clean)))
 matches={n:[x for x in tests if x['name'].split('::')[-1]==n] for n in testnames}
 commands=[]
 for m in re.finditer(r'cargo (?:test|check|build|clippy|fmt)\b[^\n`<]{0,400}',clean):
  c=m.group(0).strip();pkgs=re.findall(r'(?:-p|--package)\s+([A-Za-z0-9_-]+)',c)
  invalid=[p for p in pkgs if p not in packages]
  commands.append(dict(command=c,packages=pkgs,unknown_current_packages=invalid))
 flags=[]
 if 'leptos::ssr::render_to_string' in clean:flags.append('原文 Leptos 0.8 SSR API 路徑不相容；最新程式已用 to_html')
 if any(c['unknown_current_packages'] for c in commands):flags.append('包含不符合目前 Cargo package 名稱的命令；詳見 commands')
 if re.search(r'\[x\].{0,200}未執行',clean):flags.append('歷史勾選項同時寫未執行，不視為成功證據')
 return dict(path=path,bytes=len(b),sha256=hashlib.sha256(b).hexdigest(),code_blocks=len(re.findall(r'^```',t,re.M))//2,
  headings=re.findall(r'^#{1,4} (.+)',t,re.M),definitions=definitions,
  source_definitions={n:symbols.get(n,[]) for n in definitions},named_tests=testnames,test_observations=matches,
  unmatched_test_names=[n for n in testnames if not matches[n]],commands=commands,static_flags=flags)
lessons=catalog['lessons']+[dict(project=p.stem,day=int(p.parent.name.split('-')[1]),article=str(p.relative_to(docs))) for p in docs.glob('docs/teaching/daily/day-*/*.md')]
# Manual behavior/caller review of every canonical lesson; automated symbol matches above are supporting data only.
notes={
'chat':[
('自動化通過','ws.rs 房間 ID 守衛；現有 matching/different/blank 測試與實際 SendMessage 呼叫者已核對。'),
('自動化通過','ws.rs 空白訊息守衛與呼叫順序；非空白／空白測試通過。'),
('自動化通過','persist_then_publish 先 create 再 callback；失敗不發布契約通過。歷史 PG 已改 SQLite。'),
('自動化通過','RoomHub 同房／跨房／訂閱清理契約通過。'),
('歷史內容有差異','原文明示失敗測試只是缺完整 Mock 的骨架；現有完整持久化契約通過，不能把骨架稱可直接編譯 Green。'),
('歷史內容有差異','workspace-tests 與根 tests 路徑已移除；實際契約歸屬 server/chat-application，歷史命令不能直接用。'),
('自動化通過','原文4項事件映射測試未在suite找到；本輪外部probe對原樣library補驗3項schema/穩定ID/隱私/錯誤ID，配合現有寫入失敗不callback契約通過。仍未接真實Analytics投遞。'),
('自動化通過','persist_in_existing_room 存在房／缺房分支與 ws 呼叫者已核對；chat_application 套件名應為 chat-application。'),
('自動化通過','RoomRepository 返回錯誤 ID 時不寫入／不發布；契約通過。'),
('自動化通過','persist_ws_message 使用真正 room repository 與 SQLite；ws_message_requires_room_at_write_time 通過。'),
('自動化通過','Typing 只回到連線房間，shared codec 與 server guard 測試通過。'),
('自動化通過','POST /api/rooms→Application→SQLite 接通；有效／無效／DB錯誤測試及真 HTTP 201 通過；建房仍未授權。'),
('自動化通過','單房查詢→Repository 接通；ID 與 DB 錯誤分支測試通過，不能代替逐房 ACL。'),
('待瀏覽器驗收','列表 API／SQLite 測試通過，RoomList 與真路由存在；互動載入、錯誤重試與 hydration 未實機驗收。'),
('失敗','完整 UI SSR 的 room_page_has_real_room_identity_and_multiline_composer 失敗；仍缺歷史未接通提示。'),
('待瀏覽器驗收','參數化搜尋與 UI helper 可測；本次沒有瀏覽器輸入／刷新驗收，不能沿用舊 SHA 的畫面證據。'),
('待驗收','SQLite schema 及搜尋存在；舊庫 migration checksum 升級／稽核寫入未重跑，不能由 fresh DB 成功推定。'),
('待瀏覽器驗收','刷新失敗保留 rows 的 helper 測試通過；互動 browser error 分支未重跑。'),
('待瀏覽器驗收','refresh_result_view 測試通過；SSR 下 helper 仍為 dead-code，WASM 接線已核對但尚未互動驗收。'),
('後續實作取代','舊登入不可用提示已由 Day28 真表單取代；本次 /signin SSR 與登入 HTTP 通過，不要求舊提示永久保留。'),
('自動化通過','signup→Argon2→SQLite 原生 handler、重複／不合法資料／Origin 契約通過；無註冊信驗收。'),
('自動化通過','帳密→持久 Session→account，真伺服器重啟後 Session 仍可用；不代表 shared Storage Port 全部完成。'),
('自動化通過','signup complete／保持登出與既有 Session 原子撤銷測試通過；真 HTTP 清 cookie 通過。'),
('自動化通過','room_auth→active_member→SQLite；真 GET room 無cookie401／有效cookie200，完整房間 ACL 未完成。'),
('本篇切片通過','共用 token parser 與登出測試；真 HTTP 登出撤銷一個 Session、第二個仍可用，撤銷後 account/room/WS 拒絕。'),
('本篇切片通過','account 與 room guard 共用 active_member；adapter 測試與真 HTTP／伺服器重啟／撤銷回歸通過。'),
('文章須勘誤','最新 SignInPage 與 HTTP 表單通過，但原文測試仍用不存在的 leptos::ssr API；另有 Day16 SSR 回歸失敗。'),
('本篇切片通過','正式 Router 的 guarded_ws 已套用；真 TCP Upgrade 匿名／偽造／過期401、有效101、兩client收發及SQLite持久化通過。'),
],
'pet':[
('失敗','Node scan-preview bridge 測試缺 URLSearchParams，3項同因失敗；WASM 編譯成功不能代替 JS bridge contract。'),
('失敗','scan-preview-no-path 的原樣 Node 測試因 VM 缺 URLSearchParams 失敗；未改來源測試冒稱通過。'),
('自動化通過','PetProfile 五欄／可選覆寫純 Domain 契約通過；不代表設定 UI／素材已生成。'),
('自動化通過','ProfileCodec roundtrip／非法設定契約通過；原文部分 test package 路徑已改歸 desktop-pet。'),
('自動化通過','預覽非成功及狀態契約通過；純模型不代表真實檔案操作成功。'),
('歷史內容有差異','根 workspace-tests 已不存在；cleanup 狀態測試歸屬 desktop-pet，真 OS receipt 仍未提供。'),
('自動化通過','成功事件 ID 穩定／無私人路徑契約通過；未接真實清理與投遞。'),
('自動化通過','resolve_appearance 覆寫優先／預設回退測試通過；pet_domain 命令應改 pet-domain。'),
('自動化通過','resolve_validated_appearance 驗證後再合併契約通過。'),
('失敗','後端授權 root／SHA256 純測試通過，duplicate-preview Node bridge 因 VM 缺 URLSearchParams 失敗；真 IPC 未實機驗收。'),
('功能有差異','roam_enabled 開關測試通過；runtime 仍只找不存在的固定 pet 視窗，動態 pet-id 不會被此 loop 移動。'),
('待實機驗收','duplicate_preview UI parser 測試與 WASM 成功；真正掃描IPC→畫面未重跑，bridge Node 測試失敗另記。'),
('待實機驗收','file_scan_view 摘要／隱私丟棄契約通過；真正授權目錄到GUI尚未重跑。'),
('待實機驗收','PreviewMachine 的確認／取消純狀態通過；不代表真OS檔案操作或UI完整流程。'),
('功能失敗','控制窗 main CloseRequested 未攔截，open_control_window 只找舊窗不重建；控制窗關閉後不能恢復。'),
('功能有差異','roaming_window_label 固定 pet，與 pet_windows::label 的 pet-UUID 不同；測試只驗固定字串，不能證明生活窗漫遊。'),
('功能有差異','舊 enabled/boundary/close GUI 證據不屬本次 SHA；目前漫遊仍未接動態 pet-id。'),
('待實機驗收','unexpected_displacement 邊界純測試通過，但動態生活窗漫遊未接通，拖曳／多螢幕未驗收。'),
('待實機驗收','no_roaming_space 幾何純測試通過；runtime 動態窗目標差異仍在，不能標完成。'),
('待實機驗收','normalize_new_pet_name 與註冊 command 存在、測試通過；真正表單→IPC未互動驗收。'),
('待實機驗收','PetStore::create 及 SQLite 測試通過；控制台建立的真 IPC／畫面未重跑。'),
('待實機驗收','list/create 資料庫 pool reopen 測試通過；控制窗恢復缺陷仍影響完整使用者流程。'),
('待實機驗收','三個id獨立Runtime／標籤／SQLite選取測試通過；本SHA三窗GUI／關閉／重啟未實機驗收。'),
('待實機驗收','pet-* capability 契約通過；真WebView事件監聽與睡眠畫面未實機驗收，FIFO保持pending。'),
('待實機驗收','PetStatus 單一img class 測試通過；三個id逐幀／GUI素材與事件畫面未驗收。'),
('待實機驗收','create-and-select SQLite pool reopen 測試通過；Tauri出籠與真App重啟未實機驗收。'),
('待實機驗收','set_caged/active/select 資料庫測試通過；真App重啟、回籠後GUI尚未重跑。'),
('待實機驗收','先set_caged再close/remove已套用且native測試通過；DB注入失敗與關窗失敗的真GUI未驗收，不能封存為完成。'),
],
'analytics':[
('後續實作取代','valid_event 契約仍在；原文有效501屬舊stub，現行授權持久化成功201/200，不能倒退到501。'),
('自動化通過','valid_event_type 長度／空白契約通過；當年501說明屬歷史。'),
('後續實作取代','現行JSON/body限制仍有測試；原樣HTTP 501測試已被授權與PG ACK契約取代。'),
('自動化通過','source+event_id 記憶體冪等契約通過，不等於跨程序資料庫唯一鍵。'),
('自動化通過','同來源重複／跨來源獨立契約通過，真正持久化另外以PG驗。'),
('歷史內容有差異','workspace-tests 已移除；idempotency_contract 現歸 stats-server，PG target 本次有實跑而非0 tests。'),
('後續實作取代','schema_version=1與JSON契約仍受驗證；原樣合法501已由授權PG攝取取代。'),
('自動化通過','event_type trim/128bytes 不變量測試通過；stats_server package 命令失效。'),
('自動化通過','先valid_event再accept_once，不合法鍵不污染記憶體集合，既有測試通過。'),
('自動化通過','postgres_event_store 真PG測試通過：唯一鍵、同key內容衝突、重送結果。'),
('自動化通過','ingest→來源token→persist_event→PG ACK真測試通過；不是gRPC/SSE。'),
('功能有差異','source_summary_tests 驗 #[cfg(test)] helper；production summary 已改 Session admin，不能以舊Bearer summary測試宣稱現行路由通過。'),
('功能失敗','按事件型別 /summary/{event_type} 在正式 password_login::build 沒掛載；legacy admin router與helper測試不代表正式入口。'),
('後續實作取代','舊source token Dashboard已由管理員Session後台取代，主線不再要求人員輸入服務憑證。'),
('待驗收','正式Dashboard已採Leptos；login/change-password仍Axum字串HTML，不能把舊legacy Leptos/管理功能當整體UI完成。'),
('功能有差異','admin::touch_session 純測試仍通過但主入口不用admin::build；password_login有另一PG續期，不能直接繼承舊response條件契約。'),
('功能有差異','舊admin response-layer續期測試不是目前password_login路由；新session先更新PG，錯誤回應時的時鐘一致性待驗。'),
('功能失敗','成功登入失敗額度測試只測legacy admin；目前password_login沒有同等attempts政策。'),
('功能失敗','Retry-After 的429只在legacy admin路徑；目前帳密login無此限流，不能沿用舊測試綠燈。'),
('後續實作取代','admin username+key表單已由真Argon2帳密流程取代；舊helper測試通過不等於完整登入防護。'),
('自動化通過','bootstrap CLI真執行於隔離PG；初始化鎖表／不覆寫／加鹽雜湊測試通過。'),
('自動化通過','PG Session登入／pool reopen／撤銷測試與真HTTP首次受限登入通過；正式UI仍有字串HTML。'),
('自動化通過','PG改密碼原子／撤銷所有Session／duplicate migration測試通過；真HTTP改密與舊Session拒絕通過。'),
('自動化通過','完成初始化後仍能GET改密頁、重新改密並撤銷Session，PG測試通過。'),
('本篇切片通過','完整fresh PG migrations→bootstrap CLI→持久受限Session→首次改密→撤銷→重新登入真HTTP通過。'),
('本篇切片通過','正式password_login路由／PG非零兩來源計數測試與真HTTP切換／unknown400／未登入redirect通過。'),
('文章須勘誤','正式Dashboard Leptos to_html及PG/HTTP通過，但原文Green仍有過時SSR API；不能稱原文可原樣套用。'),
('本篇切片通過','正式HTTP兩來源僅作用中連結有aria-current，unknown400、未登入redirect通過；原文stats_server命令須在版本化勘誤改stats-server。'),
]}
assert all(len(v)==28 for v in notes.values())
rows=[]
for lesson in sorted(lessons,key=lambda x:(x['day'],x['project'])):
 p=lesson['article'];a=audit(p);verdict,note=notes[lesson['project']][lesson['day']-2]
 a.update(project=lesson['project'],day=lesson['day'],verdict=verdict,assessment=note,
  source_sha=sha,document_baseline_sha=docsha,scope='current main behavior; historical Red evidence is not re-created',
  archive_eligible=verdict=='本篇切片通過')
 a['version_paths']=list(dict.fromkeys([p]+[x['path'] for x in lesson.get('sources',[])]))
 rows.append(a)
all_docs=[]
for p in sorted((docs/'docs').rglob('*.md')):all_docs.append(audit(str(p.relative_to(docs))))
result=dict(source_sha=sha,document_baseline_sha=docsha,canonical_lesson_count=len(rows),canonical_days=[2,29],
 missing_day1='No Day1 canonical body exists in current GitHub catalog or tracked docs; Notion not used.',
 methodology='Read all tracked Markdown bytes and code blocks; static command/API/symbol/test crosswalk; manually review each of 84 canonical lessons against current callers; run current suite and isolated HTTP/DB probes. Symbol matches never constitute functional acceptance. Original Red and historical CI not re-run.',
 lessons=rows,supplements=[audit(x['path'])|{'title':x['title'],'verdict':'歷史補充；不作現行規則或新功能驗收'} for x in catalog['supplements']],
 all_markdown_count=len(all_docs),all_markdown=all_docs,
 status_counts=dict(collections.Counter(x['verdict'] for x in rows)))
(out/'all-articles-validation.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
lines=['# 三案全部文章驗證矩陣','',f'程式 `{sha}`；文件基準 `{docsha}`。', '',
 f'全部 {len(rows)} 篇主文（每案28篇，Day2–Day29）逐篇核對；另讀取全部 {len(all_docs)} 份 docs Markdown（含鏡像、修訂、索引、歷史證據），逐檔雜湊與靜態掃描見 JSON。後者不是宣稱483份文件均做過獨立功能E2E。',
 '', 'Day1 正文未在本次 GitHub 文庫找到，列來源缺口，不以 Notion 或相似標題補造。歷史Red與舊CI沒有重新執行；原文「未套用／未執行」保留當時語境，本表另記最新程式。', '',
 '「自動化通過」只表示該列描述的契約／單元範圍。只有「本篇切片通過」可在本輪新增完成封存；待實機／有差異／原文須勘誤不當作功能完成。既有封存原文不回寫。','',
 '| Day | Chat | Pet | Analytics |','| --- | --- | --- | --- |']
for day in range(2,30):
 cells=[]
 for project in ['chat','pet','analytics']:
  a=next(x for x in rows if x['day']==day and x['project']==project)
  cells.append(f"**{a['verdict']}**：{a['assessment']}（[原文](https://github.com/williechen/copilot_use/blob/{docsha}/{a['path']})）")
 lines.append('| '+str(day)+' | '+' | '.join(cells)+' |')
lines+=['','## 逐篇證據與靜態掃描','',
 '同目錄 `all-articles-validation.json` 每列含全文 SHA256、程式區塊數、文中符號對應的來源檔案／行號、具名測試對應本輪 log 的精確名字與結果、尚未匹配的測試名、Cargo 命令及現行 package 差異。未匹配或多義符號不自動判失敗或成功；由上表呼叫鏈核對說明判定。', '',
 '歷史候選／舊來源中的測試名稱與目前不同時，保留差異；沒有把0 tests當Pass，沒有直接執行歷史文章中的DB URL或修改指令。全部歷史鏡像與11份補充頁的內容已讀取、雜湊及檢查，但不將索引／規則文字當可執行程式。']
(out/'ALL_ARTICLES.md').write_text('\n'.join(lines)+'\n')
print(json.dumps({'canonical':len(rows),'markdown':len(all_docs),'counts':result['status_counts'],'archive':[(x['project'],x['day']) for x in rows if x['archive_eligible']]},ensure_ascii=False))
