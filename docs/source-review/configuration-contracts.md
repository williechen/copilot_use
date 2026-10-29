# 非 Rust 契約與入口索引

Rust types/serde/signatures 見 interface-catalog；本檔完整收錄 tracked manifests、migrations、proto、CI/scripts/config 的身分。歷史文件和圖像只列身分，不推論目前執行結果。

## reference-rust_ai_chat/.env.example

SHA256 `f4b3cf3c05f557f4dfc48a24d95974b7af0eb508a6888ef85a62e5225a35e371`。

```
1: RUST_LOG=debug
2:
3: DATABASE_POSTGR_URL=postgres://username:password@localhost:5432/database_name
4: DATABASE_SQLITE_URL=sqlite://database_name.db
5:
6: PET_AUTHORIZED_ROOT="\\Users\\Downloads"
7:
8:
```

## reference-rust_ai_chat/.github/workflows/ai_chat.yml

SHA256 `a0591a9d6383e32c38710e14d45c53263da6a16cd2512dea83dbedbf7014f80e`。

```
1: name: Rust AI Chat
2:
3: on:
4:   push:
5:     branches: ["main"]
6:   pull_request:
7:     branches: ["main"]
8:
9: env:
10:   CARGO_TERM_COLOR: always
11:
12: jobs:
13:   check-chat:
14:     runs-on: ubuntu-latest
15:
16:     steps:
17:       - uses: actions/checkout@v4
18:
19:       - name: Install Rust
20:         uses: dtolnay/rust-toolchain@stable
21:         with:
22:           targets: wasm32-unknown-unknown
23:
24:       - name: Install Linux dependencies
25:         run: |
26:           sudo apt-get update
27:           sudo apt-get install -y \
28:             build-essential \
29:             pkg-config
30:
31:       - name: Install cargo-leptos
32:         run: cargo install cargo-leptos --locked
33:
34:       - name: Check server ssr
35:         run: cargo check --locked -p server --features ssr
36:
37:       - name: Check ui ssr
38:         run: cargo check --locked -p ui --features ssr
39:
40:       - name: Check ui
41:         run: cargo check --locked -p ui --features hydrate --target wasm32-unknown-unknown
42:
43:   build-chat:
44:     runs-on: ubuntu-latest
45:
46:     steps:
47:       - uses: actions/checkout@v4
48:
49:       - name: Install Rust
50:         uses: dtolnay/rust-toolchain@stable
51:         with:
52:           targets: wasm32-unknown-unknown
53:
54:       - name: Install Linux dependencies
55:         run: |
56:           sudo apt-get update
57:           sudo apt-get install -y \
58:             build-essential \
59:             pkg-config
60:
61:       - name: Install cargo-leptos
62:         run: cargo install cargo-leptos --locked
63:
64:       - name: Build server
65:         run: cargo leptos build --cargo-locked --release
```

## reference-rust_ai_chat/.github/workflows/dashboard.yml

SHA256 `5701caea4eb6572eca37b16b567d9fae94483c6535dcb298c4793059e3c71ed3`。

```
1: name: Rust Dashboard
2:
3: on:
4:   push:
5:     branches: ["main"]
6:   pull_request:
7:     branches: ["main"]
8:
9: env:
10:   CARGO_TERM_COLOR: always
11:
12: jobs:
13:   check-dashboard:
14:     runs-on: ubuntu-latest
15:
16:     steps:
17:       - uses: actions/checkout@v4
18:
19:       - name: Install Rust
20:         uses: dtolnay/rust-toolchain@stable
21:         with:
22:           targets: wasm32-unknown-unknown
23:
24:       - name: Install Linux dependencies
25:         run: |
26:           sudo apt-get update
27:           sudo apt-get install -y \
28:             build-essential \
29:             pkg-config
30:
31:       - name: Check stats server
32:         run: cargo check --locked -p stats-server
33:
34:   build-dashboard:
35:     runs-on: ubuntu-latest
36:
37:     steps:
38:       - uses: actions/checkout@v4
39:
40:       - name: Install Rust
41:         uses: dtolnay/rust-toolchain@stable
42:         with:
43:           targets: wasm32-unknown-unknown
44:
45:       - name: Install Linux dependencies
46:         run: |
47:           sudo apt-get update
48:           sudo apt-get install -y \
49:             build-essential \
50:             pkg-config
51:
52:       - name: Build stats server
53:         run: cargo build --locked -p stats-server --release
```

## reference-rust_ai_chat/.github/workflows/desktop_pet.yml

SHA256 `5faefb71485c0d23e76402a5c3f946254b07ce317000e5fc2baefd709ec03f4c`。

```
1: name: Rust Desktop Pet
2:
3: on:
4:   push:
5:     branches: ["main"]
6:   pull_request:
7:     branches: ["main"]
8:
9: env:
10:   CARGO_TERM_COLOR: always
11:
12: jobs:
13:   check-pet:
14:     runs-on: ubuntu-latest
15:
16:     steps:
17:       - uses: actions/checkout@v4
18:
19:       - name: Install Rust
20:         uses: dtolnay/rust-toolchain@stable
21:         with:
22:           targets: wasm32-unknown-unknown
23:
24:       - name: Install Linux dependencies
25:         run: |
26:           sudo apt-get update
27:           sudo apt-get install -y \
28:             build-essential \
29:             pkg-config \
30:             curl \
31:             wget \
32:             file \
33:             libssl-dev \
34:             libxdo-dev \
35:             librsvg2-dev \
36:             libwebkit2gtk-4.1-dev
37:
38:       - name: Check native dependencies
39:         run: |
40:           echo "GLib:"
41:           pkg-config --modversion glib-2.0
42:
43:           echo "GTK:"
44:           pkg-config --modversion gtk+-3.0
45:
46:           echo "libsoup:"
47:           pkg-config --modversion libsoup-3.0
48:
49:           echo "JavaScriptCore:"
50:           pkg-config --modversion javascriptcoregtk-4.1
51:
52:           echo "WebKitGTK:"
53:           pkg-config --modversion webkit2gtk-4.1
54:
55:       - name: Install trunk
56:         run: cargo install trunk --locked
57:
58:       - name: Check native workspace
59:         run: cargo check --workspace --exclude desktop-pet-frontend
60:
61:       - name: Check desktop frontend
62:         run: cargo check --locked -p desktop-pet-frontend --target wasm32-unknown-unknown
63:
64:   build-pet:
65:     runs-on: ubuntu-latest
66:
67:     steps:
68:       - uses: actions/checkout@v4
69:
70:       - name: Install Rust
71:         uses: dtolnay/rust-toolchain@stable
72:         with:
73:           targets: wasm32-unknown-unknown
74:
75:       - name: Install Linux dependencies
76:         run: |
77:           sudo apt-get update
78:           sudo apt-get install -y \
79:             build-essential \
80:             pkg-config \
81:             curl \
82:             wget \
83:             file \
84:             libssl-dev \
85:             libxdo-dev \
86:             librsvg2-dev \
87:             libwebkit2gtk-4.1-dev
88:
89:       - name: Check native dependencies
90:         run: cargo build --workspace --exclude desktop-pet-frontend --verbose
91:
92:       - name: Install trunk
93:         run: cargo install trunk --locked
94:
95:       - name: Build desktop
96:         working-directory: apps/desktop_pet/frontend
97:         run: trunk build --locked --release
```

## reference-rust_ai_chat/.github/workflows/node.yml

SHA256 `4a9d92b11d010ef2108d0c4e2dd58e01fa46431996b04250da7202bfeeb51a3d`。

```
1: name: Node
2:
3: on:
4:   push:
5:     branches: ["main"]
6:   pull_request:
7:     branches: ["main"]
8:
9: env:
10:   CARGO_TERM_COLOR: always
11:
12: jobs:
13:   test-node:
14:     runs-on: ubuntu-latest
15:
16:     steps:
17:       - uses: actions/checkout@v4
18:
19:       - name: Install Node.js
20:         uses: actions/setup-node@v3
21:         with:
22:           node-version: '24'
23:
24:       - name: Run Node.js tests
25:         run: node --test
```

## reference-rust_ai_chat/.github/workflows/persistence.yml

SHA256 `92037ed5785f76e0b08f0c6b00a1bf604d390dbe93ba8c3cad0b66ebb80f2083`。

```
1: name: Persistence
2:
3: on:
4:   push:
5:     branches: ["main"]
6:   pull_request:
7:     branches: ["main"]
8:
9: env:
10:   CARGO_TERM_COLOR: always
11:
12: jobs:
13:   test-persistence:
14:     runs-on: ubuntu-latest
15:
16:     services:
17:       postgres:
18:         image: postgres:latest
19:         env:
20:           POSTGRES_USER: postgres
21:           POSTGRES_PASSWORD: postgres
22:           POSTGRES_DB: rust_ai_chat_test
23:         ports:
24:           - 5432:5432
25:         options: >-
26:           --health-cmd "pg_isready -U postgres -d rust_ai_chat_test"
27:           --health-interval 10s
28:           --health-timeout 5s
29:           --health-retries 5
30:
31:     env:
32:       DATABASE_PG_URL: postgres://postgres:postgres@localhost:5432/rust_ai_chat_test
33:       DATABASE_SQLITE_URL: "sqlite::memory:"
34:
35:     steps:
36:       - uses: actions/checkout@v4
37:
38:       - name: Install Rust
39:         uses: dtolnay/rust-toolchain@stable
40:
41:       - name: Install sqlx-cli
42:         run: cargo install sqlx-cli --version "^0.9" --no-default-features --features postgres,sqlite --locked
43:
44:       - name: Run database migrations
45:         run: sqlx migrate run --database-url $DATABASE_SQLITE_URL
46:
47:       - name: Test persistence
48:         run: DATABASE_URL="$DATABASE_SQLITE_URL" cargo test --locked -p persistence
```

## reference-rust_ai_chat/.github/workflows/test.yml

SHA256 `5ba18e056bee72d5da10cb404ec173d3b990bd6bbbd92bd8f46d8da9592ccb79`。

```
1: name: Test All
2:
3: on:
4:   push:
5:     branches: ["main"]
6:   pull_request:
7:     branches: ["main"]
8:
9: env:
10:   CARGO_TERM_COLOR: always
11:
12: jobs:
13:   test-workspace:
14:     runs-on: ubuntu-latest
15:
16:     steps:
17:       - uses: actions/checkout@v4
18:
19:       - name: Install Rust
20:         uses: dtolnay/rust-toolchain@stable
21:         with:
22:           targets: wasm32-unknown-unknown
23:
24:       - name: Install Linux dependencies
25:         run: |
26:           sudo apt-get update
27:           sudo apt-get install -y \
28:             build-essential \
29:             pkg-config \
30:             curl \
31:             wget \
32:             file \
33:             libssl-dev \
34:             libxdo-dev \
35:             librsvg2-dev \
36:             libwebkit2gtk-4.1-dev
37:
38:       - name: Test workspace
39:         run: cargo test --locked -p workspace-tests
40:
41:   test-desktop-pet:
42:     runs-on: ubuntu-latest
43:
44:     steps:
45:       - uses: actions/checkout@v4
46:
47:       - name: Install Rust
48:         uses: dtolnay/rust-toolchain@stable
49:         with:
50:           targets: wasm32-unknown-unknown
51:
52:       - name: Install Linux dependencies
53:         run: |
54:           sudo apt-get update
55:           sudo apt-get install -y \
56:             build-essential \
57:             pkg-config \
58:             curl \
59:             wget \
60:             file \
61:             libssl-dev \
62:             libxdo-dev \
63:             librsvg2-dev \
64:             libwebkit2gtk-4.1-dev
65:
66:       - name: Test desktop-pet
67:         run: cargo test --locked -p desktop-pet
68:
69:   test-server:
70:     runs-on: ubuntu-latest
71:
72:     steps:
73:       - uses: actions/checkout@v4
74:
75:       - name: Install Rust
76:         uses: dtolnay/rust-toolchain@stable
77:         with:
78:           targets: wasm32-unknown-unknown
79:
80:       - name: Install Linux dependencies
81:         run: |
82:           sudo apt-get update
83:           sudo apt-get install -y \
84:             build-essential \
85:             pkg-config
86:
87:       - name: Test server
88:         run: cargo test --locked -p server
89:
90:   test-stats-server:
91:     runs-on: ubuntu-latest
92:
93:     steps:
94:       - uses: actions/checkout@v4
95:
96:       - name: Install Rust
97:         uses: dtolnay/rust-toolchain@stable
98:         with:
99:           targets: wasm32-unknown-unknown
100:
101:       - name: Install Linux dependencies
102:         run: |
103:           sudo apt-get update
104:           sudo apt-get install -y \
105:             build-essential \
106:             pkg-config
107:
108:       - name: Test stats server
109:         run: cargo test --locked -p stats-server
110:
111:   test-ai-core:
112:     runs-on: ubuntu-latest
113:
114:     steps:
115:       - uses: actions/checkout@v4
116:
117:       - name: Install Rust
118:         uses: dtolnay/rust-toolchain@stable
119:         with:
120:           targets: wasm32-unknown-unknown
121:
122:       - name: Install Linux dependencies
123:         run: |
124:           sudo apt-get update
125:           sudo apt-get install -y \
126:             build-essential \
127:             pkg-config
128:
129:       - name: Test ai core
130:         run: cargo test --locked -p ai-core
131:
132:   test-chat-application:
133:     runs-on: ubuntu-latest
134:
135:     steps:
136:       - uses: actions/checkout@v4
137:
138:       - name: Install Rust
139:         uses: dtolnay/rust-toolchain@stable
140:         with:
141:           targets: wasm32-unknown-unknown
142:
143:       - name: Install Linux dependencies
144:         run: |
145:           sudo apt-get update
146:           sudo apt-get install -y \
147:             build-essential \
148:             pkg-config
149:
150:       - name: Test chat application
151:         run: cargo test --locked -p chat-application
152:
153:   test-chat-domain:
154:     runs-on: ubuntu-latest
155:
156:     steps:
157:       - uses: actions/checkout@v4
158:
159:       - name: Install Rust
160:         uses: dtolnay/rust-toolchain@stable
161:         with:
162:           targets: wasm32-unknown-unknown
163:
164:       - name: Install Linux dependencies
165:         run: |
166:           sudo apt-get update
167:           sudo apt-get install -y \
168:             build-essential \
169:             pkg-config
170:
171:       - name: Test chat domain
172:         run: cargo test --locked -p chat-domain
173:
174:   test-pet-domain:
175:     runs-on: ubuntu-latest
176:
177:     steps:
178:       - uses: actions/checkout@v4
179:
180:       - name: Install Rust
181:         uses: dtolnay/rust-toolchain@stable
182:         with:
183:           targets: wasm32-unknown-unknown
184:
185:       - name: Install Linux dependencies
186:         run: |
187:           sudo apt-get update
188:           sudo apt-get install -y \
189:             build-essential \
190:             pkg-config
191:
192:       - name: Test pet domain
193:         run: cargo test --locked -p pet-domain
194:
195:   test-shared:
196:     runs-on: ubuntu-latest
197:
198:     steps:
199:       - uses: actions/checkout@v4
200:
201:       - name: Install Rust
202:         uses: dtolnay/rust-toolchain@stable
203:         with:
204:           targets: wasm32-unknown-unknown
205:
206:       - name: Install Linux dependencies
207:         run: |
208:           sudo apt-get update
209:           sudo apt-get install -y \
210:             build-essential \
211:             pkg-config
212:
213:       - name: Test shared
214:         run: cargo test --locked -p shared
215:
216:   test-ui:
217:     runs-on: ubuntu-latest
218:
219:     steps:
220:       - uses: actions/checkout@v4
221:
222:       - name: Install Rust
223:         uses: dtolnay/rust-toolchain@stable
224:         with:
225:           targets: wasm32-unknown-unknown
226:
227:       - name: Install Linux dependencies
228:         run: |
229:           sudo apt-get update
230:           sudo apt-get install -y \
231:             build-essential \
232:             pkg-config
233:
234:       - name: Test ui
235:         run: cargo test --locked -p ui
```

## reference-rust_ai_chat/Cargo.toml

SHA256 `e448239352cf44b3be00dc870a4705cb5c318291452013a516546dbcb235bfdc`。

```
1: [workspace]
2: resolver = "3"
3: members = [
4:     "apps/desktop_pet",
5:     "apps/desktop_pet/frontend",
6:     "apps/server",
7:     "apps/stats_server",
8:     "crates/ai_core",
9:     "crates/chat_application",
10:     "crates/chat_domain",
11:     "crates/shared",
12:     "crates/persistence",
13:     "crates/pet_domain",
14:     "crates/telemetry",
15:     "crates/ui",
16:     "tests",
17: ]
18:
19: [workspace.package]
20: version = "0.2.7"
21: edition = "2024"
22:
23: [workspace.dependencies]
24: any_spawner = { version = "0.3", features = ["tokio"] }
25: async-trait = "0.1"
26: axum = { version = "0.8", features = ["ws"] }
27:
28: dotenv = "0.15"
29:
30: leptos = "0.8"
31: leptos_axum = "0.8"
32: leptos_config = "0.8"
33: leptos_router = "0.8"
34:
35: libc = "0.2"
36:
37: opentelemetry = "0.33"
38:
39: serde = { version = "1.0", features = ["derive"] }
40: serde_json = "1.0"
41:
42: sha2 = "0.11"
43:
44: sqlx = { version = "0.9", features = [
45:     "runtime-tokio",
46:     "macros",
47:     "postgres",
48:     "sqlite",
49: ] }
50:
51: thiserror = "2.0"
52: tokio = { version = "1.53", features = ["full"] }
53: tracing = "0.1"
54:
55: uuid = { version = "1.28", features = ["serde", "v4", "js"] }
56:
57: [[workspace.metadata.leptos]]
58: name = "rust_ai_chat"
59: bin-package = "server"
60: lib-package = "ui"
61:
62: bin-features = ["ssr"]
63: lib-features = ["hydrate"]
64:
65: site-root = "target/site"
66: site-pkg-dir = "pkg"
67: site-addr = "127.0.0.1:3000"
```

## reference-rust_ai_chat/apps/desktop_pet/Cargo.toml

SHA256 `9fb56a3ca1540c86713e550b3687153e6b4875a31b693ca54bb29df2d5e07ac3`。

```
1: [package]
2: name = "desktop-pet"
3: version.workspace = true
4: edition = "2024"
5: build = "build.rs"
6:
7: [dependencies]
8: ai-core = { path = "../../crates/ai_core" }
9:
10: dotenv.workspace = true
11:
12: pet-domain = { path = "../../crates/pet_domain" }
13:
14: serde.workspace = true
15: sha2.workspace = true
16:
17: tauri = { version = "2.12", features = [] }
18: thiserror.workspace = true
19: tokio.workspace = true
20:
21: [target.'cfg(unix)'.dependencies]
22: libc.workspace = true
23:
24: [build-dependencies]
25: tauri-build = { version = "2.7", features = [] }
```

## reference-rust_ai_chat/apps/desktop_pet/capabilities/default.json

SHA256 `ef3a05cbc8980eb4bef93f81642de2380cf5e692713e45d0bdbe9d7051845eb1`。

```
1: {
2:   "$schema": "../gen/schemas/desktop-schema.json",
3:   "identifier": "default",
4:   "description": "Default permissions for Rust AI Desktop Pet",
5:   "windows": ["main"],
6:   "permissions": [
7:     "core:default",
8:     "core:window:allow-start-dragging"
9:   ]
10: }
```

## reference-rust_ai_chat/apps/desktop_pet/frontend/Cargo.toml

SHA256 `fad0fb02700aecd4338d34f9ead03bcce9855ed570ee2b5f725cf7ec9b694632`。

```
1: [package]
2: name = "desktop-pet-frontend"
3: version.workspace = true
4: edition = "2024"
5:
6: [dependencies]
7: gloo-timers = "0.4"
8:
9: js-sys = "0.3"
10:
11: leptos = { version = "0.8", features = ["csr"] }
12:
13: pet-domain = { path = "../../../crates/pet_domain" }
14:
15: serde = { version = "1.0", features = ["derive"] }
16: serde-wasm-bindgen = "0.6"
17:
18: wasm-bindgen = "0.2"
19: wasm-bindgen-futures = "0.4"
20:
21: web-sys = { version = "0.3", features = ["Window"] }
```

## reference-rust_ai_chat/apps/desktop_pet/frontend/Trunk.toml

SHA256 `afb983c9167e70c116dd8268940668d5ee85748a1e2a88d7170778dba0f075ad`。

```
1: [build]
2: target = "index.html"
3: dist = "dist"
4:
5: [serve]
6: port = 1420
7: open = false
8: ws_protocol = "ws"
```

## reference-rust_ai_chat/apps/desktop_pet/frontend/index.html

SHA256 `a87a8b63900b2239ac73fb0b100524c4f1843fa9ad3ec112310f55c41cdeda3b`。

```
1: <!doctype html>
2: <html lang="zh-Hant">
3: <head>
4:   <meta charset="UTF-8" />
5:   <meta
6:     name="viewport"
7:     content="width=device-width, initial-scale=1.0"
8:   />
9:
10:   <title>Rust AI Desktop Pet</title>
11:
12:   <link data-trunk rel="css" href="style.css" />
13: </head>
14:
15: <body>
16:   <script>
17:     const { invoke } = window.__TAURI__.core;
18:     const { listen } = window.__TAURI__.event;
19:
20:     window.getPetState = () => {
21:       return invoke("get_pet_state");
22:     };
23:
24:     window.sendPetCommand = (command) => {
25:       return invoke("send_pet_command", { command });
26:     };
27:
28:     window.listenPetState = (callback) => {
29:       return listen(
30:         "pet://state-changed",
31:         (event) => callback(event.payload),
32:       );
33:     };
34:
35:     window.chatWithPet = (message) => {
36:       return invoke("chat_with_pet", { message });
37:     };
38:
39:     let petContextMenu = null;
40:
41:     async function getPetContextMenu() {
42:       if (petContextMenu) {
43:         return petContextMenu;
44:       }
45:
46:       const { Menu } = window.__TAURI__.menu;
47:
48:       petContextMenu = await Menu.new({
49:         items: [
50:           {
51:             id: "interact",
52:             text: "互動",
53:           },
54:           {
55:             id: "sleep",
56:             text: "睡覺",
57:           },
58:           {
59:             id: "wake",
60:             text: "醒來",
61:           },
62:         ],
63:       });
64:
65:       return petContextMenu;
66:     }
67:
68:     window.showPetContextMenu = async (x, y) => {
69:       const menu = await getPetContextMenu();
70:
71:       const {
72:         getCurrentWindow,
73:       } = window.__TAURI__.window;
74:
75:       const {
76:         LogicalPosition,
77:       } = window.__TAURI__.dpi;
78:
79:       const appWindow = getCurrentWindow();
80:
81:       await menu.popup(
82:         new LogicalPosition(x, y),
83:         appWindow,
84:       );
85:     };
86:
87:     window.scanFilePreview = async () => {
88:       return invoke("scan_file_preview");
89:     };
90:
91:   </script>
92:
93:   <link data-trunk rel="rust" />
94: </body>
95: </html>
```

## reference-rust_ai_chat/apps/desktop_pet/frontend/style.css

SHA256 `5ad5179a48f6fa0721ee31c303475efd25b6d9e8fe8a34db2123e5d87f05fe98`。

```
1: html, body {
2:   margin: 0;
3:   width: 100%;
4:   height: 100%;
5:   background: transparent;
6:   overflow: hidden;
7: }
8:
9: body {
10:   display: grid;
11:   place-items: center;
12:   font-family: system-ui, -apple-system, sans-serif;
13: }
14:
15: .pet-shell {
16:   position: relative;
17:   width: 220px;
18:   padding: 20px;
19:   border-radius: 28px;
20:   background: rgba(30, 30, 36, 0.82);
21:   color: white;
22:   text-align: center;
23: }
24:
25: .pet-status {
26:   display: grid;
27:   gap: 8px;
28:   margin-bottom: 16px;
29: }
30:
31: .pet-face {
32:   font-size: 56px;
33: }
34:
35: .pet-controls {
36:   display: grid;
37:   grid-template-columns: repeat(2, minmax(0, 1fr));
38:   gap: 8px;
39: }
40:
41: button {
42:   padding: 8px;
43: }
44:
45: .error {
46:   margin-top: 12px;
47:   font-size: 12px;
48: }
49:
50: .drag-handle {
51:   height: 28px;
52:   display: grid;
53:   place-items: center;
54:   cursor: grab;
55:   user-select: none;
56: }
57:
58: .drag-handle:active {
59:   cursor: grabbing;
60: }
61:
62: .drag-grip {
63:   line-height: 1;
64:   letter-spacing: 4px;
65:   opacity: 0.65;
66: }
67:
68: .chat-bubble {
69:   position: absolute;
70:   right: 12px;
71:   bottom: 72px;
72:
73:   width: 220px;
74:   padding: 12px;
75:
76:   border-radius: 14px;
77:   background: rgba(30, 30, 30, 0.92);
78:   box-shadow: 0 8px 24px rgba(0, 0, 0, 0.24);
79: }
80:
81: .chat-bubble__header {
82:   display: flex;
83:   align-items: center;
84:   justify-content: space-between;
85:   gap: 8px;
86:   margin-bottom: 8px;
87: }
88:
89: .chat-bubble__close {
90:   border: 0;
91:   background: transparent;
92:   cursor: pointer;
93: }
94:
95: .chat-bubble__input {
96:   box-sizing: border-box;
97:   width: 100%;
98:   padding: 8px 10px;
99: }
100:
101: .chat-bubble__send {
102:   width: 100%;
103:   margin-top: 8px;
104:   padding: 8px 10px;
105: }
106:
107: .chat-trigger {
108:   cursor: pointer;
109: }
```

## reference-rust_ai_chat/apps/desktop_pet/frontend/tests/scan-preview-contract.test.mjs

SHA256 `1576d3ece20e5c14f4000436ae1ecd354a52ecac93ef7532bcefc66df0e0c5b3`。

```
1: import assert from "node:assert/strict";
2: import { readFileSync } from "node:fs";
3: import test from "node:test";
4: import vm from "node:vm";
5: const html = readFileSync(new URL("../index.html", import.meta.url), "utf8");
6: const script = html.match(/<script>([\s\S]*?)<\/script>/)?.[1];
7: assert.ok(script);
8: test("scanFilePreview uses read-only command without path", async () => {
9:   const calls = [];
10:   const window = { __TAURI__: {
11:     core: { invoke: async (...args) => {
12:       calls.push(args);
13:       return { files: [], duplicateSizeCandidates: [], warnings: [] };
14:     }},
15:     event: { listen: async () => () => {} }
16:   }};
17:   vm.runInNewContext(script, { window });
18:   assert.equal(typeof window.scanFilePreview, "function");
19:   const result = await window.scanFilePreview();
20:   assert.equal(result.files.length, 0);
21:   assert.equal(calls.length, 1);
22:   assert.equal(calls[0][0], "scan_file_preview");
23:   assert.equal(calls[0].length, 1);
24: });
```

## reference-rust_ai_chat/apps/desktop_pet/frontend/tests/scan-preview-no-path.test.mjs

SHA256 `c8f76cecbb8a4d79f8c0dd1af0747e8b5e2e58846d64c3f7c7275a946bbdc2ae`。

```
1: import assert from "node:assert/strict";
2: import { readFileSync } from "node:fs";
3: import test from "node:test";
4: import vm from "node:vm";
5:
6: const html = readFileSync(new URL("../index.html", import.meta.url), "utf8");
7: const script = html.match(/<script>([\s\S]*?)<\/script>/)?.[1];
8: assert.ok(script, "找不到前端 Tauri bridge script");
9:
10: test("掃描預覽不得接受使用者指定路徑", async () => {
11:   const calls = [];
12:   const window = {
13:     __TAURI__: {
14:       core: { invoke: async (...args) => {
15:         calls.push(args);
16:         return { files: [], duplicateSizeCandidates: [], warnings: [] };
17:       }},
18:       event: { listen: async () => () => {} },
19:     },
20:   };
21:   vm.runInNewContext(script, { window });
22:   assert.equal(typeof window.scanFilePreview, "function");
23:   await window.scanFilePreview("/etc");
24:   assert.equal(calls.length, 1);
25:   assert.equal(calls[0][0], "scan_file_preview");
26:   assert.equal(calls[0].length, 1);
27: });
```

## reference-rust_ai_chat/apps/desktop_pet/public/index.html

SHA256 `49454032eee6130f188d9b6dc8d7a3a20d6f0e25075d851a4e050171e8986622`。

```
1: <!doctype html>
2: <html lang="zh-Hant">
3: <head>
4:   <meta charset="UTF-8" />
5:   <meta
6:     name="viewport"
7:     content="width=device-width, initial-scale=1.0"
8:   />
9:   <title>Rust AI Desktop Pet</title>
10:
11:   <style>
12:   html,
13:   body {
14:     margin: 0;
15:     width: 100%;
16:     height: 100%;
17:     background: transparent;
18:     overflow: hidden;
19:   }
20:
21:   body {
22:     display: grid;
23:     place-items: center;
24:     font-family: system-ui, sans-serif;
25:   }
26:
27:   .pet-shell {
28:     width: 220px;
29:     padding: 20px;
30:     border-radius: 28px;
31:     background: rgba(30, 30, 36, 0.82);
32:     color: white;
33:     text-align: center;
34:   }
35: </style>
36:
37: </head>
38:
39: <body>
40:   <main class="pet-shell">
41:     <h1>🐾 Rust AI Desktop Pet</h1>
42:
43:     <p>
44:       state:
45:       <strong id="state">loading</strong>
46:     </p>
47:
48:     <p>
49:       revision:
50:       <strong id="revision">-</strong>
51:     </p>
52:
53:     <button data-command="interact">
54:       互動
55:     </button>
56:
57:     <button data-command="finish_interaction">
58:       結束互動
59:     </button>
60:
61:     <button data-command="sleep">
62:       睡覺
63:     </button>
64:
65:     <button data-command="wake">
66:       醒來
67:     </button>
68:
69:     <pre id="error"></pre>
70:   </main>
71:
72:   <script src="./pet.js"></script>
73: </body>
74: </html>
```

## reference-rust_ai_chat/apps/desktop_pet/public/pet.js

SHA256 `2d2af56d184b5b003f1bc3346f78dcefd6097080f6f3c8609a8d3dc0f24a0bca`。

```
1: const { invoke } = window.__TAURI__.core;
2: const { listen } = window.__TAURI__.event;
3:
4: const stateEl = document.querySelector("#state");
5: const revisionEl = document.querySelector("#revision");
6: const errorEl = document.querySelector("#error");
7:
8: function render(snapshot) {
9:   stateEl.textContent = snapshot.state;
10:   revisionEl.textContent = snapshot.revision;
11: }
12:
13: async function loadState() {
14:   const snapshot = await invoke("get_pet_state");
15:   render(snapshot);
16: }
17:
18: async function send(type) {
19:   errorEl.textContent = "";
20:
21:   try {
22:     const snapshot = await invoke(
23:       "send_pet_command",
24:       {
25:         command: type
26:       },
27:     );
28:
29:     render(snapshot);
30:   } catch (error) {
31:     errorEl.textContent = String(error);
32:   }
33: }
34:
35: for (const button of document.querySelectorAll(
36:   "[data-command]",
37: )) {
38:   button.addEventListener("click", () => {
39:     send(button.dataset.command);
40:   });
41: }
42:
43: listen("pet://state-changed", (event) => {
44:   render(event.payload);
45: });
46:
47: loadState().catch((error) => {
48:   errorEl.textContent = String(error);
49: });
```

## reference-rust_ai_chat/apps/desktop_pet/tauri.conf.json

SHA256 `6850bf27067b29343887c37aec8139bb422da4039da0e293a08de74c1433c701`。

```
1: {
2:   "$schema": "https://schema.tauri.app/config/2",
3:   "productName": "Rust AI Desktop Pet",
4:   "version": "0.2.3",
5:   "identifier": "dev.rustai.desktop-pet",
6:   "build": {
7:     "devUrl": "http://localhost:1420",
8:     "frontendDist": "frontend/dist",
9:     "beforeDevCommand": {
10:       "script": "trunk serve --port 1420",
11:       "cwd": "./frontend"
12:     },
13:     "beforeBuildCommand": {
14:       "script": "trunk build --release",
15:       "cwd": "./frontend"
16:     }
17:   },
18:   "app": {
19:     "withGlobalTauri": true,
20:     "windows": [
21:       {
22:         "label": "main",
23:         "title": "Rust AI Desktop Pet",
24:         "width": 320,
25:         "height": 360,
26:         "resizable": false,
27:         "decorations": false,
28:         "transparent": true,
29:         "alwaysOnTop": true,
30:         "shadow": false
31:       }
32:     ]
33:   },
34:   "bundle": {
35:     "active": false
36:   }
37: }
```

## reference-rust_ai_chat/apps/server/Cargo.toml

SHA256 `b436777229dd4432cade9e8164850eeafbfb0bc532cdb73aaa09df68fdc28919`。

```
1: [package]
2: name = "server"
3: version.workspace = true
4: edition.workspace = true
5:
6: [features]
7: default = []
8: ssr = ["ui/ssr"]
9:
10: [dependencies]
11: any_spawner.workspace = true
12: axum.workspace = true
13:
14: chat-application = { path = "../../crates/chat_application" }
15:
16: dotenv.workspace = true
17:
18: leptos.workspace = true
19: leptos_axum.workspace = true
20: leptos_config.workspace = true
21:
22: persistence = { path = "../../crates/persistence" }
23:
24: serde.workspace = true
25: serde_json.workspace = true
26:
27: shared = { path = "../../crates/shared" }
28:
29: sqlx.workspace = true
30:
31: tokio.workspace = true
32:
33: ui = { path = "../../crates/ui" }
34: uuid.workspace = true
```

## reference-rust_ai_chat/apps/stats_server/Cargo.toml

SHA256 `583b023600ac455f1cd441dcf896a3e853eec4a7acbf98bd36750e61d2f7d71d`。

```
1: [package]
2: name = "stats-server"
3: version.workspace = true
4: edition.workspace = true
5:
6: [dependencies]
7: axum.workspace = true
8: serde.workspace = true
9: tokio.workspace = true
10: uuid.workspace = true
```

## reference-rust_ai_chat/crates/ai_core/Cargo.toml

SHA256 `8253f7e9d37b44b2d30a7949cdca7a3ad63eeaeb2f543d36f838d29c3b5e82dc`。

