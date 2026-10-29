本輪執行腳本保存於本目錄；每個HTTP seed需空fixture DB。
{
  "recorded_at": "2026-10-19T00:21:38.348478+08:00",
  "main_sha": "2c7ace259ef790cea303d841f5779cd5f022d554",
  "candidate_sha": "1abcb8aa6bc279e888a057b87069bea788281c7c",
  "candidate_branch": "codex/revalidate-f9484f2",
  "teaching_base_sha": "3f43cf8cca2c7fcc13e989bd43fb460efbd72e36",
  "main_workspace": {
    "passed": 126,
    "failed": 1,
    "ignored": 0
  },
  "candidate_workspace": {
    "passed": 131,
    "failed": 0,
    "ignored": 0
  },
  "main_leptos": "passed",
  "pet_frontend": "passed",
  "native_build": "passed",
  "main_fmt": "passed",
  "main_strict_clippy": "failed: admin.rs logout collapsible_if",
  "candidate_fmt_and_clippy": "passed; probe feature also strict clippy passed",
  "main_http_search_cases": 12,
  "candidate_http_search_cases": 12,
  "main_cookie_refresh_guard_cases": 10,
  "websocket_cases": 7,
  "schema_cases": 60,
  "native_geometry_cases": 4,
  "native_scope": "instrumented candidate only; original roam core unchanged",
  "notion_readback_pages": 10,
  "rules_changed": false,
  "day_advanced": false,
  "main_modified": false,
  "formal_full_acceptance": false,
  "product_copilot_retested": false,
  "ci": {
    "success": 16,
    "failed": 0,
    "unfinished": 1
  }
}

實際路徑：/home/willie/Documents/rust_ai_chat；候選：本任務work/acceptance-f948。共享CARGO_TARGET_DIR=/home/willie/Documents/rust_ai_chat/target，CARGO_INCREMENTAL=0。main與candidate先各自完整test，再build／freeze，HTTP使用各自凍結檔。原生測試timeout只停止自有fixture，PostgreSQL只啟停本任務pg-full63、port55463。
