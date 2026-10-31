# Analytics 63b1f93 採用核對（2026-10-31）

產品最新精確SHA `63b1f936b32f131f0f801bb4f36b3163f88c91fa`，commit `add sql`。相較64eff527只新增`analytics_apps/migrations/20261031074500_viewer_access.sql`，不是已採用整個Day45候選。產品庫保持唯讀，本報告及補正patch只寫copilot_use。

## 已採用的schema定義與使用者配置

- category仍為會員管理、資訊分析、認證管理，認證管理ID採用使用者的`analytics-token`。
- 自身密碼page歸會員管理；新增管理憑證`analytics-security`，path `/security`，歸認證管理。
- SQL建立3個category、9個page、36個CRUD action。以既有acl_parent_id連接，role與個人加減權表仍保留。

精確blob／SHA256及七份migration來源見[source-inventory.json](source-inventory.json)。這表示schema程式已入庫，不表示使用者的任何DB已實際執行migration。

## 仍未採用的程式與本次發現

正式lib.rs沒有accounts／page_acl組裝，password_login仍hardcode admin；bootstrap尚未補角色指派，persistence沒有新增Session lookup模組。頁面、來源及CRUD的R3接線未採用。空庫migration在bootstrap之前執行時，不會預先替尚不存在的admin產生uac_user_role；後續接role-aware Session時必須一併接上bootstrap修正。

管理員已取得`analytics-security`四個action，但缺少該page本身的role_acl。依page與action分別授權的契約，未來接線後仍無法進管理憑證頁；目前也尚無該頁正式handler，不宣稱實際可用。

原Day45 r3完整patch以64eff527為基準，包含使用者現已調整的同名migration，**不得直接套到63b1f93**。原文及舊SHA測試保留供重現；後續完整候選須以63b1f93重新接線，保留analytics-token、密碼page歸屬及security頁。不能改已提交／已執行migration來解衝突。

## 隔離驗證：實際命令與限制

本次用專用PG18.6、127.0.0.1:55485、DB `adoption_63b1f93`，依檔名順序對從精確SHA取得的7份SQL執行：

```sh
psql -X -h127.0.0.1 -p55485 -d adoption_63b1f93 -v ON_ERROR_STOP=1 -f <exact-sha-migration.sql>
```

7份各exit0，結果category3/page9/action36；管理憑證page的admin_grant=false而四action=true。真實輸出見[migration.log](migration.log)、[exit](migration-exits.json)、[inspection.log](inspection.log)。這是raw SQL fresh DB驗證，未跑SQLx migration runner／歷史checksum、既有舊庫升級、Rust本地全套、正式瀏覽器或CRUD流程。原R3候選62通過／1ignored不追認為63b1f93結果。

來源Rust Analytics workflow已success，但其兩個job是build-analytics與check-analytics，沒有ACL旅程測試；其他CI狀態以[本次快照](ci.json)為準，未手動重跑CI，不等待或假稱全數成功。

## 向前補正方案：只補管理憑證page預設授權

附件[analytics-security-page-forward.patch](analytics-security-page-forward.patch)，基準63b1f93；新增下一份migration，不重寫使用者原檔，也不新增runtime／CRUD handler。完整內容：

```diff
diff --git a/analytics_apps/migrations/20261031094500_grant_security_page.sql b/analytics_apps/migrations/20261031094500_grant_security_page.sql
new file mode 100644
--- /dev/null
+++ b/analytics_apps/migrations/20261031094500_grant_security_page.sql
@@ -0,0 +1,12 @@
+-- Forward-only correction: preserve the already-published viewer_access migration checksum.
+DO $$
+BEGIN
+    IF NOT EXISTS (SELECT 1 FROM uac_role WHERE id='analytics-admin' AND role_status='active')
+       OR NOT EXISTS (SELECT 1 FROM uac_acl WHERE id='analytics-security'
+           AND acl_category='page' AND acl_parent_id='analytics-token' AND acl_status='active') THEN
+        RAISE EXCEPTION 'Expected Analytics admin role and security page are missing or inactive';
+    END IF;
+END $$;
+INSERT INTO uac_role_acl(role_id,acl_id,created_at)
+VALUES('analytics-admin','analytics-security',now())
+ON CONFLICT (role_id,acl_id) DO NOTHING;
```

在隔離fresh SQL庫以`psql -1 -v ON_ERROR_STOP=1 -f 20261031094500_grant_security_page.sql`執行兩次均exit0，page與四action的admin_grants均為1，3／9／36節點數不變。見[補正執行](forward-fix.log)與[結果](forward-inspection.log)。補正patch在隔離的精確migration來源目錄`git apply --check` exit0；尚未由使用者套用到產品。

## 判定與下一步

schema定義部分採用；完整runtime未採用；補正方案已驗raw SQL但未採用；完整Gate2=false。先由使用者決定採用新增migration，再以63b1f93產製只補缺失runtime與向前相容內容的完整候選，補正式Router／Session／CRUD與使用者配置測試。next_day三案46不變，FIFO與last_full_comparison不變，不重發Day或覆寫原Day45正文。