```
1: [package]
2: name = "ai-core"
3: version.workspace = true
4: edition.workspace = true
5:
6: [dependencies]
7: async-trait.workspace = true
8: thiserror.workspace = true
9:
10: [dev-dependencies]
11: tokio = { workspace = true, features = ["macros", "rt"] }
```

## reference-rust_ai_chat/crates/chat_application/Cargo.toml

SHA256 `f3f7bf509bc01d5f1900686336def961c712b60db976c9627fddccae5a85adfb`。

```
1: [package]
2: name = "chat-application"
3: version.workspace = true
4: edition.workspace = true
5:
6: [dependencies]
7: async-trait.workspace = true
8:
9: chat-domain = { path = "../chat_domain" }
10:
11: thiserror.workspace = true
12:
13: uuid.workspace = true
14:
15: [dev-dependencies]
16: tokio = { workspace = true, features = ["macros", "rt"] }
```

## reference-rust_ai_chat/crates/chat_domain/Cargo.toml

SHA256 `845e2c6d21feb9eccdbf60548669c7baa455a559b4d2b9853b199f2f6ea4a136`。

```
1: [package]
2: name = "chat-domain"
3: version.workspace = true
4: edition.workspace = true
5:
6: [dependencies]
7: serde.workspace = true
8: uuid.workspace = true
```

## reference-rust_ai_chat/crates/persistence/Cargo.toml

SHA256 `90a3e011be596d6f15c5d184bdc19c49ac256aafd5535e3490095106fa4f60fd`。

```
1: [package]
2: name = "persistence"
3: version.workspace = true
4: edition.workspace = true
5:
6: [dependencies]
7: async-trait.workspace = true
8:
9: chat-application = { path = "../chat_application" }
10: chat-domain = { path = "../chat_domain" }
11:
12:
13: sqlx.workspace = true
14:
15: tokio.workspace = true
```

## reference-rust_ai_chat/crates/pet_domain/Cargo.toml

SHA256 `18b159f763532b13c1eec26e65ab4be1e0dda4c351e217b73fd2aa4bdc4a9a32`。

```
1: [package]
2: name = "pet-domain"
3: version.workspace = true
4: edition.workspace = true
5:
6: [dependencies]
7: serde.workspace = true
8: serde_json.workspace = true
9:
10: thiserror = "2.0"
11: uuid.workspace = true
```

## reference-rust_ai_chat/crates/shared/Cargo.toml

SHA256 `3a52b82c2a2dfc4619deb6113c0a00fb782f36c32839ff7f61c98090c9335f46`。

```
1: [package]
2: name = "shared"
3: version.workspace = true
4: edition.workspace = true
5:
6: [dependencies]
7: chat-domain = { path = "../chat_domain" }
8:
9: serde.workspace = true
10: serde_json.workspace = true
11:
12: uuid.workspace = true
```

## reference-rust_ai_chat/crates/telemetry/Cargo.toml

SHA256 `af9e3f9bb8f18ba50d6cb14cb0dcf5cdb37abe2c16464b464c6be5dcbdf57963`。

```
1: [package]
2: name = "telemetry"
3: version.workspace = true
4: edition.workspace = true
5:
6: [dependencies]
```

## reference-rust_ai_chat/crates/ui/Cargo.toml

SHA256 `2eabff6fe550a9a14f77f0babcb9ef3084dfa4734b6596b5a51b91f468cae931`。

```
1: [package]
2: name = "ui"
3: version.workspace = true
4: edition.workspace = true
5:
6: [lib]
7: crate-type = ["cdylib", "rlib"]
8:
9: [features]
10: default = []
11: ssr = ["leptos/ssr", "leptos_router/ssr"]
12: hydrate = ["leptos/hydrate"]
13:
14: [dependencies]
15: chat-domain = { path = "../chat_domain" }
16: leptos_router.workspace = true
17:
18: leptos.workspace = true
19:
20: serde_json.workspace = true
21: shared = { path = "../shared" }
22:
23: [target.'cfg(target_arch="wasm32")'.dependencies]
24: wasm-bindgen = "0.2"
25: wasm-bindgen-futures = "0.4"
26: web-sys = { version = "0.3", features = [
27:     "Headers",
28:     "MessageEvent",
29:     "Request",
30:     "RequestInit",
31:     "RequestMode",
32:     "Response",
33:     "WebSocket",
34:     "Window",
35: ] }
```

## reference-rust_ai_chat/migrations/20260930034455_crate_rooms.sql

SHA256 `c27795888fa0d86f2163426b0c3309ced59eb7b20e8acc9c698921379af5dd09`。

```
1: -- Add migration script here
2: CREATE TABLE rooms (
3:     id varchar(40) PRIMARY KEY,
4:     category_room_id varchar(40) NOT NULL,
5:     name TEXT NOT NULL,
6:     created_at TIMESTAMP NOT NULL ,
7:     updated_at TIMESTAMP NOT NULL
8: );
```

## reference-rust_ai_chat/migrations/20261007084321_create_messages.sql

SHA256 `bb8ef5644ecdd75e1c9d529b46c8def45cdf7e4d92a03768ddf4f85856db37ba`。

```
1: -- Add migration script here
2: CREATE TABLE messages (
3:     id VARCHAR(40) PRIMARY KEY,
4:     room_id VARCHAR(40) NOT NULL,
5:     author_kind VARCHAR(32) NOT NULL,
6:     author_id VARCHAR(255),
7:     content TEXT NOT NULL,
8:     created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
9:     CONSTRAINT fk_messages_room FOREIGN KEY (room_id) REFERENCES rooms(id),
10:     CONSTRAINT ck_messages_author_kind
11:         CHECK (author_kind IN ('anonymous_user', 'user', 'ai', 'system')),
12:     CONSTRAINT ck_messages_author_id
13:         CHECK (
14:             (author_kind IN ('anonymous_user', 'system') AND author_id IS NULL)
15:             OR
16:             (author_kind IN ('user', 'ai') AND author_id IS NOT NULL)
17:         )
18: );
```

## reference-rust_ai_chat/rust-toolchain.toml

SHA256 `32314ae0f25d85fda9b7ad0c033266f677a9d772b7a39038c62a386cc3085b75`。

```
1: [toolchain]
2: channel = "stable"
3: targets = ["wasm32-unknown-unknown"]
4: components = ["rustfmt", "clippy"]
```

## reference-rust_ai_chat/tests/Cargo.toml

SHA256 `ade6edd10489a01ed356350cdd535a4688a960358431d2a68ac918a5801630aa`。

```
1: [package]
2: name = "workspace-tests"
3: version.workspace = true
4: edition.workspace = true
5: publish = false
6: autotests = false
7:
8: [dev-dependencies]
9: async-trait.workspace = true
10: axum.workspace = true
11:
12: chat-application = { path = "../crates/chat_application" }
13: chat-domain = { path = "../crates/chat_domain" }
14:
15: desktop-pet = { path = "../apps/desktop_pet" }
16:
17: pet-domain = { path = "../crates/pet_domain" }
18:
19: serde_json.workspace = true
20:
21: shared = { path = "../crates/shared" }
22:
23: server = { path = "../apps/server" }
24: stats-server = { path = "../apps/stats_server" }
25:
26: tokio.workspace = true
27:
28: tower = { version = "0.5", features = ["util"] }
29:
30: uuid.workspace = true
31:
32: [[test]]
33: name = "analytics_event_kind_tests"
34: path = "analytics/event_kind_tests.rs"
35:
36: [[test]]
37: name = "analytics_http_contract_tests"
38: path = "analytics/http_contract_tests.rs"
39:
40: [[test]]
41: name = "analytics_idempotency_contract"
42: path = "analytics/idempotency_contract.rs"
43:
44: [[test]]
45: name = "analytics_postgres_ingest"
46: path = "analytics/postgres_ingest.rs"
47:
48: [[test]]
49: name = "chat_persist_publish"
50: path = "chat/persist_publish.rs"
51:
52: [[test]]
53: name = "chat_product_event_contract"
54: path = "chat/product_event_contract.rs"
55:
56: [[test]]
57: name = "pet_cleanup_contract"
58: path = "pet/cleanup_contract.rs"
59:
60: [[test]]
61: name = "pet_cleanup_event_contract"
62: path = "pet/cleanup_event_contract.rs"
63:
64: [[test]]
65: name = "pet_day5_codec_tests"
66: path = "pet/day5_codec_tests.rs"
67:
68: [[test]]
69: name = "pet_pet_machine"
70: path = "pet/pet_machine.rs"
71:
72: [[test]]
73: name = "pet_profile_contract"
74: path = "pet/profile_contract.rs"
75:
76: [[test]]
77: name = "server_day5_tests"
78: path = "server/day5_tests.rs"
```

## copilot_use/.env.example

SHA256 `96ee4e8d3b2d20e2d1506cbccab36ff786e5dec738cdc299653d35bf55ad9536`。

```
1: # Copy to .env and replace all values with independently generated random secrets.
2: # Never commit .env.
3: ANALYTICS_BOOTSTRAP_PASSWORD=REPLACE_WITH_OTHER_BOOTSTRAP_SECRET
4: ANALYTICS_DB_PASSWORD=REPLACE_WITH_ANOTHER_RANDOM_DB_PASSWORD
5: CHAT_API_TOKEN=REPLACE_WITH_RANDOM_CHAT_API_TOKEN_AT_LEAST_16
6: ANALYTICS_API_TOKEN=REPLACE_WITH_RANDOM_ANALYTICS_API_TOKEN_AT_LEAST_16
7:
8: ANALYTICS_CHAT_TOKEN=REPLACE_WITH_INDEPENDENT_CHAT_INGEST_TOKEN
9: ANALYTICS_PET_TOKEN=REPLACE_WITH_INDEPENDENT_PET_INGEST_TOKEN
10: # Optional OpenAI; unset disables AI API (503), never a mock answer.
11: OPENAI_API_KEY=
12: OPENAI_CHAT_MODEL=gpt-5.6-luna
13:
14: ANALYTICS_ADMIN_TOKEN=REPLACE_WITH_INDEPENDENT_GOVERNANCE_TOKEN
15: OPENAI_EMBEDDING_MODEL=text-embedding-3-small
16: # Optional local collector; numeric loopback URLs only, never remote metadata.
17: COPILOT_LOCAL_OTLP_ENDPOINT=
18:
19: # Analytics HTTP dashboard/query and native gRPC ingestion share this listener.
20: ANALYTICS_LISTEN=127.0.0.1:3100
21: # Existing URL is retained; gRPC uses its origin, HTTP management uses the path.
22: ANALYTICS_ENDPOINT=http://127.0.0.1:3100/api/v1/events
```

## copilot_use/.github/release-approved.json

SHA256 `a6f34c7ede85e5abb02e6cfce725222842d76c55a8c063d3b7609f385a8add9e`。

```
1: {
2:   "approved_sha": "22b44f33b9e2c9edb0fb398fb9bff3efd57b00ee",
3:   "tag": "v0.1.0",
4:   "evidence": {
5:     "live": "docs/release-evidence/live.json",
6:     "docker": "docs/release-evidence/docker.json",
7:     "packaged_gui": "docs/release-evidence/packaged_gui.json",
8:     "appimage_update": "docs/release-evidence/appimage_update.json"
9:   },
10:   "notes": "Chat／Pet 已改為可攜式 SQLite（含知識庫向量、Session、角色、Outbox 與備份），Analytics 儀表板維持 PostgreSQL。已驗證真實 OpenAI gpt-5.6-luna 串流／RAG／MCP／Persona／Pet AI與生圖、512連線／跨程序可靠傳送、同意撤回、Docker release及故障／備份還原；三平台實際安裝包 GUI 與簽章更新／冷啟動／SQLite保留由同 SHA CI核對。\n\n本次僅本地測試與版本固定，不公開部署。安裝包未使用商業平台憑證簽章；匿名事件及描述性計數不代表獨立真人數或學習成效。rust_ai_chat 嚴格唯讀；成品固定後才建立教學對齊 branch，tests 依所屬專案／模組保存。"
11: }
```

## copilot_use/.github/workflows/native-source-audit.yml

SHA256 `3d66825bc60855126cc4b464e9160bd52d237515f16d1e0bb1721df1f0ffb63c`。

```
1: name: Audit immutable release Windows provenance
2: on:
3:   push:
4:     branches: [main]
5:     paths: [.github/workflows/native-source-audit.yml]
6: permissions:
7:   contents: read
8: jobs:
9:   desktop:
10:     strategy:
11:       fail-fast: false
12:       matrix:
13:         include:
14:           - os: windows-latest
15:             bundle: nsis
16:     runs-on: ${{ matrix.os }}
17:     timeout-minutes: 60
18:     steps:
19:       - uses: actions/checkout@v4
20:         with:
21:           ref: 22b44f33b9e2c9edb0fb398fb9bff3efd57b00ee
22:       - uses: dtolnay/rust-toolchain@stable
23:         with:
24:           targets: wasm32-unknown-unknown
25:       - uses: actions/setup-node@v4
26:         with:
27:           node-version: 22
28:       - uses: actions/cache/restore@v4
29:         id: build-tools
30:         with:
31:           path: |
32:             ~/.cargo/bin/trunk*
33:             ~/.cargo/bin/cargo-tauri*
34:           key: ${{ runner.os }}-trunk-0.21.14-tauri-2.11.5
35:       - name: Linux native dependencies
36:         if: runner.os == 'Linux'
37:         run: |
38:           sudo apt-get update
39:           sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev patchelf xvfb
40:       - name: Verify Pet persistence and native platform safety
41:         run: cargo test --locked -p day9-pet --lib
42:       - name: Install build tooling
43:         shell: bash
44:         run: |
45:           if ! command -v trunk; then cargo install trunk --version 0.21.14 --locked; fi
46:           if ! command -v cargo-tauri; then cargo install tauri-cli --version 2.11.5 --locked; fi
47:       - uses: actions/cache/save@v4
48:         if: steps.build-tools.outputs.cache-hit != 'true'
49:         with:
50:           path: |
51:             ~/.cargo/bin/trunk*
52:             ~/.cargo/bin/cargo-tauri*
53:           key: ${{ runner.os }}-trunk-0.21.14-tauri-2.11.5
54:       - name: Compile Chat hydration assets
55:         shell: bash
56:         run: bash scripts/build-chat-ui.sh
57:       - name: Compile Pet hydration assets
58:         shell: bash
59:         run: node scripts/build-pet-ui.mjs
60:       - name: Pet non-square sprite geometry regression
61:         run: cargo test --locked --manifest-path apps/pet_ui/Cargo.toml
62:       - name: Actual Linux WebKit GUI verification
63:         if: runner.os == 'Linux'
64:         run: xvfb-run -a python3 scripts/pet-gui-verify.py --updater
65:       - name: Actual Windows and macOS WebView GUI verification
66:         if: runner.os != 'Linux'
67:         shell: bash
68:         run: python3 scripts/pet-gui-verify.py --updater
69:       - name: Optimized GLib security backport regression
70:         if: runner.os == 'Linux'
71:         run: cargo test --locked --release --manifest-path apps/pet_desktop/Cargo.toml --test glib_variant_safety
72:       - name: Trace exact immutable source throughout native build
73:         shell: pwsh
74:         run: |
75:           $probe = Join-Path $env:RUNNER_TEMP 'source-probe.py'
76:           @'
77:           import pathlib,subprocess,json,time,os
78:           root=pathlib.Path.cwd()
79:           sha=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
80:           assert sha=='22b44f33b9e2c9edb0fb398fb9bff3efd57b00ee'
81:           states=[];seen=set()
82:           def observe(stage):
83:               status=subprocess.check_output(['git','status','--porcelain','--untracked-files=all'],cwd=root,text=True).strip()
84:               diff=subprocess.check_output(['git','diff','--no-ext-diff'],cwd=root,text=True)
85:               identity=(status,diff)
86:               if identity not in seen:
87:                   seen.add(identity);states.append({'stage':stage,'status':status,'tracked_diff':diff})
88:                   print('SOURCE PROVENANCE: '+json.dumps(states[-1]),flush=True)
89:           observe('before_build')
90:           process=subprocess.Popen(['cargo','tauri','build','--bundles','nsis'],cwd=root/'apps/pet_desktop',env=os.environ|{'NO_COLOR':'true'})
91:           while process.poll() is None:
92:               observe('during_build');time.sleep(.2)
93:           observe('after_build')
94:           report=pathlib.Path(os.environ['RUNNER_TEMP'])/'immutable-source-provenance.json'
95:           report.write_text(json.dumps({'source_sha':sha,'states':states,'build_exit_code':process.returncode},indent=2)+'\n',encoding='utf-8')
96:           raise SystemExit(process.returncode)
97:           '@ | Set-Content -LiteralPath $probe -Encoding utf8
98:           python3 $probe
99:       - name: Preserve immutable source provenance
100:         if: always()
101:         uses: actions/upload-artifact@v4
102:         with:
103:           name: immutable-release-windows-source
104:           path: ${{ runner.temp }}/immutable-source-provenance.json
105:       - name: Execute extracted Linux release package
106:         if: runner.os == 'Linux'
107:         shell: bash
108:         run: |
109:           package=$(find apps/pet_desktop/target/release/bundle/deb -name '*.deb' -print -quit)
110:           dpkg-deb -x "$package" "$RUNNER_TEMP/pet-package"
111:           executable=$(find "$RUNNER_TEMP/pet-package" -name copilot-desktop-pet -type f -print -quit)
112:           xvfb-run -a python3 scripts/pet-gui-verify.py --executable "$executable" --updater
113:       - name: Install signed AppImage update and cold-start updated executable
114:         if: runner.os == 'Linux'
115:         shell: bash
116:         run: |
117:           cp 'apps/pet_desktop/target/release/bundle/appimage/Copilot Desktop Pet_0.1.0_amd64.AppImage' "$RUNNER_TEMP/base.AppImage"
118:           python3 scripts/build-test-update.py
119:           xvfb-run -a python3 scripts/pet-gui-verify.py --executable "$RUNNER_TEMP/base.AppImage" --updater --update-package 'apps/pet_desktop/target/release/bundle/appimage/Copilot Desktop Pet_0.1.1_amd64.AppImage'
120:           cp "$RUNNER_TEMP/base.AppImage" 'apps/pet_desktop/target/release/bundle/appimage/Copilot Desktop Pet_0.1.0_amd64.AppImage'
121:       - name: Execute macOS release app
122:         if: runner.os == 'macOS'
123:         shell: bash
124:         run: |
125:           executable=$(find apps/pet_desktop/target/release/bundle/macos -path '*/Contents/MacOS/copilot-desktop-pet' -type f -print -quit)
126:           python3 scripts/pet-gui-verify.py --executable "$executable" --updater
127:       - name: Install signed macOS bundle update and cold-start updated executable
128:         if: runner.os == 'macOS'
129:         shell: bash
130:         run: |
131:           cp -R 'apps/pet_desktop/target/release/bundle/macos/Copilot Desktop Pet.app' "$RUNNER_TEMP/base-pet.app"
132:           python3 scripts/build-test-update.py
133:           python3 scripts/pet-gui-verify.py --executable "$RUNNER_TEMP/base-pet.app/Contents/MacOS/copilot-desktop-pet" --updater --update-package 'apps/pet_desktop/target/release/bundle/macos/Copilot Desktop Pet.app.tar.gz' --expected-executable 'apps/pet_desktop/target/release/bundle/macos/Copilot Desktop Pet.app/Contents/MacOS/copilot-desktop-pet'
134:           rm -rf 'apps/pet_desktop/target/release/bundle/macos/Copilot Desktop Pet.app'
135:           cp -R "$RUNNER_TEMP/base-pet.app" 'apps/pet_desktop/target/release/bundle/macos/Copilot Desktop Pet.app'
136:       - name: Install and execute Windows release package in disposable runner
137:         if: runner.os == 'Windows'
138:         shell: pwsh
139:         run: |
140:           $installer = Get-ChildItem apps/pet_desktop/target/release/bundle/nsis -Filter *.exe | Select-Object -First 1
141:           $destination = Join-Path $env:RUNNER_TEMP 'pet-package'
142:           $process = Start-Process $installer.FullName -ArgumentList "/S /D=$destination" -Wait -PassThru
143:           if ($process.ExitCode -ne 0) { throw "NSIS installation failed: $($process.ExitCode)" }
144:           $executable = Get-ChildItem $destination -Filter copilot-desktop-pet.exe -Recurse | Select-Object -First 1
145:           if (-not $executable) { throw 'Installed executable missing' }
146:           python3 scripts/pet-gui-verify.py --executable $executable.FullName --updater
147:           if ($LASTEXITCODE -ne 0) { throw 'Installed release GUI verification failed' }
148:       - name: Install signed Windows NSIS update and cold-start updated executable
149:         if: runner.os == 'Windows'
150:         shell: pwsh
151:         run: |
152:           $executable = Get-ChildItem (Join-Path $env:RUNNER_TEMP 'pet-package') -Filter copilot-desktop-pet.exe -Recurse | Select-Object -First 1
153:           $baseInstaller = Get-ChildItem apps/pet_desktop/target/release/bundle/nsis -Filter '*_0.1.0_*.exe' | Select-Object -First 1
154:           if (-not $baseInstaller) { throw 'Base Windows installer missing' }
155:           $baseName = $baseInstaller.Name
156:           Copy-Item $baseInstaller.FullName (Join-Path $env:RUNNER_TEMP 'base-installer.exe')
157:           python3 scripts/build-test-update.py
158:           if ($LASTEXITCODE -ne 0) { throw 'Windows update fixture build failed' }
159:           $update = Get-ChildItem apps/pet_desktop/target/release/bundle/nsis -Filter '*_0.1.1_*.exe' | Select-Object -First 1
160:           if (-not $update) { throw 'Windows update package missing' }
161:           python3 scripts/pet-gui-verify.py --executable $executable.FullName --updater --update-package $update.FullName --expected-executable apps/pet_desktop/target/test-update/copilot-desktop-pet.exe
162:           if ($LASTEXITCODE -ne 0) { throw 'Actual signed Windows update failed' }
163:           Copy-Item (Join-Path $env:RUNNER_TEMP 'base-installer.exe') (Join-Path 'apps/pet_desktop/target/release/bundle/nsis' $baseName) -Force
164:       - uses: actions/upload-artifact@v4
165:         with:
166:           name: desktop-${{ runner.os }}
167:           path: |
168:             apps/pet_desktop/target/release/bundle/deb/*.deb
169:             apps/pet_desktop/target/release/bundle/appimage/*_0.1.0_*.AppImage
170:             apps/pet_desktop/target/release/bundle/macos/*.app/
171:             apps/pet_desktop/target/release/bundle/nsis/*_0.1.0_*.exe
172:             target/pet-gui-validation.json
173:
174:       - name: Preserve isolated Windows updater diagnostics on failure
175:         if: failure() && runner.os == 'Windows'
176:         uses: actions/upload-artifact@v4
177:         with:
178:           name: updater-diagnostic-Windows
179:           path: |
180:             apps/pet_desktop/target/release/bundle/nsis/*.exe
181:             apps/pet_desktop/target/release/copilot-desktop-pet.exe
182:             target/pet-gui-validation.json
```

## copilot_use/.github/workflows/release-approved.yml

SHA256 `7202ea0faf4573d8d7104665f53e0849346c60b34c4cc646a363a25ca9a59d9a`。

```
1: name: Publish verified local P9 release
2: on:
3:   push:
4:     branches: [main]
5:     paths: [.github/release-approved.json]
6: permissions:
7:   contents: write
8:   actions: read
9: concurrency:
10:   group: p9-release
11:   cancel-in-progress: false
12: jobs:
13:   publish:
14:     runs-on: ubuntu-latest
15:     steps:
16:       - uses: actions/checkout@v4
17:         with:
18:           fetch-depth: 2
19:       - name: Verify exact approved source, CI and local evidence before immutable release
20:         env:
21:           GH_TOKEN: ${{ github.token }}
22:           GH_REPOSITORY: ${{ github.repository }}
23:         run: python3 scripts/release-approved.py
```

## copilot_use/.github/workflows/verify.yml

SHA256 `ba4c59c6220592d146fce8c10f4a2a56bfad0511a6fb0ee310cd79bd2a7ae8a3`。

```
1: name: Verify copilot_use
2: on:
3:   push:
4:     branches: [main, 'teaching/**']
5:   pull_request:
6:     branches: [main]
7: jobs:
8:   rust-postgres:
9:     runs-on: ubuntu-latest
10:     services:
11:       postgres:
12:         image: postgres:17
13:         env:
14:           POSTGRES_USER: postgres
15:           POSTGRES_PASSWORD: postgres
16:           POSTGRES_DB: postgres
17:         ports:
18:           - 5432:5432
19:         options: >-
20:           --health-cmd "pg_isready -U postgres"
21:           --health-interval 10s
22:           --health-timeout 5s
23:           --health-retries 5
24:     env:
25:       DATABASE_URL: postgres://postgres:postgres@127.0.0.1:5432/postgres
26:     steps:
27:       - uses: actions/checkout@v4
28:       - uses: dtolnay/rust-toolchain@stable
29:       - name: Release gate rejects incomplete evidence without network access
30:         run: python3 scripts/tests/release-approval.py
31:       - name: Enforce pure Domain/Application dependency boundaries
32:         run: python3 scripts/architecture-boundaries.py
33:       - name: Format
34:         run: cargo fmt --all -- --check
35:       - name: Strict Clippy
36:         run: cargo clippy --workspace --all-targets --locked -- -D warnings
37:       - name: Run SQLite Chat/Pet and PostgreSQL Analytics integration tests
38:         run: cargo test --workspace --all-targets --locked
39:       - name: Verify strict event payload
40:         run: cargo test -p event-contract
41:
42:   compose-e2e:
43:     runs-on: ubuntu-latest
44:     timeout-minutes: 60
45:     steps:
46:       - uses: actions/checkout@v4
47:       - name: Set isolated CI-only secrets
48:         run: |
49:           cat > .env <<'ENV'
50:           ANALYTICS_BOOTSTRAP_PASSWORD=ci_analytics_bootstrap_only_password
51:           ANALYTICS_ADMIN_TOKEN=ci_governance_only_token_32_characters
52:           ANALYTICS_DB_PASSWORD=ci_analytics_only_random_test_password
53:           CHAT_API_TOKEN=ci_chat_only_token_32_characters
54:           ANALYTICS_API_TOKEN=ci_analytics_only_token_32_characters
55:           ANALYTICS_CHAT_TOKEN=ci_chat_ingest_only_token_32_characters
56:           ANALYTICS_PET_TOKEN=ci_pet_ingest_only_token_32_characters
57:           ENV
58:       - name: Build and start isolated app and database containers
59:         run: COPILOT_BUILD_SHA="$GITHUB_SHA" docker compose --env-file .env up -d --build
60:       - uses: actions/setup-node@v4
61:         with:
62:           node-version: 22
63:       - name: Test durable end-to-end Chat to Analytics ingestion
64:         run: |
65:           set -a
66:           source .env
67:           set +a
68:           bash scripts/smoke.sh
69:           node scripts/realtime-e2e.mjs
70:       - name: Collect service logs on failure
71:         if: failure()
72:         run: docker compose --env-file .env logs --no-color --tail=150
73:       - name: Shut down and delete CI databases
74:         if: always()
75:         run: docker compose --env-file .env down -v --remove-orphans
76:
77:   local-fault-recovery:
78:     runs-on: ubuntu-latest
79:     timeout-minutes: 30
80:     steps:
81:       - uses: actions/checkout@v4
82:       - uses: dtolnay/rust-toolchain@stable
83:         with:
84:           components: rustfmt, clippy
85:       - uses: actions/setup-node@v4
86:         with:
87:           node-version: 22
88:       - name: Install isolated-test PostgreSQL tooling
89:         run: |
90:           sudo apt-get update
91:           sudo apt-get install -y postgresql-16 postgresql-client-16 postgresql-server-dev-16 libpq-dev
92:       - name: Run HTTP, WebSocket, restart, DB interruption and backup/restore tests
93:         run: |
94:           PATH=/usr/lib/postgresql/16/bin:$PATH VERIFY_REPORT=target/local-validation-native.json python3 scripts/local-verify.py --load
95:           PATH=/usr/lib/postgresql/16/bin:$PATH python3 scripts/local-verify.py --varchar-events
96:       - uses: actions/upload-artifact@v4
97:         if: success()
98:         with:
99:           name: local-validation
100:           path: |
101:             target/local-validation.json
102:             target/local-validation-native.json
103:             target/realtime-load-validation.json
104:
105:   desktop:
106:     strategy:
107:       fail-fast: false
108:       matrix:
109:         include:
110:           - os: ubuntu-latest
111:             bundle: deb,appimage
112:           - os: macos-latest
113:             bundle: app
114:           - os: windows-latest
115:             bundle: nsis
116:     runs-on: ${{ matrix.os }}
117:     timeout-minutes: 60
118:     steps:
119:       - uses: actions/checkout@v4
120:       - uses: dtolnay/rust-toolchain@stable
121:         with:
122:           targets: wasm32-unknown-unknown
123:       - uses: actions/setup-node@v4
124:         with:
125:           node-version: 22
126:       - uses: actions/cache/restore@v4
127:         id: build-tools
128:         with:
129:           path: |
130:             ~/.cargo/bin/trunk*
131:             ~/.cargo/bin/cargo-tauri*
132:           key: ${{ runner.os }}-trunk-0.21.14-tauri-2.11.5
133:       - name: Linux native dependencies
134:         if: runner.os == 'Linux'
135:         run: |
136:           sudo apt-get update
137:           sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev patchelf xvfb
138:       - name: Verify Pet persistence and native platform safety
139:         run: cargo test --locked -p pet-domain -p day9-pet --lib
140:       - name: Install build tooling
141:         shell: bash
142:         run: |
143:           if ! command -v trunk; then cargo install trunk --version 0.21.14 --locked; fi
144:           if ! command -v cargo-tauri; then cargo install tauri-cli --version 2.11.5 --locked; fi
145:       - uses: actions/cache/save@v4
146:         if: steps.build-tools.outputs.cache-hit != 'true'
147:         with:
148:           path: |
149:             ~/.cargo/bin/trunk*
150:             ~/.cargo/bin/cargo-tauri*
151:           key: ${{ runner.os }}-trunk-0.21.14-tauri-2.11.5
152:       - name: Compile Chat hydration assets
153:         shell: bash
154:         run: bash scripts/build-chat-ui.sh
155:       - name: Compile Pet hydration assets
156:         shell: bash
157:         run: node scripts/build-pet-ui.mjs
158:       - name: Pet non-square sprite geometry regression
159:         run: cargo test --locked --manifest-path apps/desktop_pet/frontend/Cargo.toml
160:       - name: Actual Linux WebKit GUI verification
161:         if: runner.os == 'Linux'
162:         run: xvfb-run -a python3 scripts/pet-gui-verify.py --updater
163:       - name: Actual Windows and macOS WebView GUI verification
164:         if: runner.os != 'Linux'
165:         shell: bash
166:         run: python3 scripts/pet-gui-verify.py --updater
167:       - name: Optimized GLib security backport regression
168:         if: runner.os == 'Linux'
169:         run: cargo test --locked --release --manifest-path apps/desktop_pet/Cargo.toml --test glib_variant_safety
170:       - name: Build native installer
171:         shell: bash
172:         run: cd apps/desktop_pet && NO_COLOR=true cargo tauri build --bundles "${{ matrix.bundle }}"
173:       - name: Execute extracted Linux release package
174:         if: runner.os == 'Linux'
175:         shell: bash
176:         run: |
177:           package=$(find apps/desktop_pet/target/release/bundle/deb -name '*.deb' -print -quit)
178:           dpkg-deb -x "$package" "$RUNNER_TEMP/pet-package"
179:           executable=$(find "$RUNNER_TEMP/pet-package" -name copilot-desktop-pet -type f -print -quit)
180:           xvfb-run -a python3 scripts/pet-gui-verify.py --executable "$executable" --updater
181:       - name: Install signed AppImage update and cold-start updated executable
182:         if: runner.os == 'Linux'
183:         shell: bash
184:         run: |
185:           cp 'apps/desktop_pet/target/release/bundle/appimage/Copilot Desktop Pet_0.1.0_amd64.AppImage' "$RUNNER_TEMP/base.AppImage"
186:           python3 scripts/build-test-update.py
187:           xvfb-run -a python3 scripts/pet-gui-verify.py --executable "$RUNNER_TEMP/base.AppImage" --updater --update-package 'apps/desktop_pet/target/release/bundle/appimage/Copilot Desktop Pet_0.1.1_amd64.AppImage'
188:           cp "$RUNNER_TEMP/base.AppImage" 'apps/desktop_pet/target/release/bundle/appimage/Copilot Desktop Pet_0.1.0_amd64.AppImage'
189:       - name: Execute macOS release app
190:         if: runner.os == 'macOS'
191:         shell: bash
192:         run: |
193:           executable=$(find apps/desktop_pet/target/release/bundle/macos -path '*/Contents/MacOS/copilot-desktop-pet' -type f -print -quit)
194:           python3 scripts/pet-gui-verify.py --executable "$executable" --updater
195:       - name: Install signed macOS bundle update and cold-start updated executable
196:         if: runner.os == 'macOS'
197:         shell: bash
198:         run: |
199:           cp -R 'apps/desktop_pet/target/release/bundle/macos/Copilot Desktop Pet.app' "$RUNNER_TEMP/base-pet.app"
200:           python3 scripts/build-test-update.py
201:           python3 scripts/pet-gui-verify.py --executable "$RUNNER_TEMP/base-pet.app/Contents/MacOS/copilot-desktop-pet" --updater --update-package 'apps/desktop_pet/target/release/bundle/macos/Copilot Desktop Pet.app.tar.gz' --expected-executable 'apps/desktop_pet/target/release/bundle/macos/Copilot Desktop Pet.app/Contents/MacOS/copilot-desktop-pet'
202:           rm -rf 'apps/desktop_pet/target/release/bundle/macos/Copilot Desktop Pet.app'
203:           cp -R "$RUNNER_TEMP/base-pet.app" 'apps/desktop_pet/target/release/bundle/macos/Copilot Desktop Pet.app'
204:       - name: Install and execute Windows release package in disposable runner
205:         if: runner.os == 'Windows'
206:         shell: pwsh
207:         run: |
208:           $installer = Get-ChildItem apps/desktop_pet/target/release/bundle/nsis -Filter *.exe | Select-Object -First 1
209:           $destination = Join-Path $env:RUNNER_TEMP 'pet-package'
210:           $process = Start-Process $installer.FullName -ArgumentList "/S /D=$destination" -Wait -PassThru
211:           if ($process.ExitCode -ne 0) { throw "NSIS installation failed: $($process.ExitCode)" }
212:           $executable = Get-ChildItem $destination -Filter copilot-desktop-pet.exe -Recurse | Select-Object -First 1
213:           if (-not $executable) { throw 'Installed executable missing' }
214:           python3 scripts/pet-gui-verify.py --executable $executable.FullName --updater
215:           if ($LASTEXITCODE -ne 0) { throw 'Installed release GUI verification failed' }
216:       - name: Install signed Windows NSIS update and cold-start updated executable
217:         if: runner.os == 'Windows'
218:         shell: pwsh
219:         run: |
220:           $executable = Get-ChildItem (Join-Path $env:RUNNER_TEMP 'pet-package') -Filter copilot-desktop-pet.exe -Recurse | Select-Object -First 1
221:           $baseInstaller = Get-ChildItem apps/desktop_pet/target/release/bundle/nsis -Filter '*_0.1.0_*.exe' | Select-Object -First 1
222:           if (-not $baseInstaller) { throw 'Base Windows installer missing' }
223:           $baseName = $baseInstaller.Name
224:           Copy-Item $baseInstaller.FullName (Join-Path $env:RUNNER_TEMP 'base-installer.exe')
225:           python3 scripts/build-test-update.py
226:           if ($LASTEXITCODE -ne 0) { throw 'Windows update fixture build failed' }
227:           $update = Get-ChildItem apps/desktop_pet/target/release/bundle/nsis -Filter '*_0.1.1_*.exe' | Select-Object -First 1
228:           if (-not $update) { throw 'Windows update package missing' }
229:           python3 scripts/pet-gui-verify.py --executable $executable.FullName --updater --update-package $update.FullName --expected-executable apps/desktop_pet/target/test-update/copilot-desktop-pet.exe
230:           if ($LASTEXITCODE -ne 0) { throw 'Actual signed Windows update failed' }
231:           Copy-Item (Join-Path $env:RUNNER_TEMP 'base-installer.exe') (Join-Path 'apps/desktop_pet/target/release/bundle/nsis' $baseName) -Force
232:       - uses: actions/upload-artifact@v4
233:         with:
234:           name: desktop-${{ runner.os }}
235:           path: |
236:             apps/desktop_pet/target/release/bundle/deb/*.deb
237:             apps/desktop_pet/target/release/bundle/appimage/*_0.1.0_*.AppImage
238:             apps/desktop_pet/target/release/bundle/macos/*.app/
239:             apps/desktop_pet/target/release/bundle/nsis/*_0.1.0_*.exe
240:             target/pet-gui-validation.json
241:
242:       - name: Preserve isolated Windows updater diagnostics on failure
243:         if: failure() && runner.os == 'Windows'
244:         uses: actions/upload-artifact@v4
245:         with:
246:           name: updater-diagnostic-Windows
247:           path: |
248:             apps/desktop_pet/target/release/bundle/nsis/*.exe
249:             apps/desktop_pet/target/release/copilot-desktop-pet.exe
250:             target/pet-gui-validation.json
```

