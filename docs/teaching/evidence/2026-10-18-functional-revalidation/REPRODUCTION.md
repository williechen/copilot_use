# Day18 functional acceptance fixtures

Use only disposable local databases and fixture credentials. These checks do not
call paid AI. A passing fixture is evidence for its stated scope, not full product
acceptance (account/ACL, multiple pets, multiple monitors and browser lifetime
requirements remain separate gates).

## Rust and PostgreSQL

```sh
cargo test --locked -p ui --features ssr -p stats-server -p desktop-pet -p server -p desktop-pet-frontend
ANALYTICS_TEST_DATABASE_URL=postgres://day18@127.0.0.1:55418/day18 cargo test --locked -p stats-server -- --include-ignored
cargo leptos build --cargo-locked
```

Create a dedicated PostgreSQL database first. The ignored login/query test creates
and drops its own schema. Never substitute a production database URL.

`day18_tcp_cookie_and_idle_expiry_move_together` sends real loopback TCP/HTTP,
continues after the initial 2-second fixture deadline, then verifies a real
30-millisecond idle deadline expires. Production TTL remains 1800 seconds; this
does not claim a 30-minute browser soak was executed.

## Live HTTP and browser

Start the Chat server against an empty disposable SQLite file. Run
`python3 scripts/acceptance/day18_http.py chat` once to create three fixture rooms.
The Analytics server must use a disposable migrated PostgreSQL database,
`fixture-chat` / `fixture-pet` service tokens and a login key of 64 `a` characters.
Its fixture SHA256 is
`ffe054fe7ae0cb6dc65c3af9b61d5209f439851db43d0ba5997337df154668eb`.

```sh
python3 scripts/acceptance/day18_http.py analytics require-renewal
```

For browser HTTP evidence, create `work/` and `outputs/` in a dedicated acceptance
directory, set `DAY18_ACCEPTANCE_DIR` to it and run
`python3 scripts/acceptance/day18_chat_proxy.py`. Browse port 3002. Creating its
`work/rooms-fail` flag makes only GET `/api/rooms` return 503; remove the flag to
retry. The proxy records HTTP paths/status in `outputs/chat-network.jsonl`.
It does not proxy WebSockets; test chat/WS directly on port 3000.

Verify blank/all, Chinese partial names with whitespace, case-insensitive names,
no match, preserved IDs/links, HTTP 503 with prior list preserved and retry.

## Native geometry

```sh
cargo build --locked -p desktop-pet --features acceptance-probe
PET_ACCEPTANCE_SCENARIO=enabled PET_ENABLE_ROAM=true LIBGL_ALWAYS_SOFTWARE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 target/debug/desktop-pet
PET_ACCEPTANCE_SCENARIO=disabled PET_ENABLE_ROAM=false LIBGL_ALWAYS_SOFTWARE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 target/debug/desktop-pet
PET_ACCEPTANCE_SCENARIO=boundary PET_ENABLE_ROAM=true LIBGL_ALWAYS_SOFTWARE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 target/debug/desktop-pet
PET_ACCEPTANCE_SCENARIO=close PET_ENABLE_ROAM=true LIBGL_ALWAYS_SOFTWARE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 target/debug/desktop-pet
```

The opt-in probe maps real Tauri windows, waits for window-manager placement,
reads mapped size/work area, positions test windows, then invokes the actual
`roam_desktop` algorithm. It samples native `pet` and `main` positions, verifies
boundaries/disabled state, and checks closing pet never falls back to main.
It logs JSON evidence and exits 0 only for a passing scenario. Normal builds do
not include this instrumentation. The probe delays roaming until fixture mapping
completes; it does not verify normal startup timing, dragging, visual rendering,
runtime enable/disable controls, multiple monitors or multiple pets.