## copilot_use/Cargo.toml

SHA256 `d3f6dcd95d7a6016d31c8dbc47a3b942170f72d34cccbaa8844a71d7e21880a1`。

```
1: [workspace]
2: resolver = "3"
3: members = ["crates/analytics_transport", "crates/shared", "crates/ai_core", "crates/ui", "crates/telemetry", "crates/mcp_bridge", "crates/chat_domain", "crates/chat_application", "crates/persistence", "apps/server", "crates/pet_domain", "crates/pet_runtime", "apps/stats_server"]
4:
5: [workspace.package]
6: edition = "2024"
7: version = "0.1.0"
8:
9: [workspace.dependencies]
10: async-trait = "0.1"
11: axum = { version = "0.8", features = ["ws"] }
12: serde = { version = "1", features = ["derive"] }
13: serde_json = "1"
14: sqlx = { version = "0.9", features = ["runtime-tokio", "sqlite", "json", "uuid", "macros", "migrate"] }
15: tokio = { version = "1.53", features = ["full"] }
16: uuid = { version = "1", features = ["v4", "serde"] }
17: thiserror = "2"
```

## copilot_use/Dockerfile

SHA256 `031719c7f9c533871106ff79e1b5483831d72fdf431b653cef16b35f58179be5`。

```
1: # Multi-stage release build for independently deployable Chat and Analytics services.
2: # This Dockerfile does not package the Tauri Desktop Pet (native desktop build).
3: ARG COPILOT_BUILD_SHA=unversioned
4: FROM rust:1-bookworm AS builder
5: WORKDIR /src
6: RUN --mount=type=cache,id=copilot-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
7:     rustup target add wasm32-unknown-unknown && cargo install trunk --version 0.21.14 --locked
8: COPY . .
9: RUN --mount=type=cache,id=copilot-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
10:     --mount=type=cache,id=copilot-chat-wasm,target=/src/apps/server/frontend/target,sharing=locked \
11:     --mount=type=cache,id=copilot-trunk-tools,target=/root/.cache/trunk,sharing=locked \
12:     find apps crates -type f \( -name '*.rs' -o -name '*.sql' -o -name Cargo.toml \) -exec touch {} + \
13:     && bash scripts/build-chat-ui.sh
14: RUN --mount=type=cache,id=copilot-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
15:     --mount=type=cache,id=copilot-services-target,target=/src/target,sharing=locked \
16:     find apps crates -type f \( -name '*.rs' -o -name '*.sql' -o -name Cargo.toml \) -exec touch {} + \
17:     && cargo build --locked --release -p day9-chat -p day9-analytics \
18:     && mkdir /out && cp target/release/day9-chat target/release/day9-analytics /out/
19:
20: FROM debian:bookworm-slim AS chat
21: ARG COPILOT_BUILD_SHA
22: RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 curl \
23:     && rm -rf /var/lib/apt/lists/*
24: LABEL org.opencontainers.image.revision=$COPILOT_BUILD_SHA
25: WORKDIR /app
26: COPY --from=builder /out/day9-chat /app/server
27: COPY --from=builder /src/crates/persistence/migrations /app/migrations
28: COPY --from=builder /src/target/site/pkg /app/site/pkg
29: RUN mkdir -p /data && chown 65532:65532 /data && chmod 700 /data
30: ENV CHAT_DATABASE_URL=sqlite:///data/chat.sqlite3
31: ENV CHAT_SITE_DIR=/app/site/pkg
32: ENV CHAT_LISTEN=0.0.0.0:3001
33: USER 65532:65532
34: EXPOSE 3001
35: ENTRYPOINT ["/app/server"]
36:
37: FROM debian:bookworm-slim AS analytics
38: ARG COPILOT_BUILD_SHA
39: RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 curl \
40:     && rm -rf /var/lib/apt/lists/*
41: LABEL org.opencontainers.image.revision=$COPILOT_BUILD_SHA
42: WORKDIR /app
43: COPY --from=builder /out/day9-analytics /app/server
44: COPY --from=builder /src/apps/stats_server/migrations /app/migrations
45: ENV ANALYTICS_LISTEN=0.0.0.0:3100
46: USER 65532:65532
47: EXPOSE 3100
48: ENTRYPOINT ["/app/server"]
```

## copilot_use/apps/desktop_pet/Cargo.toml

SHA256 `76af1179a571c0b6add8eab615d99caf9ae3c377f272992490ee7c69aa0bb2cc`。

```
1: [package]
2: name = "copilot-desktop-pet"
3: version = "0.1.0"
4: edition = "2024"
5: build = "build.rs"
6: [workspace]
7: [dependencies]
8: day9-pet = { path = "../../crates/pet_runtime" }
9: tauri = { version = "2.12", features = [] }
10: serde = { version = "1", features = ["derive"] }
11: serde_json = "1"
12: tauri-plugin-updater = "=2.13.3"
13: updater-http= {package="reqwest",version="0.13",default-features=false}
14: base64="0.22"
15: minisign-verify="0.3"
16: semver="1"
17: url="2"
18: uuid = { version = "1", features = ["v4", "serde"] }
19: tokio = { version = "1.53", features = ["full"] }
20: copilot-telemetry = {path="../../crates/telemetry"}
21:
22: reqwest = { version = "0.12", default-features = false, features = ["rustls-tls", "json"] }
23:
24: [target.'cfg(unix)'.dependencies]
25: libc = "0.2"
26: [build-dependencies]
27: tauri-build = "2.7"
28:
29: [target.'cfg(target_os="linux")'.dependencies]
30: gtk="0.18"
31: glib="0.18.5"
32:
33: [patch.crates-io]
34: glib = { path = "../../vendor/glib-0.18.5" }
```

## copilot_use/apps/desktop_pet/capabilities/default.json

SHA256 `56e58758bc942665d03a48c054e1a6f22e8d2317baae6524545660e1a314218e`。

```
1: {
2:   "identifier": "default",
3:   "description": "本地寵物視窗基本權限",
4:   "windows": [
5:     "main",
6:     "pet"
7:   ],
8:   "permissions": [
9:     "core:default", "core:window:allow-start-dragging"
10:   ]
11: }
```

## copilot_use/apps/desktop_pet/frontend/Cargo.toml

SHA256 `311747d6728e45dc137d3a30752a077393a6909aa02154fee28cb2be1ee01d5f`。

```
1: [package]
2: name="copilot-pet-ui"
3: version="0.1.0"
4: edition="2024"
5: [workspace]
6: [dependencies]
7: leptos={version="0.8",features=["csr"]}
8: serde_json="1"
9: serde={version="1",features=["derive"]}
10: wasm-bindgen="0.2"
11: wasm-bindgen-futures="0.4"
12: js-sys="0.3"
```

## copilot_use/apps/desktop_pet/frontend/index.html

SHA256 `bdffd9ac19b27242e15efde070772654215cf7125ae629618d66124298a1c767`。

```
1: <!doctype html><html lang="zh-Hant"><head><meta charset="utf-8"><title>桌面寵物</title><link data-trunk rel="css" href="style.css"></head><body></body></html>
```

## copilot_use/apps/desktop_pet/frontend/style.css

SHA256 `893c0d215862d8f908856b765c0a4a0da091c1cfcc4e628d0e219af6c9bf9746`。

```
1: :root{font-family:system-ui;color:#22304a;background:#f4f7fb}body{margin:0;padding:24px}main{max-width:960px;margin:auto}section{background:white;padding:20px;border-radius:16px;margin-bottom:16px}h1{margin-top:0}input,textarea,select,button{font:inherit;padding:9px;border:1px solid #bdc7d8;border-radius:8px;margin:5px}textarea{width:92%;min-height:90px}button{background:#245ac4;color:white;cursor:pointer}label{display:inline-block;min-width:45%}.pet{width:200px;height:200px;background:#e4eefb;display:flex;align-items:center;justify-content:center;border-radius:24px;font-size:60px}.error{color:#b02035}article{border-top:1px solid #ddd;padding:10px}small{display:block;color:#526278}
2:
3: .pet{overflow:hidden}.sprite.animated{background-size:calc(var(--frame-w)*4) calc(var(--frame-h)*4);background-repeat:no-repeat;animation:pet-idle 1s steps(4) infinite}@keyframes pet-idle{from{background-position-x:0}to{background-position-x:calc(var(--frame-w)*-4)}}pre{white-space:pre-wrap;overflow-wrap:anywhere}
4:
5: body:has(.floating){margin:0;padding:0;background:transparent}.floating,.floating .pet{width:200px;height:200px;border-radius:0;background-color:transparent}
6:
7: .floating{position:relative}.floating .recall-prompt{position:absolute;bottom:0;left:0;width:180px;max-height:180px;overflow:auto;padding:10px;background:#ffffffed;border-radius:12px}.recall-prompt{border:1px solid #bdc7d8}
```

## copilot_use/apps/desktop_pet/tauri.conf.json

SHA256 `90ec72edaa7bd531b84b2df5a781aeeb19df316cf53724ecf63620af2abdde02`。

```
1: {
2:   "$schema": "https://schema.tauri.app/config/2",
3:   "productName": "Copilot Desktop Pet",
4:   "version": "0.1.0",
5:   "identifier": "com.copilotuse.desktoppet",
6:   "build": {
7:     "frontendDist": "frontend/dist",
8:     "beforeBuildCommand": "node ../scripts/build-pet-ui.mjs"
9:   },
10:   "app": {
11:     "withGlobalTauri": true,
12:     "windows": [
13:       {
14:         "label": "main",
15:         "title": "桌面寵物",
16:         "width": 860,
17:         "height": 720
18:       },
19:       {
20:         "label": "pet",
21:         "title": "桌面 AI 寵物",
22:         "width": 200,
23:         "height": 200,
24:         "resizable": false,
25:         "decorations": false,
26:         "transparent": true,
27:         "alwaysOnTop": true,
28:         "skipTaskbar": true,
29:         "shadow": false
30:       }
31:     ],
32:     "security": {
33:       "csp": "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; img-src 'self' data:; style-src 'self' 'unsafe-inline'; connect-src 'self' ipc: http://ipc.localhost",
34:       "capabilities": [
35:         "default"
36:       ]
37:     }
38:   },
39:   "bundle": {
40:     "active": true,
41:     "targets": [
42:       "deb",
43:       "appimage"
44:     ],
45:     "icon": [
46:       "icons/icon.png",
47:       "generated-icons/icon.ico",
48:       "generated-icons/icon.icns"
49:     ]
50:   },
51:   "plugins": {
52:     "updater": {
53:       "pubkey": "",
54:       "endpoints": [],
55:       "dangerousInsecureTransportProtocol": true,
56:       "requireSignedVersion": true
57:     }
58:   }
59: }
```

## copilot_use/apps/server/Cargo.toml

SHA256 `55854c9ca5c0477e7b984e05406774e34bee997596dc028a38da3d31a81e1ba7`。

```
1: [package]
2: name = "day9-chat"
3: version.workspace = true
4: edition.workspace = true
5:
6: [dependencies]
7: axum.workspace = true
8: serde.workspace = true
9: event-contract = { path = "../../crates/shared" }
10: serde_json.workspace = true
11: sqlx.workspace = true
12: tokio.workspace = true
13: uuid.workspace = true
14: thiserror.workspace = true
15: reqwest = { version = "0.12", default-features = false, features = ["rustls-tls", "json"] }
16:
17: tokio-stream = "0.1"
18: openai-adapter = { path = "../../crates/ai_core" }
19: argon2 = "0.5"
20: sha2 = "0.10"
21:
22: chat-ui = { path = "../../crates/ui", features = ["ssr"] }
23: leptos_axum = "0.8"
24: tower-http = {version="0.6",features=["fs"]}
25: any_spawner={version="0.3",features=["tokio"]}
26:
27: copilot-telemetry = {path="../../crates/telemetry"}
28: mcp-bridge={path="../../crates/mcp_bridge"}
29: url="2"
30: http="1"
31:
32:
33: chat-application={path="../../crates/chat_application"}
34: persistence={path="../../crates/persistence"}
35:
36: [target.'cfg(unix)'.dependencies]
37: libc = "0.2"
```

## copilot_use/apps/server/frontend/Cargo.toml

SHA256 `19be0887b82a078859aa0a99e55b399b4308e90ba044204d86ced0858dea6242`。

```
1: [package]
2: name="copilot-chat-frontend"
3: version="0.1.0"
4: edition="2024"
5: [workspace]
6: [dependencies]
7: chat-ui={path="../../../crates/ui",default-features=false,features=["hydrate"]}
```

## copilot_use/apps/server/frontend/index.html

SHA256 `470961f201157ecdb2f7201a79a0dc88e170f0c2e2c1cdddfd423d1b55c11905`。

```
1: <!doctype html><html lang="zh-Hant"><head><meta charset="utf-8"><title>Chat UI assets</title></head><body></body></html>
```

## copilot_use/apps/server/postgres-migrations/202610110001_chat.sql

SHA256 `50895618cd1eff41cdac92cd24e7550753aafbc541319f90104b98a768c56d21`。

```
1: -- 對照服務專用 DB；與 rust_ai_chat 的 Migration 完全隔離。
2: CREATE TABLE chat_rooms (
3:     id VARCHAR(40) PRIMARY KEY,
4:     name VARCHAR(200) NOT NULL
5: );
6: CREATE TABLE chat_messages (
7:     id UUID PRIMARY KEY,
8:     room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),
9:     author_kind VARCHAR(32) NOT NULL DEFAULT 'anonymous_user',
10:     author_id VARCHAR(255),
11:     content TEXT NOT NULL CHECK(length(trim(content)) > 0),
12:     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
13:     CHECK (author_kind IN ('anonymous_user','user','ai','system')),
14:     CHECK ((author_kind IN ('anonymous_user','system') AND author_id IS NULL)
15:         OR (author_kind IN ('user','ai') AND author_id IS NOT NULL))
16: );
17: CREATE TABLE chat_outbox (
18:     event_id UUID PRIMARY KEY,
19:     message_id UUID NOT NULL UNIQUE REFERENCES chat_messages(id),
20:     schema_version SMALLINT NOT NULL CHECK(schema_version=1),
21:     source_app VARCHAR(40) NOT NULL CHECK(source_app='web_chat'),
22:     event_type VARCHAR(80) NOT NULL CHECK(event_type='message.persisted'),
23:     status VARCHAR(16) NOT NULL DEFAULT 'pending'
24:         CHECK (status IN ('pending','leased','delivered','blocked')),
25:     attempts INTEGER NOT NULL DEFAULT 0 CHECK(attempts >= 0),
26:     next_attempt_at TIMESTAMPTZ NOT NULL DEFAULT now(),
27:     lease_token UUID,
28:     lease_until TIMESTAMPTZ,
29:     delivered_at TIMESTAMPTZ,
30:     last_error VARCHAR(128),
31:     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
32:     CHECK ((status='leased' AND lease_token IS NOT NULL AND lease_until IS NOT NULL)
33:         OR (status<>'leased' AND lease_token IS NULL AND lease_until IS NULL))
34: );
35: CREATE INDEX chat_outbox_due_idx ON chat_outbox (next_attempt_at, created_at)
36:  WHERE status IN ('pending','leased');
```

## copilot_use/apps/server/postgres-migrations/202610110006_auth.sql

SHA256 `a5cf9d3c93013218dc79994ebe842ee6f00449b41d2cfeeb33e7b6f72ccfb8a8`。

```
1: CREATE TABLE chat_users(id UUID PRIMARY KEY,username VARCHAR(100) NOT NULL UNIQUE,password_hash TEXT NOT NULL);
2: CREATE TABLE chat_sessions(token_hash BYTEA PRIMARY KEY,user_id UUID NOT NULL REFERENCES chat_users(id) ON DELETE CASCADE,expires_at TIMESTAMPTZ NOT NULL);
3: CREATE INDEX chat_sessions_expiry ON chat_sessions(expires_at);
4: CREATE TABLE chat_room_members(room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id) ON DELETE CASCADE,user_id UUID NOT NULL REFERENCES chat_users(id) ON DELETE CASCADE,PRIMARY KEY(room_id,user_id));
5: ALTER TABLE chat_rooms ADD COLUMN owner_id UUID REFERENCES chat_users(id);
```

## copilot_use/apps/server/postgres-migrations/202610110007_knowledge.sql

SHA256 `f87b75b53c108482e623fbd97d7e2b6c971c240638a9214e77d9df86102b999e`。

```
1: CREATE EXTENSION IF NOT EXISTS vector;
2: CREATE TABLE knowledge_documents(id UUID PRIMARY KEY,owner_id UUID REFERENCES chat_users(id) ON DELETE CASCADE,title VARCHAR(200) NOT NULL,content TEXT NOT NULL,embedding vector(1536) NOT NULL,created_at TIMESTAMPTZ NOT NULL DEFAULT now());
3: CREATE INDEX knowledge_owner ON knowledge_documents(owner_id);
```

## copilot_use/apps/server/postgres-migrations/202610110010_trace_context.sql

SHA256 `973f278ce42e9480e828b962317338f33dad44520aa073dc3d318fa17007dab8`。

```
1: ALTER TABLE chat_outbox ADD COLUMN traceparent VARCHAR(55);
```

## copilot_use/apps/server/postgres-migrations/202610110013_suggestion_policy.sql

SHA256 `1100a507bda01eef863c21955be2fcbd2c412bed5650078bff5933d0faa64ebf`。

```
1: CREATE TABLE chat_room_policy(room_id VARCHAR(40) PRIMARY KEY REFERENCES chat_rooms(id),context_messages BIGINT NOT NULL DEFAULT 30 CHECK(context_messages BETWEEN 10 AND 100),revision BIGINT NOT NULL DEFAULT 0);
2: CREATE TABLE chat_suggestion_receipts(proposal_id UUID NOT NULL,room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),before_value BIGINT NOT NULL,after_value BIGINT NOT NULL,applied_revision BIGINT NOT NULL,applied BOOLEAN NOT NULL DEFAULT TRUE,apply_receipt UUID NOT NULL,rollback_receipt UUID,PRIMARY KEY(proposal_id,room_id));
```

## copilot_use/apps/server/postgres-migrations/202610120016_connections.sql

SHA256 `99c8996ea7b1eaa1817fca30f9254922fafa289553c7ceb04f5c81b8a6c1f207`。

```
1: CREATE TABLE chat_connections(id UUID PRIMARY KEY,room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),user_id UUID REFERENCES chat_users(id),expires_at TIMESTAMPTZ NOT NULL);
2: CREATE INDEX chat_connections_room_expiry ON chat_connections(room_id,expires_at);
```

## copilot_use/apps/server/postgres-migrations/202610120017_auth_events.sql

SHA256 `21e25d654b4fe510f043f1cb945b1ff47dfed81950b5374cf6157899bd7a4b59`。

```
1: ALTER TABLE chat_outbox ALTER COLUMN message_id DROP NOT NULL;
2: ALTER TABLE chat_outbox DROP CONSTRAINT chat_outbox_event_type_check;
3: ALTER TABLE chat_outbox ADD CONSTRAINT chat_outbox_event_type_check CHECK(event_type IN ('message.persisted','login.succeeded','logout.succeeded'));
```

## copilot_use/apps/server/postgres-migrations/202610120018_personas.sql

SHA256 `9cc90f1d00bec8f1558a443746baa9759bf430eb8cbe4f77385e871484d0b21c`。

```
1: CREATE TABLE chat_personas(id UUID PRIMARY KEY,room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),name VARCHAR(80) NOT NULL,instructions TEXT NOT NULL CHECK(octet_length(instructions)<=4096),memory TEXT NOT NULL DEFAULT '' CHECK(octet_length(memory)<=2048),cooldown_seconds INTEGER NOT NULL DEFAULT 2 CHECK(cooldown_seconds BETWEEN 1 AND 30),enabled BOOLEAN NOT NULL DEFAULT true,revision BIGINT NOT NULL DEFAULT 0,last_spoke_at TIMESTAMPTZ,created_at TIMESTAMPTZ NOT NULL DEFAULT now());
2: CREATE INDEX chat_personas_room ON chat_personas(room_id,created_at,id);
3: CREATE TABLE chat_persona_runs(id UUID PRIMARY KEY,room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),parameters JSONB NOT NULL,status TEXT NOT NULL CHECK(status IN ('running','completed','failed','cancelled','budget_exhausted')),max_turns INTEGER NOT NULL CHECK(max_turns BETWEEN 1 AND 8),max_output_budget INTEGER NOT NULL CHECK(max_output_budget BETWEEN 512 AND 16384),reserved_tokens INTEGER NOT NULL DEFAULT 0,result JSONB NOT NULL DEFAULT '[]',updated_at TIMESTAMPTZ NOT NULL DEFAULT now());
4: CREATE TABLE chat_ai_room_locks(room_id VARCHAR(40) PRIMARY KEY REFERENCES chat_rooms(id),lease_token UUID NOT NULL,expires_at TIMESTAMPTZ NOT NULL);
```

## copilot_use/apps/server/postgres-migrations/202610120020_persona_failure.sql

SHA256 `ff4a21437f0e0fa46e85dfe99a4415b5501ff98f95b872a61e30cd77e5cc4c9a`。

```
1: ALTER TABLE chat_persona_runs ADD COLUMN failure_status INTEGER;
```

## copilot_use/apps/server/postgres-migrations/202610120021_auth_throttle.sql

SHA256 `2d146621cf22083caa2b4a14e5f3b7c403798825e5e8904bb6b09001013a7443`。

```
1: CREATE TABLE chat_auth_throttle(key TEXT PRIMARY KEY,attempts INTEGER NOT NULL,expires_at TIMESTAMPTZ NOT NULL);
2: CREATE INDEX chat_auth_throttle_expiry ON chat_auth_throttle(expires_at);
```

## copilot_use/apps/server/postgres-migrations/202610120024_topic_consent.sql

SHA256 `f5eb40c3d7294e80f6f2158adc6cc49a2983a09584d8a7f6e3bd3214a4b76e0d`。

```
1: ALTER TABLE chat_outbox DROP CONSTRAINT chat_outbox_event_type_check;
2: ALTER TABLE chat_outbox ADD CONSTRAINT chat_outbox_event_type_check CHECK(event_type IN ('message.persisted','login.succeeded','login.failed','logout.succeeded','keyword.rust','keyword.ownership','keyword.borrow','keyword.lifetime','keyword.async','keyword.trait','keyword.sql','keyword.postgresql','keyword.leptos','keyword.tauri'));
3: CREATE TABLE chat_topic_consent(user_id UUID NOT NULL REFERENCES chat_users(id),room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),enabled BOOLEAN NOT NULL DEFAULT false,withdraw_before TIMESTAMPTZ,withdraw_pending BOOLEAN NOT NULL DEFAULT false,PRIMARY KEY(user_id,room_id));
4: CREATE TABLE chat_topic_events(event_id UUID PRIMARY KEY,message_id UUID NOT NULL REFERENCES chat_messages(id),user_id UUID NOT NULL REFERENCES chat_users(id),room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),created_at TIMESTAMPTZ NOT NULL DEFAULT now(),withdrawal_queued BOOLEAN NOT NULL DEFAULT false,withdrawal_ack BOOLEAN NOT NULL DEFAULT false);
5: CREATE INDEX chat_topic_events_owner_idx ON chat_topic_events(user_id,room_id,created_at) WHERE NOT withdrawal_ack;
6: CREATE TABLE chat_topic_withdraw_outbox(id UUID PRIMARY KEY,event_ids UUID[] NOT NULL CHECK(cardinality(event_ids) BETWEEN 1 AND 500),status VARCHAR(16) NOT NULL DEFAULT 'pending' CHECK(status IN ('pending','leased','delivered')),attempts INTEGER NOT NULL DEFAULT 0,next_attempt_at TIMESTAMPTZ NOT NULL DEFAULT now(),lease_token UUID,lease_until TIMESTAMPTZ);
```

## copilot_use/apps/stats_server/Cargo.toml

SHA256 `bf721704aca33aadce104c97a27b9925964fedea35158d59251a6dde7f503106`。

```
1: [package]
2: name = "day9-analytics"
3: version.workspace = true
4: edition.workspace = true
5:
6: [dependencies]
7: analytics-transport = { path = "../../crates/analytics_transport" }
8: axum.workspace = true
9: event-contract = { path = "../../crates/shared" }
10: serde.workspace = true
11: serde_json.workspace = true
12: sqlx = {workspace = true, features = ["postgres"]}
13: tokio.workspace = true
14: uuid.workspace = true
15: chrono = { version = "0.4", features = ["serde"] }
16: sha2 = "0.10"
17: openai-adapter = { path = "../../crates/ai_core" }
18:
19: copilot-telemetry = {path="../../crates/telemetry"}
20:
21: tonic = "0.14"
22: async-stream = "0.3"
23: futures-core = "0.3"
24:
25: [dev-dependencies]
26: persistence = { path = "../../crates/persistence" }
27: day9-pet = { path = "../../crates/pet_runtime" }
28: tempfile = "3"
29: reqwest = { version = "0.12", default-features = false, features = ["rustls-tls", "json", "stream"] }
30: futures-util = "0.3"
```

## copilot_use/apps/stats_server/migrations/202610110001_analytics.sql

SHA256 `adb8cbb4c48ecc15e28b2d77be73549226fed3a6e82c9f1de838bd426d5d3346`。

```
1: -- Analytics 必須使用與 Chat 不同的 PostgreSQL database/角色。
2: CREATE TABLE product_events (
3:     source_app VARCHAR(40) NOT NULL CHECK(source_app IN ('web_chat','desktop_pet')),
4:     event_id UUID NOT NULL,
5:     schema_version SMALLINT NOT NULL CHECK(schema_version=1),
6:     event_type VARCHAR(128) NOT NULL CHECK(length(trim(event_type)) > 0),
7:     received_at TIMESTAMPTZ NOT NULL DEFAULT now(),
8:     PRIMARY KEY(source_app, event_id)
9: );
10: CREATE INDEX product_events_received_idx ON product_events (received_at);
```

## copilot_use/apps/stats_server/migrations/202610110008_archive.sql

SHA256 `cfa1a82a7ac0d8535aeb9f94925e6e29d4461c6a9463ecd8dffab2fb6e937a25`。

```
1: ALTER TABLE product_events ADD COLUMN archived BOOLEAN NOT NULL DEFAULT FALSE;
2: CREATE TABLE event_dedup_keys(key_hash BYTEA PRIMARY KEY CHECK(octet_length(key_hash)=32),type_hash BYTEA NOT NULL CHECK(octet_length(type_hash)=32));
3: INSERT INTO event_dedup_keys(key_hash,type_hash) SELECT sha256(convert_to(source_app||':'||event_id::text,'UTF8')),sha256(convert_to(event_type,'UTF8')) FROM product_events;
4: CREATE TABLE analysis_suggestions(id UUID PRIMARY KEY,target_app TEXT NOT NULL CHECK(target_app IN ('web_chat','desktop_pet')),metric_snapshot JSONB NOT NULL,summary TEXT NOT NULL,proposal JSONB NOT NULL,status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending','applied','rejected','rolled_back')),created_at TIMESTAMPTZ NOT NULL DEFAULT now(),updated_at TIMESTAMPTZ NOT NULL DEFAULT now());
```

## copilot_use/apps/stats_server/migrations/202610110012_suggestion_review.sql

SHA256 `f7d55aa8eca684c2d95d345d6d5f8f353819e33648cb16c043148593783fe9e2`。

```
1: ALTER TABLE analysis_suggestions DROP CONSTRAINT analysis_suggestions_status_check;
2: ALTER TABLE analysis_suggestions ADD CONSTRAINT analysis_suggestions_status_check CHECK(status IN ('pending','approved','applied','rejected','rolled_back'));
3: ALTER TABLE analysis_suggestions ADD COLUMN receipt_id UUID;
4: ALTER TABLE analysis_suggestions ADD COLUMN evaluation JSONB;
```

## copilot_use/apps/stats_server/migrations/202610120025_topic_withdrawal.sql

SHA256 `267bad2901ca717b4e9b4b6f05c9c1a9adc02409606ce044885389798abc9e99`。

```
1: ALTER TABLE product_events ADD COLUMN consent_withdrawn BOOLEAN NOT NULL DEFAULT false;
2: CREATE TABLE topic_privacy_state(event_id UUID PRIMARY KEY,withdrawn BOOLEAN NOT NULL DEFAULT false);
```

## copilot_use/apps/stats_server/migrations/202610130001_dashboard_revision.sql

SHA256 `3776063e65407bf0ce49ad5d8c7c4ec56ef68869aeb334d07dc18fb771b15938`。

```
1: -- The revision changes in the SAME transaction as event data. Polling observes only commits.
2: CREATE TABLE analytics_revision (
3:     singleton boolean PRIMARY KEY DEFAULT true CHECK (singleton),
4:     revision bigint NOT NULL DEFAULT 0 CHECK (revision >= 0)
5: );
6: INSERT INTO analytics_revision(singleton) VALUES (true);
7: CREATE FUNCTION advance_analytics_revision() RETURNS trigger LANGUAGE plpgsql AS $$
8: BEGIN
9:     UPDATE analytics_revision SET revision = revision + 1 WHERE singleton;
10:     RETURN NULL;
11: END;
12: $$;
13: CREATE TRIGGER analytics_events_changed
14: AFTER INSERT OR UPDATE OR DELETE ON product_events
15: FOR EACH ROW EXECUTE FUNCTION advance_analytics_revision();
```

## copilot_use/compose.yaml

SHA256 `b7974fad8d2ec910a77460b1f6bfc6f59b6b6274dfc9abfa066e0c3ea8d1abf6`。

```
1: # Isolated development integration environment; production requires TLS termination,
2: # dedicated secrets manager, metrics, backups and a hardened deployment.
3: services:
4:   analytics-db:
5:     image: postgres:17
6:     environment:
7:       POSTGRES_DB: analytics_db
8:       POSTGRES_USER: bootstrap_owner
9:       POSTGRES_PASSWORD: ${ANALYTICS_BOOTSTRAP_PASSWORD:?set ANALYTICS_BOOTSTRAP_PASSWORD}
10:       PG_APP_USER: analytics_user
11:       PG_APP_PASSWORD: ${ANALYTICS_DB_PASSWORD:?set ANALYTICS_DB_PASSWORD}
12:     volumes: [analytics-data:/var/lib/postgresql/data, "./scripts/postgres-init.sh:/docker-entrypoint-initdb.d/10-app.sh:ro"]
13:     networks: [backend]
14:     healthcheck:
15:       test: ["CMD-SHELL", "pg_isready -U bootstrap_owner -d analytics_db"]
16:       interval: 5s
17:       timeout: 5s
18:       retries: 20
19:   analytics:
20:     build:
21:       context: .
22:       target: analytics
23:       args: {COPILOT_BUILD_SHA: "${COPILOT_BUILD_SHA:-unversioned}"}
24:     environment:
25:       ANALYTICS_DATABASE_URL: postgres://analytics_user:${ANALYTICS_DB_PASSWORD:?set ANALYTICS_DB_PASSWORD}@analytics-db:5432/analytics_db
26:       ANALYTICS_API_TOKEN: ${ANALYTICS_API_TOKEN:?set ANALYTICS_API_TOKEN}
27:       ANALYTICS_ADMIN_TOKEN: ${ANALYTICS_ADMIN_TOKEN:?set ANALYTICS_ADMIN_TOKEN}
28:       ANALYTICS_CHAT_TOKEN: ${ANALYTICS_CHAT_TOKEN:?set ANALYTICS_CHAT_TOKEN}
29:       ANALYTICS_PET_TOKEN: ${ANALYTICS_PET_TOKEN:?set ANALYTICS_PET_TOKEN}
30:       ANALYTICS_LISTEN: 0.0.0.0:3100
31:       OPENAI_API_KEY: ${OPENAI_API_KEY:-}
32:       OPENAI_CHAT_MODEL: ${OPENAI_CHAT_MODEL:-gpt-5.6-luna}
33:     depends_on:
34:       analytics-db: {condition: service_healthy}
35:     networks: [backend]
36:     ports: ["127.0.0.1:3100:3100"]
37:     healthcheck:
38:       test: ["CMD", "curl", "--fail", "--silent", "--output", "/dev/null", "http://127.0.0.1:3100/healthz"]
39:       interval: 5s
40:       timeout: 3s
41:       retries: 20
42:       start_period: 15s
43:     restart: unless-stopped
44:     read_only: true
45:     tmpfs: ["/tmp:size=16m,mode=1777"]
46:     cap_drop: [ALL]
47:     security_opt: ["no-new-privileges:true"]
48:     pids_limit: 128
49:     mem_limit: 512m
50:     cpus: 2
51:   chat:
52:     build:
53:       context: .
54:       target: chat
55:       args: {COPILOT_BUILD_SHA: "${COPILOT_BUILD_SHA:-unversioned}"}
56:     environment:
57:       CHAT_DATABASE_URL: sqlite:///data/chat.sqlite3
58:       CHAT_API_TOKEN: ${CHAT_API_TOKEN:?set CHAT_API_TOKEN}
59:       ANALYTICS_CHAT_TOKEN: ${ANALYTICS_CHAT_TOKEN:?set ANALYTICS_CHAT_TOKEN}
60:       ANALYTICS_ENDPOINT: http://analytics:3100/api/v1/events
61:       ALLOW_INTERNAL_HTTP_DEV: "true"
62:       CHAT_LISTEN: 0.0.0.0:3001
63:       OPENAI_API_KEY: ${OPENAI_API_KEY:-}
64:       OPENAI_CHAT_MODEL: ${OPENAI_CHAT_MODEL:-gpt-5.6-luna}
65:       OPENAI_EMBEDDING_MODEL: ${OPENAI_EMBEDDING_MODEL:-text-embedding-3-small}
66:     depends_on:
67:       analytics: {condition: service_healthy}
68:     volumes: [chat-sqlite-data:/data]
69:     ports: ["127.0.0.1:3001:3001"]
70:     healthcheck:
71:       test: ["CMD", "curl", "--fail", "--silent", "--output", "/dev/null", "http://127.0.0.1:3001/healthz"]
72:       interval: 5s
73:       timeout: 3s
74:       retries: 20
75:       start_period: 15s
76:     networks: [backend]
77:     restart: unless-stopped
78:     read_only: true
79:     tmpfs: ["/tmp:size=16m,mode=1777"]
80:     cap_drop: [ALL]
81:     security_opt: ["no-new-privileges:true"]
82:     pids_limit: 128
83:     mem_limit: 512m
84:     cpus: 2
85: networks:
86:   backend:
87:     internal: false
88: volumes:
89:   chat-sqlite-data:
90:   analytics-data:
```

## copilot_use/crates/ai_core/Cargo.toml

SHA256 `874c445d85901ba7d6e647cd46d277bb95d21cc415f22d17d0410c29bebda662`。

```
1: [package]
2: name="openai-adapter"
3: version.workspace=true
4: edition.workspace=true
5: [dependencies]
6: reqwest={version="0.12",default-features=false,features=["rustls-tls","json"]}
7: serde_json.workspace=true
8: thiserror.workspace=true
9: base64="0.22"
10: serde.workspace=true
```

## copilot_use/crates/analytics_transport/Cargo.toml

SHA256 `b7b1fcede288e7186e8ce2b2ca7bfb454cb866947cbcbd19f784b2f45762f1a6`。

```
1: [package]
2: name = "analytics-transport"
3: version.workspace = true
4: edition.workspace = true
5:
6: [dependencies]
7: event-contract = { path = "../shared" }
8: tonic = { version = "0.14", features = ["tls-ring", "tls-native-roots"] }
9: tonic-prost = "0.14"
10: prost = "0.14"
11: uuid.workspace = true
12:
13: [build-dependencies]
14: tonic-prost-build = "0.14"
15: protoc-bin-vendored = "3"
```

## copilot_use/crates/chat_application/Cargo.toml

SHA256 `d2d9ccb51c14c9a9157b18ddbd1b3dca736cd480db0ce58b1a19044bee49d1d2`。

```
1: [package]
2: name="chat-application"
3: version.workspace=true
4: edition.workspace=true
5: [dependencies]
6: chat-domain={path="../chat_domain"}
7: [dev-dependencies]
8: tokio.workspace=true
9: uuid.workspace=true
```

## copilot_use/crates/chat_domain/Cargo.toml

SHA256 `14dd10c1ed8e5fe7b5bbe284b3ddee880d95c927f85c1d076b86866100de1637`。

```
1: [package]
2: name="chat-domain"
3: version.workspace=true
4: edition.workspace=true
5: [dependencies]
6: uuid.workspace=true
```

## copilot_use/crates/mcp_bridge/Cargo.toml

SHA256 `655709df4a6603fad91b379f239cad56b443eec38afc4502ea75d00131455bca`。

```
1: [package]
2: name="mcp-bridge"
3: version.workspace=true
4: edition.workspace=true
5: [dependencies]
6: serde.workspace=true
7: serde_json.workspace=true
8: thiserror.workspace=true
9: base64="0.22"
10: http="1"
11: reqwest={version="0.12",default-features=false,features=["rustls-tls","json"]}
12: url="2"
13: openai-adapter={path="../ai_core"}
```

## copilot_use/crates/persistence/Cargo.toml

SHA256 `a94c0893869c87c152e180f31f88e0becd6aeb0b167b0e8eafdf6158504df25a`。

```
1: [package]
2: name="persistence"
3: version.workspace=true
4: edition.workspace=true
5: [dependencies]
6: analytics-transport = { path = "../analytics_transport" }
7: chat-application={path="../chat_application"}
8: event-contract={path="../shared"}
9: copilot-telemetry={path="../telemetry"}
10: sqlx.workspace=true
11: tokio.workspace=true
12: uuid.workspace=true
13: serde_json.workspace=true
14: reqwest={version="0.12",default-features=false,features=["rustls-tls","json"]}
15: [target.'cfg(unix)'.dependencies]
16: libc="0.2"
```

## copilot_use/crates/persistence/migrations/202610120001_portable_chat.sql

SHA256 `5f522dc05e12b7848e3189de6dca026a76809eb9f31f89e97ed4ffe3768de72e`。

```
1: -- 對照服務專用 DB；與 rust_ai_chat 的 Migration 完全隔離。
2: CREATE TABLE chat_rooms (
3:     id VARCHAR(40) PRIMARY KEY,
4:     name VARCHAR(200) NOT NULL
5: );
6: CREATE TABLE chat_messages (
7:     id BLOB PRIMARY KEY,
8:     room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),
9:     author_kind VARCHAR(32) NOT NULL DEFAULT 'anonymous_user',
10:     author_id VARCHAR(255),
11:     content TEXT NOT NULL CHECK(length(trim(content)) > 0),
12:     created_at REAL NOT NULL DEFAULT (unixepoch('subsec')),
13:     CHECK (author_kind IN ('anonymous_user','user','ai','system')),
14:     CHECK ((author_kind IN ('anonymous_user','system') AND author_id IS NULL)
15:         OR (author_kind IN ('user','ai') AND author_id IS NOT NULL))
16: );
17: CREATE TABLE chat_outbox (
18:     event_id BLOB PRIMARY KEY,
19:     message_id BLOB UNIQUE REFERENCES chat_messages(id),
20:     schema_version SMALLINT NOT NULL CHECK(schema_version=1),
21:     source_app VARCHAR(40) NOT NULL CHECK(source_app='web_chat'),
22:     event_type VARCHAR(80) NOT NULL CHECK(event_type IN ('message.persisted','login.succeeded','login.failed','logout.succeeded','keyword.rust','keyword.ownership','keyword.borrow','keyword.lifetime','keyword.async','keyword.trait','keyword.sql','keyword.postgresql','keyword.leptos','keyword.tauri')),
23:     status VARCHAR(16) NOT NULL DEFAULT 'pending'
24:         CHECK (status IN ('pending','leased','delivered','blocked')),
25:     attempts INTEGER NOT NULL DEFAULT 0 CHECK(attempts >= 0),
26:     next_attempt_at REAL NOT NULL DEFAULT (unixepoch('subsec')),
27:     lease_token BLOB,
28:     lease_until REAL,
29:     delivered_at REAL,
30:     last_error VARCHAR(128),
31:     created_at REAL NOT NULL DEFAULT (unixepoch('subsec')),
32:     CHECK ((status='leased' AND lease_token IS NOT NULL AND lease_until IS NOT NULL)
33:         OR (status<>'leased' AND lease_token IS NULL AND lease_until IS NULL))
34: );
35: CREATE INDEX chat_outbox_due_idx ON chat_outbox (next_attempt_at, created_at)
36:  WHERE status IN ('pending','leased');
37: CREATE TABLE chat_users(id BLOB PRIMARY KEY,username VARCHAR(100) NOT NULL UNIQUE,password_hash TEXT NOT NULL);
38: CREATE TABLE chat_sessions(token_hash BLOB PRIMARY KEY,user_id BLOB NOT NULL REFERENCES chat_users(id) ON DELETE CASCADE,expires_at REAL NOT NULL);
39: CREATE INDEX chat_sessions_expiry ON chat_sessions(expires_at);
40: CREATE TABLE chat_room_members(room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id) ON DELETE CASCADE,user_id BLOB NOT NULL REFERENCES chat_users(id) ON DELETE CASCADE,PRIMARY KEY(room_id,user_id));
41: ALTER TABLE chat_rooms ADD COLUMN owner_id BLOB REFERENCES chat_users(id);
42:
43:
44: CREATE TABLE knowledge_documents(id BLOB PRIMARY KEY,owner_id BLOB REFERENCES chat_users(id) ON DELETE CASCADE,title VARCHAR(200) NOT NULL,content TEXT NOT NULL,embedding TEXT NOT NULL CHECK(json_valid(embedding) AND json_array_length(embedding)=1536),created_at REAL NOT NULL DEFAULT (unixepoch('subsec')));
45: CREATE INDEX knowledge_owner ON knowledge_documents(owner_id);
46:
47: ALTER TABLE chat_outbox ADD COLUMN traceparent VARCHAR(55);
48:
49: CREATE TABLE chat_room_policy(room_id VARCHAR(40) PRIMARY KEY REFERENCES chat_rooms(id),context_messages BIGINT NOT NULL DEFAULT 30 CHECK(context_messages BETWEEN 10 AND 100),revision BIGINT NOT NULL DEFAULT 0);
50: CREATE TABLE chat_suggestion_receipts(proposal_id BLOB NOT NULL,room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),before_value BIGINT NOT NULL,after_value BIGINT NOT NULL,applied_revision BIGINT NOT NULL,applied BOOLEAN NOT NULL DEFAULT TRUE,apply_receipt BLOB NOT NULL,rollback_receipt BLOB,PRIMARY KEY(proposal_id,room_id));
51:
52: CREATE TABLE chat_connections(id BLOB PRIMARY KEY,room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),user_id BLOB REFERENCES chat_users(id),expires_at REAL NOT NULL);
53: CREATE INDEX chat_connections_room_expiry ON chat_connections(room_id,expires_at);
54:
55:
56:
57:
58:
59: CREATE TABLE chat_personas(id BLOB PRIMARY KEY,room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),name VARCHAR(80) NOT NULL,instructions TEXT NOT NULL CHECK(length(CAST(instructions AS BLOB))<=4096),memory TEXT NOT NULL DEFAULT '' CHECK(length(CAST(memory AS BLOB))<=2048),cooldown_seconds INTEGER NOT NULL DEFAULT 2 CHECK(cooldown_seconds BETWEEN 1 AND 30),enabled BOOLEAN NOT NULL DEFAULT true,revision BIGINT NOT NULL DEFAULT 0,last_spoke_at REAL,created_at REAL NOT NULL DEFAULT (unixepoch('subsec')));
60: CREATE INDEX chat_personas_room ON chat_personas(room_id,created_at,id);
61: CREATE TABLE chat_persona_runs(id BLOB PRIMARY KEY,room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),parameters TEXT NOT NULL,status TEXT NOT NULL CHECK(status IN ('running','completed','failed','cancelled','budget_exhausted')),max_turns INTEGER NOT NULL CHECK(max_turns BETWEEN 1 AND 8),max_output_budget INTEGER NOT NULL CHECK(max_output_budget BETWEEN 512 AND 16384),reserved_tokens INTEGER NOT NULL DEFAULT 0,result TEXT NOT NULL DEFAULT '[]',updated_at REAL NOT NULL DEFAULT (unixepoch('subsec')));
62: CREATE TABLE chat_ai_room_locks(room_id VARCHAR(40) PRIMARY KEY REFERENCES chat_rooms(id),lease_token BLOB NOT NULL,expires_at REAL NOT NULL);
63:
64: ALTER TABLE chat_persona_runs ADD COLUMN failure_status INTEGER;
65:
66: CREATE TABLE chat_auth_throttle(key TEXT PRIMARY KEY,attempts INTEGER NOT NULL,expires_at REAL NOT NULL);
67: CREATE INDEX chat_auth_throttle_expiry ON chat_auth_throttle(expires_at);
68:
69:
70:
71: CREATE TABLE chat_topic_consent(user_id BLOB NOT NULL REFERENCES chat_users(id),room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),enabled BOOLEAN NOT NULL DEFAULT false,withdraw_before REAL,withdraw_pending BOOLEAN NOT NULL DEFAULT false,PRIMARY KEY(user_id,room_id));
72: CREATE TABLE chat_topic_events(event_id BLOB PRIMARY KEY,message_id BLOB NOT NULL REFERENCES chat_messages(id),user_id BLOB NOT NULL REFERENCES chat_users(id),room_id VARCHAR(40) NOT NULL REFERENCES chat_rooms(id),created_at REAL NOT NULL DEFAULT (unixepoch('subsec')),withdrawal_queued BOOLEAN NOT NULL DEFAULT false,withdrawal_ack BOOLEAN NOT NULL DEFAULT false);
73: CREATE INDEX chat_topic_events_owner_idx ON chat_topic_events(user_id,room_id,created_at) WHERE NOT withdrawal_ack;
74: CREATE TABLE chat_topic_withdraw_outbox(id BLOB PRIMARY KEY,event_ids TEXT NOT NULL CHECK(json_valid(event_ids) AND json_array_length(event_ids) BETWEEN 1 AND 500),status VARCHAR(16) NOT NULL DEFAULT 'pending' CHECK(status IN ('pending','leased','delivered')),attempts INTEGER NOT NULL DEFAULT 0,next_attempt_at REAL NOT NULL DEFAULT (unixepoch('subsec')),lease_token BLOB,lease_until REAL);
75:
76: -- Persistent process-neutral notifications, never uncommitted message content.
77: CREATE TABLE chat_realtime(seq INTEGER PRIMARY KEY AUTOINCREMENT,payload TEXT NOT NULL CHECK(json_valid(payload)),created_at REAL NOT NULL DEFAULT(unixepoch('subsec')));
78: CREATE INDEX chat_realtime_created ON chat_realtime(created_at);
```

## copilot_use/crates/pet_domain/Cargo.toml

SHA256 `8a32922a94c31b63f2959995aa6eecfd260e22af69a5cd51cee889f0a2c45834`。

```
1: [package]
2: name = "pet-domain"
3: version.workspace = true
4: edition.workspace = true
5:
6: [dependencies]
7: toml = "0.9"
8: serde.workspace = true
9: serde_json.workspace = true
10: thiserror.workspace = true
11: uuid = { version = "1", features = ["serde"] }
12:
13: [dev-dependencies]
14: uuid = { version = "1", features = ["v4"] }
15: tokio.workspace = true
```

## copilot_use/crates/pet_runtime/Cargo.toml

SHA256 `d52c046d52b358a012cf27a41f6c49b6537b7099264579d23093fd6c6f74b49d`。

```
1: [package]
2: name = "day9-pet"
3: version.workspace = true
4: edition.workspace = true
5:
6: [dependencies]
7: analytics-transport = { path = "../analytics_transport" }
8: pet-domain = { path = "../pet_domain" }
9: toml = "0.9"
10: serde_json.workspace = true
11: event-contract = { path = "../shared" }
12: serde.workspace = true
13: uuid.workspace = true
14: thiserror.workspace = true
15: sqlx = { workspace = true, features = ["sqlite"] }
16: tokio.workspace = true
17: reqwest = { version = "0.12", default-features = false, features = ["rustls-tls", "json"] }
18: base64 = "0.22"
19: sha2 = "0.10"
20:
21: openai-adapter = { path = "../ai_core" }
22:
23: copilot-telemetry = {path="../telemetry"}
24:
25: mcp-bridge={path="../mcp_bridge"}
26:
27: [target.'cfg(unix)'.dependencies]
28: libc = "0.2"
29:
30: [target.'cfg(windows)'.dependencies]
31: windows-sys = {version="0.61",features=["Wdk_Foundation","Wdk_Storage_FileSystem","Win32_Foundation","Win32_Security","Win32_Security_Authorization","Win32_Storage_FileSystem","Win32_System_IO","Win32_System_Threading","Win32_System_WindowsProgramming"]}
32:
33: [dev-dependencies]
34: tempfile = "3"
```

## copilot_use/crates/pet_runtime/migrations/202610110003_pet_state.sql

SHA256 `1c435ac74414494bd095669ec3cba512507d6e346446915d7764f3111e32cbc5`。

```
1: CREATE TABLE pets(id TEXT PRIMARY KEY, profile TEXT NOT NULL);
2: CREATE TABLE memories(id TEXT PRIMARY KEY, pet_id TEXT NOT NULL REFERENCES pets(id) ON DELETE CASCADE, content TEXT NOT NULL, created_at INTEGER NOT NULL);
3: CREATE TABLE cards(id TEXT PRIMARY KEY, pet_id TEXT NOT NULL REFERENCES pets(id) ON DELETE CASCADE, question TEXT NOT NULL, answer TEXT NOT NULL, repetitions INTEGER NOT NULL DEFAULT 0, interval_days INTEGER NOT NULL DEFAULT 0, ease REAL NOT NULL DEFAULT 2.5, due_at INTEGER NOT NULL);
4: CREATE TABLE pet_outbox(event_id TEXT PRIMARY KEY, payload TEXT NOT NULL, delivered INTEGER NOT NULL DEFAULT 0, attempts INTEGER NOT NULL DEFAULT 0, next_attempt_at INTEGER NOT NULL DEFAULT 0);
5: CREATE TABLE conversations(id TEXT PRIMARY KEY, pet_id TEXT NOT NULL REFERENCES pets(id) ON DELETE CASCADE, role TEXT NOT NULL CHECK(role IN ('user','assistant')), content TEXT NOT NULL, created_at INTEGER NOT NULL);
```

## copilot_use/crates/pet_runtime/migrations/202610110004_pet_assets.sql

SHA256 `bd815470e3677749051fa2738194e2828f7b1337b54c8169c299191a62637f08`。

```
1: CREATE TABLE assets(pet_id TEXT PRIMARY KEY REFERENCES pets(id) ON DELETE CASCADE, png_base64 TEXT NOT NULL, width INTEGER NOT NULL, height INTEGER NOT NULL, version INTEGER NOT NULL, generated_at INTEGER NOT NULL);
```

## copilot_use/crates/pet_runtime/migrations/202610110005_cleanup_receipts.sql

SHA256 `ae11b701e081cbc33701dd560a28a549a7a2254ba7bcdd3c4353e29932855818`。

```
1: CREATE TABLE cleanup_receipts(operation_id TEXT PRIMARY KEY,receipt TEXT NOT NULL,created_at INTEGER NOT NULL);
```

## copilot_use/crates/pet_runtime/migrations/202610110009_settings.sql

SHA256 `3122b8e9b7c6ebc46041bbaaec85c9da42813a34bed49d30a2aa76b3720b6397`。

```
1: CREATE TABLE settings(key TEXT PRIMARY KEY,value TEXT NOT NULL);
```

## copilot_use/crates/pet_runtime/migrations/202610110011_outbox_leases.sql

SHA256 `1a7de2c0dd8ccf396bd199a43a685aa6a63cd3820615efda3f78d1849cf4e4a6`。

```
1: ALTER TABLE pet_outbox ADD COLUMN lease_token TEXT;
2: ALTER TABLE pet_outbox ADD COLUMN lease_until INTEGER;
3: ALTER TABLE pet_outbox ADD COLUMN last_error TEXT;
4: ALTER TABLE pet_outbox ADD COLUMN traceparent TEXT;
5: CREATE INDEX pet_outbox_ready ON pet_outbox(delivered,next_attempt_at,lease_until);
```

## copilot_use/crates/pet_runtime/migrations/202610110014_suggestion_policy.sql

SHA256 `64056b043153c7ecb5a9d8396b7c4a933ab53262e775f974c6f5787ccd5bf451`。

```
1: CREATE TABLE pet_policy(pet_id TEXT PRIMARY KEY REFERENCES pets(id) ON DELETE CASCADE,review_min_days INTEGER NOT NULL DEFAULT 1 CHECK(review_min_days BETWEEN 1 AND 30),revision INTEGER NOT NULL DEFAULT 0);
2: CREATE TABLE pet_suggestion_receipts(proposal_id TEXT NOT NULL,pet_id TEXT NOT NULL REFERENCES pets(id) ON DELETE CASCADE,before_value INTEGER NOT NULL,after_value INTEGER NOT NULL,applied_revision INTEGER NOT NULL,applied INTEGER NOT NULL DEFAULT 1,apply_receipt TEXT NOT NULL,rollback_receipt TEXT,PRIMARY KEY(proposal_id,pet_id));
```

## copilot_use/crates/pet_runtime/migrations/202610120015_restore_receipts.sql

SHA256 `67544b566fcfc8972760655dff07baeaf972a836ddf16c4eb4af5ec3cc143251`。

```
1: CREATE TABLE restore_receipts(operation_id TEXT PRIMARY KEY,receipt TEXT NOT NULL,created_at INTEGER NOT NULL);
```

## copilot_use/crates/pet_runtime/migrations/202610120019_conversation_order.sql

SHA256 `717eb443a861846601adc1fcd9437b31047544ec05ed0c9c007d26b0c36ddced`。

```
1: ALTER TABLE conversations ADD COLUMN ordinal INTEGER NOT NULL DEFAULT 0;
2: UPDATE conversations SET ordinal=rowid;
3: CREATE UNIQUE INDEX conversations_pet_ordinal ON conversations(pet_id,ordinal);
```

## copilot_use/crates/pet_runtime/migrations/202610120022_asset_profile.sql

SHA256 `1ea5a8a7c2983e71c98d094c562975b2379dc1104d3398768a61cc71881c668f`。

```
1: ALTER TABLE assets ADD COLUMN profile_hash TEXT NOT NULL DEFAULT '';
```

## copilot_use/crates/pet_runtime/migrations/202610120023_pet_archive.sql

SHA256 `00b2282c38beb7125e92d25fb0aebf0022655e8ffa1235fe32c7e781e13d2578`。

```
1: ALTER TABLE pets ADD COLUMN archived_at INTEGER;
```

## copilot_use/crates/shared/Cargo.toml

SHA256 `678113ba6d3371114ca91d3d9ad0c351d8e8188e6eefae627705d1b315849fc3`。

```
1: [package]
2: name = "event-contract"
3: version.workspace = true
4: edition.workspace = true
5:
6: [dependencies]
7: serde.workspace = true
8: uuid.workspace = true
9:
10: [dev-dependencies]
11: serde_json.workspace = true
```

## copilot_use/crates/shared/proto/analytics.proto

SHA256 `26d386852a4750d4cbaf63e1c51250e38be66b1c4fce3566cd449c8aa5e6b11e`。

```
1: syntax = "proto3";
2: package copilot.analytics.v1;
3:
4: // Only the existing four low-sensitivity fields cross the service boundary.
5: message ProductEvent {
6:   uint32 schema_version = 1;
7:   string event_id = 2;
8:   string source_app = 3;
9:   string event_type = 4;
10: }
11: message IngestEventsRequest { repeated ProductEvent events = 1; }
12: enum Outcome {
13:   OUTCOME_UNSPECIFIED = 0;
14:   STORED = 1;
15:   ALREADY_EXISTS = 2;
16:   INVALID_EVENT = 3;
17:   IDENTITY_CONFLICT = 4;
18: }
19: message EventResult {
20:   string event_id = 1;
21:   string source_app = 2;
22:   Outcome outcome = 3;
23: }
24: message IngestEventsResponse { repeated EventResult results = 1; }
25: service EventIngestion {
26:   // 1..64 events; each successful result acknowledges a committed transaction.
27:   rpc IngestEvents(IngestEventsRequest) returns (IngestEventsResponse);
28: }
```

## copilot_use/crates/telemetry/Cargo.toml

SHA256 `5772c1ecfad55cc6fa7c1d02f20efc099afbb3a9916c90b94acc8f1e42e4b040`。

```
1: [package]
2: name="copilot-telemetry"
3: version.workspace=true
4: edition.workspace=true
5: [dependencies]
6: opentelemetry = {version="0.33",features=["trace","metrics","logs"]}
7: opentelemetry_sdk = {version="0.33",features=["trace","metrics","logs"]}
8: opentelemetry-otlp = {version="0.33",default-features=false,features=["trace","metrics","logs","http-json","reqwest-blocking-client"]}
9: axum.workspace=true
10: tokio.workspace=true
11: url="2"
12: reqwest={version="0.13",default-features=false,features=["blocking","rustls"]}
13: opentelemetry-http={version="0.33",features=["reqwest-blocking"]}
14: async-trait.workspace=true
15: http="1"
16: bytes="1"
17:
18: [dev-dependencies]
19: futures-executor="0.3"
```

## copilot_use/crates/ui/Cargo.toml

SHA256 `00f5e9f8e5ca87e7a62a52ddc58e32a7f1d9606749058797157589fe952fa90e`。

```
1: [package]
2: name="chat-ui"
3: version.workspace=true
4: edition.workspace=true
5: [features]
6: default=["ssr"]
7: ssr=["leptos/ssr","leptos_router/ssr"]
8: hydrate=["leptos/hydrate"]
9: [dependencies]
10: leptos={version="0.8",default-features=false}
11: leptos_router="0.8"
12: serde_json.workspace=true
13: [target.'cfg(target_arch="wasm32")'.dependencies]
14: wasm-bindgen="0.2"
15: wasm-bindgen-futures="0.4"
16: js-sys="0.3"
17: gloo-net={version="0.6",features=["http"]}
18: web-sys={version="0.3",features=["Window","Document","Element","Location","WebSocket","MessageEvent","Event","CloseEvent","HtmlInputElement"]}
19: send_wrapper="0.6"
20: gloo-timers={version="0.3",features=["futures"]}
```

## copilot_use/crates/ui/style.css

SHA256 `ba0006968bd154077e71bdf435f3937b42a8814012ba2633af294068202469ab`。

```
1: :root{font-family:system-ui;color:#22304a;background:#f4f7fb}body{margin:0;padding:24px}main{max-width:1000px;margin:auto}nav{display:flex;gap:18px;margin-bottom:20px}section{background:white;border-radius:16px;padding:22px;margin:16px 0}input,textarea,button,select{font:inherit;border-radius:8px;padding:10px;border:1px solid #bdc7d8;margin:6px}button{background:#245ac4;color:white;cursor:pointer}button:disabled{opacity:.5}.error{color:#b02035}#messages{height:360px;overflow:auto;list-style:none;padding:10px;background:#f7f9fc}li{padding:10px;border-bottom:1px solid #ddd;overflow-wrap:anywhere}small{display:block;color:#66768c;font-size:11px}textarea{width:90%;height:120px}pre{white-space:pre-wrap;overflow-wrap:anywhere}label{display:block}
```

## copilot_use/docs/legacy-migrations/202610110001_day9_chat.sql

SHA256 `274f2690e01f55770ae502e5f44d0ced0968809713bf6f81afbc1acd357343e8`。

```
1: -- 此為 copilot_use 獨立 PostgreSQL 對照環境；不可套用至 rust_ai_chat 原資料庫。
2: -- 本版先選 native UUID；切換 VARCHAR(40) 時僅變更 Migration 與 persistence adapter。
3: CREATE TABLE chat_messages (
4:     id UUID PRIMARY KEY,
5:     room_id TEXT NOT NULL,
6:     content TEXT NOT NULL,
7:     created_at TIMESTAMPTZ NOT NULL DEFAULT now()
8: );
9:
10: CREATE TABLE chat_outbox (
11:     event_id UUID PRIMARY KEY REFERENCES chat_messages(id),
12:     schema_version SMALLINT NOT NULL CHECK (schema_version = 1),
13:     source_app TEXT NOT NULL,
14:     event_type TEXT NOT NULL,
15:     status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'leased', 'delivered', 'blocked')),
16:     attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
17:     next_attempt_at TIMESTAMPTZ NOT NULL DEFAULT now(),
18:     lease_token UUID,
19:     lease_until TIMESTAMPTZ,
20:     last_error TEXT,
21:     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
22:     delivered_at TIMESTAMPTZ
23: );
24: CREATE INDEX chat_outbox_due ON chat_outbox (next_attempt_at, created_at)
25: WHERE status IN ('pending', 'leased');
```

## copilot_use/docs/legacy-migrations/202610110002_day9_analytics.sql

SHA256 `d02e675f06625ecb4881726bd46943de75ca7891dfbc801828b943451a332559`。

```
1: -- Analytics 獨立 PostgreSQL 資料庫。不要在 Chat DB 套用此 migration。
2: CREATE TABLE product_events (
3:     source_app TEXT NOT NULL,
4:     event_id UUID NOT NULL,
5:     schema_version SMALLINT NOT NULL CHECK (schema_version = 1),
6:     event_type TEXT NOT NULL,
7:     received_at TIMESTAMPTZ NOT NULL DEFAULT now(),
8:     PRIMARY KEY (source_app, event_id)
9: );
10: CREATE INDEX product_events_received_at_idx ON product_events(received_at);
```

## copilot_use/examples/pet.toml

SHA256 `717096edbf6929b6a48aaffb867c867161250455d0a80920903a954934caa3c6`。

```
1: "名字" = "小啾"
2: "種類" = "鳥"
3: "喜好" = ["唱歌", "水果"]
4: "個性" = ["活潑"]
5: "興趣" = ["音樂", "探索"]
```

## copilot_use/scripts/architecture-boundaries.py

SHA256 `2b8c6984bbaacbbcd7643fac4b6998b189cf8fa6708f35434f9af5c24a28b3a0`。

```
1: #!/usr/bin/env python3
2: """驗證實際 Cargo 依賴圖，防止 Driver/Adapter 回流到純 Domain/Application。"""
3: import json
4: import pathlib
5: import subprocess
6: import sys
7:
8: ROOT = pathlib.Path(__file__).resolve().parents[1]
9: PURE = {"chat-domain", "chat-application", "pet-domain"}
10: FORBIDDEN = {"reqwest", "axum", "hyper", "tauri", "gtk", "glib", "day9-pet",
11:              "day9-chat", "day9-analytics", "persistence", "mcp-bridge",
12:              "openai-adapter", "copilot-telemetry", "analytics-transport", "tonic", "prost"}
13:
14:
15: def main():
16:     metadata = json.loads(subprocess.check_output(
17:         ["cargo", "metadata", "--locked", "--format-version", "1"]
18:         + (["--offline"] if "--offline" in sys.argv else []), cwd=ROOT
19:     ))
20:     names = {p["id"]: p["name"] for p in metadata["packages"]}
21:     nodes = {n["id"]: n for n in metadata["resolve"]["nodes"]}
22:     roots = {p["name"]: p["id"] for p in metadata["packages"]
23:              if p["id"] in metadata["workspace_members"] and p["name"] in PURE}
24:     if set(roots) != PURE:
25:         raise SystemExit("Missing pure workspace packages: " + str(PURE - roots.keys()))
26:     failures = []
27:     for root_name, root_id in roots.items():
28:         pending = [(root_id, [root_name])]
29:         seen = set()
30:         while pending:
31:             package, chain = pending.pop()
32:             if package in seen:
33:                 continue
34:             seen.add(package)
35:             for dependency in nodes[package]["deps"]:
36:                 # dev dependencies belong to test harnesses, never runtime architecture.
37:                 if not any(k["kind"] in (None, "build") for k in dependency["dep_kinds"]):
38:                     continue
39:                 child = dependency["pkg"]
40:                 name = names[child]
41:                 route = chain + [name]
42:                 if name in FORBIDDEN or name.startswith(("sqlx", "tauri-")):
43:                     failures.append(" -> ".join(route))
44:                 pending.append((child, route))
45:     if failures:
46:         raise SystemExit("Adapter dependency crossed core boundary:\n" + "\n".join(failures))
47:     for root_name in sorted(roots):
48:         print("PASS pure normal/build dependencies:", root_name)
49:
50:
51: if __name__ == "__main__":
52:     main()
```

## copilot_use/scripts/build-chat-ui.sh

SHA256 `ab9a1549d317fbce80f55e30b5345953a56d665a4f14f47b2d3e36a3d59c6a59`。

```
1: #!/usr/bin/env bash
2: set -euo pipefail
3: ROOT=$(cd "$(dirname "$0")/.." && pwd)
4: (cd "$ROOT/apps/server/frontend" && NO_COLOR=true trunk build --release=true --no-color)
5: mkdir -p "$ROOT/target/site/pkg"
6: cp "$ROOT"/apps/server/frontend/dist/*.js "$ROOT/target/site/pkg/chat_ui.js"
7: cp "$ROOT"/apps/server/frontend/dist/*_bg.wasm "$ROOT/target/site/pkg/chat_ui_bg.wasm"
8: cp "$ROOT/crates/ui/style.css" "$ROOT/target/site/pkg/chat.css"
```

## copilot_use/scripts/build-pet-ui.mjs

SHA256 `c26bb834deecbd334294cd9cc2f96ed91f66d93a7d18dcaea5ffef6ab466e969`。

```
1: #!/usr/bin/env node
2: // Build platform resources from the single source icon; keep generated copies out of Git.
3: import { mkdtempSync, mkdirSync, copyFileSync, rmSync } from 'node:fs';
4: import { tmpdir } from 'node:os';
5: import { dirname, join, resolve } from 'node:path';
6: import { fileURLToPath } from 'node:url';
7: import { spawnSync } from 'node:child_process';
8: const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
9: const native = join(root, 'apps/desktop_pet');
10: function run(command, args, cwd, stdio='inherit') {
11:   const result=spawnSync(command,args,{cwd,stdio,env:{...process.env,NO_COLOR:'true'}});
12:   if(result.error)throw result.error;
13:   if(result.status!==0)throw new Error(`${command} exited ${result.status}`);
14: }
15: const scratch=mkdtempSync(join(tmpdir(),'copilot-icons-'));
16: try {
17:   run('cargo',['tauri','icon',join(native,'icons/icon.png'),'--output',scratch],native,['ignore','ignore','inherit']);
18:   const generated=join(native,'generated-icons');mkdirSync(generated,{recursive:true});
19:   for(const name of ['icon.ico','icon.icns'])copyFileSync(join(scratch,name),join(generated,name));
20: } finally {rmSync(scratch,{recursive:true,force:true});}
21: run('trunk',['build','--release=true','--no-color'],join(root,'apps/desktop_pet/frontend'));
```

## copilot_use/scripts/build-test-update.py

SHA256 `a25a9c139c3a2da194a97b8d4bef2af2553c8c4145ee4c2dff6edbf18f9a1208`。

```
1: #!/usr/bin/env python3
2: """Build a disposable 0.1.1 package and preserve an exact packaged-executable reference."""
3: import os,pathlib,shutil,subprocess,sys,tarfile
4:
5: def verify_nsis_reference(compiled: bytes, packaged: bytes):
6:     # Tauri restores UNK in target/release after embedding NSS into NSIS.
7:     # Only that official bundle marker may differ from the compiled program.
8:     raw=b'__TAURI_BUNDLE_TYPE_VAR_UNK';nsis=b'__TAURI_BUNDLE_TYPE_VAR_NSS'
9:     if compiled.count(raw)!=1 or packaged.count(nsis)!=1 or compiled.replace(raw,nsis,1)!=packaged:
10:         raise ValueError('NSIS payload differs from compiled program beyond its bundle marker')
11:
12: def build():
13:     root=pathlib.Path(__file__).resolve().parents[1]/'apps/desktop_pet'
14:     paths=[root/'Cargo.toml',root/'Cargo.lock'];saved={p:p.read_bytes() for p in paths}
15:     try:
16:         bundles='app' if sys.platform=='darwin' else 'nsis' if sys.platform=='win32' else 'appimage'
17:         subprocess.run(['cargo','tauri','build','--bundles',bundles,'--config','{"version":"0.1.1"}'],cwd=root,env=os.environ|{'NO_COLOR':'true'},check=True)
18:         if sys.platform=='darwin':
19:             app=root/'target/release/bundle/macos/Copilot Desktop Pet.app'
20:             with tarfile.open(app.with_suffix('.app.tar.gz'),'w:gz')as archive:archive.add(app,arcname=app.name)
21:         elif sys.platform=='win32':
22:             seven_zip=shutil.which('7z')
23:             if not seven_zip:
24:                 known=pathlib.Path(os.environ.get('ProgramFiles','C:/Program Files'))/'7-Zip/7z.exe'
25:                 if known.is_file():seven_zip=str(known)
26:             if not seven_zip:raise RuntimeError('7-Zip required to verify actual NSIS payload')
27:             package=root/'target/release/bundle/nsis/Copilot Desktop Pet_0.1.1_x64-setup.exe'
28:             extracted=root/'target/test-update/nsis'
29:             if extracted.exists():shutil.rmtree(extracted)
30:             subprocess.run([seven_zip,'x',str(package),'-o'+str(extracted),'-y'],check=True,stdout=subprocess.DEVNULL)
31:             executables=list(extracted.rglob('copilot-desktop-pet.exe'))
32:             if len(executables)!=1:raise RuntimeError('NSIS must contain precisely one main executable')
33:             verify_nsis_reference((root/'target/release/copilot-desktop-pet.exe').read_bytes(),executables[0].read_bytes())
34:             shutil.copyfile(executables[0],root/'target/test-update/copilot-desktop-pet.exe')
35:     finally:
36:         for p,content in saved.items():p.write_bytes(content)
37:
38: if __name__=='__main__':build()
```

## copilot_use/scripts/docker-local-verify.py

SHA256 `f4d0723c38058d4e6a00419b09af3da9938a000ac18b4011ac9adc5348d50cc8`。

```
1: #!/usr/bin/env python3
2: """Verify real local release containers without printing credentials or full inspect data."""
3: import argparse,hashlib,json,os,pathlib,subprocess,time,urllib.request,urllib.error,uuid
4: ROOT=pathlib.Path(__file__).resolve().parents[1]
5: SOURCE_HEAD=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
6: SOURCE_DIRTY=bool(subprocess.check_output(['git','status','--porcelain'],cwd=ROOT,text=True).strip())
7: a=argparse.ArgumentParser();a.add_argument('--env-file',default='.env.docker-local');a.add_argument('--project',default='copilot-p9-local');args=a.parse_args()
8: settings=ROOT/args.env_file
9: if settings.stat().st_mode&0o077:raise SystemExit('Private settings file must have mode 0600')
10: values={line.split('=',1)[0]:line.split('=',1)[1]for line in settings.read_text().splitlines()if '='in line and not line.startswith('#')}
11: compose=['docker','compose','--env-file',str(settings),'-p',args.project]
12: checks=[]
13: def command(extra,**kwargs):return subprocess.check_output(compose+extra,cwd=ROOT,**kwargs)
14: def checked(name,ok=True):assert ok,name;checks.append(name);print('PASS:',name,flush=True)
15: class NoRedirect(urllib.request.HTTPRedirectHandler):
16:  def redirect_request(self,*_):return None
17: client=urllib.request.build_opener(urllib.request.ProxyHandler({}),NoRedirect())
18: def http(base,path,key=None,body=None):
19:  r=urllib.request.Request(base+path,data=json.dumps(body).encode()if body is not None else None,headers={'Content-Type':'application/json',**({'Authorization':'Bearer '+key}if key else{})})
20:  try:
21:   with client.open(r,timeout=30)as response:
22:    data=response.read();return response.status,json.loads(data)if data else None
23:  except urllib.error.HTTPError as e:return e.code,None
24:  except (urllib.error.URLError,OSError):return 0,None
25: def wait(test,seconds=45):
26:  limit=time.monotonic()+seconds
27:  while not test():
28:   if time.monotonic()>limit:raise AssertionError('condition timed out')
29:   time.sleep(.3)
30: def sql(service,query,db):return command(['exec','-T',service,'psql','-U','bootstrap_owner','-d',db,'-At','-v','ON_ERROR_STOP=1','-c',query],text=True).strip()
31: chat='http://127.0.0.1:3001';analytics='http://127.0.0.1:3100';ct=values['CHAT_API_TOKEN'];at=values['ANALYTICS_API_TOKEN']
32: wait(lambda:http(chat,'/healthz')[0]==200 and http(analytics,'/healthz')[0]==200)
33: checked('both release services ready on localhost')
34: container_info={}
35: for service in ('chat','analytics'):
36:  cid=command(['ps','-q',service],text=True).strip()
37:  fmt='{{json .Config.User}} {{json .HostConfig.ReadonlyRootfs}} {{json .HostConfig.CapDrop}} {{json .HostConfig.SecurityOpt}} {{json .HostConfig.Memory}} {{json .HostConfig.PidsLimit}}'
38:  raw=subprocess.check_output(['docker','inspect','--format',fmt,cid],text=True).strip()
39:  checked(service+' uses non-root, read-only filesystem, dropped capabilities and resource bounds','"65532:65532" true ["ALL"] ["no-new-privileges:true"] 536870912 128'==raw)
40:  revision=subprocess.check_output(['docker','inspect','--format','{{index .Config.Labels "org.opencontainers.image.revision"}}',cid],text=True).strip()
41:  checked(service+' actual image revision matches validation source SHA',revision==SOURCE_HEAD)
42:  container_info[service]={'revision':revision,'image_id':subprocess.check_output(['docker','inspect','--format','{{.Image}}',cid],text=True).strip(),'hardening':raw}
43: checked('Analytics app database role is not privileged',sql('analytics-db',"SELECT rolsuper,rolcreatedb,rolcreaterole FROM pg_roles WHERE rolname='analytics_user'",'analytics_db')=='f|f|f')
44: checked('Chat cannot reuse its role in Analytics database',sql('analytics-db',"SELECT count(*) FROM pg_roles WHERE rolname='chat_user'",'analytics_db')=='0')
45: with client.open(chat+'/pkg/chat_ui_bg.wasm')as response:checked('real WASM hydration asset is shipped',response.read(4)==b'\0asm')
46: checked('unauthenticated writes rejected',http(chat,'/api/v1/messages',body={'room_id':'forbidden','content':'fixture'})[0]==401)
47: room='docker-'+uuid.uuid4().hex[:20];assert http(chat,'/api/v1/rooms',ct,{'id':room,'name':'Docker release fixture'})[0]==201
48: mid=str(uuid.uuid4());body={'id':mid,'room_id':room,'content':'Docker release integration fixture'}
49: assert http(chat,'/api/v1/messages',ct,body)[0]==201
50: wait(lambda:http(analytics,'/api/v1/events/web_chat/'+mid,at)[0]==200)
51: checked('this container Chat message reaches persistent Analytics')
52: checked('same UUID retry stays idempotent',http(chat,'/api/v1/messages',ct,body)[0]==200)
53: command(['up','-d','--force-recreate','--no-deps','--wait','chat','analytics']);wait(lambda:http(chat,'/healthz')[0]==200 and http(analytics,'/healthz')[0]==200)
54: checked('messages survive container recreation',any(row['id']==mid for row in http(chat,'/api/v1/messages?room_id='+room,ct)[1]))
55: command(['stop','analytics']);delayed=str(uuid.uuid4());assert http(chat,'/api/v1/messages',ct,{'id':delayed,'room_id':room,'content':'durable container outage fixture'})[0]==201
56: command(['start','analytics']);wait(lambda:http(analytics,'/api/v1/events/web_chat/'+delayed,at)[0]==200)
57: checked('durable retry recovers from an actual Analytics container outage')
58: command(['stop','analytics-db']);wait(lambda:http(analytics,'/healthz')[0]!=200)
59: checked('SQLite Chat stays ready while Analytics PostgreSQL is stopped',http(chat,'/healthz')[0]==200)
60: queued=str(uuid.uuid4());assert http(chat,'/api/v1/messages',ct,{'id':queued,'room_id':room,'content':'SQLite while PostgreSQL offline'})[0]==201
61: command(['start','analytics-db']);wait(lambda:http(analytics,'/healthz')[0]==200)
62: wait(lambda:http(analytics,'/api/v1/events/web_chat/'+queued,at)[0]==200)
63: checked('SQLite outbox delivers after actual PostgreSQL recovery')
64: portable='/data/docker-portable-'+uuid.uuid4().hex+'.sqlite3'
65: command(['exec','-T','chat','/app/server','--backup',portable])
66: checked('release Chat creates integrity-checked portable SQLite snapshot')
67: command(['exec','-T','chat','rm',portable])
68: backup=command(['exec','-T','analytics-db','pg_dump','-U','bootstrap_owner','-d','analytics_db','-Fc'])
69: fixture='p9_restore_'+uuid.uuid4().hex[:12]
70: command(['exec','-T','analytics-db','createdb','-U','bootstrap_owner',fixture])
71: try:
72:  subprocess.run(compose+['exec','-T','analytics-db','pg_restore','-U','bootstrap_owner','--no-owner','--exit-on-error','-d',fixture],input=backup,check=True,cwd=ROOT)
73:  query='SELECT source_app,event_id,event_type FROM product_events ORDER BY source_app,event_id'
74:  checked('container database backup restores exact event rows',sql('analytics-db',query,'analytics_db')==sql('analytics-db',query,fixture))
75: finally:command(['exec','-T','analytics-db','dropdb','-U','bootstrap_owner',fixture])
76: checked('validation source remains unchanged',subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()==SOURCE_HEAD)
77: report={'status':'passed','checks':checks,'containers':container_info,'backup_sha256':hashlib.sha256(backup).hexdigest(),'scope':'actual local release Docker integration; complete P9 gates remain separate','source_head':SOURCE_HEAD,'working_tree_dirty':SOURCE_DIRTY or bool(subprocess.check_output(['git','status','--porcelain'],cwd=ROOT,text=True).strip())}
78: (ROOT/'target/docker-local-validation.json').write_text(json.dumps(report,indent=2)+'\n')
```

## copilot_use/scripts/gui/pet-self-test.js

SHA256 `1b0776cc49b72eb648ebd232caf1c40aa8dcd50a066564047dcdd86d91549874`。

```
1: window.addEventListener('error',e=>window.__PET_LOAD_ERROR=e.message);window.addEventListener('unhandledrejection',e=>window.__PET_LOAD_ERROR=String(e.reason));
2: // 只在明確啟用且通過全新 fixture 檢查的 GUI 驗收中執行。
3: (async()=>{
4:  const assert=(value,message)=>{if(!value)throw Error(message)};
5:  const limit=Date.now()+15000;
6:  while(!document.querySelector('.pet')){if(Date.now()>limit)throw Error('Leptos mount timeout: '+document.body.innerHTML.slice(0,800)+' '+window.__PET_LOAD_ERROR);await new Promise(r=>setTimeout(r,50));}
7:  const box=document.querySelector('.pet').getBoundingClientRect();assert(box.width===200&&box.height===200,'200x200 display');
8:  const invoke=window.__TAURI__.core.invoke;
9:  const pet='8df47580-64e1-4335-8e50-6b593051a398';
10:  const other='a4105c9b-67f8-4c60-8376-181c706de117';
11:  const memory='b4197c3b-3f50-4dc1-b12f-fd1a7143ec85';
12:  const profile={'名字':'GUI 驗證','種類':'鳥','喜好':['音樂'],'個性':['活潑'],'興趣':['Rust']};
13:  await invoke('save_profile',{id:pet,profile});await invoke('save_profile',{id:other,profile});
14:  const pets=await invoke('list_pets');assert(pets.some(x=>x.id===pet),'profile persistence');
15:  await invoke('save_memory',{pet,id:memory,content:'安全隔離的測試記憶'});
16:  assert((await invoke('list_memories',{pet})).some(x=>x.id===memory),'memory saved');
17:  assert(!(await invoke('list_memories',{pet:other})).some(x=>x.id===memory),'memory isolation');
18:  await invoke('delete_memory',{pet:other,id:memory});assert((await invoke('list_memories',{pet})).some(x=>x.id===memory),'cross-pet deletion denied');
19:  await invoke('delete_memory',{pet,id:memory});assert(!(await invoke('list_memories',{pet})).some(x=>x.id===memory),'memory deletion');
20:  await invoke('add_card',{pet,question:'Rust 的所有權如何避免 double free？',answer:'單一所有者與借用檢查'});
21:  const cards=await invoke('list_cards',{pet});assert(cards.length>0,'review card saved');
22:  const reviewed=await invoke('review_card',{pet,id:cards[0].id,quality:5});assert(reviewed.interval_days>0,'spaced review scheduling');
23:  const knowledge=crypto.randomUUID();await invoke('save_memory',{pet,id:knowledge,content:'Agent 驗收代碼為 ASTRO-771。probe memory owned by this pet.'});
24:  await invoke('save_memory',{pet:other,id:crypto.randomUUID(),content:'probe other pet hidden code STALE-999'});
25:  const retrieved=await invoke('search_memory_tool',{pet,query:'probe'});assert(retrieved.hits.some(x=>x.id===knowledge)&&!retrieved.hits.some(x=>x.content.includes('STALE-999')),'actual readonly MCP stdio scope');
26:
27:  if(window.__PET_TEST_SUGGESTION){
28:   const id=window.__PET_TEST_SUGGESTION;
29:   let refused=false;try{await invoke('apply_analytics_suggestion',{pet,id,confirmed:false})}catch(_){refused=true;}assert(refused,'policy explicit confirmation');
30:   const applied=await invoke('apply_analytics_suggestion',{pet,id,confirmed:true});
31:   assert(applied.value===3&&applied.analytics_acknowledged,'native policy apply acknowledgement');
32:   const replay=await invoke('apply_analytics_suggestion',{pet,id,confirmed:true});assert(replay.receipt_id===applied.receipt_id,'native policy idempotence');
33:   const rolled=await invoke('rollback_analytics_suggestion',{pet,id,confirmed:true});assert(rolled.value===1&&rolled.analytics_acknowledged,'native policy rollback acknowledgement');
34:  }
35:  await invoke('gui_test_progress',{phase:'core-complete'});
36:  if(window.__PET_TEST_LIVE_AI){
37:   await invoke('gui_test_progress',{phase:'live-ai'});
38:   let deltas=0;const request=crypto.randomUUID();const unlisten=await window.__TAURI__.event.listen('pet-reply-delta',e=>{if(e.payload.request===request)deltas++});
39:   const output=await invoke('pet_talk',{pet,request,text:'請用一句繁體中文說明 Rust 的所有權，再問一個理解問題。'});
40:   unlisten();assert(output.length>0&&deltas>0,'real AI streaming');
41:   const agent=await invoke('pet_agent',{pet,text:'請使用記憶檢索工具回答 Agent 驗收代碼，引用資料並提出一個理解問題。'});
42:   assert(agent.answer.includes('ASTRO-771')&&agent.sources.includes(knowledge)&&agent.steps.length>0&&agent.follow_up_question,'real Pet MCP Agent structured answer');
43:
44:  }
45:  if(window.__PET_TEST_LIVE_IMAGE){
46:   await invoke('gui_test_progress',{phase:'live-image'});
47:   const asset=await invoke('generate_asset',{pet});assert(asset.png_base64.length>0&&asset.width>0&&asset.height>0,'real OpenAI sprite');
48:   const cached=await invoke('load_asset',{pet});assert(cached.png_base64===asset.png_base64,'sprite cache persistence');
49:   await invoke('save_profile',{id:pet,profile:{...profile,'名字':'已修改角色'}});
50:   assert((await invoke('load_asset',{pet}))===null,'changed profile cannot render stale generated sprite');
51:   await invoke('save_profile',{id:pet,profile});
52:   assert((await invoke('load_asset',{pet})).png_base64===asset.png_base64,'matching original profile restores cached sprite');
53:  }
54:  await invoke('gui_test_progress',{phase:'cleanup'});
55:  const files=await invoke('scan_file_preview');const sample=files.find(f=>f.name==='sample.txt');assert(sample&&sample.size_bytes>0,'file preview');
56:  const rootPath=await invoke('authorized_directory');assert(rootPath,'authorized directory');
57:  await invoke('revoke_directory');
58:  let revoked=false;try{await invoke('confirm_cleanup',{id:sample.id,confirmed:true})}catch(_){revoked=true;}assert(revoked,'revocation invalidates old previews');
59:  let unconfirmed=false;try{await invoke('authorize_directory',{path:rootPath,confirmed:false})}catch(_){unconfirmed=true;}assert(unconfirmed,'directory authorization requires confirmation');
60:  await invoke('authorize_directory',{path:rootPath,confirmed:true});
61:  const fresh=await invoke('scan_file_preview');sample.id=fresh.find(x=>x.name==='sample.txt').id;
62:  let refused=false;try{await invoke('confirm_cleanup',{id:sample.id,confirmed:false})}catch(_){refused=true;}assert(refused,'explicit confirmation');
63:  const receipt=await invoke('confirm_cleanup',{id:sample.id,confirmed:true});assert(receipt.operation_id&&receipt.size_bytes>0,'real cleanup receipt');
64:  const waitUntil=async(test,label)=>{const until=Date.now()+10000;while(!test()){if(Date.now()>until)throw Error(label+'; UI: '+(document.querySelector('[role="alert"]')?.textContent||''));await new Promise(r=>setTimeout(r,25));}};
65:  document.querySelector('#scan-files').click();
66:  const section=document.querySelector('#scan-files').closest('section');
67:  await waitUntil(()=>[...section.querySelectorAll('article')].some(x=>x.textContent.includes('second.txt')),'organizer UI preview');
68:  const row=[...section.querySelectorAll('article')].find(x=>x.textContent.includes('second.txt'));
69:  const button=row.querySelector('button');assert(button.disabled,'organizer button requires confirmation');
70:  const checkbox=row.querySelector('input');checkbox.checked=true;checkbox.dispatchEvent(new Event('change',{bubbles:true}));
71:  await waitUntil(()=>!button.disabled,'organizer confirmation checkbox');button.click();
72:  await waitUntil(()=>section.textContent.includes('已隔離；收據'),'organizer UI success receipt');
73:  const recovered=await invoke('recover_cleanup_receipts');assert(recovered.length===2&&recovered.some(x=>x.operation_id===receipt.operation_id),'durable journal recovery');
74:  assert((await invoke('recover_cleanup_receipts')).length===2,'repeat recovery idempotence');
75:  let restoreDenied=false;try{await invoke('restore_file',{id:receipt.operation_id,confirmed:false})}catch(_){restoreDenied=true;}assert(restoreDenied,'restore explicit confirmation');
76:  document.querySelector('#load-trash').click();const trash=document.querySelector('#trash-browser');
77:  await waitUntil(()=>[...trash.querySelectorAll('article')].some(x=>x.textContent.includes('sample.txt')),'trash UI listing');
78:  const trashRow=[...trash.querySelectorAll('article')].find(x=>x.textContent.includes('sample.txt'));const restoreButton=trashRow.querySelector('button');assert(restoreButton.disabled,'restore UI confirmation');
79:  const restoreBox=trashRow.querySelector('input');restoreBox.checked=true;restoreBox.dispatchEvent(new Event('change',{bubbles:true}));await waitUntil(()=>!restoreButton.disabled,'restore checkbox');restoreButton.click();
80:  await waitUntil(()=>trash.textContent.includes('已還原；收據'),'restore UI receipt');
81:  const restores=await invoke('recover_restore_receipts');assert(restores.length===1,'restore recovery');
82:  const retry=await invoke('restore_file',{id:receipt.operation_id,confirmed:true});assert(retry.operation_id===restores[0].operation_id,'restore stable retry receipt');
83:  assert((await invoke('recover_restore_receipts')).length===1,'restore recovery idempotence');
84:  assert((await invoke('scan_file_preview')).some(x=>x.name==='sample.txt'),'restored file available');
85:
86:  const select=document.querySelector('select');
87:  // 實際觸發 UI 的選擇與資料載入事件。
88:  const option=document.createElement('option');option.value=pet;select.appendChild(option);select.value=pet;select.dispatchEvent(new Event('change',{bubbles:true}));
89:  await waitUntil(()=>document.body.textContent.includes('Rust 的所有權'),'Leptos renders native data');
90:  if(window.__PET_TEST_LIVE_IMAGE){await waitUntil(()=>document.querySelector('.sprite'),'generated sprite rendered');const current=await invoke('load_asset',{pet});const frame=document.querySelector('.sprite').getBoundingClientRect();assert(frame.width<=200&&frame.height<=200&&Math.abs(frame.width/frame.height-current.width/current.height)<0.01,'actual sprite frame preserves original aspect inside 200x200');}
91:
92:
93:  for(const [field,value]of [['名字','GUI 驗證'],['種類','鳥']]){
94:   const input=[...document.querySelectorAll('label')].find(x=>x.textContent===field).querySelector('input');input.value=value;input.dispatchEvent(new Event('input',{bubbles:true}));
95:  }
96:  const color=[...document.querySelectorAll('label')].find(x=>x.textContent==='顏色').querySelector('input');
97:  color.value='翠綠色';color.dispatchEvent(new Event('input',{bubbles:true}));
98:  [...document.querySelectorAll('button')].find(x=>x.textContent==='保存設定').click();
99:  await waitUntil(()=>document.body.textContent.includes('我的寵物'),'profile UI');
100:  let savedProfile;const profileDeadline=Date.now()+10000;
101:  do{savedProfile=JSON.parse((await invoke('list_pets')).find(x=>x.id===pet).profile);if(savedProfile['外觀']?.['顏色']==='翠綠色')break;await new Promise(r=>setTimeout(r,25));}while(Date.now()<profileDeadline);
102:  assert(savedProfile['外觀']['顏色']==='翠綠色','advanced override UI persisted');
103:  let deniedImport=false;try{await invoke('import_pet_profile',{pet,confirmed:false})}catch(_){deniedImport=true}assert(deniedImport,'profile import requires confirmation');await invoke('import_pet_profile',{pet,confirmed:true});
104:  const importButton=document.querySelector('#import-pet-profile');assert(importButton.disabled,'profile import UI confirmation');
105:  const importConfirm=document.querySelector('#confirm-profile-import');importConfirm.checked=true;importConfirm.dispatchEvent(new Event('change',{bubbles:true}));await waitUntil(()=>!importButton.disabled,'profile import checkbox');importButton.click();
106:  await waitUntil(()=>document.querySelector('#import-pet-profile').disabled,'actual profile import completed');assert(document.querySelector('#profile-location').textContent.includes(pet),'selected pet configuration path rendered');
107:
108:  const memorySection=[...document.querySelectorAll('section')].find(x=>x.querySelector('h2')?.textContent==='可管理的記憶');
109:  await waitUntil(()=>[...memorySection.querySelectorAll('article')].some(x=>x.textContent.includes('ASTRO-771')),'memory UI load after profile save');
110:  const memoryRow=[...memorySection.querySelectorAll('article')].find(x=>x.textContent.includes('ASTRO-771'));
111:  memoryRow.querySelector('button').click();const editor=memorySection.querySelector('textarea');editor.value='edited ASTRO-771 test memory';editor.dispatchEvent(new Event('input',{bubbles:true}));
112:  [...memorySection.querySelectorAll('button')].find(x=>x.textContent==='保存記憶').click();
113:  let edited;const editDeadline=Date.now()+10000;do{edited=(await invoke('list_memories',{pet})).find(x=>x.id===knowledge);if(edited?.content==='edited ASTRO-771 test memory')break;await new Promise(r=>setTimeout(r,25));}while(Date.now()<editDeadline);
114:  assert(edited.content==='edited ASTRO-771 test memory','memory edit UI preserves scoped ID');
115:
116:  const actionControl=document.querySelector('#pet-action');actionControl.value='learning';actionControl.dispatchEvent(new Event('change',{bubbles:true}));
117:  const speedControl=document.querySelector('#pet-speed');speedControl.value='fast';speedControl.dispatchEvent(new Event('change',{bubbles:true}));
118:  document.querySelector('#save-interaction').click();
119:  const interactionDeadline=Date.now()+10000;let preferences;
120:  do{preferences=await invoke('interaction',{pet});if(preferences.action==='learning'&&preferences.speed==='fast')break;await new Promise(r=>setTimeout(r,25));}while(Date.now()<interactionDeadline);
121:  assert(preferences.action==='learning'&&preferences.speed==='fast'&&!preferences.proactive,'animation action/speed UI persisted with proactive off');
122:  assert((await invoke('learning_summary',{pet})).total>0,'actual learning summary');
123:  assert(await invoke('recall_prompt')===null,'proactivity requires opt in');
124:  const backups=document.querySelector('#pet-backups');document.querySelector('#create-snapshot').click();
125:  await waitUntil(()=>backups.textContent.includes('備份完成'),'backup UI completion');const snapshots=await invoke('list_snapshots');assert(snapshots.length===1&&snapshots[0].sha256.length===64,'snapshot manifest');
126:  await invoke('save_memory',{pet,id:knowledge,content:'changed after snapshot'});
127:  let deniedBackup=false;try{await invoke('restore_snapshot',{id:snapshots[0].id,confirmed:false})}catch(_){deniedBackup=true;}assert(deniedBackup,'snapshot restore explicit confirmation');
128:  const backupSelect=backups.querySelector('select');backupSelect.value=snapshots[0].id;backupSelect.dispatchEvent(new Event('change',{bubbles:true}));
129:  const restoreSnapshotButton=backups.querySelector('#restore-snapshot');assert(restoreSnapshotButton.disabled,'snapshot UI confirmation');
130:  const backupConfirm=backups.querySelector('input');backupConfirm.checked=true;backupConfirm.dispatchEvent(new Event('change',{bubbles:true}));await waitUntil(()=>!restoreSnapshotButton.disabled,'snapshot confirm checkbox');restoreSnapshotButton.click();
131:  await waitUntil(()=>backups.textContent.includes('還原完成'),'snapshot restore UI completion');assert((await invoke('list_memories',{pet})).find(x=>x.id===knowledge).content==='edited ASTRO-771 test memory','SQLite memory restored');assert((await invoke('list_snapshots')).length===2,'pre-restore safety snapshot retained');
132:  if(window.__PET_TEST_ANALYTICS){
133:   await invoke('gui_test_progress',{phase:'analytics-outbox'});
134:   const deadline=Date.now()+30000;let outbox;
135:   do {outbox=await invoke('outbox_status');if(outbox.delivered>=4&&!outbox.pending&&!outbox.leased&&!outbox.blocked)break;await new Promise(r=>setTimeout(r,100));}while(Date.now()<deadline);
136:   assert(outbox.delivered>=4&&!outbox.pending&&!outbox.leased&&!outbox.blocked,'actual Pet to Analytics delivery');
137:  }
138:  // Wait for the actual UI timer, shared between the settings and floating windows.
139:  await invoke('add_card',{pet,question:'主動回想測試題：Rust 所有權',answer:'只有一個擁有者'});
140:  const proactiveControl=document.querySelector('#pet-proactive');proactiveControl.checked=true;proactiveControl.dispatchEvent(new Event('change',{bubbles:true}));document.querySelector('#save-interaction').click();
141:  const proactiveDeadline=Date.now()+70000;
142:  while(!document.querySelector('.recall-prompt')?.textContent.includes('主動回想測試題')){if(Date.now()>proactiveDeadline)throw Error('actual proactive recall timer did not display');await new Promise(r=>setTimeout(r,100));}
143:  assert(await invoke('recall_prompt')===null,'displayed reminder is rate limited across windows');
144:  document.querySelector('.recall-prompt button').click();
145:
146:  const registry=document.querySelector('#pet-archive');const archivePet=document.querySelector('#archive-pet');assert(archivePet.disabled,'pet removal UI confirmation');
147:  let removalDenied=false;try{await invoke('set_pet_archived',{pet,archived:true,confirmed:false})}catch(_){removalDenied=true}assert(removalDenied,'pet removal IPC confirmation');
148:  const removeConfirm=document.querySelector('#confirm-pet-archive');removeConfirm.checked=true;removeConfirm.dispatchEvent(new Event('change',{bubbles:true}));await waitUntil(()=>!archivePet.disabled,'pet removal checkbox');archivePet.click();
149:  await waitUntil(()=>registry.querySelector(`article[data-pet="${pet}"]`),'removed pet appears in recoverable archive');
150:  assert(!(await invoke('list_pets')).some(x=>x.id===pet)&&(await invoke('list_pets')).some(x=>x.id===other),'actual pet removal preserves other pet');
151:  const archivedRow=registry.querySelector(`article[data-pet="${pet}"]`);const restorePet=archivedRow.querySelector('button');assert(restorePet.disabled,'pet restore UI confirmation');const restorePetConfirm=archivedRow.querySelector('input');restorePetConfirm.checked=true;restorePetConfirm.dispatchEvent(new Event('change',{bubbles:true}));await waitUntil(()=>!restorePet.disabled,'pet restore checkbox');restorePet.click();
152:  await waitUntil(()=>registry.textContent.includes('已還原寵物與原有資料'),'actual pet restore receipt');assert((await invoke('list_memories',{pet})).some(x=>x.id===knowledge),'restored pet retains saved memory');
153:  assert((await invoke('list_pets')).length===2,'restored pet registry');
154:  if(window.__PET_TEST_UPDATER){
155:   const available=await invoke('check_local_update');assert(available.status==='available'&&available.version==='0.1.1','local signed update discovered');
156:   let unconfirmedInstall=false;try{await invoke('install_local_update',{id:available.id,confirmed:false})}catch(_){unconfirmedInstall=true}assert(unconfirmedInstall,'updater installation requires confirmation');
157:   const verified=await invoke('verify_local_update',{id:available.id});assert(verified.status==='signature_verified'&&verified.bytes>0,'actual updater signature download verified');
158:   if(window.__PET_TEST_INSTALL_UPDATE){
159:    const installed=await invoke('install_local_update',{id:available.id,confirmed:true});
160:    assert(installed.status==='installed_restart_required'&&installed.version==='0.1.1'&&installed.before_snapshot,'actual signed package installed after SQLite snapshot');
161:   }else{
162:   const wrongVersion=await invoke('check_local_update');let versionDenied=false;try{await invoke('verify_local_update',{id:wrongVersion.id})}catch(_){versionDenied=true}assert(versionDenied,'signed version mismatch rejected');
163:   const tampered=await invoke('check_local_update');let tamperDenied=false;try{await invoke('verify_local_update',{id:tampered.id})}catch(_){tamperDenied=true}assert(tamperDenied,'modified updater package rejected');
164:   let remoteDenied=false;try{await invoke('check_local_update')}catch(_){remoteDenied=true}assert(remoteDenied,'updater remote URL rejected before download');
165:   }
166:  }
167:  await invoke('gui_test_result',{success:true,message:'PASS: WebKit/WASM, 200x200, IPC, profiles, memory isolation/deletion, recall scheduling, advanced overrides, memory editing, opaque-ID confirmed cleanup, organizer GUI, journal/restore recovery, Trash UI, animation choices, rendered proactive reminder and cooldown, learning summary, SQLite/assets snapshot restore, per-pet TOML import and recoverable registry, MCP stdio scope and rendered data; AI='+!!window.__PET_TEST_LIVE_AI+'; image='+!!window.__PET_TEST_LIVE_IMAGE+'; analytics='+!!window.__PET_TEST_ANALYTICS+'; updater_signature='+!!window.__PET_TEST_UPDATER});
168: })().catch(async e=>{await window.__TAURI__.core.invoke('gui_test_result',{success:false,message:String(e?.message||e)})});
```

## copilot_use/scripts/import-chat-postgres.py

SHA256 `dd2d55e8cfa44c66fbceca70233aabcf2a7e5144781c2bd58e4092527c8ccf11`。

```
1: #!/usr/bin/env python3
2: """Read-only legacy Chat export into a new portable SQLite; never overwrite either database."""
3: import argparse,datetime,json,os,pathlib,sqlite3,subprocess,tempfile,uuid,urllib.parse
4: ROOT=pathlib.Path(__file__).resolve().parents[1]
5: a=argparse.ArgumentParser();a.add_argument('--source-env',default='CHAT_LEGACY_DATABASE_URL');a.add_argument('--output',type=pathlib.Path,required=True);a.add_argument('--binary',type=pathlib.Path,default=ROOT/'target/debug/day9-chat');args=a.parse_args()
6: source=os.environ.get(args.source_env)
7: if not source:raise SystemExit('Set the selected source environment variable; do not pass credentials on the command line')
8: output=args.output.resolve();output.parent.mkdir(parents=True,exist_ok=True)
9: if output.exists():raise SystemExit('Destination already exists; import never overwrites data')
10: # psql does not print the credential-bearing connection string or raw rows to the terminal.
11: url=urllib.parse.urlsplit(source)
12: if url.scheme not in ('postgres','postgresql') or not url.hostname or not url.path.strip('/'):
13:     raise SystemExit('Source must be a PostgreSQL URL with an explicit database')
14: env=os.environ|{'PGHOST':url.hostname,'PGPORT':str(url.port or 5432),'PGDATABASE':urllib.parse.unquote(url.path.lstrip('/')),'PGOPTIONS':'-c default_transaction_read_only=on -c statement_timeout=300000'}
15: if url.username is not None:env['PGUSER']=urllib.parse.unquote(url.username)
16: if url.password is not None:env['PGPASSWORD']=urllib.parse.unquote(url.password)
17: allowed={'sslmode':'PGSSLMODE','sslrootcert':'PGSSLROOTCERT','sslcert':'PGSSLCERT','sslkey':'PGSSLKEY','connect_timeout':'PGCONNECT_TIMEOUT'}
18: for key,value in urllib.parse.parse_qsl(url.query):
19:     if key not in allowed:raise SystemExit('Unsupported PostgreSQL URL option')
20:     env[allowed[key]]=value
21: psql=['psql','--no-psqlrc','-X','-A','-t','-v','ON_ERROR_STOP=1']
22: process=None;connection=None
23: with tempfile.TemporaryDirectory(prefix='.chat-import-',dir=output.parent)as scratch:
24:     staged=pathlib.Path(scratch)/'chat.sqlite3'
25:     subprocess.run([args.binary,'--initialize'],cwd=ROOT,env=os.environ|{'CHAT_DATABASE_URL':'sqlite://'+str(staged)},check=True,stdout=subprocess.DEVNULL)
26:     connection=sqlite3.connect(staged)
27:     connection.execute('PRAGMA foreign_keys=OFF')
28:     connection.execute('BEGIN IMMEDIATE')
29:     target={row[0] for row in connection.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name<>'_sqlx_migrations'")}
30:     metadata=subprocess.run(psql+['-c',"SELECT json_agg(tablename ORDER BY tablename) FROM pg_tables WHERE schemaname='public' AND tablename<>'_sqlx_migrations'"],env=env,text=True,capture_output=True)
31:     if metadata.returncode:raise SystemExit('Read-only PostgreSQL schema inspection failed')
32:     tables=json.loads(metadata.stdout)or[]
33:     if set(tables)-target:raise SystemExit('Legacy schema contains unsupported tables; refusing incomplete import')
34:     statements=['BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY;']
35:     for table in tables:
36:         # Identifiers were checked against target schema, never accepted from arbitrary user SQL.
37:         statements.append(f'''SELECT json_build_object('table','{table}','row',row_to_json(t)) FROM "{table}" t;''')
38:     statements.append('COMMIT;')
39:     counts={};columns={t:{r[1]:r[2].upper() for r in connection.execute(f'PRAGMA table_info("{t}")')}for t in tables}
40:     with open(pathlib.Path(scratch)/'source-error.log','w')as error:
41:         process=subprocess.Popen(psql,env=env,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=error,text=True)
42:         process.stdin.write('\n'.join(statements));process.stdin.close()
43:         try:
44:             for line in process.stdout:
45:                 line=line.strip()
46:                 if not line or line in ('BEGIN','COMMIT'):continue
47:                 if len(line)>1024*1024:raise ValueError('Legacy row exceeds import safety limit')
48:                 item=json.loads(line);table=item['table'];row=item['row'];values=[]
49:                 if set(row)-columns[table].keys():raise ValueError('Unsupported legacy columns; refusing data loss')
50:                 for key,value in row.items():
51:                     kind=columns[table][key]
52:                     if value is not None:
53:                         if kind=='BLOB':value=bytes.fromhex(value[2:])if value.startswith('\\x')else uuid.UUID(value).bytes
54:                         elif kind=='REAL':value=datetime.datetime.fromisoformat(value.replace('Z','+00:00')).timestamp()if isinstance(value,str)else value
55:                         elif isinstance(value,(dict,list)):value=json.dumps(value,ensure_ascii=False,sort_keys=True,separators=(',',':'))
56:                     values.append(value)
57:                 names=','.join('"'+name+'"'for name in row)
58:                 placeholders=','.join('?'for _ in row)
59:                 connection.execute(f'INSERT INTO "{table}"({names})VALUES({placeholders})',values)
60:                 counts[table]=counts.get(table,0)+1
61:             if process.wait()!=0:raise ValueError('Read-only source snapshot failed')
62:             if connection.execute('PRAGMA foreign_key_check').fetchall():raise ValueError('Imported foreign keys are inconsistent')
63:             if connection.execute('PRAGMA integrity_check').fetchone()[0]!='ok':raise ValueError('Imported SQLite integrity failed')
64:             connection.commit();connection.execute('PRAGMA wal_checkpoint(TRUNCATE)');connection.close();connection=None
65:             # Atomic no-overwrite publication, even if another process creates output during import.
66:             os.link(staged,output)
67:             staged.unlink()
68:             print(json.dumps({'status':'passed','tables':counts,'source_modified':False,'database':'SQLite'},sort_keys=True))
69:         finally:
70:             if process.poll()is None:process.terminate();process.wait()
71:             if connection is not None:connection.rollback();connection.close()
```

## copilot_use/scripts/local-verify.py

SHA256 `584854df19b85d79bb2db61b806bf6c72aacf364c77e6f8ea5c5813b04d90698`。

```
1: #!/usr/bin/env python3
2: """建立全新隔離 SQLite Chat/Pet 與 PostgreSQL Analytics，以真實 binary/API 驗證；結束時移除測試庫。"""
3: import sqlite3
4: import json, uuid, os, pathlib, secrets, signal, socket, subprocess, sys, tempfile, time, urllib.request, urllib.error, threading
5: from http.server import ThreadingHTTPServer,BaseHTTPRequestHandler
6: ROOT = pathlib.Path(__file__).resolve().parents[1]
7: SOURCE_SHA=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
8: SOURCE_DIRTY=bool(subprocess.check_output(['git','status','--porcelain'],cwd=ROOT,text=True).strip())
9: PG = pathlib.Path(subprocess.check_output(['pg_config','--bindir'],text=True).strip())
10: REPORT = pathlib.Path(os.environ.get('VERIFY_REPORT', ROOT / 'target/local-validation.json'))
11: live_key=os.environ.get('OPENAI_API_KEY') if any(flag in sys.argv for flag in ('--live-openai','--live-insights','--browser-live')) else None
12: if any(flag in sys.argv for flag in ('--live-openai','--live-insights','--browser-live')) and not live_key: raise SystemExit('OPENAI_API_KEY not configured')
13: def interrupted(*_): raise KeyboardInterrupt
14: signal.signal(signal.SIGTERM,interrupted)
15: results=[]
16: processes=[]
17:
18: def run(argv, **kw):
19:     subprocess.run([str(a) for a in argv], check=True, cwd=ROOT, **kw)
20:
21: def port():
22:     with socket.socket() as s:
23:         s.bind(('127.0.0.1',0)); return s.getsockname()[1]
24:
25: def check(label, ok=True):
26:     assert ok, label
27:     results.append(label)
28:     print('PASS:',label,flush=True)
29:
30: def http(base,path,token=None,body=None,method=None):
31:     req=urllib.request.Request(base+path, data=json.dumps(body).encode() if body is not None else None,
32:         headers={'Content-Type':'application/json', **({'Authorization':'Bearer '+token} if token else {})},method=method)
33:     try:
34:         with urllib.request.urlopen(req,timeout=120) as r:
35:             data=r.read(); return r.status, json.loads(data) if data else None
36:     except urllib.error.HTTPError as e:
37:         return e.code,None
38:
39: def wait(fn):
40:     limit=time.monotonic()+20
41:     while time.monotonic()<limit:
42:         try:
43:             if fn(): return
44:         except (OSError,urllib.error.URLError): pass
45:         time.sleep(.2)
46:     raise AssertionError('condition timed out')
47:
48: def stop(proc):
49:     if proc.poll() is None:
50:         proc.terminate()
51:         try: proc.wait(10)
52:         except subprocess.TimeoutExpired: proc.kill();proc.wait()
53:
54: with tempfile.TemporaryDirectory(prefix='copilot-verify-') as scratch:
55:     d=pathlib.Path(scratch); dbport=port(); chatport=port(); analyticsport=port()
56:     admin=f'postgresql://{os.environ.get("USER","willie")}@127.0.0.1:{dbport}/postgres'
57:     common=os.environ.copy(); common['DATABASE_URL']=admin
58:     common.pop('OPENAI_API_KEY',None)  # 基礎可靠性測試不觸發外部付費呼叫。
59:     chat_token=secrets.token_urlsafe(32); analytics_token=secrets.token_urlsafe(32)
60:     ingest_token=secrets.token_urlsafe(32); pet_token=secrets.token_urlsafe(32)
61:     chat_url=f'http://127.0.0.1:{chatport}'; analytics_url=f'http://127.0.0.1:{analyticsport}'
62:     env=common | {'CHAT_API_TOKEN':chat_token,'ANALYTICS_API_TOKEN':analytics_token,
63:         'ANALYTICS_ADMIN_TOKEN':secrets.token_urlsafe(32),'ANALYTICS_CHAT_TOKEN':ingest_token,'ANALYTICS_PET_TOKEN':pet_token,
64:         'CHAT_URL':chat_url,'ANALYTICS_URL':analytics_url,
65:         'CHAT_LISTEN':f'127.0.0.1:{chatport}','ANALYTICS_LISTEN':f'127.0.0.1:{analyticsport}',
66:         'ANALYTICS_ENDPOINT':analytics_url+'/api/v1/events'}
67:     observed=[];observed_headers=[]
68:     class Collector(BaseHTTPRequestHandler):
69:         def log_message(self,*_):pass
70:         def do_POST(self):
71:             length=int(self.headers.get('Content-Length','0'))
72:             if length>2000000:self.send_error(413);return
73:             payload=self.rfile.read(length)
74:             observed.append((self.path,payload))
75:             observed_headers.append(dict(self.headers))
76:             self.send_response(200);self.send_header('Content-Type','application/json');self.end_headers();self.wfile.write(b'{}')
77:     collector=ThreadingHTTPServer(('127.0.0.1',0),Collector)
78:     collector_thread=threading.Thread(target=collector.serve_forever,daemon=True);collector_thread.start()
79:     env['COPILOT_LOCAL_OTLP_ENDPOINT']=f'http://127.0.0.1:{collector.server_port}'
80:     env['OTEL_EXPORTER_OTLP_HEADERS']='Authorization=metadata-synthetic-secret,x-private=metadata-synthetic-secret'
81:     for signal_name in ('TRACES','METRICS','LOGS'):
82:         env['OTEL_EXPORTER_OTLP_'+signal_name+'_HEADERS']='Authorization=metadata-synthetic-secret'
83:     env['OTEL_BSP_SCHEDULE_DELAY']='100';env['OTEL_BLRP_SCHEDULE_DELAY']='100';env['OTEL_METRIC_EXPORT_INTERVAL']='250'
84:     started=False
85:     try:
86:         run([PG/'initdb','-D',d/'data','-A','trust','--no-locale','-E','UTF8'],stdout=subprocess.DEVNULL)
87:         run([PG/'pg_ctl','-D',d/'data','-l',d/'pg.log','-o',f'-h 127.0.0.1 -p {dbport} -k {scratch}','start'],stdout=subprocess.DEVNULL)
88:         started=True
89:         run(['cargo','fmt','--all','--','--check'])
90:         run(['cargo','clippy','--workspace','--all-targets','--locked','--','-D','warnings'])
91:         run(['cargo','test','--workspace','--all-targets','--locked'],env=common)
92:         run(['cargo','build','--workspace','--locked'])
93:         check('fmt, strict clippy, SQLite Chat/Pet and PostgreSQL Analytics tests, binaries')
94:         # 應用使用各自 non-superuser role；測試 admin 只用於新建測試庫及備份。
95:         for app in ('analytics',):
96:             password=secrets.token_hex(24)
97:             sql=f"CREATE ROLE {app}_test LOGIN PASSWORD '{password}'; CREATE DATABASE {app}_test OWNER {app}_test;"
98:             run([PG/'psql',admin,'-v','ON_ERROR_STOP=1','-c',sql.split(';')[0]],stdout=subprocess.DEVNULL)
99:             run([PG/'psql',admin,'-v','ON_ERROR_STOP=1','-c',sql.split(';')[1]],stdout=subprocess.DEVNULL)
100:             env[app.upper()+'_DATABASE_URL']=f'postgresql://{app}_test:{password}@127.0.0.1:{dbport}/{app}_test'
101:         env['CHAT_DATABASE_URL']='sqlite://'+str(d/'chat.sqlite3')
102:         # Legacy import fixture uses actual PostgreSQL UUID/bytea/timestamp/JSONB/UUID[] codecs.
103:         run([PG/'psql',admin,'-v','ON_ERROR_STOP=1','-c','CREATE DATABASE legacy_chat_import'],stdout=subprocess.DEVNULL)
104:         legacy=admin.rsplit('/',1)[0]+'/legacy_chat_import'
105:         legacy_sql="""
106: CREATE TABLE chat_rooms(id varchar(40) PRIMARY KEY,name varchar(200),owner_id uuid);
107: INSERT INTO chat_rooms VALUES('import-fixture','Imported room',NULL);
108: CREATE TABLE chat_users(id uuid PRIMARY KEY,username text,password_hash text);
109: INSERT INTO chat_users VALUES('11111111-1111-4111-8111-111111111111','import-user','synthetic-unused-hash');
110: CREATE TABLE chat_sessions(token_hash bytea PRIMARY KEY,user_id uuid,expires_at timestamptz);
111: INSERT INTO chat_sessions VALUES(decode('abcd','hex'),'11111111-1111-4111-8111-111111111111','2030-01-01T00:00:00Z');
112: CREATE TABLE chat_messages(id uuid PRIMARY KEY,room_id varchar(40),author_kind text,author_id text,content text,created_at timestamptz);
113: INSERT INTO chat_messages VALUES('22222222-2222-4222-8222-222222222222','import-fixture','anonymous_user',NULL,'imported message','2026-10-11T00:00:00Z');
114: CREATE TABLE chat_persona_runs(id uuid PRIMARY KEY,room_id varchar(40),parameters jsonb,status text,max_turns integer,max_output_budget integer,reserved_tokens integer,result jsonb,updated_at timestamptz,failure_status integer);
115: INSERT INTO chat_persona_runs VALUES('33333333-3333-4333-8333-333333333333','import-fixture','{"topic":"fixture","id":"33333333-3333-4333-8333-333333333333"}','completed',1,512,512,'[{"answer":"imported"}]','2026-10-11T00:00:00Z',NULL);
116: CREATE TABLE chat_topic_withdraw_outbox(id uuid PRIMARY KEY,event_ids uuid[],status text,attempts integer,next_attempt_at timestamptz,lease_token uuid,lease_until timestamptz);
117: INSERT INTO chat_topic_withdraw_outbox VALUES('44444444-4444-4444-8444-444444444444',ARRAY['55555555-5555-4555-8555-555555555555']::uuid[],'pending',0,'2026-10-11T00:00:00Z',NULL,NULL);
118: """
119:         run([PG/'psql',legacy,'-v','ON_ERROR_STOP=1','-c',legacy_sql],stdout=subprocess.DEVNULL)
120:         run(['python3','scripts/import-chat-postgres.py','--output',d/'imported.sqlite3'],env=env|{'CHAT_LEGACY_DATABASE_URL':legacy})
121:         with sqlite3.connect(d/'imported.sqlite3') as imported:
122:             check('read-only legacy import preserves UUID, bytes, time, JSON and arrays',imported.execute('SELECT id,content,created_at FROM chat_messages').fetchone()==(uuid.UUID('22222222-2222-4222-8222-222222222222').bytes,'imported message',1791676800.0) and imported.execute('SELECT token_hash FROM chat_sessions').fetchone()[0]==bytes.fromhex('abcd') and json.loads(imported.execute('SELECT event_ids FROM chat_topic_withdraw_outbox').fetchone()[0])==['55555555-5555-4555-8555-555555555555'] and json.loads(imported.execute('SELECT result FROM chat_persona_runs').fetchone()[0])[0]['answer']=='imported')
123:         before=(d/'imported.sqlite3').read_bytes()
124:         refused=subprocess.run(['python3','scripts/import-chat-postgres.py','--output',d/'imported.sqlite3'],env=env|{'CHAT_LEGACY_DATABASE_URL':legacy},stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
125:         check('legacy import refuses overwrite and leaves source PostgreSQL data unchanged',refused.returncode!=0 and before==(d/'imported.sqlite3').read_bytes() and subprocess.check_output([PG/'psql',legacy,'-At','-c','SELECT content FROM chat_messages'],text=True).strip()=='imported message')
126:         handles=[]
127:         def start(app):
128:             log=open(d/f'{app}.log','a');handles.append(log)
129:             proc=subprocess.Popen([ROOT/'target/debug'/f'day9-{app}'],cwd=ROOT,env=env,stdout=log,stderr=log)
130:             processes.append(proc)
131:             url=chat_url if app=='chat' else analytics_url
132:             wait(lambda:http(url,'/healthz')[0]==200)
133:             return proc
134:         chat=start('chat'); analytics=start('analytics')
135:         check('empty-database migrations and two independently running services')
136:         if '--varchar-events' in sys.argv:
137:             run([PG/'psql',env['ANALYTICS_DATABASE_URL'],'-v','ON_ERROR_STOP=1','-c','ALTER TABLE product_events ALTER COLUMN event_id TYPE VARCHAR(40) USING event_id::text'],stdout=subprocess.DEVNULL)
138:             check('Analytics production event adapter uses VARCHAR(40) with unchanged UUID domain/API')
139:
140:         abuse_headers={'Content-Type':'application/json','X-Copilot-Csrf':'1'}
141:         def rejected_login(base):
142:             req=urllib.request.Request(base+'/api/v1/auth/login',data=json.dumps({'username':'missing_abuse_fixture','password':'wrong-synthetic-password'}).encode(),headers=abuse_headers)
143:             try:
144:                 with urllib.request.urlopen(req,timeout=10)as response:return response.status
145:             except urllib.error.HTTPError as e:return e.code
146:         check('sequential unknown-account attempts bounded before expensive password work',all(rejected_login(chat_url)==401 for _ in range(8)) and rejected_login(chat_url)==429)
147:
148:         # 跨角色隔離不能只看資料庫名稱，實際拒絕 CONNECT。
149:         for app in ('analytics',):
150:             run([PG/'psql',admin,'-v','ON_ERROR_STOP=1','-c',f'REVOKE CONNECT ON DATABASE {app}_test FROM PUBLIC'],stdout=subprocess.DEVNULL)
151:         check('Chat is file-backed SQLite, Analytics remains isolated PostgreSQL',env['CHAT_DATABASE_URL'].startswith('sqlite:') and env['ANALYTICS_DATABASE_URL'].startswith('postgresql:'))
152:         check('unauthorized Chat write rejected',http(chat_url,'/api/v1/rooms',body={'id':'bad','name':'bad'})[0]==401)
153:         check('unauthorized Analytics read rejected',http(analytics_url,'/api/v1/events/count')[0]==401)
154:         def mcp_request(method,params=None,extra=None):
155:             params=params or {};params['_meta']={'io.modelcontextprotocol/protocolVersion':'2026-07-28','io.modelcontextprotocol/clientInfo':{'name':'local-verify','version':'0.1'},'io.modelcontextprotocol/clientCapabilities':{}}
156:             headers={'Content-Type':'application/json','Authorization':'Bearer '+chat_token,'MCP-Protocol-Version':'2026-07-28','Mcp-Method':method,'Accept':'application/json, text/event-stream'}
157:             if 'name' in params:headers['Mcp-Name']=params['name']
158:             headers.update(extra or {})
159:             request=urllib.request.Request(chat_url+'/api/v1/mcp',data=json.dumps({'jsonrpc':'2.0','id':1,'method':method,'params':params}).encode(),headers=headers)
160:             try:
161:                 with urllib.request.urlopen(request,timeout=120) as response:return response.status,json.loads(response.read())
162:             except urllib.error.HTTPError as e:return e.code,None
163:         check('actual MCP modern server discovery',mcp_request('server/discover')[1]['result']['supportedVersions']==['2026-07-28'])
164:         check('MCP catalog exposes only authorized readonly knowledge tool',mcp_request('tools/list')[1]['result']['tools'][0]['name']=='knowledge_search')
165:         check('MCP rejects method/header mismatch',mcp_request('tools/list',extra={'Mcp-Method':'tools/call'})[0]==400)
166:         check('MCP rejects untrusted browser Origin',mcp_request('tools/list',extra={'Origin':'https://untrusted.invalid'})[0]==403)
167:         check('MCP cannot invoke file or arbitrary execution tools',mcp_request('tools/call',{'name':'delete_file','arguments':{'query':'anything'}})[0]==400)
168:         room='test-'+secrets.token_hex(8)
169:         check('unconfigured AI returns 503 without mock output',http(chat_url,'/api/v1/ai/reply',chat_token,{'room_id':room})[0]==503)
170:         check('room created',http(chat_url,'/api/v1/rooms',chat_token,{'id':room,'name':'本地測試'})[0]==201)
171:         status,message=http(chat_url,'/api/v1/messages',chat_token,{'room_id':room,'content':'transactional test'})
172:         check('message persisted',status==201)
173:         check('nil message replay ID rejected before persistence',http(chat_url,'/api/v1/messages',chat_token,{'id':'00000000-0000-0000-0000-000000000000','room_id':room,'content':'invalid id'})[0]==400)
174:         event={'schema_version':1,'event_id':message['id'],'source_app':'web_chat','event_type':'message.persisted'}
175:         lookup='/api/v1/events/web_chat/'+message['id']
176:         wait(lambda:http(analytics_url,lookup,analytics_token)[0]==200)
177:         check('this exact Chat message traversed Outbox to Analytics')
178:         before=http(analytics_url,'/api/v1/events/count',analytics_token)[1]['total']
179:         for _ in range(3):
180:             assert http(analytics_url,'/api/v1/events',ingest_token,event)[0]==202
181:         check('repeated event counted once',http(analytics_url,'/api/v1/events/count',analytics_token)[1]['total']==before)
182:         check('sensitive/unknown payload field rejected',http(analytics_url,'/api/v1/events',ingest_token,event|{'content':'private'})[0]==422)
183:         check('Chat credential cannot impersonate Pet',http(analytics_url,'/api/v1/events',ingest_token,event|{'source_app':'desktop_pet','event_type':'file.cleanup.succeeded'})[0]==401)
184:         check('read credential cannot ingest',http(analytics_url,'/api/v1/events',analytics_token,event)[0]==401)
185:         check('invalid schema rejected',http(analytics_url,'/api/v1/events',ingest_token,event|{'schema_version':99})[0]==400)
186:         def account(name):
187:             payload={'username':name,'password':secrets.token_urlsafe(24)}
188:             req=urllib.request.Request(chat_url+'/api/v1/auth/register',data=json.dumps(payload).encode(),headers={'Content-Type':'application/json','X-Copilot-CSRF':'1'})
189:             with urllib.request.urlopen(req) as response: assert response.status==201
190:             req=urllib.request.Request(chat_url+'/api/v1/auth/login',data=json.dumps(payload).encode(),headers={'Content-Type':'application/json','X-Copilot-CSRF':'1'})
191:             with urllib.request.urlopen(req) as response:
192:                 cookie=response.headers['Set-Cookie']; assert 'HttpOnly' in cookie and 'SameSite=Strict' in cookie
193:                 return cookie.split(';')[0]
194:         def member_http(cookie,path,body=None,csrf=True):
195:             req=urllib.request.Request(chat_url+path,data=json.dumps(body).encode() if body is not None else None,headers={'Content-Type':'application/json','Cookie':cookie,**({'X-Copilot-CSRF':'1'} if csrf else {})})
196:             try:
197:                 with urllib.request.urlopen(req,timeout=120) as response:
198:                     text=response.read();return response.status,json.loads(text) if text else None
199:             except urllib.error.HTTPError as e:return e.code,None
200:         alice_name='alice_'+secrets.token_hex(5);bob_name='bob_'+secrets.token_hex(5)
201:         alice=account(alice_name);bob=account(bob_name);private='private-'+secrets.token_hex(5)
202:         check('session-authenticated owner creates private room',member_http(alice,'/api/v1/rooms',{'id':private,'name':'私人測試'})[0]==201)
203:         check('uninvited user cannot read private history',member_http(bob,'/api/v1/messages?room_id='+private)[0]==403)
204:         check('uninvited user cannot write private message',member_http(bob,'/api/v1/messages',{'room_id':private,'content':'denied'})[0]==403)
205:         check('session writes require CSRF header',member_http(alice,'/api/v1/messages',{'room_id':private,'content':'csrf'},csrf=False)[0]==403)
206:         check('owner can invite another user',member_http(alice,'/api/v1/rooms/invite',{'room_id':private,'username':bob_name})[0]==204)
207:         check('invited user can send private message',member_http(bob,'/api/v1/messages',{'room_id':private,'content':'authorized'})[0]==201)
208:         persona_route='/api/v1/rooms/'+private+'/personas'
209:         persona_id=str(uuid.uuid4());persona_body={'id':persona_id,'name':'老師','instructions':'協助學習 Rust，只使用授權資料。','memory':'','cooldown_seconds':2,'enabled':True}
210:         check('room member cannot change owner personas',member_http(bob,persona_route,persona_body)[0]==403)
211:         check('owner can configure personas',member_http(alice,persona_route,persona_body)[0]==204)
212:         check('persona configuration persists with scoped memory',member_http(alice,persona_route)[1][0]['id']==persona_id)
213:         run_route='/api/v1/rooms/'+private+'/persona-runs'
214:         bounded={'id':str(uuid.uuid4()),'topic':'只討論測試資料','max_turns':2,'max_output_budget':1024,'tools':False,'confirmed':True}
215:         check('persona runs reject unconfirmed requests',member_http(alice,run_route,bounded|{'confirmed':False})[0]==400)
216:         check('persona turn budget rejects excessive rounds',member_http(alice,run_route,bounded|{'max_turns':99})[0]==400)
217:         check('persona run with no provider configuration fails without a fake answer',member_http(alice,run_route,bounded)[0]==503)
218:         assert member_http(bob,'/api/v1/auth/logout',{})[0]==204
219:
220:         check('logout revokes server-side session',member_http(bob,'/api/v1/messages?room_id='+private)[0]==401)
221:         wait(lambda:any(row['event_type']=='login.succeeded' and row['total']>=2 for row in http(analytics_url,'/api/v1/events/summary',analytics_token)[1]))
222:         wait(lambda:any(row['event_type']=='logout.succeeded' and row['total']>=1 for row in http(analytics_url,'/api/v1/events/summary',analytics_token)[1]))
223:         check('successful login/logout emits durable anonymous Analytics events')
224:
225:         second_port=port();second_url=f'http://127.0.0.1:{second_port}'
226:         second_env=env|{'CHAT_LISTEN':f'127.0.0.1:{second_port}'}
227:         second_log=open(d/'second-chat.log','a');handles.append(second_log)
228:         second_chat=subprocess.Popen([ROOT/'target/debug/day9-chat'],cwd=ROOT,env=second_env,stdout=second_log,stderr=second_log);processes.append(second_chat)
229:         wait(lambda:http(second_url,'/healthz')[0]==200)
230:         run(['node','scripts/realtime-e2e.mjs'],env=env|{'CHAT_URL_SECOND':second_url})
231:         check('two independent Chat processes share presence, typing and canonical message fan-out')
232:         check('real WebSocket two-client and room-isolation E2E')
233:         run(['node','scripts/session-ws-e2e.mjs'],env=env|{'CHAT_URL_SECOND':second_url})
234:         stop(second_chat)
235:         check('revoked cookie session cannot retain WebSocket access')
236:         # Seed reviewable, non-AI fixture to prove the real PostgreSQL/HTTP policy lifecycle.
237:         suggestion=str(uuid.uuid4())
238:         proposal=json.dumps({'summary':'測試資料只有事件次數。','action':'chat_context_messages','value':42,'rationale':'僅測試安全政策變更，不推論品質。'})
239:         sql="INSERT INTO analysis_suggestions(id,target_app,metric_snapshot,summary,proposal) VALUES ('"+suggestion+"','web_chat','{\"rows\":[]}','測試資料只有事件次數。','"+proposal+"')"
240:         run([PG/'psql',env['ANALYTICS_DATABASE_URL'],'-v','ON_ERROR_STOP=1','-c',sql],stdout=subprocess.DEVNULL)
241:         route='/api/v1/rooms/'+room+'/suggestions/'+suggestion
242:         admin_token=env['ANALYTICS_ADMIN_TOKEN']
243:         check('pending suggestion cannot be applied',http(chat_url,route+'/apply',chat_token,{'confirmed':True})[0]==409)
244:         check('cross-source proposal reads rejected',http(analytics_url,'/api/v1/suggestions/'+suggestion,pet_token)[0]==404)
245:         check('non-admin suggestion review rejected',http(analytics_url,'/api/v1/suggestions/'+suggestion+'/review',analytics_token,{'decision':'approved','confirmed':True})[0]==401)
246:         check('unapplied suggestion cannot be evaluated',http(analytics_url,'/api/v1/suggestions/'+suggestion+'/evaluate',admin_token,{'confirmed':True})[0]==409)
247:         assert http(analytics_url,'/api/v1/suggestions/'+suggestion+'/review',admin_token,{'decision':'approved','confirmed':True})[0]==204
248:         check('policy change needs explicit confirmation',http(chat_url,route+'/apply',chat_token,{'confirmed':False})[0]==400)
249:         status,applied=http(chat_url,route+'/apply',chat_token,{'confirmed':True})
250:         assert status==200 and applied['analytics_acknowledged'] and applied['value']==42
251:         check('scoped policy adoption has stable idempotent receipt',http(chat_url,route+'/apply',chat_token,{'confirmed':True})[1]==applied)
252:         with sqlite3.connect(d/'chat.sqlite3') as connection: actual=str(connection.execute('SELECT context_messages FROM chat_room_policy WHERE room_id=?',(room,)).fetchone()[0])
253:         check('adoption modifies actual room context limit',actual=='42')
254:         status,rolled=http(chat_url,route+'/rollback',chat_token,{'confirmed':True})
255:         assert status==200 and rolled['value']==30 and rolled['analytics_acknowledged']
256:         check('policy rollback is acknowledged and idempotent',http(chat_url,route+'/rollback',chat_token,{'confirmed':True})[1]==rolled)
257:         status,evaluation=http(analytics_url,'/api/v1/suggestions/'+suggestion+'/evaluate',admin_token,{'confirmed':True})
258:         check('evaluation stores actual descriptive counts without a causal claim',status==200 and evaluation['after']['target_app']=='web_chat' and 'cannot prove causal impact' in evaluation['interpretation'])
259:
260:         stop(chat);stop(analytics);chat=start('chat');analytics=start('analytics')
261:         recent=http(chat_url,'/api/v1/messages?room_id='+room,chat_token)[1]
262:         cursor=recent[-1]['id']
263:         older=http(chat_url,'/api/v1/messages?room_id='+room+'&before='+cursor,chat_token)
264:         check('history cursor excludes its anchor without overlapping messages',older[0]==200 and all(row['id']!=cursor for row in older[1]))
265:         check('foreign or missing history cursor is rejected',http(chat_url,'/api/v1/messages?room_id='+room+'&before='+str(uuid.uuid4()),chat_token)[0]==404)
266:         check('history survives service restart',any(x['id']==message['id'] for x in http(chat_url,'/api/v1/messages?room_id='+room,chat_token)[1]))
267:         count=http(analytics_url,'/api/v1/events/count',analytics_token)[1]['total']
268:         assert http(analytics_url,'/api/v1/events',ingest_token,event)[0]==202
269:         check('idempotency survives service restart',http(analytics_url,'/api/v1/events/count',analytics_token)[1]['total']==count)
270:         stop(analytics)
271:         status,queued=http(chat_url,'/api/v1/messages',chat_token,{'room_id':room,'content':'retry after outage'})
272:         assert status==201
273:         time.sleep(1.5)
274:         analytics=start('analytics')
275:         wait(lambda:http(analytics_url,'/api/v1/events/web_chat/'+queued['id'],analytics_token)[0]==200)
276:         check('Analytics outage causes durable retry and later delivery')
277:         run([ROOT/'target/debug/day9-chat','--backup',d/'chat-backup.sqlite3'],env=env)
278:         with sqlite3.connect(d/'chat.sqlite3') as original, sqlite3.connect(d/'chat-backup.sqlite3') as copied:
279:             tables=[r[0] for r in original.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name")]
280:             check('portable Chat backup includes exact committed data from every table',all(original.execute(f'SELECT * FROM "{t}" ORDER BY rowid').fetchall()==copied.execute(f'SELECT * FROM "{t}" ORDER BY rowid').fetchall() for t in tables))
281:         stop(chat)
282:         relocated=d/'relocated';relocated.mkdir(mode=0o700);(d/'chat-backup.sqlite3').rename(relocated/'chat.sqlite3')
283:         env['CHAT_DATABASE_URL']='sqlite://'+str(relocated/'chat.sqlite3')
284:         chat=start('chat')
285:         check('relocated SQLite Chat starts with identical history',any(row['id']==message['id'] for row in http(chat_url,'/api/v1/messages?room_id='+room,chat_token)[1]))
286:         # 真實 pg_dump/pg_restore 到第三個隔離庫，逐列核對，不以備份檔存在宣稱成功。
287:         run([PG/'pg_dump',env['ANALYTICS_DATABASE_URL'],'-Fc','-f',d/'analytics.dump'])
288:         run([PG/'psql',admin,'-v','ON_ERROR_STOP=1','-c','CREATE DATABASE analytics_restore'],stdout=subprocess.DEVNULL)
289:         restored=admin.rsplit('/',1)[0]+'/analytics_restore'
290:         run([PG/'pg_restore','--no-owner','--no-acl','--exit-on-error','-d',restored,d/'analytics.dump'])
291:         sql="SELECT source_app,event_id,event_type FROM product_events ORDER BY source_app,event_id"
292:         a=subprocess.check_output([PG/'psql',env['ANALYTICS_DATABASE_URL'],'-At','-c',sql])
293:         b=subprocess.check_output([PG/'psql',restored,'-At','-c',sql])
294:         check('Analytics backup restored with exact matching event rows',a==b)
295:         # PostgreSQL 故障時 health 非 200，復原後 worker 仍工作。
296:         run([PG/'pg_ctl','-D',d/'data','stop','-m','fast'],stdout=subprocess.DEVNULL);started=False
297:         check('PostgreSQL outage does not make SQLite Chat unready',http(chat_url,'/healthz')[0]==200)
298:         check('database outage makes Analytics unready',http(analytics_url,'/healthz')[0]==503)
299:         run([PG/'pg_ctl','-D',d/'data','-l',d/'pg.log','-o',f'-h 127.0.0.1 -p {dbport} -k {scratch}','start'],stdout=subprocess.DEVNULL);started=True
300:         wait(lambda:http(chat_url,'/healthz')[0]==200 and http(analytics_url,'/healthz')[0]==200)
301:         status,recovered=http(chat_url,'/api/v1/messages',chat_token,{'room_id':room,'content':'after DB recovery'})
302:         assert status==201
303:         wait(lambda:http(analytics_url,'/api/v1/events/web_chat/'+recovered['id'],analytics_token)[0]==200)
304:         check('dispatcher recovers after PostgreSQL interruption')
305:         # Rotate fixture-only credentials through real process restart; no old credential grace.
306:         old_chat=chat_token;old_read=analytics_token;old_ingest=ingest_token
307:         chat_token=secrets.token_urlsafe(32);analytics_token=secrets.token_urlsafe(32);ingest_token=secrets.token_urlsafe(32)
308:         env.update(CHAT_API_TOKEN=chat_token,ANALYTICS_API_TOKEN=analytics_token,ANALYTICS_CHAT_TOKEN=ingest_token)
309:         stop(analytics);analytics=start('analytics')
310:         check('old Analytics query key is revoked after rotation',http(analytics_url,'/api/v1/events/count',old_read)[0]==401 and http(analytics_url,'/api/v1/events/count',analytics_token)[0]==200)
311:         rotated_event={'schema_version':1,'event_id':str(uuid.uuid4()),'source_app':'web_chat','event_type':'message.persisted'}
312:         check('old source ingest key is revoked after rotation',http(analytics_url,'/api/v1/events',old_ingest,rotated_event)[0]==401)
313:         stop(chat);chat=start('chat')
314:         check('old Chat administrative key is revoked after rotation',http(chat_url,'/api/v1/rooms',old_chat)[0]==401 and http(chat_url,'/api/v1/rooms',chat_token)[0]==200)
315:         rotated_id=str(uuid.uuid4());assert http(chat_url,'/api/v1/messages',chat_token,{'id':rotated_id,'room_id':room,'content':'credential rotation fixture'})[0]==201
316:         wait(lambda:http(analytics_url,'/api/v1/events/web_chat/'+rotated_id,analytics_token)[0]==200)
317:         check('new Chat/Analytics source keys preserve durable cross-service delivery')
318:
319:         if '--load' in sys.argv:
320:             load_urls=[chat_url]
321:             additional=[]
322:             for index in range(4):
323:                 loadport=port();url=f'http://127.0.0.1:{loadport}';log=open(d/f'load-chat-{index}.log','a');handles.append(log)
324:                 process=subprocess.Popen([ROOT/'target/debug/day9-chat'],cwd=ROOT,env=env|{'CHAT_LISTEN':f'127.0.0.1:{loadport}'},stdout=log,stderr=log);processes.append(process);additional.append(process);wait(lambda:http(url,'/healthz')[0]==200);load_urls.append(url)
325:             run(['node','scripts/realtime-load-e2e.mjs'],env=env|{'CHAT_LOAD_URLS':json.dumps(load_urls),'CHAT_LOAD_REPORT':str(ROOT/'target/realtime-load-validation.json')})
326:             check('real 512-socket admission boundaries and 64-message private cross-process fan-out load')
327:             for process in additional:stop(process)
328:         if '--live-openai' in sys.argv:
329:             stop(chat)
330:             env['OPENAI_API_KEY']=live_key
331:             env['OPENAI_CHAT_MODEL']=os.environ.get('OPENAI_CHAT_MODEL','gpt-5.6-luna')
332:             chat=start('chat')
333:             live_room='live-'+secrets.token_hex(8)
334:             assert http(chat_url,'/api/v1/rooms',chat_token,{'id':live_room,'name':'OpenAI 驗證'})[0]==201
335:             assert http(chat_url,'/api/v1/messages',chat_token,{'room_id':live_room,'content':'請只回答四個中文字：測試成功。'})[0]==201
336:             status,document=http(chat_url,'/api/v1/knowledge',chat_token,{'title':'授權的實驗資料','content':'本地驗證的實驗代碼是 COMET-432。這是供 RAG 測試使用的非私人資料。'})
337:             check('real OpenAI embedding persisted in portable SQLite',status==201)
338:             status,hits=http(chat_url,'/api/v1/knowledge/search',chat_token,{'query':'實驗代碼是什麼？'})
339:             check('SQLite cosine retrieval finds the expected document',status==200 and any(hit['id']==document['id'] for hit in hits))
340:             request=urllib.request.Request(chat_url+'/api/v1/ai/reply',data=json.dumps({'room_id':live_room,'rag':True}).encode(),headers={'Authorization':'Bearer '+chat_token,'Content-Type':'application/json'})
341:             streamed='';saved_id=None;event_type=None
342:             with urllib.request.urlopen(request,timeout=100) as response:
343:                 for line in response:
344:                     text=line.decode().strip()
345:                     if text.startswith('event:'): event_type=text[6:].strip()
346:                     if text.startswith('data:'):
347:                         data=text[5:].strip()
348:                         assert event_type!='error', 'OpenAI stream failed without persistence'
349:                         if event_type=='delta':streamed+=json.loads(data)['text']
350:                         if event_type=='done':saved_id=json.loads(data)['id']
351:             check('real OpenAI Responses stream reaches completion',bool(streamed.strip()) and bool(saved_id))
352:             records=http(chat_url,'/api/v1/messages?room_id='+live_room,chat_token)[1]
353:             check('completed real AI stream matches persisted message',any(row['id']==saved_id and row['content']==streamed for row in records))
354:             wait(lambda:http(analytics_url,'/api/v1/events/web_chat/'+saved_id,analytics_token)[0]==200)
355:             check('real AI answer emits durable Analytics event')
356:             status,agent=http(chat_url,'/api/v1/ai/agent',chat_token,{'room_id':live_room,'question':'請使用知識庫工具找到本地驗證的實驗代碼，回答代碼、引用文件，並提出一個理解問題。'})
357:             check('real OpenAI Function Calling through authenticated local MCP and strict JSON answer',status==200 and agent['steps'] and agent['follow_up_question'] and document['id'] in agent['sources'] and 'COMET-432' in agent['answer'])
358:             wait(lambda:http(analytics_url,'/api/v1/events/web_chat/'+agent['id'],analytics_token)[0]==200)
359:             check('completed Agent answer emits a durable Analytics event')
360:             persona_base='/api/v1/rooms/'+live_room+'/personas'
361:             for label,prompt in [('老師A','你是老師，短句解釋 Rust 所有權，再問一個理解問題。記憶不超過30字。'),('學生B','你是學生，回應老師的說明並提出一個簡短理解問題。記憶不超過30字。')]:
362:                 assert http(chat_url,persona_base,chat_token,{'id':str(uuid.uuid4()),'name':label,'instructions':prompt,'memory':'','cooldown_seconds':1,'enabled':True})[0]==204
363:             run_base='/api/v1/rooms/'+live_room+'/persona-runs'
364:             discussion={'id':str(uuid.uuid4()),'topic':'只用短句討論 Rust 所有權。本例只有測試資料。','max_turns':2,'max_output_budget':1024,'tools':False,'confirmed':True}
365:             status,exchange=http(chat_url,run_base,chat_token,discussion)
366:             check('real two-persona round-robin completes within reserved output budget',status==200 and exchange['status']=='completed' and len(exchange['turns'])==2 and len({turn['persona_id']for turn in exchange['turns']})==2 and exchange['reserved_output_tokens']==1024)
367:             check('completed persona run replays without new provider calls',http(chat_url,run_base,chat_token,discussion)[1]==exchange)
368:             for turn in exchange['turns']:
369:                 wait(lambda:http(analytics_url,'/api/v1/events/web_chat/'+turn['message_id'],analytics_token)[0]==200)
370:             saved_personas=http(chat_url,persona_base,chat_token)[1]
371:             check('each persona retains its own bounded memory',len(saved_personas)==2 and all(isinstance(p['memory'],str) and len(p['memory'].encode())<=2048 for p in saved_personas))
372:             exhausted=discussion|{'id':str(uuid.uuid4()),'max_output_budget':512}
373:             status,stopped=http(chat_url,run_base,chat_token,exhausted)
374:             check('real persona loop stops before a second request when output budget is exhausted',status==200 and stopped['status']=='budget_exhausted' and len(stopped['turns'])==1 and stopped['reserved_output_tokens']==512)
375:             tool_run=discussion|{'id':str(uuid.uuid4()),'max_turns':1,'tools':True,'topic':'使用知識庫工具找到本地驗證實驗代碼，短句回答、引用文件並提出理解問題。'}
376:             status,grounded=http(chat_url,run_base,chat_token,tool_run)
377:             print('Synthetic persona tool result:',status,json.dumps(grounded,ensure_ascii=False),flush=True)
378:             check('real bounded persona Function Calling uses scoped MCP citations',status==200 and grounded['status']=='completed' and grounded['turns'][0]['steps'] and document['id'] in grounded['turns'][0]['sources'] and 'COMET-432' in grounded['turns'][0]['answer'])
379:
380:             check('knowledge document can be deleted',http(chat_url,'/api/v1/knowledge/'+document['id'],chat_token,method='DELETE')[0]==204)
381:         if live_key:
382:             env['OPENAI_API_KEY']=live_key
383:             env['OPENAI_CHAT_MODEL']=os.environ.get('OPENAI_CHAT_MODEL','gpt-5.6-luna')
384:             stop(analytics);analytics=start('analytics')
385:             status,insight=http(analytics_url,'/api/v1/insights',env['ANALYTICS_ADMIN_TOKEN'],{'target_app':'web_chat'})
386:             check('real OpenAI Analytics structured insight uses a persisted aggregate snapshot',status==200 and insight['snapshot']['rows'] and insight['proposal']['action']=='chat_context_messages' and insight['status']=='pending')
387:
388:         wait(lambda:all(any(path==expected for path,_ in observed) for expected in ('/v1/traces','/v1/metrics','/v1/logs')))
389:         check('real local OTLP traces, metrics and logs received')
390:         def trace_ids(expected_service,expected_route=None):
391:             ids=set()
392:             for path,payload in observed:
393:                 if path!='/v1/traces':continue
394:                 for resource in json.loads(payload).get('resourceSpans',[]):
395:                     attrs={x['key']:x.get('value',{}).get('stringValue') for x in resource.get('resource',{}).get('attributes',[])}
396:                     if attrs.get('service.name')!=expected_service:continue
397:                     for scope in resource.get('scopeSpans',[]):
398:                         for span in scope.get('spans',[]):
399:                             values={x['key']:x.get('value',{}).get('stringValue') for x in span.get('attributes',[])}
400:                             if expected_route is None or values.get('http.route')==expected_route:ids.add(span['traceId'])
401:             return ids
402:         chat_ids=trace_ids('chat','/api/v1/messages');analytics_ids=trace_ids('analytics','/copilot.analytics.v1.EventIngestion/{*rest}')
403:         check('Chat Outbox and gRPC Analytics preserve W3C trace correlation',bool(chat_ids&analytics_ids))
404:
405:         forbidden=([live_key] if live_key else [])+[chat_token,analytics_token,ingest_token,pet_token,'transactional test','retry after outage','not committed','Rust token=synthetic-secret','Rust rust ownership']
406:         if '--pet-gui' in sys.argv:
407:             pet_suggestion=str(uuid.uuid4())
408:             pet_proposal=json.dumps({'summary':'複習事件測試。','action':'pet_review_min_days','value':3,'rationale':'測試可回退設定，不推論學習品質。'})
409:             sql="INSERT INTO analysis_suggestions(id,target_app,metric_snapshot,summary,proposal,status) VALUES ('"+pet_suggestion+"','desktop_pet','{\"rows\":[]}','複習事件測試。','"+pet_proposal+"','approved')"
410:             run([PG/'psql',env['ANALYTICS_DATABASE_URL'],'-v','ON_ERROR_STOP=1','-c',sql],stdout=subprocess.DEVNULL)
411:             native_env=env | {'PET_GUI_TEST_ANALYTICS':'true','PET_GUI_TEST_SUGGESTION':pet_suggestion}
412:             run(['python3','scripts/pet-gui-verify.py','--analytics']+(['--live-ai'] if '--live-openai' in sys.argv else []),env=native_env)
413:             summary=http(analytics_url,'/api/v1/events/summary',analytics_token)[1]
414:             bytype={(row['source_app'],row['event_type']):row['total'] for row in summary}
415:             check('native Pet policy apply and rollback acknowledged by Analytics',http(analytics_url,'/api/v1/suggestions/'+pet_suggestion,pet_token)[1]['status']=='rolled_back')
416:             check('actual Pet restore event delivered exactly once to Analytics',bytype.get(('desktop_pet','file.restore.succeeded'))==1)
417:             check('actual Pet GUI receipts and learning events delivered exactly once to Analytics',bytype.get(('desktop_pet','file.cleanup.succeeded'))==2 and bytype.get(('desktop_pet','learning.review.completed'))==1)
418:             # Native file operations preserve traceparent into durable SQLite outbox.
419:             time.sleep(.5)
420:             pet_ids=trace_ids('desktop_pet')
421:             analytics_ids=trace_ids('analytics','/copilot.analytics.v1.EventIngestion/{*rest}')
422:             check('Pet and Analytics preserve W3C trace correlation',bool(pet_ids&analytics_ids))
423:             forbidden += ['安全隔離的測試記憶','Rust 的所有權如何避免 double free？','sample.txt','second.txt']
424:         env['VERIFY_TOPIC_FIXTURE']=str(d/'topic-fixture.json')
425:         run(['node','scripts/topic-consent-e2e.mjs'],env=env)
426:         stop(analytics)
427:         run(['node','scripts/topic-consent-e2e.mjs','--withdraw'],env=env)
428:         analytics=start('analytics')
429:         run(['node','scripts/topic-consent-e2e.mjs','--verify-withdrawal'],env=env)
430:         check('explicit default-off topic consent, fixed vocabulary, filtering, idempotency and durable withdrawal across Analytics outage')
431:         check('OTLP transport strips inherited authentication headers',all('metadata-synthetic-secret' not in str(headers) and 'Authorization' not in headers for headers in observed_headers))
432:         check('telemetry excludes credentials and message contents',all(value.encode() not in payload for value in forbidden for _,payload in observed))
433:         REPORT.parent.mkdir(parents=True,exist_ok=True)
434:         REPORT.write_text(json.dumps({'status':'passed','checks':results,'scope':'local synthetic reliability and integration; other release gates have separate evidence','source_sha':SOURCE_SHA,'source_dirty':SOURCE_DIRTY,'event_identity_storage':'varchar(40)' if '--varchar-events' in sys.argv else 'uuid','live_openai':bool(live_key),'openai_model':env.get('OPENAI_CHAT_MODEL') if live_key else None},ensure_ascii=False,indent=2)+'\n')
435:         if '--browser' in sys.argv or '--browser-live' in sys.argv:
436:             if '--browser-live' in sys.argv:
437:                 stop(chat);chat=start('chat')
438:             fixture={'chat_url':chat_url,'analytics_url':analytics_url,'analytics_read_token':analytics_token,'analytics_admin_token':env['ANALYTICS_ADMIN_TOKEN'],'username':'browser_'+secrets.token_hex(5),'password':secrets.token_urlsafe(24)}
439:             req=urllib.request.Request(chat_url+'/api/v1/auth/register',data=json.dumps({'username':fixture['username'],'password':fixture['password']}).encode(),headers={'Content-Type':'application/json','X-Copilot-CSRF':'1'})
440:             with urllib.request.urlopen(req) as response: assert response.status==201
441:             run([PG/'psql',env['ANALYTICS_DATABASE_URL'],'-v','ON_ERROR_STOP=1','-c',"UPDATE product_events SET received_at=received_at-interval '1 day' WHERE event_id='"+message['id']+"'"],stdout=subprocess.DEVNULL)
442:             path=ROOT.parent/'browser-fixture.json';path.write_text(json.dumps(fixture));path.chmod(0o600)
443:             print('BROWSER_FIXTURE_READY',flush=True)
444:             control=ROOT.parent/'browser-control.txt'
445:             control.unlink(missing_ok=True)
446:             while True:
447:                 if control.exists():
448:                     command=control.read_text().strip();control.unlink()
449:                     if command=='stop_chat':stop(chat)
450:                     elif command=='start_chat' and chat.poll() is not None:
451:                         chat=start('chat')
452:                     (ROOT.parent/'browser-control-result.json').write_text(json.dumps({'command':command,'chat_running':chat.poll() is None}))
453:                 time.sleep(.2)
454:
455:     finally:
456:         for proc in processes:stop(proc)
457:         collector.shutdown();collector.server_close()
458:         if started:subprocess.run([PG/'pg_ctl','-D',d/'data','stop','-m','fast'],stdout=subprocess.DEVNULL)
```

## copilot_use/scripts/pet-gui-verify.py

SHA256 `e1c73fa66d433990f9e4b6274a53043554892ece31e27f1603ad1512dd04a74f`。

```
1: #!/usr/bin/env python3
2: """Exercise this application's own WebKit/WASM/IPC in an isolated disposable state."""
3: import time, json, os, pathlib, subprocess, tempfile, sys, threading, argparse, hashlib, shutil, sqlite3, tomllib
4: from http.server import ThreadingHTTPServer,BaseHTTPRequestHandler
5: from contextlib import closing
6: ROOT = pathlib.Path(__file__).resolve().parents[1]
7: REPORT = ROOT / "target/pet-gui-validation.json"
8: if sys.platform.startswith("linux") and not os.environ.get("DISPLAY"):
9:     raise SystemExit("A Linux desktop/X11 display is required for actual GUI execution")
10: parser=argparse.ArgumentParser()
11: parser.add_argument('--executable',type=pathlib.Path)
12: parser.add_argument('--update-package',type=pathlib.Path)
13: parser.add_argument('--expected-executable',type=pathlib.Path)
14: parser.add_argument('--live-ai',action='store_true')
15: parser.add_argument('--live-image',action='store_true')
16: parser.add_argument('--analytics',action='store_true')
17: parser.add_argument('--updater',action='store_true')
18: options=parser.parse_args()
19: if not options.executable:
20:     if not (ROOT/'apps/desktop_pet/generated-icons/icon.ico').exists():
21:         subprocess.run(['node','scripts/build-pet-ui.mjs'],cwd=ROOT,check=True)
22:     subprocess.run(["cargo", "build", "--manifest-path", str(ROOT / "apps/desktop_pet/Cargo.toml"), "--locked"], check=True, cwd=ROOT)
23: executable=options.executable.resolve() if options.executable else ROOT / ("apps/desktop_pet/target/debug/copilot-desktop-pet.exe" if sys.platform=="win32" else "apps/desktop_pet/target/debug/copilot-desktop-pet")
24: with tempfile.TemporaryDirectory(prefix="copilot-pet-gui-") as scratch:
25:     # macOS /var is a symlink; run disposable bundles through the canonical
26:     # /private path so Tauri keeps its executable-path protection enabled.
27:     directory = pathlib.Path(scratch).resolve()
28:     if sys.platform=='win32':
29:         # Private disposable fixture only; chmod on Windows does not establish a DACL.
30:         permissions=r"""
31: $identity=[System.Security.Principal.WindowsIdentity]::GetCurrent().User
32: $acl=New-Object System.Security.AccessControl.DirectorySecurity
33: $acl.SetOwner($identity)
34: $acl.SetAccessRuleProtection($true,$false)
35: foreach($sid in @($identity.Value,'S-1-5-18','S-1-5-32-544')) {
36:  $rule=New-Object System.Security.AccessControl.FileSystemAccessRule([System.Security.Principal.SecurityIdentifier]::new($sid),'FullControl','ContainerInherit,ObjectInherit','None','Allow')
37:  $acl.AddAccessRule($rule)
38: }
39: Set-Acl -LiteralPath $env:COPILOT_GUI_FIXTURE_DIRECTORY -AclObject $acl
40: """
41:         permission_env=os.environ|{'COPILOT_GUI_FIXTURE_DIRECTORY':str(directory)}
42:         # Do not make Windows PowerShell load inherited PowerShell 7 modules.
43:         permission_env.pop('PSModulePath',None)
44:         permission_shell=shutil.which('pwsh') or shutil.which('powershell')
45:         if not permission_shell:raise SystemExit('Windows fixture ACL tool unavailable')
46:         subprocess.run([permission_shell,'-NoProfile','-NonInteractive','-Command',permissions],env=permission_env,check=True,stdout=subprocess.DEVNULL)
47:     files = directory / "files"
48:     files.mkdir(mode=0o700)
49:     sample = files / "sample.txt"
50:     sample.write_text("disposable GUI verification fixture")
51:     sample.chmod(0o600)
52:     second=files / "second.txt"
53:     second.write_text("disposable UI confirmation fixture")
54:     second.chmod(0o600)
55:     env = os.environ.copy()
56:     live=options.live_ai or options.live_image
57:     if not live:env.pop("OPENAI_API_KEY", None)
58:     if live and not env.get("OPENAI_API_KEY"):raise SystemExit("OPENAI_API_KEY not configured")
59:     analytics="--analytics" in sys.argv
60:     if not analytics:env.pop("ANALYTICS_ENDPOINT", None)
61:     env["PET_GUI_TEST_ANALYTICS"]="true" if analytics else "false"
62:     env.update(PET_GUI_TEST="true", PET_GUI_TEST_AI="true" if options.live_ai else "false", PET_GUI_TEST_IMAGE="true" if options.live_image else "false",
63:                PET_GUI_TEST_REPORT=str(directory / "report.json"), PET_DATA_DIR=str(directory / "state"),
64:                PET_AUTHORIZED_ROOT=str(files))
65:     server=None
66:     if options.updater:
67:         key=directory/'updater-test.key';payload=directory/'fixture.bin'
68:         if options.update_package:
69:             if not options.executable:raise SystemExit('Actual update acceptance requires a packaged executable')
70:             shutil.copyfile(options.update_package,payload)
71:             original_executable=executable
72:             base_executable_hash=hashlib.sha256(executable.read_bytes()).hexdigest()
73:             if sys.platform.startswith('linux'):
74:                 if executable.suffix!='.AppImage':raise SystemExit('Linux actual update requires AppImage')
75:                 disposable=directory/'current.AppImage';shutil.copy2(executable,disposable);executable=disposable
76:             elif sys.platform=='darwin':
77:                 if not options.expected_executable:raise SystemExit('Expected updated executable required')
78:                 bundle=next((p for p in executable.parents if p.suffix=='.app'),None)
79:                 if bundle is None:raise SystemExit('macOS update requires app bundle')
80:                 disposable=directory/'application'/bundle.name;shutil.copytree(bundle,disposable,symlinks=True)
81:                 executable=disposable/executable.relative_to(bundle)
82:             elif sys.platform=='win32':
83:                 if not options.expected_executable:raise SystemExit('Expected updated executable required')
84:                 disposable=directory/'application';shutil.copytree(executable.parent,disposable)
85:                 executable=disposable/executable.name
86:             else:raise SystemExit('Unsupported update platform')
87:             expected_updated_hash=hashlib.sha256((options.expected_executable or options.update_package).read_bytes()).hexdigest()
88:             if sys.platform=='win32':
89:                 fixture_version=subprocess.check_output([options.expected_executable.resolve(),'--version'],cwd=ROOT,env=env,timeout=30,text=True,encoding='utf-8').strip()
90:                 if fixture_version!='0.1.1':raise SystemExit('Windows update fixture version differs: '+repr(fixture_version))
91:             env['PET_GUI_TEST_INSTALL_UPDATE']='true'
92:         else:payload.write_bytes(b'isolated updater signature fixture')
93:         subprocess.run(['cargo','tauri','signer','generate','--ci','--password','','--write-keys',str(key)],check=True,cwd=ROOT,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
94:         key.chmod(0o600)
95:         subprocess.run(['cargo','tauri','signer','sign','--private-key-path',str(key),'--password','','--app-version','0.1.1',str(payload)],check=True,cwd=ROOT,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
96:         signature=payload.with_suffix('.bin.sig').read_text(encoding="utf-8").strip();public=key.with_suffix('.key.pub').read_text(encoding="utf-8").strip();requests=[]
97:         class UpdateServer(BaseHTTPRequestHandler):
98:             def log_message(self,*_):pass
99:             def do_GET(self):
100:                 if self.path=='/manifest':
101:                     requests.append(self.path);mode=len(requests)
102:                     value={'version':'0.1.2' if mode==2 else '0.1.1','url':f'http://127.0.0.1:{self.server.server_port}/'+('tampered' if mode==3 else 'package'),'signature':signature}
103:                     if mode>=4:value['url']='http://192.0.2.1/package'
104:                     data=json.dumps(value).encode();kind='application/json'
105:                 elif self.path=='/package':data=payload.read_bytes();kind='application/octet-stream'
106:                 elif self.path=='/tampered':data=b'tampered fixture';kind='application/octet-stream'
107:                 else:self.send_error(404);return
108:                 self.send_response(200);self.send_header('Content-Type',kind);self.send_header('Content-Length',str(len(data)));self.end_headers();self.wfile.write(data)
109:         server=ThreadingHTTPServer(('127.0.0.1',0),UpdateServer);threading.Thread(target=server.serve_forever,daemon=True).start()
110:         env.update(PET_UPDATE_ENDPOINT=f'http://127.0.0.1:{server.server_port}/manifest',PET_UPDATE_PUBLIC_KEY=public,PET_GUI_TEST_UPDATER='true')
111:     command=[executable]+(["--appimage-extract-and-run"] if executable.suffix==".AppImage" else [])+["--acceptance-gui"]
112:     completed = subprocess.run(command, env=env, cwd=ROOT, check=False, timeout=420 if live or options.update_package else 120)
113:     if server:server.shutdown();server.server_close()
114:     result = json.loads((directory / "report.json").read_text(encoding="utf-8"))
115:     if options.update_package and (result.get('success')or(sys.platform=='win32'and result.get('update_pending')and result.get('prerequisites_passed')and result.get('before_snapshot'))):
116:         deadline=time.monotonic()+180
117:         cold_command=[executable]+(['--appimage-extract-and-run']if executable.suffix=='.AppImage'else[])+['--version']
118:         actual_updated_hash=None;cold_version=None;installer_activity=[]
119:         if sys.platform=='win32':
120:             query="Get-CimInstance Win32_Process | Where-Object { $_.ExecutablePath -like '*-updater-*\\*-installer.exe' } | Select-Object Name,ProcessId,CommandLine | ConvertTo-Json -Compress"
121:             inspected=subprocess.run([permission_shell,'-NoProfile','-NonInteractive','-Command',query],env=permission_env,capture_output=True,text=True,encoding='utf-8',timeout=30)
122:             if inspected.returncode==0 and inspected.stdout.strip():installer_activity=json.loads(inspected.stdout)
123:             print('Isolated NSIS activity: '+json.dumps(installer_activity),flush=True)
124:         while True:
125:             try:
126:                 actual_updated_hash=hashlib.sha256(executable.read_bytes()).hexdigest()
127:                 # Avoid launching the old executable while NSIS checks running
128:                 # processes and replaces it. Cold-start only the expected bytes.
129:                 if actual_updated_hash==expected_updated_hash:
130:                     cold_version=subprocess.check_output(cold_command,env=env,cwd=ROOT,timeout=30,text=True,encoding='utf-8').strip()
131:                     if cold_version=='0.1.1':break
132:             except (OSError,subprocess.SubprocessError):pass
133:             if time.monotonic()>=deadline:
134:                 diagnostics={'error':'Signed update did not install expected bytes and cold version','expected_sha256':expected_updated_hash,'base_sha256':base_executable_hash,'actual_sha256':actual_updated_hash,'cold_version':cold_version,'platform':sys.platform,'installer_activity':installer_activity}
135:                 if sys.platform=='win32':
136:                     import winreg
137:                     diagnostics['registry']=[]
138:                     for hive in (winreg.HKEY_CURRENT_USER,winreg.HKEY_LOCAL_MACHINE):
139:                         try:
140:                             with winreg.OpenKey(hive,r'Software\Microsoft\Windows\CurrentVersion\Uninstall\Copilot Desktop Pet')as registered:
141:                                 diagnostics['registry'].append({key:winreg.QueryValueEx(registered,key)[0] for key in ('InstallLocation','DisplayVersion')})
142:                         except OSError:pass
143:                     try:diagnostics['failed_cold_probe']=subprocess.check_output(cold_command,env=env,cwd=ROOT,timeout=30,text=True,encoding='utf-8').strip()
144:                     except (OSError,subprocess.SubprocessError)as error:diagnostics['failed_cold_probe']=type(error).__name__
145:                     diagnostics['original_location_sha256']=hashlib.sha256(original_executable.read_bytes()).hexdigest() if original_executable.exists()else None
146:                 result['update_diagnostics']=diagnostics
147:                 REPORT.write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
148:                 raise SystemExit(json.dumps(diagnostics))
149:             time.sleep(1)
150:         with closing(sqlite3.connect(directory/'state/pet.sqlite'))as database:
151:             if database.execute('PRAGMA integrity_check').fetchone()[0]!='ok'or database.execute('SELECT count(*) FROM pets').fetchone()[0]<2:raise SystemExit('Updated app lost SQLite integrity or user data')
152:         manifests=list((directory/'state/backups').glob('*/manifest.json'))
153:         if not manifests:raise SystemExit('Update did not preserve pre-install snapshot')
154:         for manifest in manifests:
155:             metadata=json.loads(manifest.read_text(encoding='utf-8'));snapshot=manifest.parent/'snapshot.sqlite'
156:             if metadata['sha256']!=hashlib.sha256(snapshot.read_bytes()).hexdigest():raise SystemExit('Pre-install snapshot checksum mismatch')
157:         if result.get('before_snapshot') and not any(p.parent.name==result['before_snapshot'] for p in manifests):raise SystemExit('Actual pre-install snapshot receipt missing')
158:         result['success']=True
159:         result['update_install']={'sha256':actual_updated_hash,'cold_version':cold_version,'sqlite_integrity':'ok','pets_preserved':True,'pre_update_snapshot':True,'platform':sys.platform}
160:         result['message']='PASS: complete pre-install GUI, verified signature, actual update bytes, cold 0.1.1, SQLite and snapshot preservation'
161:     if result.get('success'):
162:         with closing(sqlite3.connect(directory/'state/pet.sqlite')) as database:rows=database.execute('SELECT id,profile FROM pets').fetchall()
163:         for identity,profile in rows:
164:             pet_directory=directory/'state/pets'/identity
165:             actual=tomllib.loads((pet_directory/'pet.toml').read_text(encoding="utf-8"));expected=json.loads(profile)
166:             for field in ['名字','種類','喜好','個性','興趣']:
167:                 if actual[field]!=expected[field]:raise SystemExit('Per-pet configuration differs from canonical state')
168:             if json.loads((pet_directory/'profile.manifest.json').read_text(encoding="utf-8"))['pet_id']!=identity:raise SystemExit('Per-pet identity manifest mismatch')
169:         result['per_pet_files']={'profiles':len(rows),'canonical_fields_match':True,'identity_manifests':True}
170:         if options.live_image:
171:             png_files=list((directory/'state/pets/8df47580-64e1-4335-8e50-6b593051a398/assets/generated').glob('*/sheet.png'))
172:             if not png_files or not all(p.read_bytes().startswith(b'\x89PNG\r\n\x1a\n') for p in png_files):raise SystemExit('Generated sprite was not persisted to per-pet assets')
173:             result['per_pet_files']['generated_png']=True
174:     REPORT.parent.mkdir(parents=True, exist_ok=True)
175:     REPORT.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
176:     if completed.returncode != 0 or result.get("success") is not True:
177:         raise SystemExit(result)
178:     print(result["message"])
```

## copilot_use/scripts/postgres-init.sh

SHA256 `91cf8245bf3539e4f3818e0b4c1f3cf2345f98501e337c72d892e3e2719094db`。

```
1: #!/usr/bin/env bash
2: set -euo pipefail
3: # The container bootstrap owner is never an application login.
4: psql --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" \
5:   --set ON_ERROR_STOP=1 --set app_user="$PG_APP_USER" \
6:   --set app_password="$PG_APP_PASSWORD" --set app_db="$POSTGRES_DB" <<'SQL'
7: CREATE ROLE :"app_user" LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE PASSWORD :'app_password';
8: ALTER DATABASE :"app_db" OWNER TO :"app_user";
9: REVOKE CONNECT ON DATABASE postgres FROM PUBLIC;
10: SQL
11: if [ "${PG_ENABLE_VECTOR:-false}" = true ]; then
12:   psql --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" --set ON_ERROR_STOP=1 -c 'CREATE EXTENSION vector'
13: fi
```

## copilot_use/scripts/realtime-e2e.mjs

SHA256 `e837a1670554446bd01571d251a3ec948143a37517fcaa0bb4b9f53f2cb790f3`。

```
1: // Node 22+ 原生 WebSocket：兩客戶端、房間隔離、認證、typing、history 恢復。
2: import assert from 'node:assert/strict';
3: const base = process.env.CHAT_URL || 'http://127.0.0.1:3001';
4: const token = process.env.CHAT_API_TOKEN;
5: const suffix = crypto.randomUUID();
6: const rooms = [`ws-${suffix.slice(0,20)}`, `isolated-${suffix.slice(0,20)}`];
7: const api = async (path, body) => {
8:   const r = await fetch(base + path, {method: body ? 'POST' : 'GET',
9:     headers: {Authorization: `Bearer ${token}`, 'Content-Type':'application/json'},
10:     body: body ? JSON.stringify(body) : undefined});
11:   assert.ok(r.ok, `HTTP ${r.status}`);
12:   return r.status === 201 && path.endsWith('/rooms') ? null : r.json();
13: };
14: for (const id of rooms) await api('/api/v1/rooms', {id, name:id});
15: const clients=[];
16: async function connect(room, credential=token, endpoint=base) {
17:   const ws = new WebSocket(endpoint.replace(/^http/, 'ws') + '/api/v1/ws/' + room);
18:   const events=[];
19:   ws.addEventListener('message', e => events.push(JSON.parse(e.data)));
20:   clients.push(ws);
21:   await new Promise((resolve,reject)=>{
22:     const timer=setTimeout(()=>reject(Error('WS connect timeout')),5000);
23:     ws.addEventListener('open',()=>{clearTimeout(timer);ws.send(JSON.stringify({token:credential}));resolve()}, {once:true});
24:     ws.addEventListener('error',()=>{clearTimeout(timer);reject(Error('WS error'))},{once:true});
25:   });
26:   return {ws,events};
27: }
28: async function until(predicate) {
29:   const limit=Date.now()+6000;
30:   while(Date.now()<limit){if(predicate())return;await new Promise(r=>setTimeout(r,30));}
31:   throw Error('WS event timeout');
32: }
33: try {
34:   const denied=await connect(rooms[0],'invalid');
35:   await until(()=>denied.events.some(e=>e.code==='unauthorized'));
36:   assert.ok(!denied.events.some(e=>e.type==='ready'));
37:   const a=await connect(rooms[0]), b=await connect(rooms[0],token,process.env.CHAT_URL_SECOND||base), other=await connect(rooms[1]);
38:   await until(()=>[a,b,other].every(c=>c.events.some(e=>e.type==='ready')));
39:   assert.ok(b.events.find(e=>e.type==='ready').presence.length>=2,'presence snapshot');
40:   a.ws.send('{"type":"typing"}');
41:   await until(()=>b.events.some(e=>e.type==='typing'));
42:   const saved=await api('/api/v1/messages',{room_id:rooms[0],content:'WS integration test'});
43:   await until(()=>[a,b].every(c=>c.events.some(e=>e.type==='message' && e.id===saved.id)));
44:   assert.ok(!other.events.some(e=>e.type==='message' && e.id===saved.id));
45:   b.ws.close();
46:   const missed=await api('/api/v1/messages',{room_id:rooms[0],content:'missed while disconnected'});
47:   const back=await connect(rooms[0]);
48:   await until(()=>back.events.some(e=>e.type==='ready'));
49:   const history=await api('/api/v1/messages?room_id='+rooms[0]);
50:   assert.ok(history.some(m=>m.id===missed.id));
51:   console.log('PASS: authenticated WS, presence snapshot, cross-process clients, room isolation, typing, reconnect/history');
52: } finally {for(const ws of clients) ws.close();}
```

## copilot_use/scripts/realtime-load-e2e.mjs

SHA256 `a9128b1bcf4af266f1af2baa120016c9b8ac48302d58abb270dfb55f73c1e84b`。

```
1: // Real bounded sockets and HTTP load against five isolated Chat processes.
2: import assert from 'node:assert/strict';import net from 'node:net';import crypto from 'node:crypto';import fs from 'node:fs';
3: const urls=JSON.parse(process.env.CHAT_LOAD_URLS);assert.equal(urls.length,5);const sockets=[];
4: async function until(test,seconds=30){const deadline=Date.now()+seconds*1000;while(!await test()){if(Date.now()>deadline)throw Error('bounded load deadline exceeded');await new Promise(r=>setTimeout(r,20));}}
5: async function api(base,path,cookie,body){return fetch(new URL(path,base),{method:body?'POST':'GET',headers:{'Content-Type':'application/json','X-Copilot-CSRF':'1',Cookie:cookie||''},body:body?JSON.stringify(body):undefined});}
6: async function connect(base,room,cookie){
7:  const url=new URL(base),socket=net.connect(Number(url.port),url.hostname),client={socket,messages:[],closed:false,status:0,error:null};sockets.push(client);let data=Buffer.alloc(0),upgraded=false;
8:  socket.on('close',()=>client.closed=true);socket.on('error',e=>client.error=e);
9:  socket.on('data',chunk=>{try{data=Buffer.concat([data,chunk]);assert.ok(data.length<1048576);
10:   if(!upgraded){const end=data.indexOf('\r\n\r\n');if(end<0)return;client.status=Number(data.subarray(0,end).toString().split(' ')[1]);if(client.status!==101){socket.end();return}data=data.subarray(end+4);upgraded=true;const mask=crypto.randomBytes(4),body=Buffer.from('{}');for(let i=0;i<body.length;i++)body[i]^=mask[i%4];socket.write(Buffer.concat([Buffer.from([0x81,0x80|body.length]),mask,body]));}
11:   while(data.length>=2){let length=data[1]&127,offset=2;if(length===126){if(data.length<4)return;length=data.readUInt16BE(2);offset=4;}if(length===127){if(data.length<10)return;length=Number(data.readBigUInt64BE(2));offset=10;}assert.ok(length<=262144);if(data.length<offset+length)return;const opcode=data[0]&15,body=data.subarray(offset,offset+length);data=data.subarray(offset+length);if(opcode===1)client.messages.push(JSON.parse(body));if(opcode===8)socket.end();}
12:  }catch(error){client.error=error;socket.destroy();}});
13:  await new Promise(resolve=>socket.once('connect',resolve));socket.write(`GET /api/v1/ws/${room} HTTP/1.1\r\nHost: ${url.host}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: ${crypto.randomBytes(16).toString('base64')}\r\nCookie: ${cookie}\r\nOrigin: ${url.origin}\r\n\r\n`);
14:  await until(()=>client.error||client.status&&client.status!==101||client.messages.some(m=>m.type==='ready'||m.type==='error'));if(client.error)throw client.error;return client;
15: }
16: const ready=c=>c.messages.some(m=>m.type==='ready');const denied=c=>c.messages.some(m=>m.code==='connection_quota');const users=[],opened=[];const started=performance.now();
17: try{
18:  for(let i=0;i<17;i++){const credentials={username:'load_'+crypto.randomUUID().slice(0,12),password:'synthetic-load-password-'+crypto.randomUUID()};assert.equal((await api(urls[0],'/api/v1/auth/register',null,credentials)).status,201);const response=await api(urls[0],'/api/v1/auth/login',null,credentials);assert.equal(response.status,200);users.push(response.headers.get('set-cookie').split(';')[0]);}
19:  for(let user=0;user<16;user++){for(let j=0;j<32;j++){const room=crypto.randomUUID(),server=j%4;assert.equal((await api(urls[server],'/api/v1/rooms',users[user],{id:room,name:'Synthetic bounded load room'})).status,201);const client=await connect(urls[server],room,users[user]);assert.ok(ready(client));opened.push({client,room,user,server});}}
20:  assert.ok(opened.every(({client})=>!client.closed&&!client.error));
21:  const extra=crypto.randomUUID();assert.equal((await api(urls[4],'/api/v1/rooms',users[16],{id:extra,name:'Synthetic quota boundary room'})).status,201);
22:  const global=await connect(urls[4],extra,users[16]);assert.ok(denied(global)&&!ready(global));
23:  const released=opened.find(x=>x.user===1&&x.server===1);released.client.socket.destroy();await until(async()=>{const r=await api(urls[1],`/api/v1/rooms/${released.room}/presence`,users[1]);return r.status===200&&(await r.json()).length===0;});
24:  const perUser=await connect(urls[4],opened[0].room,users[0]);assert.ok(denied(perUser)&&!ready(perUser));
25:  const perProcess=await connect(urls[0],extra,users[16]);assert.equal(perProcess.status,429);
26:  const restored=await connect(urls[1],released.room,users[1]);assert.ok(ready(restored));
27:  console.log('PASS: 512 real sockets across four processes; fifth-process global, per-user and per-process boundaries reject excess');
28:  for(const socket of sockets)socket.socket.destroy();await until(()=>sockets.every(s=>s.closed));
29:  const subscribers=[];for(let i=0;i<32;i++){const c=await connect(urls[i%4],extra,users[16]);assert.ok(ready(c));subscribers.push(c);}
30:  const ids=Array.from({length:64},()=>crypto.randomUUID());const latency=[];
31:  await Promise.all(ids.map(async(id,i)=>{const start=performance.now();const r=await api(urls[i%4],'/api/v1/messages',users[16],{id,room_id:extra,content:'Synthetic bounded fan-out fixture '+i});assert.equal(r.status,201);latency.push(performance.now()-start);}));
32:  await until(()=>subscribers.every(s=>ids.every(id=>s.messages.some(m=>m.id===id))),60);
33:  for(const s of subscribers){for(const id of ids)assert.equal(s.messages.filter(m=>m.id===id).length,1);}
34:  const history=await(await api(urls[0],`/api/v1/messages?room_id=${extra}`,users[16])).json();assert.equal(history.length,64);assert.equal(new Set(history.map(m=>m.id)).size,64);
35:  const analytics=process.env.ANALYTICS_URL;const pending=new Set(ids);
36:  await until(async()=>{for(const id of [...pending]){const response=await fetch(new URL(`/api/v1/events/web_chat/${id}`,analytics),{headers:{Authorization:'Bearer '+process.env.ANALYTICS_API_TOKEN}});if(response.status===200)pending.delete(id);else assert.equal(response.status,404);}return pending.size===0;},60);
37:  latency.sort((a,b)=>a-b);const report={status:'passed',peak_sockets:512,processes:5,fanout_subscribers:32,concurrent_messages:64,exactly_once_observed:true,p50_ms:latency[31],p95_ms:latency[60],max_ms:latency.at(-1),elapsed_ms:performance.now()-started,scope:'local synthetic bounded admission, fan-out, persistence and exact Analytics IDs'};
38:  fs.writeFileSync(process.env.CHAT_LOAD_REPORT,JSON.stringify(report,null,2)+'\n');console.log('PASS: 64 concurrent private writes delivered once to all 32 sockets, durable history and Analytics IDs');console.log(JSON.stringify(report));
39: }finally{for(const s of sockets)s.socket.destroy();}
```

## copilot_use/scripts/release-approved.py

SHA256 `8b5fec162dd445c7f04a35f2aa3236126162e9a0a259daa038ad8d8c55c53357`。

```
1: #!/usr/bin/env python3
2: """Publish only an explicitly approved, tested immutable source; no public deployment."""
3: import hashlib,json,os,pathlib,re,subprocess,tempfile,urllib.request,urllib.error,urllib.parse
4: ROOT=pathlib.Path(__file__).resolve().parents[1]
5: repository=os.environ['GH_REPOSITORY']
6: if repository!='williechen/copilot_use':raise SystemExit('Release is scoped to williechen/copilot_use')
7: approval=json.loads((ROOT/'.github/release-approved.json').read_text())
8: sha=approval['approved_sha'];tag=approval['tag']
9: if not re.fullmatch('[0-9a-f]{40}',sha)or not re.fullmatch(r'v\d+\.\d+\.\d+(?:-p9)?',tag):raise SystemExit('Invalid immutable source/tag')
10: if subprocess.check_output(['git','rev-parse','HEAD^'],cwd=ROOT,text=True).strip()!=sha:raise SystemExit('Approval must follow the exact tested commit')
11: class NoRedirect(urllib.request.HTTPRedirectHandler):
12:     def redirect_request(self,*args):return None
13: client=urllib.request.build_opener(NoRedirect())
14: def request(url,data=None,method=None):
15:     headers={'Authorization':'Bearer '+os.environ['GH_TOKEN'],'Accept':'application/vnd.github+json','X-GitHub-Api-Version':'2022-11-28'}
16:     body=None if data is None else json.dumps(data).encode()
17:     return client.open(urllib.request.Request(url,data=body,headers=headers,method=method),timeout=120)
18: def api(path,data=None,method=None):
19:     with request('https://api.github.com/repos/'+repository+path,data,method)as response:return json.load(response)
20: def optional(path):
21:     try:return api(path)
22:     except urllib.error.HTTPError as error:
23:         if error.code==404:return None
24:         raise
25: required={'live','docker','packaged_gui','appimage_update'}
26: if set(approval['evidence'])!=required:raise SystemExit('Approval must contain precisely the four required reports')
27: for kind in sorted(required):
28:     evidence=(ROOT/approval['evidence'][kind]).resolve()
29:     if not evidence.is_relative_to(ROOT/'docs/release-evidence')or evidence.suffix!='.json':raise SystemExit('Evidence must be checked-in release JSON')
30:     report=json.loads((ROOT/approval['evidence'][kind]).read_text())
31:     actual=report.get('source_sha',report.get('source_head'))
32:     dirty=report.get('source_dirty',report.get('working_tree_dirty',True))
33:     passed=report.get('status')=='passed'or report.get('success')is True
34:     if actual!=sha or dirty is not False or not passed:raise SystemExit('Local evidence does not prove exact clean source: '+kind)
35:     if kind=='docker'and (set(report.get('containers',{}))!={'chat','analytics'}or any(c.get('revision')!=sha or not re.fullmatch('sha256:[0-9a-f]{64}',c.get('image_id',''))for c in report['containers'].values())):raise SystemExit('Actual Docker image revisions do not match approved source')
36:     if kind=='live'and(report.get('live_openai')is not True or report.get('openai_model')!='gpt-5.6-luna'):raise SystemExit('Real configured-model validation missing')
37:     if kind in ('packaged_gui','appimage_update')and report.get('release')is not True:raise SystemExit('Packaged release validation missing')
38:     if kind=='appimage_update'and not report.get('update_install'):raise SystemExit('Actual signed update installation missing')
39: runs=api('/actions/runs?head_sha='+sha+'&per_page=100')['workflow_runs']
40: matching=[r for r in runs if r['path']=='.github/workflows/verify.yml'and r['head_sha']==sha]
41: if not matching or matching[0]['status']!='completed'or matching[0]['conclusion']!='success':raise SystemExit('Exact-source verification workflow has not passed')
42: run=matching[0]
43: jobs=api('/actions/runs/'+str(run['id'])+'/jobs?per_page=100')['jobs']
44: expected={'rust-postgres','compose-e2e','local-fault-recovery','desktop (ubuntu-latest, deb,appimage)','desktop (macos-latest, app)','desktop (windows-latest, nsis)'}
45: if {j['name']for j in jobs}!=expected or any(j['conclusion']!='success'for j in jobs):raise SystemExit('All six required jobs must pass; skipped jobs are not success')
46: artifacts=api('/actions/runs/'+str(run['id'])+'/artifacts?per_page=100')['artifacts']
47: selected=[a for a in artifacts if a['name'].startswith('desktop-')and not a['expired']]
48: if len(selected)!=3:raise SystemExit('All three actual platform artifacts are required')
49: with tempfile.TemporaryDirectory(prefix='verified-p9-')as scratch:
50:     files=[]
51:     for artifact in selected:
52:         try:
53:             request('https://api.github.com/repos/'+repository+'/actions/artifacts/'+str(artifact['id'])+'/zip')
54:             raise SystemExit('Expected artifact download redirect')
55:         except urllib.error.HTTPError as redirect:
56:             if redirect.code!=302:raise
57:             location=redirect.headers['Location']
58:         parsed=urllib.parse.urlsplit(location)
59:         if parsed.scheme!='https'or parsed.username or parsed.password:raise SystemExit('Unsafe artifact redirect')
60:         file=pathlib.Path(scratch)/(artifact['name']+'.zip');total=0
61:         # Presigned storage request never receives the GitHub credential header.
62:         with urllib.request.urlopen(location,timeout=120)as response,file.open('wb')as output:
63:             while chunk:=response.read(65536):
64:                 total+=len(chunk)
65:                 if total>512*1024*1024:raise SystemExit('Artifact exceeds release bound')
66:                 output.write(chunk)
67:         files.append(file)
68:     for value in approval['evidence'].values():files.append(ROOT/value)
69:     manifest={'source_sha':sha,'tag':tag,'ci_run':run['html_url'],'files':{p.name:hashlib.sha256(p.read_bytes()).hexdigest()for p in files}}
70:     manifest_file=pathlib.Path(scratch)/'release-manifest.json';manifest_file.write_text(json.dumps(manifest,indent=2)+'\n');files.append(manifest_file)
71:     ref=optional('/git/ref/tags/'+tag)
72:     if ref and ref['object']['sha']!=sha:raise SystemExit('Existing tag differs; immutable tags are never overwritten')
73:     if not ref:api('/git/refs',{'ref':'refs/tags/'+tag,'sha':sha})
74:     release=optional('/releases/tags/'+tag)
75:     body=approval['notes']+'\n\nSource: `'+sha+'`\n\nCI: '+run['html_url']
76:     if release and (release['body']!=body or not release['draft']):raise SystemExit('Existing published or unrelated release is immutable')
77:     if not release:release=api('/releases',{'tag_name':tag,'target_commitish':sha,'name':tag+' — local P9 validation','body':body,'draft':True,'prerelease':False})
78:     upload_base=release['upload_url'].split('{')[0]
79:     if urllib.parse.urlsplit(upload_base).hostname!='uploads.github.com':raise SystemExit('Unexpected asset upload host')
80:     existing={asset['name']:asset for asset in release['assets']}
81:     for file in files:
82:         if file.name in existing:
83:             expected='sha256:'+hashlib.sha256(file.read_bytes()).hexdigest()
84:             if existing[file.name].get('digest')!=expected:raise SystemExit('Existing draft asset digest differs; refusing overwrite')
85:             continue
86:         upload=upload_base+'?name='+urllib.parse.quote(file.name,safe='')
87:         headers={'Authorization':'Bearer '+os.environ['GH_TOKEN'],'Content-Type':'application/octet-stream','Accept':'application/vnd.github+json'}
88:         with client.open(urllib.request.Request(upload,data=file.read_bytes(),headers=headers),timeout=180)as response:json.load(response)
89:     release=api('/releases/'+str(release['id']),{'draft':False},method='PATCH')
90:     print(json.dumps({'status':'published','tag':tag,'source_sha':sha,'url':release['html_url']}))
```

## copilot_use/scripts/session-ws-e2e.mjs

SHA256 `fb42fb61d3a60504dcec176ad003ac23a9ee493388d5d67bac25bd0cba24fbf7`。

```
1: // Actual RFC6455 cookie clients on two processes: shared quota and revoked access.
2: import assert from 'node:assert/strict';
3: import net from 'node:net';
4: import crypto from 'node:crypto';
5: const base=new URL(process.env.CHAT_URL);assert.equal(base.protocol,'http:');
6: const token=process.env.CHAT_API_TOKEN;const name='expiry-'+crypto.randomUUID().slice(0,12);const room=name;
7: const call=async(path,body,credential)=>fetch(new URL(path,base),{method:body?'POST':'GET',headers:{'Content-Type':'application/json','X-Copilot-CSRF':'1',...(credential==='admin'?{Authorization:'Bearer '+token}:{Cookie:credential||''})},body:body?JSON.stringify(body):undefined});
8: const credentials={username:name,password:'isolated-test-password-'+crypto.randomUUID()};
9: assert.equal((await call('/api/v1/auth/register',credentials)).status,201);
10: const login=await call('/api/v1/auth/login',credentials);assert.equal(login.status,200);
11: const cookie=login.headers.get('set-cookie').split(';')[0];
12: assert.equal((await call('/api/v1/rooms',{id:room,name:room},cookie)).status,201);
13: const clients=[];
14: const until=async fn=>{const limit=Date.now()+10000;while(!fn()){if(Date.now()>limit)throw Error('session socket timed out');await new Promise(r=>setTimeout(r,25));}};
15: async function connect(endpoint){
16:  const target=new URL(endpoint);const socket=net.connect(Number(target.port),target.hostname);let frames=Buffer.alloc(0),upgraded=false;const client={socket,messages:[],closed:false};clients.push(client);
17:  socket.on('close',()=>client.closed=true);socket.on('error',e=>{throw e});
18:  socket.on('data',part=>{
19:   frames=Buffer.concat([frames,part]);
20:   if(!upgraded){const end=frames.indexOf('\r\n\r\n');if(end<0)return;assert.ok(frames.subarray(0,end).toString().startsWith('HTTP/1.1 101'));frames=frames.subarray(end+4);upgraded=true;
21:    const mask=crypto.randomBytes(4),payload=Buffer.from('{}');for(let i=0;i<payload.length;i++)payload[i]^=mask[i%4];socket.write(Buffer.concat([Buffer.from([0x81,0x80|payload.length]),mask,payload]));
22:   }
23:   while(frames.length>=2){let len=frames[1]&127,offset=2;if(len===126){if(frames.length<4)return;len=frames.readUInt16BE(2);offset=4;}assert.notEqual(len,127,'test frame must be bounded');if(frames.length<offset+len)return;
24:    const opcode=frames[0]&15,payload=frames.subarray(offset,offset+len);frames=frames.subarray(offset+len);
25:    if(opcode===1)client.messages.push(JSON.parse(payload.toString()));if(opcode===8)socket.end();
26:   }
27:  });
28:  await new Promise(resolve=>socket.once('connect',resolve));
29:  socket.write(`GET /api/v1/ws/${room} HTTP/1.1\r\nHost: ${target.host}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: ${crypto.randomBytes(16).toString('base64')}\r\nCookie: ${cookie}\r\nOrigin: ${target.origin}\r\n\r\n`);
30:  await until(()=>client.messages.some(x=>x.type==='ready'||x.type==='error'));return client;
31: }
32: try{
33:  for(let i=0;i<32;i++){const c=await connect(i%2?(process.env.CHAT_URL_SECOND||base):base);assert.ok(c.messages.some(x=>x.type==='ready'));}
34:  const denied=await connect(process.env.CHAT_URL_SECOND||base);assert.ok(denied.messages.some(x=>x.code==='connection_quota'));assert.ok(!denied.messages.some(x=>x.type==='ready'));
35:  console.log('PASS: 32 per-user sockets admitted and 33rd rejected across independent processes');
36:  assert.equal((await call('/api/v1/auth/logout',{},cookie)).status,204);
37:  const saved=await (await call('/api/v1/messages',{room_id:room,content:'must not reach revoked session'},'admin')).json();
38:  await until(()=>clients.every(c=>c.closed));assert.ok(clients.every(c=>!c.messages.some(x=>x.id===saved.id)));
39:  console.log('PASS: all real cookie WebSockets close after logout without receiving subsequent private messages');
40: }finally{for(const c of clients)c.socket.destroy();}
```

## copilot_use/scripts/smoke.sh

SHA256 `358aaa36d4c4eb7238faecfe544323789f832c12bb0616cceef82357b7a6682e`。

```
1: #!/usr/bin/env bash
2: # Local or CI only: requires Docker Compose services already started and safe test credentials.
3: set -euo pipefail
4: : "${CHAT_API_TOKEN:?missing CHAT_API_TOKEN}"
5: : "${ANALYTICS_API_TOKEN:?missing ANALYTICS_API_TOKEN}"
6: CHAT_URL="${CHAT_URL:-http://127.0.0.1:3001}"
7: ANALYTICS_URL="${ANALYTICS_URL:-http://127.0.0.1:3100}"
8: for _ in $(seq 1 60); do
9:   if curl -fsS "${CHAT_URL}/healthz" >/dev/null &&
10:      curl -fsS "${ANALYTICS_URL}/healthz" >/dev/null; then break; fi
11:   sleep 2
12: done
13: curl -fsS "${CHAT_URL}/healthz" >/dev/null
14: curl -fsS "${ANALYTICS_URL}/healthz" >/dev/null
15: ROOM_ID="smoke-$(date +%s)-$RANDOM"
16: curl -fsS -X POST "${CHAT_URL}/api/v1/rooms" \
17:   -H "Authorization: Bearer ${CHAT_API_TOKEN}" -H "Content-Type: application/json" \
18:   -d "{\"id\":\"${ROOM_ID}\",\"name\":\"smoke-room\"}" >/dev/null
19: MESSAGE_ID=$(curl -fsS -X POST "${CHAT_URL}/api/v1/messages" \
20:   -H "Authorization: Bearer ${CHAT_API_TOKEN}" -H "Content-Type: application/json" \
21:   -d "{\"room_id\":\"${ROOM_ID}\",\"content\":\"smoke message\"}" | python3 -c 'import json,sys; print(json.load(sys.stdin)["id"])')
22: curl -fsS "${CHAT_URL}/api/v1/messages?room_id=${ROOM_ID}" \
23:   -H "Authorization: Bearer ${CHAT_API_TOKEN}" | python3 -c '
24: import json,sys
25: values=json.load(sys.stdin)
26: assert any(x["content"]=="smoke message" for x in values)
27: '
28: # 本次訊息的唯一 ID 必須抵達；不能由舊的 summary 計數代替。
29: for _ in $(seq 1 30); do
30:   if curl -fsS "${ANALYTICS_URL}/api/v1/events/web_chat/${MESSAGE_ID}" \
31:       -H "Authorization: Bearer ${ANALYTICS_API_TOKEN}" >/dev/null 2>&1; then
32:     echo "PASS: this Chat message -> transactional outbox -> Analytics ingest"
33:     exit 0
34:   fi
35:   sleep 2
36: done
37: echo "FAIL: this durable Chat event not observed" >&2
38: exit 1
```

## copilot_use/scripts/tests/release-approval.py

SHA256 `1ef6118d6b1ec20aa0b30a789561867898e277b66f30d1b0f6a5db197fcfac51`。

```
1: #!/usr/bin/env python3
2: """Ensure incomplete local evidence cannot reach any network request or tag mutation."""
3: import json,os,pathlib,shutil,subprocess,tempfile,unittest
4: ROOT=pathlib.Path(__file__).resolve().parents[2]
5: class ReleaseGate(unittest.TestCase):
6:     def test_rejects_missing_real_clean_packaged_update_evidence_before_network(self):
7:         with tempfile.TemporaryDirectory()as scratch:
8:             root=pathlib.Path(scratch);(root/'scripts').mkdir();(root/'.github').mkdir();(root/'docs/release-evidence').mkdir(parents=True)
9:             shutil.copyfile(ROOT/'scripts/release-approved.py',root/'scripts/release-approved.py')
10:             (root/'sitecustomize.py').write_text("import urllib.request\ndef deny(*args,**kwargs):raise AssertionError('NETWORK_TOUCHED')\nurllib.request.OpenerDirector.open=deny\n")
11:             def git(*args):return subprocess.check_output(['git',*args],cwd=root,stderr=subprocess.DEVNULL,text=True).strip()
12:             git('init','-q');(root/'source').write_text('fixture');git('add','source');git('-c','user.name=Test','-c','user.email=test@example.invalid','-c','commit.gpgsign=false','commit','-qm','source');sha=git('rev-parse','HEAD')
13:             (root/'source').write_text('approval');git('add','source');git('-c','user.name=Test','-c','user.email=test@example.invalid','-c','commit.gpgsign=false','commit','-qm','approval')
14:             evidence={k:'docs/release-evidence/'+k+'.json'for k in ('live','docker','packaged_gui','appimage_update')}
15:             approval={'approved_sha':sha,'tag':'v0.1.0-p9','evidence':evidence,'notes':'Disposable gate regression'}
16:             (root/'.github/release-approved.json').write_text(json.dumps(approval))
17:             cases=[('docker',{'containers':{'chat':{'revision':'0'*40,'image_id':'sha256:'+'1'*64},'analytics':{'revision':sha,'image_id':'sha256:'+'1'*64}}},'Actual Docker image revisions'),('live',{'source_sha':'0'*40},'exact clean source'),('live',{'source_dirty':True},'exact clean source'),('live',{'live_openai':False},'Real configured-model'),('packaged_gui',{'release':False},'Packaged release'),('appimage_update',{'update_install':False},'Actual signed update')]
18:             for kind,changes,expected in cases:
19:                 with self.subTest(kind=kind,changes=changes):
20:                     for name,path in evidence.items():
21:                         report={'source_sha':sha,'source_dirty':False,'status':'passed','success':True,'live_openai':True,'openai_model':'gpt-5.6-luna','release':True,'update_install':{'fixture':True},'containers':{name:{'revision':sha,'image_id':'sha256:'+'1'*64}for name in ('chat','analytics')}}
22:                         if name==kind:report.update(changes)
23:                         (root/path).write_text(json.dumps(report))
24:                     result=subprocess.run(['python3',root/'scripts/release-approved.py'],cwd=root,env=os.environ|{'GH_REPOSITORY':'williechen/copilot_use','GH_TOKEN':'synthetic-unused','PYTHONPATH':str(root)},text=True,capture_output=True)
25:                     self.assertNotEqual(result.returncode,0);self.assertIn(expected,result.stderr);self.assertNotIn('NETWORK_TOUCHED',result.stderr)
26: if __name__=='__main__':unittest.main()
```

## copilot_use/scripts/topic-consent-e2e.mjs

SHA256 `63d4569dcf1ac7e1b4b2e91e919f3920263b4ae1fb696a3ee16e09aec0477567`。

```
1: // Real cookie HTTP and durable cross-service checks using disposable synthetic users only.
2: import {randomUUID} from 'node:crypto';
3: import {readFile,writeFile} from 'node:fs/promises';
4: const chat=process.env.CHAT_URL,analytics=process.env.ANALYTICS_URL,path=process.env.VERIFY_TOPIC_FIXTURE;
5: for(const base of [chat,analytics]){const u=new URL(base);if(!['127.0.0.1','localhost','[::1]'].includes(u.hostname))throw Error('isolated localhost fixture required');}
6: const assert=(ok,label)=>{if(!ok)throw Error(label);};
7: async function request(base,path,body,headers={}){const r=await fetch(base+path,{method:body===undefined?'GET':'POST',redirect:'error',headers:{'Content-Type':'application/json','X-Copilot-Csrf':'1',...headers},body:body===undefined?undefined:JSON.stringify(body)});return{status:r.status,value:await r.json().catch(()=>null),headers:r.headers};}
8: async function wait(test){const limit=Date.now()+20000;while(!await test()){if(Date.now()>limit)throw Error('topic condition timeout');await new Promise(r=>setTimeout(r,100));}}
9: const summary=async()=>{const r=await request(analytics,'/api/v1/events/summary',undefined,{Authorization:'Bearer '+process.env.ANALYTICS_API_TOKEN});assert(r.status===200,'summary authenticated');return Object.fromEntries(r.value.filter(v=>v.source_app==='web_chat'&&v.event_type.startsWith('keyword.')).map(v=>[v.event_type,v.total]));};
10: if(process.argv.includes('--withdraw')){
11:  const f=JSON.parse(await readFile(path,'utf8'));const r=await request(chat,`/api/v1/rooms/${f.room}/topic-consent`,{enabled:false},{Cookie:f.cookie});assert(r.status===200&&!r.value.enabled&&r.value.withdrawal_pending>0,'offline Analytics withdrawal remains pending durably');console.log('PASS: consent disabled while Analytics is offline; exact prior topic IDs await acknowledgement');
12: }else if(process.argv.includes('--verify-withdrawal')){
13:  const f=JSON.parse(await readFile(path,'utf8'));await wait(async()=>{const r=await request(chat,`/api/v1/rooms/${f.room}/topic-consent`,undefined,{Cookie:f.cookie});return r.status===200&&!r.value.enabled&&r.value.withdrawal_pending===0;});
14:  const totals=await summary();for(const key of ['keyword.rust','keyword.ownership'])assert((totals[key]??0)===(f.before[key]??0),'withdrawn vocabulary excluded');
15:  console.log('PASS: service recovery acknowledges withdrawal and removes exact opted-in vocabulary counts');
16: }else{
17:  async function user(){const username='topic_'+randomUUID().replaceAll('-','').slice(0,20),password='fixture-'+randomUUID();assert((await request(chat,'/api/v1/auth/register',{username,password})).status===201,'synthetic user registered');const login=await request(chat,'/api/v1/auth/login',{username,password});assert(login.status===200,'synthetic user logged in');return login.headers.get('set-cookie').split(';')[0];}
18:  const cookie=await user(),foreign=await user(),room='topics-'+randomUUID().replaceAll('-','').slice(0,24);
19:  assert((await request(chat,'/api/v1/rooms',{id:room,name:'Synthetic topic consent fixture'},{Cookie:cookie})).status===201,'owned room created');
20:  const endpoint=`/api/v1/rooms/${room}/topic-consent`,initial=await request(chat,endpoint,undefined,{Cookie:cookie});assert(initial.status===200&&initial.value.enabled===false&&initial.headers.get('cache-control')==='no-store','explicit default-off private consent');
21:  assert([403,404].includes((await request(chat,endpoint,{enabled:true},{Cookie:foreign})).status),'foreign room cannot grant consent');
22:  const before=await summary();
23:  async function send(content,id=randomUUID()){const body={id,room_id:room,content};assert([200,201].includes((await request(chat,'/api/v1/messages',body,{Cookie:cookie})).status),'message persisted');await wait(async()=>(await request(analytics,'/api/v1/events/web_chat/'+id,undefined,{Authorization:'Bearer '+process.env.ANALYTICS_API_TOKEN})).status===200);return body;}
24:  await send('Rust rust ownership');assert(JSON.stringify(await summary())===JSON.stringify(before),'no counting before consent');
25:  assert((await request(chat,endpoint,{enabled:true},{Cookie:cookie})).value.enabled===true,'explicit consent saved');
26:  const body=await send('Rust rust ownership');await wait(async()=>{const totals=await summary();return(totals['keyword.rust']??0)===(before['keyword.rust']??0)+2&&(totals['keyword.ownership']??0)===(before['keyword.ownership']??0)+1;});
27:  assert((await request(chat,'/api/v1/messages',body,{Cookie:cookie})).status===200,'message retry idempotent');
28:  await send('Rust token=synthetic-secret');const totals=await summary();assert((totals['keyword.rust']??0)===(before['keyword.rust']??0)+2,'credential-like content excluded and retry does not duplicate topics');
29:  const recent=await request(analytics,'/api/v1/events/recent?source_app=web_chat&event_type=keyword.rust',undefined,{Authorization:'Bearer '+process.env.ANALYTICS_API_TOKEN});assert(recent.status===200&&recent.value.length>=2&&recent.value.length<=100&&recent.value.every(row=>row.event_type==='keyword.rust'&&row.event_id&&Number.isInteger(row.received_unix_ms)&&!('content'in row)),'real bounded anonymous event details and filter');
30:  await writeFile(path,JSON.stringify({cookie,room,before}),{mode:0o600});
31:  console.log('PASS: real default-off/own-room consent, fixed-word occurrence counts, sensitive filtering and idempotent retry');
32: }
```
