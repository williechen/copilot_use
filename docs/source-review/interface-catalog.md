# 全部自有 Rust 宣告、介面與呼叫點

來源固定於本報告 SHA。含 public/private/trait methods、DTO fields/serde attributes、常數、reexports、外部 JS interface、cfg/test。呼叫清單為語法節點，不展開巨集、不解析接收者型別／動態dispatch；跨檔案可靠接線見資料流程報告。此清單不是 compiler call graph。

## reference-rust_ai_chat/apps/desktop_pet/build.rs · P04

職責：Pet Tauri 入口。判定：40 frontend command names 對應44 backend；非參考 bridge/DTO 等同；詳列呼叫端與簽章

### L1 · function ·  main

```rust
fn main ()
```

可見性／屬性：``；``。

函式內呼叫：`tauri_build :: build`。

## reference-rust_ai_chat/apps/desktop_pet/frontend/src/bridge.rs · P05

職責：Pet Frontend。判定：前端 schema/狀態不同；保留既有功能，不能直接替换參考 component

### L1 · use ·

```rust
use js_sys :: { Function , Promise } ;
```

### L2 · use ·

```rust
use pet_domain :: { PetCommand , PetSnapshot } ;
```

### L3 · use ·

```rust
use wasm_bindgen :: { JsCast , closure :: Closure , prelude :: * } ;
```

### L4 · use ·

```rust
use wasm_bindgen_futures :: JsFuture ;
```

### L6 · foreign_interface ·

```rust
# [wasm_bindgen] extern "C" { # [wasm_bindgen (js_namespace = window , js_name = getPetState)] fn get_pet_state_js () -> Promise ; # [wasm_bindgen (js_namespace = window , js_name = sendPetCommand)] fn send_pet_command_js (command : JsValue) -> Promise ; # [wasm_bindgen (js_namespace = window , js_name = listenPetState)] fn listen_pet_state_js (callback : & Function) -> Promise ; # [wasm_bindgen (js_namespace = window , js_name = showPetContextMenu)] fn show_pet_context_menu_js (x : i32 , y : i32) -> Promise ; # [wasm_bindgen (js_namespace = window , js_name = chatWithPet)] fn chat_with_pet_js (message : & str) -> Promise ; # [wasm_bindgen (js_namespace = window , js_name = scanFilePreview)] fn scan_file_preview_js () -> Promise ; }
```

### L46 · function ·  js_error

```rust
fn js_error (value : JsValue) -> String
```

可見性／屬性：``；``。

函式內呼叫：`.unwrap_or_else`；`.as_string`；`format!`。

### L51 · function ·  get_pet_state

```rust
async fn get_pet_state () -> Result < PetSnapshot , String >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.map_err`；`JsFuture :: from`；`get_pet_state_js`；`.map_err`；`serde_wasm_bindgen :: from_value`；`.to_string`。

### L58 · function ·  send_pet_command

```rust
async fn send_pet_command (command : PetCommand) -> Result < PetSnapshot , String >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.map_err`；`serde_wasm_bindgen :: to_value`；`.to_string`；`.map_err`；`JsFuture :: from`；`send_pet_command_js`；`.map_err`；`serde_wasm_bindgen :: from_value`；`.to_string`。

### L69 · function ·  listen_pet_state

```rust
async fn listen_pet_state (mut on_snapshot : impl FnMut (PetSnapshot) + 'static ,) -> Result < () , String >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Closure :: < dyn FnMut (JsValue) > :: new`；`serde_wasm_bindgen :: from_value`；`on_snapshot`；`.map_err`；`JsFuture :: from`；`listen_pet_state_js`；`.unchecked_ref`；`.as_ref`；`.forget`；`Ok`。

### L89 · function ·  show_pet_context_menu

```rust
async fn show_pet_context_menu (x : i32 , y : i32) -> Result < () , String >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.map_err`；`JsFuture :: from`；`show_pet_context_menu_js`；`Ok`。

### L98 · function ·  chat_with_pet

```rust
async fn chat_with_pet (message : String) -> Result < String , String >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.map_err`；`JsFuture :: from`；`chat_with_pet_js`；`.ok_or_else`；`.as_string`；`.to_string`。

### L109 · function ·  scan_file_preview

```rust
async fn scan_file_preview () -> Result < JsValue , String >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.map_err`；`JsFuture :: from`；`scan_file_preview_js`。

## reference-rust_ai_chat/apps/desktop_pet/frontend/src/component/activity.rs · P05

職責：Pet Frontend。判定：前端 schema/狀態不同；保留既有功能，不能直接替换參考 component

### L1 · use ·

```rust
use gloo_timers :: callback :: { Interval , Timeout } ;
```

### L2 · use ·

```rust
use leptos :: prelude :: * ;
```

### L3 · use ·

```rust
use pet_domain :: { PetSnapshot , PetState } ;
```

### L4 · use ·

```rust
use wasm_bindgen_futures :: spawn_local ;
```

### L6 · use ·

```rust
use crate :: bridge ;
```

### L8 · enum ·  PetPose

```rust
# [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum PetPose { Idle , Blink , InteractA , InteractB , SleepA , SleepB , }
```

### L18 · function ·  project_pose

```rust
fn project_pose (state : PetState , blinking : bool , animation_phase : bool) -> PetPose
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L46 · function ·  DragHandle

```rust
fn DragHandle () -> impl IntoView
```

可見性／屬性：`pub`；`# [component]`。

函式內呼叫：`view!`。

### L65 · function ·  AnimationClock

```rust
fn AnimationClock (set_animation_phase : WriteSignal < bool >) -> impl IntoView
```

可見性／屬性：`pub`；`# [component]`。

函式內呼叫：`Interval :: new`；`.update`；`StoredValue :: new_local`；`.into_any`。

### L76 · function ·  BlinkController

```rust
fn BlinkController (snapshot : ReadSignal < Option < PetSnapshot > > , set_blinking : WriteSignal < bool > ,) -> impl IntoView
```

可見性／屬性：`pub`；`# [component]`。

函式內呼叫：`Interval :: new`；`.unwrap_or`；`.map`；`.get_untracked`；`.set`；`.set`；`.forget`；`Timeout :: new`；`.set`；`StoredValue :: new_local`；`.into_any`。

### L109 · function ·  PetStatus

```rust
fn PetStatus (snapshot : ReadSignal < Option < PetSnapshot > > , blinking : ReadSignal < bool > , animation_phase : ReadSignal < bool > ,) -> impl IntoView
```

可見性／屬性：`pub`；`# [component]`。

函式內呼叫：`.map`；`.get`；`project_pose`；`.get`；`.get`；`Signal :: derive`；`pose`；`pose`；`.unwrap_or_else`；`.map`；`.get`；`.to_string`；`.into`；`view!`。

### L180 · module ·  tests

```rust
 mod tests (inline=True)
```

### L182 · use ·

```rust
use super :: * ;
```

### L185 · function · tests idle_blink_has_priority

```rust
fn idle_blink_has_priority ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert_eq!`。

### L190 · function · tests interacting_uses_animation_phase

```rust
fn interacting_uses_animation_phase ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert_eq!`；`assert_eq!`。

### L203 · function · tests sleeping_uses_animation_phase

```rust
fn sleeping_uses_animation_phase ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert_eq!`；`assert_eq!`。

## reference-rust_ai_chat/apps/desktop_pet/frontend/src/component/chat.rs · P05

職責：Pet Frontend。判定：前端 schema/狀態不同；保留既有功能，不能直接替换參考 component

### L1 · use ·

```rust
use crate :: bridge ;
```

### L2 · use ·

```rust
use leptos :: prelude :: * ;
```

### L3 · use ·

```rust
use wasm_bindgen_futures :: spawn_local ;
```

### L6 · function ·  ChatBubble

```rust
fn ChatBubble (chat_open : ReadSignal < bool > , set_chat_open : WriteSignal < bool > , chat_input : ReadSignal < String > , set_chat_input : WriteSignal < String > , chat_loading : ReadSignal < bool > , set_chat_loading : WriteSignal < bool > , chat_response : ReadSignal < Option < String > > , set_chat_response : WriteSignal < Option < String > > , chat_error : ReadSignal < Option < String > > , set_chat_error : WriteSignal < Option < String > > ,) -> impl IntoView
```

可見性／屬性：`pub`；`# [component]`。

函式內呼叫：`.get_untracked`；`.to_owned`；`.trim`；`.get_untracked`；`.is_empty`；`.set`；`.set`；`spawn_local`；`bridge :: chat_with_pet`；`.set`；`Some`；`.set`；`String :: new`；`.set`；`Some`；`.set`；`view!`。

## reference-rust_ai_chat/apps/desktop_pet/frontend/src/component/control.rs · P05

職責：Pet Frontend。判定：前端 schema/狀態不同；保留既有功能，不能直接替换參考 component

### L1 · use ·

```rust
use crate :: bridge ;
```

### L2 · use ·

```rust
use leptos :: prelude :: * ;
```

### L3 · use ·

```rust
use pet_domain :: { PetCommand , PetSnapshot } ;
```

### L4 · use ·

```rust
use wasm_bindgen_futures :: spawn_local ;
```

### L6 · use ·

```rust
use crate :: component :: apply_snapshot ;
```

### L8 · function ·  dispatch_command

```rust
fn dispatch_command (command : PetCommand , snapshot : ReadSignal < Option < PetSnapshot > > , set_snapshot : WriteSignal < Option < PetSnapshot > > , set_error : WriteSignal < Option < String > > ,)
```

可見性／屬性：``；``。

函式內呼叫：`.set`；`spawn_local`；`bridge :: send_pet_command`；`apply_snapshot`；`.set`；`Some`。

### L30 · function ·  PetControls

```rust
fn PetControls (snapshot : ReadSignal < Option < PetSnapshot > > , set_snapshot : WriteSignal < Option < PetSnapshot > > , set_error : WriteSignal < Option < String > > ,) -> impl IntoView
```

可見性／屬性：`pub`；`# [component]`。

函式內呼叫：`view!`。

## reference-rust_ai_chat/apps/desktop_pet/frontend/src/component/mod.rs · P05

職責：Pet Frontend。判定：前端 schema/狀態不同；保留既有功能，不能直接替换參考 component

### L1 · module ·  activity

```rust
pub mod activity (inline=False)
```

### L2 · module ·  chat

```rust
pub mod chat (inline=False)
```

### L3 · module ·  control

```rust
pub mod control (inline=False)
```

### L5 · use ·

```rust
use leptos :: prelude :: * ;
```

### L6 · use ·

```rust
use pet_domain :: PetSnapshot ;
```

### L8 · use ·

```rust
pub use activity :: * ;
```

### L9 · use ·

```rust
pub use chat :: * ;
```

### L10 · use ·

```rust
pub use control :: * ;
```

### L13 · function ·  apply_snapshot

```rust
fn apply_snapshot (incoming : PetSnapshot , snapshot : ReadSignal < Option < PetSnapshot > > , set_snapshot : WriteSignal < Option < PetSnapshot > > ,)
```

可見性／屬性：`pub`；``。

函式內呼叫：`.unwrap_or`；`.map`；`.get_untracked`；`.set`；`Some`。

## reference-rust_ai_chat/apps/desktop_pet/frontend/src/main.rs · P05

職責：Pet Frontend。判定：前端 schema/狀態不同；保留既有功能，不能直接替换參考 component

### L1 · module ·  bridge

```rust
 mod bridge (inline=False)
```

### L2 · module ·  component

```rust
 mod component (inline=False)
```

### L4 · use ·

```rust
pub use component :: * ;
```

### L6 · use ·

```rust
use leptos :: mount :: mount_to_body ;
```

### L7 · use ·

```rust
use leptos :: { prelude :: * , task :: spawn_local } ;
```

### L8 · use ·

```rust
use pet_domain :: PetSnapshot ;
```

### L10 · function ·  main

```rust
fn main ()
```

可見性／屬性：``；``。

函式內呼叫：`mount_to_body`；`view!`。

### L19 · function ·  App

```rust
fn App () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`signal :: < Option < PetSnapshot > >`；`signal :: < Option < String > >`；`signal`；`signal`；`signal`；`signal`；`String :: new`；`signal`；`signal :: < Option < String > >`；`signal :: < Option < String > >`；`spawn_local`；`bridge :: listen_pet_state`；`apply_snapshot`；`.set`；`Some`；`spawn_local`；`bridge :: get_pet_state`；`apply_snapshot`；`.set`；`Some`；`spawn_local`；`bridge :: scan_file_preview`；`.set`；`Some`；`view!`。

### L116 · function ·  ErrorMessage

```rust
fn ErrorMessage (error : ReadSignal < Option < String > >) -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`view!`。

## reference-rust_ai_chat/apps/desktop_pet/src/chat.rs · P11

職責：Pet Chat/Agent/生圖。判定：參考 mock 不等同成品AI；真實AI本輪未執行，readonlytool限制保留

### L1 · use ·

```rust
use ai_core :: ChatModel ;
```

### L2 · use ·

```rust
use std :: sync :: Arc ;
```

### L4 · struct ·  AiState

```rust
pub struct AiState { pub model : Arc < dyn ChatModel > , }
```

### L9 · function · impl AiState new

```rust
fn new (model : Arc < dyn ChatModel >) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

## reference-rust_ai_chat/apps/desktop_pet/src/file_organizer.rs · P07

職責：Pet preview/duplicates。判定：對照P08：安全preview存在，重複檔案雜湊分組尚未對應

### L1 · use ·

```rust
use serde :: Serialize ;
```

### L2 · use ·

```rust
use sha2 :: { Digest , Sha256 } ;
```

### L3 · use ·

```rust
use std :: collections :: BTreeMap ;
```

### L4 · use ·

```rust
use std :: fs ;
```

### L5 · use ·

```rust
use std :: io :: BufReader ;
```

### L6 · use ·

```rust
use std :: io :: Error ;
```

### L7 · use ·

```rust
use std :: io :: Read ;
```

### L8 · use ·

```rust
use std :: path :: Path ;
```

### L9 · use ·

```rust
use std :: path :: PathBuf ;
```

### L10 · use ·

```rust
use thiserror :: Error ;
```

### L12 · enum ·  FileOrganizerError

```rust
# [derive (Debug , Error)] pub enum FileOrganizerError { # [error ("desktop pet must not run as root")] RootProcess , # [error ("authorized root does not exist or cannot be resolved: {0}")] InvalidRoot (String) , # [error ("authorized root is not a directory")] RootNotDirectory , # [error ("file item is no longer a regular file: {0}")] UnsafeItem (String) , # [error ("file item changed after scan: {0}")] ItemChanged (String) , # [error ("filesystem scan failed: {0}")] Io (# [from] std :: io :: Error) , }
```

### L33 · struct ·  FilePreview

```rust
# [derive (Debug , Clone , Serialize , PartialEq , Eq)] # [serde (rename_all = "camelCase")] pub struct FilePreview { pub id : String , pub relative_path : String , pub size_bytes : u64 , }
```

### L41 · struct ·  SizeCandidateGroup

```rust
# [derive (Debug , Clone , Serialize , PartialEq , Eq)] # [serde (rename_all = "camelCase")] pub struct SizeCandidateGroup { pub size_bytes : u64 , pub item_ids : Vec < String > , }
```

### L48 · struct ·  ScanWarning

```rust
# [derive (Debug , Clone , Serialize , PartialEq , Eq)] # [serde (rename_all = "camelCase")] pub struct ScanWarning { pub relative_path : String , pub message : String , }
```

### L55 · struct ·  ScanPreview

```rust
# [derive (Debug , Clone , Serialize , PartialEq , Eq)] # [serde (rename_all = "camelCase")] pub struct ScanPreview { pub files : Vec < FilePreview > , pub duplicate_size_candidates : Vec < SizeCandidateGroup > , pub warnings : Vec < ScanWarning > , }
```

### L63 · function ·  reject_root_euid

```rust
fn reject_root_euid (euid : u32) -> Result < () , FileOrganizerError >
```

可見性／屬性：``；``。

函式內呼叫：`Err`；`Ok`。

### L72 · function ·  ensure_non_root_process

```rust
fn ensure_non_root_process () -> Result < () , FileOrganizerError >
```

可見性／屬性：``；`# [cfg (unix)]`。

函式內呼叫：`libc :: geteuid`；`reject_root_euid`。

### L78 · function ·  ensure_non_root_process

```rust
fn ensure_non_root_process () -> Result < () , FileOrganizerError >
```

可見性／屬性：``；`# [cfg (not (unix))]`。

函式內呼叫：`Ok`。

### L82 · struct ·  AuthorizedRoot

```rust
# [derive (Debug , Clone)] pub struct AuthorizedRoot { canonical : PathBuf , }
```

### L88 · function · impl AuthorizedRoot new

```rust
fn new (path : impl AsRef < Path >) -> Result < Self , FileOrganizerError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`ensure_non_root_process`；`.map_err`；`fs :: canonicalize`；`.as_ref`；`FileOrganizerError :: InvalidRoot`；`.to_string`；`.is_dir`；`Err`；`Ok`。

### L101 · function · impl AuthorizedRoot path

```rust
fn path (& self) -> & Path
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L106 · function ·  display_relative

```rust
fn display_relative (root : & Path , path : & Path) -> String
```

可見性／屬性：``；``。

函式內呼叫：`.into_owned`；`.to_string_lossy`；`.unwrap_or`；`.strip_prefix`。

### L113 · function ·  scan_dir

```rust
fn scan_dir (root : & Path , current : & Path , files : & mut Vec < FilePreview > , warnings : & mut Vec < ScanWarning > ,) -> Result < () , FileOrganizerError >
```

可見性／屬性：``；``。

函式內呼叫：`fs :: read_dir`；`.push`；`.to_string`；`.to_string`；`.path`；`fs :: symlink_metadata`；`.push`；`display_relative`；`.to_string`；`.file_type`；`.is_symlink`；`.is_dir`；`scan_dir`；`.push`；`display_relative`；`.to_string`；`.is_file`；`.push`；`String :: new`；`display_relative`；`.len`；`Ok`。

### L176 · function ·  build_size_candidates

```rust
fn build_size_candidates (files : & [FilePreview]) -> Vec < SizeCandidateGroup >
```

可見性／屬性：``；``。

函式內呼叫：`BTreeMap :: new`；`.push`；`.or_default`；`.entry`；`.clone`；`.collect`；`.filter_map`；`.into_iter`；`.then_some`；`.len`。

### L197 · function ·  scan

```rust
fn scan (root : & AuthorizedRoot) -> Result < FileScan , FileOrganizerError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`ensure_non_root_process`；`Vec :: new`；`Vec :: new`；`BTreeMap :: new`；`scan_dir`；`.path`；`.path`；`.sort_by`；`.cmp`；`.enumerate`；`.iter_mut`；`format!`；`.insert`；`.clone`；`.clone`；`.join`；`PathBuf :: from`；`.path`；`build_size_candidates`；`Ok`。

### L232 · function ·  scan_preview

```rust
fn scan_preview (root : & AuthorizedRoot) -> Result < ScanPreview , FileOrganizerError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Ok`；`scan`。

### L236 · struct ·  ScannedFile

```rust
# [derive (Debug , Clone)] struct ScannedFile { id : String , path : PathBuf , size_bytes : u64 , }
```

### L243 · struct ·  FileScan

```rust
# [derive (Debug)] pub struct FileScan { pub preview : ScanPreview , items : BTreeMap < String , ScannedFile > , }
```

### L249 · struct ·  DuplicateGroup

```rust
# [derive (Debug , Clone , PartialEq , Eq , Serialize)] # [serde (rename_all = "camelCase")] pub struct DuplicateGroup { pub digest_sha256 : String , pub size_bytes : u64 , pub item_ids : Vec < String > , }
```

### L257 · struct ·  DuplicatePreview

```rust
# [derive (Debug , Clone , PartialEq , Eq , Serialize)] # [serde (rename_all = "camelCase")] pub struct DuplicatePreview { pub groups : Vec < DuplicateGroup > , pub warnings : Vec < ScanWarning > , }
```

### L264 · function ·  revalidate_scanned_file

```rust
fn revalidate_scanned_file (item : & ScannedFile) -> Result < () , FileOrganizerError >
```

可見性／屬性：``；``。

函式內呼叫：`ensure_non_root_process`；`fs :: symlink_metadata`；`.is_symlink`；`.file_type`；`.is_file`；`.file_type`；`Err`；`FileOrganizerError :: UnsafeItem`；`.clone`；`.len`；`Err`；`FileOrganizerError :: ItemChanged`；`.clone`；`fs :: canonicalize`；`Err`；`FileOrganizerError :: UnsafeItem`；`.clone`；`Ok`。

### L286 · function ·  sha256_file

```rust
fn sha256_file (path : & Path) -> Result < String , Error >
```

可見性／屬性：``；``。

函式內呼叫：`fs :: File :: open`；`BufReader :: new`；`Sha256 :: new`；`.read`；`.update`；`.finalize`；`.collect`；`.map`；`.iter`；`format!`；`Ok`。

### L309 · function ·  duplicate_preview

```rust
fn duplicate_preview (scan : & FileScan) -> DuplicatePreview
```

可見性／屬性：`pub`；``。

函式內呼叫：`Vec :: new`；`Vec :: new`；`BTreeMap :: new`；`.get`；`.push`；`.to_string`；`format!`；`revalidate_scanned_file`；`.push`；`.to_string`；`.to_string`；`sha256_file`；`.push`；`.or_default`；`.entry`；`.clone`；`.push`；`.to_string`；`.to_string`；`.len`；`.push`。

### L359 · module ·  tests

```rust
 mod tests (inline=True)
```

### L361 · use ·

```rust
use super :: * ;
```

### L362 · use ·

```rust
use std :: { fs , time :: { SystemTime , UNIX_EPOCH } , } ;
```

### L367 · struct ·  TestDir

```rust
struct TestDir { path : PathBuf , }
```

### L372 · function · tests::impl TestDir new

```rust
fn new (name : & str) -> Self
```

可見性／屬性：``；``。

函式內呼叫：`.as_nanos`；`.expect`；`.duration_since`；`SystemTime :: now`；`.join`；`std :: env :: temp_dir`；`format!`；`.expect`；`fs :: create_dir_all`。

### L387 · function · tests::impl Drop for TestDir drop

```rust
fn drop (& mut self)
```

可見性／屬性：``；``。

函式內呼叫：`fs :: remove_dir_all`。

### L393 · function · tests scans_regular_files_and_builds_size_candidates

```rust
fn scans_regular_files_and_builds_size_candidates ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`TestDir :: new`；`.unwrap`；`fs :: write`；`.join`；`.unwrap`；`fs :: write`；`.join`；`.unwrap`；`fs :: write`；`.join`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`scan_preview`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`。

### L409 · function · tests ids_are_opaque_not_absolute_paths

```rust
fn ids_are_opaque_not_absolute_paths ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`TestDir :: new`；`.unwrap`；`fs :: write`；`.join`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`scan_preview`；`assert!`；`assert!`；`assert_eq!`。

### L428 · function · tests skips_symlink_instead_of_following_it

```rust
fn skips_symlink_instead_of_following_it ()
```

可見性／屬性：``；`# [cfg (unix)] # [test]`。

函式內呼叫：`TestDir :: new`；`TestDir :: new`；`.unwrap`；`fs :: write`；`.join`；`.unwrap`；`symlink`；`.join`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`scan_preview`；`assert!`。

### L444 · function · tests rejects_file_as_authorized_root

```rust
fn rejects_file_as_authorized_root ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`TestDir :: new`；`.join`；`.unwrap`；`fs :: write`；`.unwrap_err`；`AuthorizedRoot :: new`；`assert!`。

### L454 · function · tests rejects_root_euid

```rust
fn rejects_root_euid ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`；`assert!`。

### L463 · function · tests same_size_different_content_is_not_duplicate

```rust
fn same_size_different_content_is_not_duplicate ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`TestDir :: new`；`.unwrap`；`fs :: write`；`.join`；`.unwrap`；`fs :: write`；`.join`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`scan`；`duplicate_preview`；`assert!`。

### L476 · function · tests same_content_is_grouped_as_duplicate

```rust
fn same_content_is_grouped_as_duplicate ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`TestDir :: new`；`.unwrap`；`fs :: write`；`.join`；`.unwrap`；`fs :: write`；`.join`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`scan`；`duplicate_preview`；`assert_eq!`；`assert_eq!`；`assert_eq!`。

### L491 · function · tests unique_size_file_is_not_hashed_into_duplicate_group

```rust
fn unique_size_file_is_not_hashed_into_duplicate_group ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`TestDir :: new`；`.unwrap`；`fs :: write`；`.join`；`.unwrap`；`fs :: write`；`.join`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`scan`；`duplicate_preview`；`assert!`。

### L504 · function · tests duplicate_preview_warns_when_file_size_changes_after_scan

```rust
fn duplicate_preview_warns_when_file_size_changes_after_scan ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`TestDir :: new`；`.join`；`.join`；`.unwrap`；`fs :: write`；`.unwrap`；`fs :: write`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`scan`；`.unwrap`；`fs :: write`；`duplicate_preview`；`assert!`；`assert_eq!`。

### L526 · function · tests duplicate_preview_warns_when_file_is_replaced_by_symlink

```rust
fn duplicate_preview_warns_when_file_is_replaced_by_symlink ()
```

可見性／屬性：``；`# [cfg (unix)] # [test]`。

函式內呼叫：`TestDir :: new`；`TestDir :: new`；`.join`；`.join`；`.join`；`.unwrap`；`fs :: write`；`.unwrap`；`fs :: write`；`.unwrap`；`fs :: write`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`scan`；`.unwrap`；`fs :: remove_file`；`.unwrap`；`symlink`；`duplicate_preview`；`assert!`；`assert_eq!`。

## reference-rust_ai_chat/apps/desktop_pet/src/lib.rs · P04

職責：Pet Tauri 入口。判定：40 frontend command names 對應44 backend；非參考 bridge/DTO 等同；詳列呼叫端與簽章

### L1 · module ·  chat

```rust
 mod chat (inline=False)
```

### L2 · module ·  file_organizer

```rust
 mod file_organizer (inline=False)
```

### L3 · module ·  roam

```rust
 mod roam (inline=False)
```

### L5 · use ·

```rust
use ai_core :: { ChatRequest , MockChatModel } ;
```

### L6 · use ·

```rust
use pet_domain :: { PetCommand , PetMachine , PetSnapshot } ;
```

### L7 · use ·

```rust
use std :: sync :: { Arc , Mutex } ;
```

### L8 · use ·

```rust
use tauri :: { Emitter , Manager , State } ;
```

### L10 · use ·

```rust
use crate :: { chat :: AiState , roam :: roam_desktop } ;
```

### L13 · function ·  dispatch_pet_command

```rust
fn dispatch_pet_command (app : & tauri :: AppHandle , pet : & Mutex < PetMachine > , command : PetCommand ,) -> Result < PetSnapshot , String >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`.lock`；`.to_string`；`.map_err`；`.dispatch`；`.to_string`；`.map_err`；`.emit`；`.to_string`；`Ok`。

### L34 · function ·  get_pet_state

```rust
fn get_pet_state (pet : State < '_ , Mutex < PetMachine > >) -> Result < PetSnapshot , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.lock`；`Ok`；`.snapshot`。

### L41 · function ·  send_pet_command

```rust
fn send_pet_command (app : tauri :: AppHandle , pet : State < '_ , Mutex < PetMachine > > , command : PetCommand ,) -> Result < PetSnapshot , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`dispatch_pet_command`；`.inner`。

### L51 · function ·  chat_with_pet

```rust
async fn chat_with_pet (message : String , state : tauri :: State < '_ , AiState > ,) -> Result < String , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.chat`；`ChatRequest :: new`；`.to_string`；`Ok`。

### L65 · function ·  scan_file_preview

```rust
fn scan_file_preview () -> Result < file_organizer :: ScanPreview , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`std :: env :: var`；`.to_string`；`.map_err`；`file_organizer :: AuthorizedRoot :: new`；`format!`；`.map_err`；`file_organizer :: scan_preview`；`format!`。

### L74 · function ·  application

```rust
fn application ()
```

可見性／屬性：`pub`；``。

函式內呼叫：`.expect`；`.run`；`.invoke_handler`；`.setup`；`.manage`；`.manage`；`tauri :: Builder :: default`；`Mutex :: new`；`PetMachine :: default`；`AiState :: new`；`Arc :: new`；`.on_menu_event`；`.as_str`；`.id`；`Some`；`Some`；`Some`；`.state`；`dispatch_pet_command`；`.inner`；`eprintln!`；`.clone`；`.handle`；`tauri :: async_runtime :: spawn`；`roam_desktop`；`Ok`；`tauri :: generate_handler!`；`tauri :: generate_context!`。

## reference-rust_ai_chat/apps/desktop_pet/src/main.rs · P04

職責：Pet Tauri 入口。判定：40 frontend command names 對應44 backend；非參考 bridge/DTO 等同；詳列呼叫端與簽章

### L1 · function ·  main

```rust
fn main ()
```

可見性／屬性：``；``。

函式內呼叫：`.ok`；`dotenv :: dotenv`；`desktop_pet :: application`。

## reference-rust_ai_chat/apps/desktop_pet/src/roam.rs · P06

職責：Pet 真視窗漫遊。判定：參考功能尚未對應成品；缺失為 native window roaming，不把sprite移动宣稱等同

### L1 · use ·

```rust
use tauri :: { Manager , PhysicalPosition , PhysicalRect , PhysicalSize } ;
```

### L3 · const ·  ROAM_INITIAL_DX

```rust
const ROAM_INITIAL_DX : i32 = 2 ;
```

### L4 · const ·  ROAM_INITIAL_DY

```rust
const ROAM_INITIAL_DY : i32 = 1 ;
```

### L5 · const ·  ROAM_INTERVAL

```rust
const ROAM_INTERVAL : std :: time :: Duration = std :: time :: Duration :: from_millis (50) ;
```

### L6 · const ·  ROAM_VERIFY_INTERVAL

```rust
const ROAM_VERIFY_INTERVAL : std :: time :: Duration = std :: time :: Duration :: from_millis (200) ;
```

### L7 · const ·  ROAM_STALL_LIMIT

```rust
const ROAM_STALL_LIMIT : u8 = 3 ;
```

### L10 · function ·  bounded_next_position

```rust
fn bounded_next_position (before : PhysicalPosition < i32 > , window_size : PhysicalSize < u32 > , work_area : & PhysicalRect < i32 , u32 > , dx : & mut i32 , dy : & mut i32 ,) -> PhysicalPosition < i32 >
```

可見性／屬性：``；``。

函式內呼叫：`i64 :: from`；`i64 :: from`；`i64 :: from`；`i64 :: from`；`i64 :: from`；`i64 :: from`；`i64 :: from`；`i64 :: from`；`i64 :: from`；`i64 :: from`；`.saturating_abs`；`.saturating_abs`；`.saturating_abs`；`.saturating_abs`；`.clamp`；`i64 :: from`；`i64 :: from`；`.clamp`；`i64 :: from`；`i64 :: from`。

### L60 · function ·  roam_desktop

```rust
async fn roam_desktop (app : tauri :: AppHandle)
```

可見性／屬性：`pub`；``。

函式內呼叫：`.get_webview_window`；`println!`；`.outer_position`；`eprintln!`；`.outer_size`；`eprintln!`；`.current_monitor`；`eprintln!`；`eprintln!`；`bounded_next_position`；`.work_area`；`.set_position`；`eprintln!`；`tokio :: time :: sleep`；`.outer_position`；`eprintln!`；`eprintln!`；`tokio :: time :: sleep`。

### L139 · module ·  tests

```rust
 mod tests (inline=True)
```

### L141 · use ·

```rust
use super :: * ;
```

### L143 · function · tests area

```rust
fn area (x : i32 , y : i32 , width : u32 , height : u32) -> PhysicalRect < i32 , u32 >
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L151 · function · tests moves_inside_work_area

```rust
fn moves_inside_work_area ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`bounded_next_position`；`area`；`assert_eq!`；`assert_eq!`。

### L169 · function · tests bounces_at_right_and_bottom_edges

```rust
fn bounces_at_right_and_bottom_edges ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`bounded_next_position`；`area`；`assert_eq!`；`assert_eq!`。

### L187 · function · tests supports_negative_monitor_origin

```rust
fn supports_negative_monitor_origin ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`bounded_next_position`；`area`；`assert_eq!`；`assert_eq!`。

### L205 · function · tests oversized_window_is_pinned_without_panicking

```rust
fn oversized_window_is_pinned_without_panicking ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`bounded_next_position`；`area`；`assert_eq!`；`assert_eq!`。

## reference-rust_ai_chat/apps/server/src/http.rs · C05

職責：Chat 入口/HTTP/SSR。判定：API 不相容；產品入口保留，教學需明示 adapter/URL/DTO

### L1 · use ·

```rust
use crate :: state :: AppState ;
```

### L2 · use ·

```rust
use axum :: extract :: State ;
```

### L3 · use ·

```rust
use axum :: { Json , http :: StatusCode } ;
```

### L4 · use ·

```rust
use serde :: Serialize ;
```

### L5 · use ·

```rust
use shared :: { CreateRoomRequest , CreateRoomResponse } ;
```

### L7 · struct ·  HealthResponse

```rust
# [derive (Debug , Serialize)] pub struct HealthResponse { pub status : & 'static str , pub service : & 'static str , }
```

### L13 · function ·  health

```rust
async fn health (State (state) : State < AppState >) -> Json < HealthResponse >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Json`。

### L20 · function ·  create_room

```rust
async fn create_room (State (state) : State < AppState > , Json (request) : Json < CreateRoomRequest > ,) -> Result < Json < CreateRoomResponse > , StatusCode >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.to_string`；`uuid :: Uuid :: new_v4`；`.map_err`；`chat_application :: create_room`；`.as_ref`；`.to_string`；`Ok`；`Json`。

## reference-rust_ai_chat/apps/server/src/hub.rs · C06

職責：Chat WS/Fanout。判定：保留跨程序 durability/ACL；不能把參考 socket 當成品 client

### L1 · use ·

```rust
use std :: collections :: HashMap ;
```

### L2 · use ·

```rust
use std :: sync :: Arc ;
```

### L3 · use ·

```rust
use std :: sync :: Mutex ;
```

### L5 · use ·

```rust
use shared :: ServerEvent ;
```

### L6 · use ·

```rust
use tokio :: sync :: broadcast ;
```

### L8 · struct ·  RoomHub

```rust
# [derive (Debug , Clone)] pub struct RoomHub { rooms : Arc < Mutex < HashMap < String , broadcast :: Sender < ServerEvent > > > > , capacity : usize , }
```

### L15 · function · impl RoomHub new

```rust
fn new (capacity : usize) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：`Arc :: new`；`Mutex :: new`；`HashMap :: new`。

### L22 · function · impl RoomHub subscribe

```rust
fn subscribe (& self , room_id : String ,) -> (broadcast :: Sender < ServerEvent > , broadcast :: Receiver < ServerEvent > ,)
```

可見性／屬性：`pub`；``。

函式內呼叫：`.unwrap`；`.lock`；`.clone`；`.or_insert_with`；`.entry`；`broadcast :: channel`；`.subscribe`。

### L44 · function · impl RoomHub subscriber_count

```rust
fn subscriber_count (& self , room_id : & str) -> usize
```

可見性／屬性：`pub`；``。

函式內呼叫：`.map_or`；`.get`；`.unwrap`；`.lock`；`.receiver_count`。

### L52 · function · impl RoomHub publish

```rust
fn publish (& self , room_id : String , event : ServerEvent)
```

可見性／屬性：`pub`；``。

函式內呼叫：`.unwrap`；`.lock`；`.cloned`；`.get`；`.send`。

### L62 · function · impl RoomHub cleanup

```rust
fn cleanup (& self , room_id : String , sender : & broadcast :: Sender < ServerEvent >)
```

可見性／屬性：`pub`；``。

函式內呼叫：`.unwrap`；`.lock`；`.is_some_and`；`.get`；`.same_channel`；`.receiver_count`；`.remove`。

## reference-rust_ai_chat/apps/server/src/lib.rs · C05

職責：Chat 入口/HTTP/SSR。判定：API 不相容；產品入口保留，教學需明示 adapter/URL/DTO

### L1 · module ·  http

```rust
pub mod http (inline=False)
```

### L2 · module ·  hub

```rust
pub mod hub (inline=False)
```

### L3 · module ·  ssr

```rust
pub mod ssr (inline=False)
```

### L4 · module ·  state

```rust
pub mod state (inline=False)
```

### L5 · module ·  ws

```rust
pub mod ws (inline=False)
```

### L7 · use ·

```rust
use std :: sync :: Arc ;
```

### L9 · use ·

```rust
use any_spawner :: Executor ;
```

### L10 · use ·

```rust
use axum :: { Router , routing :: { any , get , post } , } ;
```

### L14 · use ·

```rust
use leptos_axum :: { site_pkg_dir_service , site_pkg_dir_service_route_path } ;
```

### L15 · use ·

```rust
use leptos_config :: get_configuration ;
```

### L17 · function ·  application

```rust
async fn application () -> Result < () , Box < dyn std :: error :: Error > >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Executor :: init_tokio`；`get_configuration`；`site_pkg_dir_service_route_path`；`site_pkg_dir_service`；`std :: env :: var`；`sqlx :: SqlitePool :: connect`；`persistence :: migrate`；`Arc :: new`；`persistence :: SqliteRoomRepository :: new`；`.clone`；`Arc :: new`；`persistence :: SqliteMessageRepository :: new`；`.clone`；`state :: AppState :: new`；`.with_state`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route_service`；`Router :: new`；`get`；`post`；`any`；`get`；`get`；`tokio :: net :: TcpListener :: bind`；`println!`；`axum :: serve`；`Ok`。

## reference-rust_ai_chat/apps/server/src/main.rs · C05

職責：Chat 入口/HTTP/SSR。判定：API 不相容；產品入口保留，教學需明示 adapter/URL/DTO

### L2 · function ·  main

```rust
async fn main () -> Result < () , Box < dyn std :: error :: Error > >
```

可見性／屬性：``；`# [tokio :: main]`。

函式內呼叫：`.ok`；`dotenv :: dotenv`；`server :: application`。

## reference-rust_ai_chat/apps/server/src/ssr.rs · C05

職責：Chat 入口/HTTP/SSR。判定：API 不相容；產品入口保留，教學需明示 adapter/URL/DTO

### L1 · use ·

```rust
use crate :: state :: AppState ;
```

### L2 · use ·

```rust
use axum :: { body :: Body , extract :: State , http :: Request , response :: IntoResponse } ;
```

### L3 · use ·

```rust
use leptos_axum :: render_app_to_stream ;
```

### L5 · function ·  app_handler

```rust
async fn app_handler (State (state) : State < AppState > , request : Request < Body > ,) -> impl IntoResponse
```

可見性／屬性：`pub`；``。

函式內呼叫：`.clone`；`render_app_to_stream`；`ui :: shell`；`.clone`；`.into_response`；`handler`。

## reference-rust_ai_chat/apps/server/src/state.rs · C05

職責：Chat 入口/HTTP/SSR。判定：API 不相容；產品入口保留，教學需明示 adapter/URL/DTO

### L1 · use ·

```rust
use crate :: hub :: RoomHub ;
```

### L2 · use ·

```rust
use chat_application :: { ChatService , repository :: { MessageRepository , RoomRepository } , } ;
```

### L6 · use ·

```rust
use std :: sync :: Arc ;
```

### L8 · struct ·  AppState

```rust
# [derive (Clone)] pub struct AppState { pub chat_service : Arc < ChatService > , pub leptos_options : leptos_config :: LeptosOptions , pub room_hub : RoomHub , pub room_repository : Arc < dyn RoomRepository > , pub message_repository : Arc < dyn MessageRepository > , pub service_name : & 'static str , }
```

### L19 · function · impl AppState new

```rust
fn new (leptos_options : leptos_config :: LeptosOptions , room_repository : Arc < dyn RoomRepository > , message_repository : Arc < dyn MessageRepository > ,) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：`Arc :: new`；`ChatService :: new`；`RoomHub :: new`。

## reference-rust_ai_chat/apps/server/src/ws.rs · C06

職責：Chat WS/Fanout。判定：保留跨程序 durability/ACL；不能把參考 socket 當成品 client

### L1 · use ·

```rust
use crate :: state :: AppState ;
```

### L2 · use ·

```rust
use axum :: { extract :: { Path , State , ws :: { Message , WebSocket , WebSocketUpgrade } , } , response :: Response , } ;
```

### L9 · use ·

```rust
use shared :: { ClientEvent , ServerEvent } ;
```

### L10 · use ·

```rust
use tokio :: sync :: broadcast ;
```

### L12 · function ·  send_event

```rust
async fn send_event (socket : & mut WebSocket , event : ServerEvent ,) -> Result < () , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：``；``。

函式內呼叫：`serde_json :: to_string`；`.send`；`Message :: Text`；`.into`；`Ok`。

### L21 · function ·  ws_handler

```rust
async fn ws_handler (Path (room_id) : Path < String > , State (state) : State < AppState > , ws : WebSocketUpgrade ,) -> Response
```

可見性／屬性：`pub`；``。

函式內呼叫：`.find_by_id`；`.on_upgrade`；`handle_socket`；`.unwrap`；`.body`；`.status`；`Response :: builder`；`.into`；`eprintln!`；`.unwrap`；`.body`；`.status`；`Response :: builder`；`.into`。

### L43 · function ·  valid_message_content

```rust
fn valid_message_content (content : & str) -> bool
```

可見性／屬性：``；``。

函式內呼叫：`.is_empty`；`.trim`。

### L48 · function ·  room_matches_connection

```rust
fn room_matches_connection (connection : & str , event : & str) -> bool
```

可見性／屬性：``；``。

函式內呼叫：`.is_empty`；`.trim`；`.is_empty`；`.trim`。

### L52 · function ·  handle_socket

```rust
async fn handle_socket (mut socket : WebSocket , state : AppState , room_id : String)
```

可見性／屬性：``；``。

函式內呼叫：`.subscribe`；`.clone`；`tokio :: select!`；`drop`；`.cleanup`；`.clone`。

### L100 · function ·  handle_client_event

```rust
async fn handle_client_event (socket : & mut WebSocket , state : & AppState , text : & str , event_room_id : String ,) -> Result < () , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：``；``。

函式內呼叫：`serde_json :: from_str`；`send_event`；`room_matches_connection`；`send_event`；`.to_string`；`.to_string`；`Ok`；`valid_message_content`；`send_event`；`.to_owned`；`.to_owned`；`Ok`；`.send_message`；`.clone`；`chat_application :: persist_then_publish`；`.as_ref`；`.publish`；`.clone`；`ServerEvent :: MessageCreated`；`eprintln!`；`send_event`；`.to_owned`；`.to_owned`；`Ok`；`Ok`。

### L169 · module ·  test

```rust
 mod test (inline=True)
```

### L171 · use ·

```rust
use super :: { room_matches_connection , valid_message_content } ;
```

### L174 · function · test matching_room

```rust
fn matching_room ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`。

### L178 · function · test different_room

```rust
fn different_room ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`。

### L182 · function · test blank_room

```rust
fn blank_room ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`；`assert!`。

### L187 · function · test accepts_nonblank_content

```rust
fn accepts_nonblank_content ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`；`assert!`。

### L192 · function · test rejects_blank_content

```rust
fn rejects_blank_content ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`；`assert!`。

## reference-rust_ai_chat/apps/stats_server/src/lib.rs · A01

職責：Analytics reference契約。判定：參考未實作不是成品應退回501；strict event whitelist不等同任意非空kind

### L1 · use ·

```rust
use axum :: { Json , Router , extract :: DefaultBodyLimit , http :: StatusCode , routing :: { get , post } , } ;
```

### L7 · use ·

```rust
use serde :: Deserialize ;
```

### L8 · use ·

```rust
use uuid :: Uuid ;
```

### L10 · struct ·  EventInput

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub struct EventInput { pub schema_version : u16 , pub event_id : Uuid , pub source_app : String , pub event_type : String , }
```

### L19 · struct ·  EventIdentity

```rust
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct EventIdentity { source_app : String , event_id : String , }
```

### L26 · function · impl EventIdentity new

```rust
fn new (source_app : impl Into < String > , event_id : impl Into < String >) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：`.into`；`.into`。

### L34 · struct ·  InMemoryDeduplicator

```rust
# [derive (Default)] pub struct InMemoryDeduplicator { seen : std :: collections :: HashSet < EventIdentity > , }
```

### L40 · function · impl InMemoryDeduplicator accept_once

```rust
fn accept_once (& mut self , identity : EventIdentity) -> bool
```

可見性／屬性：`pub`；``。

函式內呼叫：`.insert`。

### L46 · function ·  valid_event_type

```rust
fn valid_event_type (value : & str) -> bool
```

可見性／屬性：`pub`；``。

函式內呼叫：`.trim`；`.is_empty`；`.len`。

### L52 · function ·  valid_event

```rust
fn valid_event (event : & EventInput) -> bool
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_nil`；`matches!`；`valid_event_type`；`.as_str`。

### L60 · function ·  ingest

```rust
async fn ingest (Json (event) : Json < EventInput >) -> StatusCode
```

可見性／屬性：`pub`；``。

函式內呼叫：`valid_event`。

### L68 · function ·  health

```rust
async fn health () -> StatusCode
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L72 · function ·  build_app

```rust
fn build_app () -> Router
```

可見性／屬性：`pub`；``。

函式內呼叫：`.layer`；`.route`；`.route`；`Router :: new`；`get`；`post`；`DefaultBodyLimit :: max`。

### L79 · module ·  tests

```rust
 mod tests (inline=True)
```

### L81 · use ·

```rust
use super :: * ;
```

### L83 · function · tests nil_event_id_is_not_accepted

```rust
fn nil_event_id_is_not_accepted ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`Uuid :: nil`；`.into`；`.into`；`assert!`；`Uuid :: new_v4`；`assert!`。

### L99 · function · tests rejects_bad_source_version_and_event_type

```rust
fn rejects_bad_source_version_and_event_type ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`Uuid :: new_v4`；`.into`；`.into`；`assert!`；`.into`；`assert!`；`.into`；`assert!`；`.into`；`assert!`。

## reference-rust_ai_chat/apps/stats_server/src/main.rs · A01

職責：Analytics reference契約。判定：參考未實作不是成品應退回501；strict event whitelist不等同任意非空kind

### L1 · use ·

```rust
use stats_server :: build_app ;
```

### L4 · function ·  main

```rust
async fn main () -> Result < () , Box < dyn std :: error :: Error > >
```

可見性／屬性：``；`# [tokio :: main]`。

函式內呼叫：`build_app`；`tokio :: net :: TcpListener :: bind`；`axum :: serve`；`Ok`。

## reference-rust_ai_chat/crates/ai_core/src/lib.rs · X02

職責：AI Provider。判定：具體Adapter非純Domain；參考trait抽象沒有直接實作等同保證

### L1 · use ·

```rust
use async_trait :: async_trait ;
```

### L2 · use ·

```rust
use thiserror :: Error ;
```

### L4 · struct ·  ChatRequest

```rust
# [derive (Debug , Clone , PartialEq , Eq)] pub struct ChatRequest { pub message : String , }
```

### L10 · function · impl ChatRequest new

```rust
fn new (message : impl Into < String >) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：`.into`。

### L17 · struct ·  ChatResponse

```rust
# [derive (Debug , Clone , PartialEq , Eq)] pub struct ChatResponse { pub text : String , }
```

### L22 · enum ·  AiError

```rust
# [derive (Debug , Error , PartialEq , Eq)] pub enum AiError { # [error ("chat message cannot be empty")] EmptyMessage , # [error ("model request failed: {0}")] Model (String) , }
```

### L31 · trait ·  ChatModel

```rust
# [async_trait] pub trait ChatModel : Send + Sync { async fn chat (& self , request : ChatRequest) -> Result < ChatResponse , AiError > ; }
```

### L33 · function · ChatModel chat

```rust
async fn chat (& self , request : ChatRequest) -> Result < ChatResponse , AiError >
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L36 · struct ·  MockChatModel

```rust
# [derive (Debug , Default)] pub struct MockChatModel ;
```

### L41 · function · impl ChatModel for MockChatModel chat

```rust
async fn chat (& self , request : ChatRequest) -> Result < ChatResponse , AiError >
```

可見性／屬性：``；``。

函式內呼叫：`.trim`；`.is_empty`；`Err`；`Ok`；`format!`。

### L54 · module ·  tests

```rust
 mod tests (inline=True)
```

### L56 · use ·

```rust
use std :: sync :: Arc ;
```

### L58 · use ·

```rust
use super :: * ;
```

### L61 · function · tests mock_chat_returns_response

```rust
async fn mock_chat_returns_response ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.expect`；`.chat`；`ChatRequest :: new`；`assert_eq!`。

### L73 · function · tests empty_message_is_rejected

```rust
async fn empty_message_is_rejected ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.expect_err`；`.chat`；`ChatRequest :: new`；`assert_eq!`。

### L85 · function · tests chat_model_can_be_used_as_trait_object

```rust
async fn chat_model_can_be_used_as_trait_object ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`Arc :: new`；`.expect`；`.chat`；`ChatRequest :: new`；`assert_eq!`。

## reference-rust_ai_chat/crates/chat_application/src/lib.rs · C02

職責：Chat Application。判定：儲存後才通知的不變量對應；無 RoomRepository 等同實作；保留 durable Adapter

### L1 · module ·  product_event

```rust
pub mod product_event (inline=False)
```

### L2 · module ·  repository

```rust
pub mod repository (inline=False)
```

### L4 · use ·

```rust
use async_trait :: async_trait ;
```

### L5 · use ·

```rust
use chat_domain :: { ChatMessage , MessageAuthor , Room } ;
```

### L6 · use ·

```rust
use uuid :: Uuid ;
```

### L8 · use ·

```rust
use crate :: repository :: { MessageRepository , RepositoryError , RoomRepository } ;
```

### L10 · struct ·  ChatService

```rust
# [derive (Debug , Clone , Default)] pub struct ChatService ;
```

### L14 · function · impl ChatService new

```rust
fn new () -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L18 · function · impl ChatService send_message

```rust
fn send_message (& self , room_id : String , content : String) -> ChatMessage
```

可見性／屬性：`pub`；``。

函式內呼叫：`.to_string`；`Uuid :: new_v4`。

### L28 · struct ·  CreateRoom

```rust
# [derive (Debug , Clone)] pub struct CreateRoom { pub id : String , pub category_room_id : String , pub name : String , }
```

### L35 · function ·  create_room

```rust
async fn create_room (room_repository : & dyn RoomRepository , input : CreateRoom ,) -> Result < Room , RepositoryError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.create`；`Ok`。

### L49 · function ·  persist_then_publish

```rust
async fn persist_then_publish (repository : & dyn MessageRepository , message : ChatMessage , publish : impl FnOnce (ChatMessage) ,) -> Result < () , RepositoryError >
```

可見性／屬性：`pub`；`# [doc = "成功 create 後才推播；DB 失敗時 closure 不執行。"]`。

函式內呼叫：`.create`；`publish`；`Ok`。

### L59 · struct ·  SuccessfulRepository

```rust
struct SuccessfulRepository ;
```

### L62 · function · impl MessageRepository for SuccessfulRepository create

```rust
async fn create (& self , _ : & ChatMessage) -> Result < () , RepositoryError >
```

可見性／屬性：``；``。

函式內呼叫：`Ok`。

### L67 · module ·  tests

```rust
 mod tests (inline=True)
```

### L69 · use ·

```rust
use super :: * ;
```

### L70 · use ·

```rust
use std :: sync :: { Arc , atomic :: { AtomicUsize , Ordering } , } ;
```

### L74 · struct ·  FailingRepository

```rust
struct FailingRepository ;
```

### L77 · function · tests::impl MessageRepository for FailingRepository create

```rust
async fn create (& self , _ : & ChatMessage) -> Result < () , RepositoryError >
```

可見性／屬性：``；``。

函式內呼叫：`Err`；`RepositoryError :: MessageAccessError`；`.into`。

### L82 · function · tests failed_write_never_publishes

```rust
async fn failed_write_never_publishes ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`Arc :: new`；`AtomicUsize :: new`；`.clone`；`.into`；`.into`；`.into`；`persist_then_publish`；`.fetch_add`；`assert!`；`assert_eq!`。

### L100 · function · tests successful_write_publishes_exactly_once

```rust
async fn successful_write_publishes_exactly_once ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`Arc :: new`；`AtomicUsize :: new`；`.clone`；`.into`；`.into`；`.into`；`.expect`；`persist_then_publish`；`assert_eq!`；`.fetch_add`；`assert_eq!`。

## reference-rust_ai_chat/crates/chat_application/src/product_event.rs · C03

職責：Chat 事件。判定：四欄事件子集可適配，不能宣稱所有事件／發送方式相同

### L1 · use ·

```rust
use chat_domain :: ChatMessage ;
```

### L2 · use ·

```rust
use uuid :: Uuid ;
```

### L4 · struct ·  PersistedMessageEvent

```rust
# [derive (Debug , Clone , PartialEq , Eq)] pub struct PersistedMessageEvent { pub schema_version : u16 , pub event_id : Uuid , pub source_app : & 'static str , pub event_type : & 'static str , }
```

### L12 · function ·  persisted_message_event

```rust
fn persisted_message_event (message : & ChatMessage ,) -> Result < PersistedMessageEvent , uuid :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Ok`；`Uuid :: parse_str`。

## reference-rust_ai_chat/crates/chat_application/src/repository.rs · C02

職責：Chat Application。判定：儲存後才通知的不變量對應；無 RoomRepository 等同實作；保留 durable Adapter

### L1 · use ·

```rust
use async_trait :: async_trait ;
```

### L2 · use ·

```rust
use chat_domain :: { ChatMessage , Room } ;
```

### L3 · use ·

```rust
use thiserror :: Error ;
```

### L5 · enum ·  RepositoryError

```rust
# [derive (Debug , Error)] pub enum RepositoryError { # [error ("房間存取錯誤: {0}")] RoomAccessError (String) , # [error ("訊息存取錯誤: {0}")] MessageAccessError (String) , }
```

### L13 · trait ·  RoomRepository

```rust
# [async_trait] pub trait RoomRepository : Send + Sync { async fn find_by_id (& self , room_id : & str) -> Result < Option < Room > , RepositoryError > ; async fn create (& self , room : & Room) -> Result < () , RepositoryError > ; }
```

### L15 · function · RoomRepository find_by_id

```rust
async fn find_by_id (& self , room_id : & str) -> Result < Option < Room > , RepositoryError >
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L17 · function · RoomRepository create

```rust
async fn create (& self , room : & Room) -> Result < () , RepositoryError >
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L20 · trait ·  MessageRepository

```rust
# [async_trait] pub trait MessageRepository : Send + Sync { async fn create (& self , message : & ChatMessage) -> Result < () , RepositoryError > ; }
```

### L22 · function · MessageRepository create

```rust
async fn create (& self , message : & ChatMessage) -> Result < () , RepositoryError >
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

## reference-rust_ai_chat/crates/chat_domain/src/lib.rs · C01

職責：Chat Domain。判定：概念可映射、公開型別與序列化不等同；category 不能當 topic

### L1 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L3 · enum ·  MessageAuthor

```rust
# [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [serde (tag = "kind" , rename_all = "snake_case")] pub enum MessageAuthor { AnonymousUser , User { user_id : String } , Ai { persona_id : String } , System , }
```

### L12 · struct ·  ChatMessage

```rust
# [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] pub struct ChatMessage { pub id : String , pub room_id : String , pub author : MessageAuthor , pub content : String , }
```

### L20 · struct ·  Room

```rust
# [derive (Debug , Clone , PartialEq , Eq)] pub struct Room { pub id : String , pub category_room_id : String , pub name : String , }
```

## reference-rust_ai_chat/crates/persistence/src/lib.rs · C04

職責：Chat SQLite。判定：同為 SQLite，schema/交易邊界不相同；參考 41 字元測試斷言與文字相反

### L1 · use ·

```rust
use async_trait :: async_trait ;
```

### L2 · use ·

```rust
use chat_application :: repository :: { MessageRepository , RepositoryError , RoomRepository } ;
```

### L3 · use ·

```rust
use chat_domain :: Room ;
```

### L4 · use ·

```rust
use sqlx :: FromRow ;
```

### L5 · use ·

```rust
use sqlx :: SqlitePool ;
```

### L7 · function ·  connect

```rust
async fn connect (database_url : & str) -> Result < sqlx :: SqlitePool , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.connect`；`.max_connections`；`sqlx :: sqlite :: SqlitePoolOptions :: new`。

### L14 · function ·  migrate

```rust
async fn migrate (pool : & sqlx :: SqlitePool) -> Result < () , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.run`；`sqlx :: migrate!`；`Ok`。

### L19 · struct ·  RoomRow

```rust
# [derive (Debug , FromRow)] pub struct RoomRow { pub id : String , pub category_room_id : String , pub name : String , }
```

### L27 · function · impl From < RoomRow > for Room from

```rust
fn from (row : RoomRow) -> Self
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L36 · struct ·  SqliteRoomRepository

```rust
# [derive (Clone)] pub struct SqliteRoomRepository { pool : SqlitePool , }
```

### L42 · function · impl SqliteRoomRepository new

```rust
fn new (pool : sqlx :: SqlitePool) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L49 · function · impl RoomRepository for SqliteRoomRepository find_by_id

```rust
async fn find_by_id (& self , room_id : & str) -> Result < Option < Room > , RepositoryError >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`.map`；`.fetch_optional`；`.bind`；`sqlx :: query_as :: < _ , RoomRow >`；`.map`；`RepositoryError :: RoomAccessError`；`.to_string`。

### L65 · function · impl RoomRepository for SqliteRoomRepository create

```rust
async fn create (& self , room : & Room) -> Result < () , RepositoryError >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`.map`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`RepositoryError :: RoomAccessError`；`.to_string`。

### L88 · struct ·  SqliteMessageRepository

```rust
# [derive (Clone)] pub struct SqliteMessageRepository { pool : sqlx :: SqlitePool , }
```

### L94 · function · impl SqliteMessageRepository new

```rust
fn new (pool : sqlx :: SqlitePool) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L101 · function · impl MessageRepository for SqliteMessageRepository create

```rust
async fn create (& self , message : & chat_domain :: ChatMessage) -> Result < () , RepositoryError >
```

可見性／屬性：``；``。

函式內呼叫：`Some`；`Some`；`.map_err`；`.map`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`RepositoryError :: MessageAccessError`；`.to_string`。

### L131 · module ·  tests

```rust
 mod tests (inline=True)
```

### L133 · use ·

```rust
use super :: * ;
```

### L136 · function · tests test_room_repository_find_by_id

```rust
async fn test_room_repository_find_by_id (pool : SqlitePool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "../../migrations")]`。

函式內呼叫：`.expect`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`SqliteRoomRepository :: new`；`.expect`；`.expect`；`.find_by_id`；`assert_eq!`；`assert_eq!`；`assert_eq!`。

### L170 · function · tests test_room_repository_create_then_find

```rust
async fn test_room_repository_create_then_find (pool : SqlitePool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "../../migrations")]`。

函式內呼叫：`SqliteRoomRepository :: new`；`.to_string`；`.to_string`；`.to_string`；`.expect`；`.create`；`.expect`；`.expect`；`.find_by_id`；`assert_eq!`。

### L194 · function · tests insert_test_room

```rust
async fn insert_test_room (pool : & SqlitePool , id : & str)
```

可見性／屬性：``；``。

函式內呼叫：`.expect`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`。

### L208 · function · tests test_message_repository_all_author_kinds

```rust
async fn test_message_repository_all_author_kinds (pool : SqlitePool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "../../migrations")]`。

函式內呼叫：`insert_test_room`；`SqliteMessageRepository :: new`；`.clone`；`.into`；`Some`；`.into`；`Some`；`.to_string`；`.to_string`；`format!`；`.expect`；`.create`；`.expect`；`.fetch_one`；`.bind`；`sqlx :: query_as`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`。

### L264 · function · tests test_message_repository_rejects_missing_room

```rust
async fn test_message_repository_rejects_missing_room (pool : SqlitePool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "../../migrations")]`。

函式內呼叫：`SqliteMessageRepository :: new`；`.to_string`；`.to_string`；`.to_string`；`assert!`。

### L280 · function · tests test_message_schema_rejects_invalid_author_and_length

```rust
async fn test_message_schema_rejects_invalid_author_and_length (pool : SqlitePool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "../../migrations")]`。

函式內呼叫：`insert_test_room`；`Some`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`assert!`；`.repeat`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`assert!`。

## reference-rust_ai_chat/crates/pet_domain/src/cleanup.rs · P03

職責：Pet cleanup Domain。判定：事件名稱不相容；對照成品 P08，不以狀態成功代替真操作成功

### L1 · enum ·  CleanupState

```rust
# [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum CleanupState { Previewed , Confirmed , Succeeded , Failed , }
```

### L8 · enum ·  CleanupError

```rust
# [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum CleanupError { ConfirmationRequired , AlreadyFinished , }
```

### L14 · struct ·  CleanupOperation

```rust
# [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct CleanupOperation { state : CleanupState , }
```

### L20 · function · impl CleanupOperation new

```rust
fn new () -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L25 · function · impl CleanupOperation confirm

```rust
fn confirm (& mut self) -> Result < () , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Err`；`Ok`。

### L32 · function · impl CleanupOperation complete

```rust
fn complete (& mut self , actual_success : bool) -> Result < () , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Err`；`Err`；`Ok`。

### L46 · function · impl CleanupOperation may_emit_success_event

```rust
fn may_emit_success_event (& self) -> bool
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L52 · function · impl Default for CleanupOperation default

```rust
fn default () -> Self
```

可見性／屬性：``；``。

函式內呼叫：`Self :: new`。

### L57 · module ·  tests

```rust
 mod tests (inline=True)
```

### L59 · use ·

```rust
use super :: * ;
```

### L61 · function · tests completion_is_one_time

```rust
fn completion_is_one_time ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`CleanupOperation :: new`；`assert_eq!`；`.unwrap`；`.confirm`；`.unwrap`；`.complete`；`assert!`；`assert_eq!`；`assert!`。

### L71 · function · tests failure_never_emits_success

```rust
fn failure_never_emits_success ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`CleanupOperation :: new`；`.unwrap`；`.confirm`；`.unwrap`；`.complete`；`assert!`；`assert_eq!`。

## reference-rust_ai_chat/crates/pet_domain/src/cleanup_event.rs · P03

職責：Pet cleanup Domain。判定：事件名稱不相容；對照成品 P08，不以狀態成功代替真操作成功

### L1 · use ·

```rust
use crate :: cleanup :: CleanupOperation ;
```

### L2 · use ·

```rust
use uuid :: Uuid ;
```

### L4 · struct ·  CleanupSucceededEvent

```rust
# [doc = " 只包含統計所需的最小事件欄位；不可攜帶私人檔案路徑。"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct CleanupSucceededEvent { pub schema_version : u16 , pub event_id : Uuid , pub source_app : & 'static str , pub event_type : & 'static str , }
```

### L16 · function ·  succeeded_event

```rust
fn succeeded_event (operation : & CleanupOperation , operation_id : & str ,) -> Result < Option < CleanupSucceededEvent > , uuid :: Error >
```

可見性／屬性：`pub`；`# [doc = " 此映射只能由可信任 filesystem Adapter 收到「確實成功」結果、"] # [doc = " 並呼叫 operation.complete(true) 之後使用。"] # [doc = " 僅 Domain 成功狀態不構成實際清理憑證。"]`。

函式內呼叫：`.may_emit_success_event`；`Ok`；`Uuid :: parse_str`；`Ok`；`Some`。

### L32 · module ·  tests

```rust
 mod tests (inline=True)
```

### L34 · use ·

```rust
use super :: * ;
```

### L36 · function · tests successful_operation_has_minimal_event

```rust
fn successful_operation_has_minimal_event ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`CleanupOperation :: new`；`assert!`；`.unwrap`；`.confirm`；`assert!`；`.unwrap`；`.complete`；`.unwrap`；`.unwrap`；`succeeded_event`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`。

### L50 · function · tests unsuccessful_operation_emits_nothing

```rust
fn unsuccessful_operation_emits_nothing ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`CleanupOperation :: new`；`.unwrap`；`.confirm`；`.unwrap`；`.complete`；`assert!`。

## reference-rust_ai_chat/crates/pet_domain/src/error.rs · P01

職責：Pet FSM。判定：狀態與命令不相容；Sleep/Wake/revision 目前無等同流程，不能空造同名型別

### L1 · use ·

```rust
use crate :: { PetCommand , PetState } ;
```

### L2 · use ·

```rust
use thiserror :: Error ;
```

### L4 · enum ·  PetError

```rust
# [derive (Debug , Clone , PartialEq , Eq , Error)] pub enum PetError { # [error ("cannot execute {command:?} while pet is {state:?}")] InvalidTransition { state : PetState , command : PetCommand , } , }
```

## reference-rust_ai_chat/crates/pet_domain/src/lib.rs · P01

職責：Pet FSM。判定：狀態與命令不相容；Sleep/Wake/revision 目前無等同流程，不能空造同名型別

### L1 · module ·  cleanup

```rust
pub mod cleanup (inline=False)
```

### L2 · module ·  cleanup_event

```rust
pub mod cleanup_event (inline=False)
```

### L3 · module ·  error

```rust
pub mod error (inline=False)
```

### L4 · module ·  profile

```rust
pub mod profile (inline=False)
```

### L6 · use ·

```rust
use crate :: error :: PetError ;
```

### L7 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L9 · enum ·  PetState

```rust
# [doc = " 可顯示的狀態；動畫與 UI 不屬於 domain。"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Serialize , Deserialize)] # [serde (rename_all = "snake_case")] pub enum PetState { Idle , Interacting , Sleeping , }
```

### L18 · enum ·  PetCommand

```rust
# [doc = " 外部送入狀態機的意圖，與輸出的事件分開。"] # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [serde (rename_all = "snake_case")] pub enum PetCommand { Interact , FinishInteraction , Sleep , Wake , }
```

### L28 · enum ·  PetEvent

```rust
# [derive (Debug , Clone , Copy , PartialEq , Eq , Serialize)] # [serde (tag = "type" , rename_all = "snake_case")] pub enum PetEvent { StateChanged { from : PetState , to : PetState } , }
```

### L34 · struct ·  PetSnapshot

```rust
# [derive (Debug , Clone , Copy , PartialEq , Eq , Serialize , Deserialize)] pub struct PetSnapshot { pub state : PetState , pub revision : u64 , }
```

### L41 · function · impl Default for PetSnapshot default

```rust
fn default () -> Self
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L49 · struct ·  PetMachine

```rust
# [derive (Debug)] pub struct PetMachine { state : PetState , revision : u64 , }
```

### L56 · function · impl PetMachine snapshot

```rust
fn snapshot (& self) -> PetSnapshot
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L62 · function · impl PetMachine dispatch

```rust
fn dispatch (& mut self , command : PetCommand) -> Result < PetSnapshot , PetError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Err`；`.clone`；`Ok`；`.snapshot`。

### L83 · function · impl Default for PetMachine default

```rust
fn default () -> Self
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L91 · enum ·  OperationState

```rust
# [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum OperationState { Previewed , Confirmed , Succeeded , Failed , }
```

### L98 · function ·  count_as_cleanup

```rust
fn count_as_cleanup (state : OperationState) -> bool
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

## reference-rust_ai_chat/crates/pet_domain/src/profile.rs · P02

職責：Pet Profile。判定：概念對應，格式/驗證/錯誤不等同；ProjectionPending 必須保留

### L1 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L2 · use ·

```rust
use thiserror :: Error ;
```

### L4 · enum ·  ProfileError

```rust
# [derive (Debug , Error , PartialEq , Eq)] pub enum ProfileError { # [error ("名字不可空白")] EmptyName , # [error ("種類不可空白")] EmptySpecies , # [error ("喜好、個性、興趣不可包含空白項目")] EmptyTag , # [error ("外觀覆寫不可為空白")] EmptyOverride , }
```

### L16 · struct ·  AppearanceOverride

```rust
# [derive (Debug , Clone , Default , PartialEq , Eq , Serialize , Deserialize)] pub struct AppearanceOverride { # [serde (rename = "顏色" , default , skip_serializing_if = "Option::is_none")] pub color : Option < String > , # [serde (rename = "風格" , default , skip_serializing_if = "Option::is_none")] pub style : Option < String > , }
```

### L23 · struct ·  PetProfile

```rust
# [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] pub struct PetProfile { # [serde (rename = "名字")] pub name : String , # [serde (rename = "種類")] pub species : String , # [serde (rename = "喜好")] pub likes : Vec < String > , # [serde (rename = "個性")] pub personality : Vec < String > , # [serde (rename = "興趣")] pub interests : Vec < String > , # [serde (rename = "外觀" , default , skip_serializing_if = "Option::is_none")] pub appearance : Option < AppearanceOverride > , }
```

### L40 · function · impl PetProfile validate

```rust
fn validate (& self) -> Result < () , ProfileError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_empty`；`.trim`；`Err`；`.is_empty`；`.trim`；`Err`；`.any`；`.chain`；`.chain`；`.iter`；`.iter`；`.iter`；`.is_empty`；`.trim`；`Err`；`.is_some_and`；`.as_deref`；`.is_empty`；`.trim`；`.is_some_and`；`.as_deref`；`.is_empty`；`.trim`；`Err`；`Ok`。

### L67 · enum ·  ProfileCodecError

```rust
# [derive (Debug , thiserror :: Error)] pub enum ProfileCodecError { # [error ("invalid profile: {0}")] Invalid (# [from] ProfileError) , # [error ("json codec error: {0}")] Json (# [from] serde_json :: Error) , }
```

### L75 · function ·  decode_profile

```rust
fn decode_profile (input : & str) -> Result < PetProfile , ProfileCodecError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`serde_json :: from_str`；`.validate`；`Ok`。

### L81 · function ·  encode_profile

```rust
fn encode_profile (profile : & PetProfile) -> Result < String , ProfileCodecError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.validate`；`Ok`；`serde_json :: to_string_pretty`。

## reference-rust_ai_chat/crates/shared/src/lib.rs · X01

職責：Shared wire/type。判定：同名crate不同職責；禁止把 WS Message本文加入low-sensitivity analytics schema

### L1 · use ·

```rust
use chat_domain :: ChatMessage ;
```

### L2 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L11 · enum ·  ClientEvent

```rust
# [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [serde (tag = "type" , content = "data" , rename_all = "snake_case")] pub enum ClientEvent { SendMessage { room_id : String , content : String } , Typing { room_id : String , active : bool } , Ping , }
```

### L19 · enum ·  ServerEvent

```rust
# [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [serde (tag = "type" , content = "data" , rename_all = "snake_case")] pub enum ServerEvent { MessageCreated (ChatMessage) , AiDelta { message_id : String , delta : String } , AiCompleted { message_id : String } , PresenceChanged { user_id : String , online : bool } , Error { code : String , message : String } , Pong , }
```

### L30 · struct ·  CreateRoomRequest

```rust
# [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] pub struct CreateRoomRequest { pub name : String , }
```

### L35 · struct ·  CreateRoomResponse

```rust
# [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] pub struct CreateRoomResponse { pub id : String , }
```

### L40 · module ·  tests

```rust
 mod tests (inline=True)
```

### L42 · use ·

```rust
use chat_domain :: MessageAuthor ;
```

### L44 · use ·

```rust
use super :: * ;
```

### L47 · function · tests client_event_json_roundtrip

```rust
fn client_event_json_roundtrip ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.to_string`；`String :: from`；`.unwrap`；`serde_json :: to_string`；`.unwrap`；`serde_json :: from_str`；`assert_eq!`。

### L59 · function · tests ai_message_round_trip

```rust
fn ai_message_round_trip ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.to_string`；`.to_string`；`.to_string`；`.to_string`；`ServerEvent :: MessageCreated`；`.unwrap`；`serde_json :: to_string`；`.unwrap`；`serde_json :: from_str`；`assert_eq!`。

### L79 · function · tests create_room_http_contract_roundtrip

```rust
fn create_room_http_contract_roundtrip ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.to_string`；`.expect`；`serde_json :: to_string`；`.expect`；`serde_json :: from_str`；`assert_eq!`；`.to_string`；`.expect`；`serde_json :: to_string`；`.expect`；`serde_json :: from_str`；`assert_eq!`。

## reference-rust_ai_chat/crates/telemetry/src/lib.rs · X05

職責：Telemetry。判定：產品擴充；沒有空模組功能等同；不以AST說明取代部署collector驗證

## reference-rust_ai_chat/crates/ui/src/chat_socket.rs · C07

職責：Chat UI。判定：映射頁面／生命週期，非逐檔移植；參考 hardcoded localhost WS 不適用部署

### L1 · use ·

```rust
use shared :: { ClientEvent , ServerEvent } ;
```

### L3 · function ·  decode_server_event

```rust
fn decode_server_event (text : & str) -> Result < ServerEvent , serde_json :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`serde_json :: from_str :: < ServerEvent >`。

### L7 · module ·  browser

```rust
 mod browser (inline=True)
```

### L9 · use ·

```rust
use wasm_bindgen :: { JsCast , closure :: Closure } ;
```

### L10 · use ·

```rust
use web_sys :: { MessageEvent , WebSocket } ;
```

### L12 · use ·

```rust
use super :: * ;
```

### L14 · struct ·  ChatSocket

```rust
pub struct ChatSocket { socket : WebSocket , _onmessage : Closure < dyn FnMut (MessageEvent) > , }
```

### L20 · function · browser::impl ChatSocket connect

```rust
fn connect < F > (url : & str , mut on_event : F) -> Result < Self , String > where F : 'static + FnMut (ServerEvent) ,
```

可見性／屬性：`pub`；``。

函式內呼叫：`.map_err`；`WebSocket :: new`；`format!`；`Closure :: wrap`；`Box :: new`；`.as_string`；`.data`；`web_sys :: console :: error_1`；`.into`；`format!`；`decode_server_event`；`on_event`；`web_sys :: console :: error_1`；`.into`；`format!`；`.set_onmessage`；`Some`；`.unchecked_ref`；`.as_ref`；`Ok`。

### L51 · function · browser::impl ChatSocket send

```rust
fn send (& self , event : & ClientEvent) -> Result < () , String >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.map_err`；`serde_json :: to_string`；`format!`；`.map_err`；`.send_with_str`；`format!`。

### L61 · function · browser::impl Drop for ChatSocket drop

```rust
fn drop (& mut self)
```

可見性／屬性：``；``。

函式內呼叫：`.set_onmessage`；`.close`。

### L68 · use ·

```rust
# [cfg (target_arch = "wasm32")] pub use browser :: ChatSocket ;
```

### L71 · struct ·  ChatSocket

```rust
# [cfg (not (target_arch = "wasm32"))] pub struct ChatSocket ;
```

### L76 · function · impl ChatSocket server_stub

```rust
fn server_stub () -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L80 · function · impl ChatSocket send

```rust
fn send (& self , _event : & ClientEvent) -> Result < () , String >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Err`；`.into`。

### L85 · module ·  tests

```rust
 mod tests (inline=True)
```

### L87 · use ·

```rust
use super :: * ;
```

### L90 · function · tests decodes_pong_event

```rust
fn decodes_pong_event ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.expect`；`decode_server_event`；`assert!`。

## reference-rust_ai_chat/crates/ui/src/lib.rs · C07

職責：Chat UI。判定：映射頁面／生命週期，非逐檔移植；參考 hardcoded localhost WS 不適用部署

### L1 · module ·  chat_socket

```rust
 mod chat_socket (inline=False)
```

### L3 · module ·  room_api

```rust
 mod room_api (inline=False)
```

### L6 · use ·

```rust
use std :: rc :: Rc ;
```

### L8 · use ·

```rust
use chat_domain :: { ChatMessage , MessageAuthor } ;
```

### L9 · use ·

```rust
use leptos :: prelude :: * ;
```

### L10 · use ·

```rust
use leptos_router :: { components :: { Route , Router , Routes } , hooks :: { use_navigate , use_params_map } , path , } ;
```

### L15 · use ·

```rust
use shared :: { ClientEvent , ServerEvent } ;
```

### L17 · use ·

```rust
use chat_socket :: ChatSocket ;
```

### L19 · function ·  author_label

```rust
fn author_label (author : & MessageAuthor) -> String
```

可見性／屬性：``；``。

函式內呼叫：`.to_string`；`format!`；`format!`；`.to_string`。

### L28 · function ·  shell

```rust
fn shell (options : LeptosOptions) -> impl IntoView
```

可見性／屬性：`pub`；``。

函式內呼叫：`view!`。

### L47 · function ·  hydrate

```rust
fn hydrate ()
```

可見性／屬性：`pub`；`# [cfg (feature = "hydrate")] # [wasm_bindgen :: prelude :: wasm_bindgen]`。

函式內呼叫：`leptos :: mount :: hydrate_body`。

### L52 · function ·  App

```rust
fn App () -> impl IntoView
```

可見性／屬性：`pub`；`# [component]`。

函式內呼叫：`view!`。

### L69 · function ·  HomePage

```rust
fn HomePage () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`use_navigate`；`signal`；`signal`；`.get_untracked`；`.set`；`.set`；`.clone`；`wasm_bindgen_futures :: spawn_local`；`room_api :: create_room`；`format!`；`navigate`；`Default :: default`；`.set`；`Some`；`.set`；`view!`。

### L121 · function ·  RoomPage

```rust
fn RoomPage () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`use_params_map`；`.get`；`.read`；`.into_any`；`view!`；`.into_any`；`view!`。

### L135 · function ·  ChatPage

```rust
fn ChatPage (room_id : String) -> impl IntoView
```

可見性／屬性：`pub`；`# [component]`。

函式內呼叫：`signal`；`String :: new`；`signal`；`Vec :: < ChatMessage > :: new`；`format!`；`Rc :: new`；`.expect`；`ChatSocket :: connect`；`.update`；`.push`；`Rc :: new`；`ChatSocket :: server_stub`；`Rc :: clone`；`.clone`；`.to_string`；`.trim`；`.get_untracked`；`.is_empty`；`.is_ok`；`.send`；`.set`；`String :: new`；`view!`。

## reference-rust_ai_chat/crates/ui/src/room_api.rs · C07

職責：Chat UI。判定：映射頁面／生命週期，非逐檔移植；參考 hardcoded localhost WS 不適用部署

### L1 · use ·

```rust
use shared :: { CreateRoomRequest , CreateRoomResponse } ;
```

### L2 · use ·

```rust
use wasm_bindgen :: { JsCast , JsValue } ;
```

### L3 · use ·

```rust
use wasm_bindgen_futures :: JsFuture ;
```

### L4 · use ·

```rust
use web_sys :: { Request , RequestInit , RequestMode } ;
```

### L6 · function ·  create_room

```rust
async fn create_room (name : & str) -> Result < CreateRoomResponse , String >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.map_err`；`serde_json :: to_string`；`.to_string`；`format!`；`RequestInit :: new`；`.set_method`；`.set_mode`；`.set_body`；`JsValue :: from_str`；`.map_err`；`Request :: new_with_str_and_init`；`format!`；`.map_err`；`.set`；`.headers`；`format!`；`.ok_or_else`；`web_sys :: window`；`.to_string`；`.map_err`；`JsFuture :: from`；`.fetch_with_request`；`format!`；`.map_err`；`.dyn_into`；`.to_string`；`.ok`；`Err`；`format!`；`.ok_or_else`；`.as_string`；`.map_err`；`JsFuture :: from`；`.map_err`；`.text`；`format!`；`format!`；`.to_string`；`.map_err`；`serde_json :: from_str :: < CreateRoomResponse >`；`format!`。

## reference-rust_ai_chat/tests/analytics/event_kind_tests.rs · A01

職責：Analytics reference契約。判定：參考未實作不是成品應退回501；strict event whitelist不等同任意非空kind

### L1 · use ·

```rust
use axum :: Json ;
```

### L2 · use ·

```rust
use axum :: http :: StatusCode ;
```

### L3 · use ·

```rust
use stats_server :: { EventInput , ingest , valid_event , valid_event_type } ;
```

### L4 · use ·

```rust
use uuid :: Uuid ;
```

### L7 · function ·  accepts_normal_kind

```rust
fn accepts_normal_kind ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`。

### L12 · function ·  rejects_blank_or_long_kind

```rust
fn rejects_blank_or_long_kind ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`；`assert!`；`assert!`。

### L18 · function ·  input

```rust
fn input (source : & str , kind : & str) -> EventInput
```

可見性／屬性：``；``。

函式內呼叫：`Uuid :: new_v4`；`.into`；`.into`。

### L28 · function ·  validates_event_inputs

```rust
fn validates_event_inputs ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`；`assert!`；`assert!`；`assert!`；`input`；`assert!`。

### L39 · function ·  valid_event_remains_unimplemented

```rust
async fn valid_event_remains_unimplemented ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`assert_eq!`。

### L47 · function ·  invalid_event_is_rejected

```rust
async fn invalid_event_is_rejected ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`assert_eq!`。

## reference-rust_ai_chat/tests/analytics/http_contract_tests.rs · A01

職責：Analytics reference契約。判定：參考未實作不是成品應退回501；strict event whitelist不等同任意非空kind

### L1 · use ·

```rust
use axum :: { body :: Body , http :: { Request , StatusCode } , } ;
```

### L5 · use ·

```rust
use stats_server :: build_app ;
```

### L6 · use ·

```rust
use tower :: ServiceExt ;
```

### L8 · const ·  ID

```rust
const ID : & str = "00000000-0000-4000-8000-000000000001" ;
```

### L10 · function ·  post

```rust
fn post (json : & str) -> Request < Body >
```

可見性／屬性：``；``。

函式內呼叫：`.unwrap`；`.body`；`.header`；`.uri`；`.method`；`Request :: builder`；`Body :: from`；`.to_owned`。

### L19 · function ·  response_status

```rust
async fn response_status (req : Request < Body >) -> StatusCode
```

可見性／屬性：``；``。

函式內呼叫：`.status`；`.unwrap`；`.oneshot`；`build_app`。

### L24 · function ·  health_is_200

```rust
async fn health_is_200 ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.unwrap`；`.body`；`.uri`；`Request :: builder`；`Body :: empty`；`assert_eq!`。

### L33 · function ·  valid_event_still_returns_501

```rust
async fn valid_event_still_returns_501 ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`format!`；`assert_eq!`。

### L45 · function ·  invalid_source_is_400

```rust
async fn invalid_source_is_400 ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`format!`；`assert_eq!`。

### L54 · function ·  unknown_field_is_422

```rust
async fn unknown_field_is_422 ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`format!`；`assert_eq!`。

### L66 · function ·  malformed_json_is_400

```rust
async fn malformed_json_is_400 ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`assert_eq!`。

### L71 · function ·  payload_over_4k_is_413

```rust
async fn payload_over_4k_is_413 ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`format!`；`assert_eq!`。

### L84 · function ·  unsupported_schema_version_returns_400

```rust
async fn unsupported_schema_version_returns_400 ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`format!`；`assert_eq!`。

### L94 · function ·  missing_schema_version_returns_422

```rust
async fn missing_schema_version_returns_422 ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`format!`；`assert_eq!`。

## reference-rust_ai_chat/tests/analytics/idempotency_contract.rs · A01

職責：Analytics reference契約。判定：參考未實作不是成品應退回501；strict event whitelist不等同任意非空kind

### L1 · use ·

```rust
use stats_server :: { EventIdentity , InMemoryDeduplicator } ;
```

### L2 · use ·

```rust
use std :: collections :: HashSet ;
```

### L4 · struct ·  Key

```rust
# [derive (Debug , Clone , PartialEq , Eq , Hash)] struct Key { source : String , id : String , }
```

### L9 · trait ·  EventStore

```rust
trait EventStore { fn insert_once (& mut self , key : Key) -> bool ; }
```

### L10 · function · EventStore insert_once

```rust
fn insert_once (& mut self , key : Key) -> bool
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L12 · struct ·  FakeStore

```rust
# [derive (Default)] struct FakeStore { seen : HashSet < Key > , }
```

### L17 · function · impl EventStore for FakeStore insert_once

```rust
fn insert_once (& mut self , key : Key) -> bool
```

可見性／屬性：``；``。

函式內呼叫：`.insert`。

### L23 · function ·  duplicate_is_counted_once_across_retries

```rust
fn duplicate_is_counted_once_across_retries ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`FakeStore :: default`；`.into`；`.into`；`assert!`；`assert!`；`assert!`。

### L38 · function ·  event_identity_deduplicates_per_source

```rust
fn event_identity_deduplicates_per_source ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`InMemoryDeduplicator :: default`；`assert!`；`assert!`；`assert!`；`assert!`。

## reference-rust_ai_chat/tests/analytics/postgres_ingest.rs · A01

職責：Analytics reference契約。判定：參考未實作不是成品應退回501；strict event whitelist不等同任意非空kind

## reference-rust_ai_chat/tests/chat/persist_publish.rs · C13

職責：Chat persist/publish 回歸。判定：測試目的映射，保留 owning crate tests，不建立根 tests workspace

### L1 · use ·

```rust
use async_trait :: async_trait ;
```

### L2 · use ·

```rust
use chat_application :: repository :: { MessageRepository , RepositoryError } ;
```

### L3 · use ·

```rust
use chat_application :: { ChatService , persist_then_publish } ;
```

### L4 · use ·

```rust
use chat_domain :: { ChatMessage , MessageAuthor } ;
```

### L5 · use ·

```rust
use std :: sync :: Mutex ;
```

### L8 · struct ·  FakeRepo

```rust
struct FakeRepo { fail : bool , saved : Mutex < Vec < ChatMessage > > , }
```

### L15 · function · impl MessageRepository for FakeRepo create

```rust
async fn create (& self , message : & ChatMessage) -> Result < () , RepositoryError >
```

可見性／屬性：``；``。

函式內呼叫：`Err`；`RepositoryError :: MessageAccessError`；`.into`；`.push`；`.unwrap`；`.lock`；`.clone`；`Ok`。

### L25 · function ·  message

```rust
fn message () -> ChatMessage
```

可見性／屬性：``；``。

函式內呼叫：`.into`；`.into`；`.into`。

### L35 · function ·  success_persists_before_publish_once

```rust
async fn success_persists_before_publish_once ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`Mutex :: new`；`vec!`；`Mutex :: new`；`Vec :: new`；`message`；`.expect`；`persist_then_publish`；`.clone`；`assert_eq!`；`.push`；`.unwrap`；`.lock`；`assert_eq!`；`assert_eq!`。

### L57 · function ·  failed_persistence_never_publishes

```rust
async fn failed_persistence_never_publishes ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`Mutex :: new`；`vec!`；`Mutex :: new`；`Vec :: < ChatMessage > :: new`；`persist_then_publish`；`message`；`.push`；`.unwrap`；`.lock`；`assert!`；`assert!`；`assert!`。

### L79 · function ·  send_message_creates_user_message

```rust
fn send_message_creates_user_message ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`ChatService :: new`；`.to_string`；`.send_message`；`.clone`；`.to_string`；`assert_eq!`；`assert_eq!`；`assert_eq!`。

## reference-rust_ai_chat/tests/chat/product_event_contract.rs · C03

職責：Chat 事件。判定：四欄事件子集可適配，不能宣稱所有事件／發送方式相同

### L1 · use ·

```rust
use async_trait :: async_trait ;
```

### L2 · use ·

```rust
use chat_application :: { persist_then_publish , product_event :: persisted_message_event , repository :: { MessageRepository , RepositoryError } , } ;
```

### L7 · use ·

```rust
use chat_domain :: { ChatMessage , MessageAuthor } ;
```

### L8 · use ·

```rust
use std :: sync :: Mutex ;
```

### L9 · use ·

```rust
use uuid :: Uuid ;
```

### L11 · struct ·  FakeRepository

```rust
struct FakeRepository { fail : bool , saved : Mutex < Vec < ChatMessage > > , }
```

### L18 · function · impl MessageRepository for FakeRepository create

```rust
async fn create (& self , message : & ChatMessage) -> Result < () , RepositoryError >
```

可見性／屬性：``；``。

函式內呼叫：`Err`；`RepositoryError :: MessageAccessError`；`.into`；`.push`；`.unwrap`；`.lock`；`.clone`；`Ok`。

### L27 · function ·  message

```rust
fn message () -> ChatMessage
```

可見性／屬性：``；``。

函式內呼叫：`.to_string`；`Uuid :: from_u128`；`.into`；`.into`。

### L37 · function ·  success_generates_stable_minimal_event_only_after_persist

```rust
async fn success_generates_stable_minimal_event_only_after_persist ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`Mutex :: new`；`vec!`；`Mutex :: new`；`Vec :: new`；`message`；`.unwrap`；`persist_then_publish`；`.clone`；`assert_eq!`；`.push`；`.unwrap`；`.lock`；`.unwrap`；`persisted_message_event`；`.unwrap`；`.lock`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`。

### L63 · function ·  database_failure_never_generates_event

```rust
async fn database_failure_never_generates_event ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`Mutex :: new`；`vec!`；`Mutex :: new`；`persist_then_publish`；`message`；`.is_ok`；`persisted_message_event`；`.unwrap`；`.lock`；`assert!`；`assert_eq!`。

### L80 · function ·  same_saved_message_keeps_same_event_id_and_does_not_leak_content

```rust
fn same_saved_message_keeps_same_event_id_and_does_not_leak_content ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`message`；`.unwrap`；`persisted_message_event`；`.unwrap`；`persisted_message_event`；`assert_eq!`；`format!`；`assert!`；`assert!`。

### L91 · function ·  invalid_message_id_is_rejected_not_replaced

```rust
fn invalid_message_id_is_rejected_not_replaced ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`message`；`.into`；`assert!`。

## reference-rust_ai_chat/tests/common/mod.rs · A01

職責：Analytics reference契約。判定：參考未實作不是成品應退回501；strict event whitelist不等同任意非空kind

## reference-rust_ai_chat/tests/pet/cleanup_contract.rs · P03

職責：Pet cleanup Domain。判定：事件名稱不相容；對照成品 P08，不以狀態成功代替真操作成功

### L1 · use ·

```rust
use pet_domain :: cleanup :: { CleanupError , CleanupOperation } ;
```

### L2 · use ·

```rust
use pet_domain :: { OperationState , count_as_cleanup } ;
```

### L5 · function ·  preview_cannot_be_counted_or_completed

```rust
fn preview_cannot_be_counted_or_completed ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`CleanupOperation :: new`；`assert!`；`assert_eq!`。

### L18 · function ·  confirmed_success_counts_only_after_completion

```rust
fn confirmed_success_counts_only_after_completion ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`CleanupOperation :: new`；`.expect`；`.confirm`；`assert!`；`.expect`；`.complete`；`assert!`；`assert_eq!`。

### L32 · function ·  failed_operation_never_counts

```rust
fn failed_operation_never_counts ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`CleanupOperation :: new`；`.expect`；`.confirm`；`.expect`；`.complete`；`assert!`。

### L42 · function ·  four_legacy_operation_states_preserve_counting_rule

```rust
fn four_legacy_operation_states_preserve_counting_rule ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`；`assert!`；`assert!`；`assert!`。

## reference-rust_ai_chat/tests/pet/cleanup_event_contract.rs · P03

職責：Pet cleanup Domain。判定：事件名稱不相容；對照成品 P08，不以狀態成功代替真操作成功

### L1 · use ·

```rust
use pet_domain :: cleanup :: CleanupOperation ;
```

### L2 · use ·

```rust
use pet_domain :: cleanup_event :: succeeded_event ;
```

### L3 · use ·

```rust
use uuid :: Uuid ;
```

### L5 · function ·  operation_id

```rust
fn operation_id () -> String
```

可見性／屬性：``；``。

函式內呼叫：`.to_string`；`Uuid :: from_u128`。

### L10 · function ·  preview_is_not_a_success_event

```rust
fn preview_is_not_a_success_event ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`CleanupOperation :: new`；`assert_eq!`。

### L16 · function ·  confirmation_without_filesystem_completion_is_not_success

```rust
fn confirmation_without_filesystem_completion_is_not_success ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`CleanupOperation :: new`；`.unwrap`；`.confirm`；`assert_eq!`。

### L23 · function ·  failed_operation_is_not_counted

```rust
fn failed_operation_is_not_counted ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`CleanupOperation :: new`；`.unwrap`；`.confirm`；`.unwrap`；`.complete`；`assert_eq!`。

### L31 · function ·  succeeded_operation_produces_stable_minimal_event

```rust
fn succeeded_operation_produces_stable_minimal_event ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`CleanupOperation :: new`；`.unwrap`；`.confirm`；`.unwrap`；`.complete`；`.unwrap`；`.unwrap`；`succeeded_event`；`operation_id`；`.unwrap`；`.unwrap`；`succeeded_event`；`operation_id`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert!`。

### L47 · function ·  invalid_operation_id_never_falls_back_to_new_random_id

```rust
fn invalid_operation_id_never_falls_back_to_new_random_id ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`CleanupOperation :: new`；`.unwrap`；`.confirm`；`.unwrap`；`.complete`；`assert!`。

## reference-rust_ai_chat/tests/pet/day5_codec_tests.rs · P02

職責：Pet Profile。判定：概念對應，格式/驗證/錯誤不等同；ProjectionPending 必須保留

### L1 · use ·

```rust
use pet_domain :: profile :: { PetProfile , ProfileCodecError , decode_profile , encode_profile } ;
```

### L3 · const ·  VALID

```rust
const VALID : & str = r#"{"名字":"小啾","種類":"鳥","喜好":["水果"],"個性":["活潑"],"興趣":["音樂"]}"# ;
```

### L7 · function ·  valid_five_fields_round_trip

```rust
fn valid_five_fields_round_trip ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.unwrap`；`decode_profile`；`.unwrap`；`encode_profile`；`.unwrap`；`decode_profile`；`assert_eq!`；`assert!`。

### L16 · function ·  rejects_blank_name_on_decode

```rust
fn rejects_blank_name_on_decode ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.replace`；`assert!`。

### L25 · function ·  rejects_invalid_json

```rust
fn rejects_invalid_json ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`。

### L33 · function ·  optional_appearance_is_preserved

```rust
fn optional_appearance_is_preserved ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.unwrap`；`decode_profile`；`.unwrap`；`encode_profile`；`.unwrap`；`decode_profile`；`assert_eq!`。

### L43 · function ·  invalid_profile_cannot_be_encoded

```rust
fn invalid_profile_cannot_be_encoded ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.unwrap`；`decode_profile`；`.into`；`assert!`。

## reference-rust_ai_chat/tests/pet/pet_machine.rs · P01

職責：Pet FSM。判定：狀態與命令不相容；Sleep/Wake/revision 目前無等同流程，不能空造同名型別

### L1 · use ·

```rust
use pet_domain :: { PetCommand , PetMachine , PetSnapshot , PetState , error :: PetError } ;
```

### L4 · function ·  pet_starts_idle

```rust
fn pet_starts_idle ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`PetMachine :: default`；`assert_eq!`。

### L17 · function ·  interact_moves_idle_to_interacting

```rust
fn interact_moves_idle_to_interacting ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`PetMachine :: default`；`.unwrap`；`.dispatch`；`assert_eq!`；`assert_eq!`。

### L27 · function ·  sleeping_pet_cannot_interact

```rust
fn sleeping_pet_cannot_interact ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`PetMachine :: default`；`.unwrap`；`.dispatch`；`.snapshot`；`.dispatch`；`assert!`；`assert_eq!`。

### L42 · function ·  wake_moves_sleeping_pet_back_to_idle

```rust
fn wake_moves_sleeping_pet_back_to_idle ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`PetMachine :: default`；`.unwrap`；`.dispatch`；`.unwrap`；`.dispatch`；`assert_eq!`；`assert_eq!`。

## reference-rust_ai_chat/tests/pet/profile_contract.rs · P02

職責：Pet Profile。判定：概念對應，格式/驗證/錯誤不等同；ProjectionPending 必須保留

### L1 · use ·

```rust
use pet_domain :: profile :: PetProfile ;
```

### L3 · function ·  sample

```rust
fn sample () -> & 'static str
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L10 · function ·  only_five_fields_are_enough

```rust
fn only_five_fields_are_enough ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.unwrap`；`serde_json :: from_str`；`sample`；`.unwrap`；`.validate`；`assert_eq!`；`assert_eq!`；`assert!`。

### L19 · function ·  optional_override_keeps_other_fields_auto

```rust
fn optional_override_keeps_other_fields_auto ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.unwrap`；`serde_json :: from_str`；`.unwrap`；`.validate`；`.unwrap`；`assert_eq!`；`assert!`。

### L30 · function ·  missing_or_blank_fields_fail

```rust
fn missing_or_blank_fields_fail ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`；`.unwrap`；`serde_json :: from_str`；`sample`；`.into`；`assert!`；`.into`；`.push`；`.into`；`assert!`。

### L42 · function ·  species_is_extensible

```rust
fn species_is_extensible ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.replace`；`sample`；`.unwrap`；`serde_json :: from_str`；`.unwrap`；`.validate`；`assert_eq!`。

## reference-rust_ai_chat/tests/server/day5_tests.rs · C05

職責：Chat 入口/HTTP/SSR。判定：API 不相容；產品入口保留，教學需明示 adapter/URL/DTO

### L1 · use ·

```rust
use server :: hub :: RoomHub ;
```

### L2 · use ·

```rust
use shared :: ServerEvent ;
```

### L3 · use ·

```rust
use tokio :: sync :: broadcast :: error :: TryRecvError ;
```

### L6 · function ·  message_stays_inside_room

```rust
fn message_stays_inside_room ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`RoomHub :: new`；`.subscribe`；`.into`；`.subscribe`；`.into`；`.publish`；`.into`；`assert_eq!`；`assert!`。

### L16 · function ·  subscribers_share_events

```rust
fn subscribers_share_events ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`RoomHub :: new`；`.subscribe`；`.into`；`.subscribe`；`.into`；`.publish`；`.into`；`assert_eq!`；`assert_eq!`。

### L26 · function ·  last_receiver_cleanup_allows_new_subscription

```rust
fn last_receiver_cleanup_allows_new_subscription ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`RoomHub :: new`；`.subscribe`；`.into`；`drop`；`.cleanup`；`.into`；`.subscribe`；`.into`；`.publish`；`.into`；`assert_eq!`。

### L37 · function ·  count_tracks_receiver_lifecycle

```rust
fn count_tracks_receiver_lifecycle ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`RoomHub :: new`；`assert_eq!`；`.subscribe`；`.into`；`assert_eq!`；`drop`；`.cleanup`；`.into`；`assert_eq!`。

## copilot_use/apps/desktop_pet/build.rs · P04

職責：Pet Tauri 入口。判定：40 frontend command names 對應44 backend；非參考 bridge/DTO 等同；詳列呼叫端與簽章

### L1 · function ·  main

```rust
fn main ()
```

可見性／屬性：``；``。

函式內呼叫：`.and_then`；`.filter`；`.ok`；`.output`；`.args`；`std :: process :: Command :: new`；`.success`；`.ok`；`String :: from_utf8`；`.unwrap_or_else`；`git`；`.into`；`.unwrap_or`；`.map`；`git`；`.is_empty`；`.trim`；`println!`；`println!`；`git`；`println!`；`println!`；`println!`；`println!`；`tauri_build :: build`。

## copilot_use/apps/desktop_pet/frontend/src/main.rs · P05

職責：Pet Frontend。判定：前端 schema/狀態不同；保留既有功能，不能直接替换參考 component

### L2 · use ·

```rust
use leptos :: prelude :: * ;
```

### L3 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L4 · use ·

```rust
use wasm_bindgen :: prelude :: * ;
```

### L5 · foreign_interface ·

```rust
# [wasm_bindgen] extern "C" { # [wasm_bindgen (js_namespace = ["window" , "__TAURI__" , "core"] , js_name = invoke)] fn invoke (command : & str , args : JsValue) -> js_sys :: Promise ; # [wasm_bindgen (js_namespace = ["window" , "crypto"] , js_name = randomUUID)] fn random_uuid () -> String ; # [wasm_bindgen (js_namespace = ["window" , "__TAURI__" , "window"] , js_name = getCurrentWindow)] fn current_window () -> JsValue ; # [wasm_bindgen (js_namespace = ["window" , "__TAURI__" , "event"] , js_name = listen)] fn listen (event : & str , handler : & js_sys :: Function) -> js_sys :: Promise ; # [wasm_bindgen (js_namespace = ["window" , "__TAURI__" , "event"] , js_name = emit)] fn emit_event (event : & str , payload : JsValue) -> js_sys :: Promise ; }
```

### L18 · function ·  call

```rust
async fn call (command : & str , args : Value) -> Result < Value , String >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`js_sys :: JSON :: parse`；`.to_string`；`.to_owned`；`.map_err`；`wasm_bindgen_futures :: JsFuture :: from`；`invoke`；`.unwrap_or_else`；`.as_string`；`.into`；`.unwrap_or`；`.as_string`；`.map_err`；`js_sys :: JSON :: stringify`；`.to_owned`；`.into`；`.map_err`；`serde_json :: from_str`；`.into`。

### L29 · function ·  uuid

```rust
fn uuid () -> String
```

可見性／屬性：``；``。

函式內呼叫：`random_uuid`。

### L32 · function ·  strings

```rust
fn strings (s : String) -> Vec < String >
```

可見性／屬性：``；``。

函式內呼叫：`.collect`；`.map`；`.filter`；`.map`；`.split`；`.is_empty`。

### L39 · function ·  fit_sprite

```rust
fn fit_sprite (width : f64 , height : f64) -> (f64 , f64)
```

可見性／屬性：``；``。

函式內呼叫：`.is_finite`；`.is_finite`；`.max`。

### L46 · function ·  animation_y

```rust
fn animation_y (value : Value , height : f64) -> String
```

可見性／屬性：``；``。

函式內呼叫：`.unwrap_or`；`.as_str`；`format!`。

### L56 · function ·  SpriteView

```rust
fn SpriteView (image : RwSignal < String > , animation : RwSignal < Value > , shape : RwSignal < (f64 , f64) > ,) -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`view!`。

### L63 · function ·  animation_duration

```rust
fn animation_duration (value : Value) -> String
```

可見性／屬性：``；``。

函式內呼叫：`.into`；`.unwrap_or`；`.as_str`。

### L72 · function ·  RecallPrompt

```rust
fn RecallPrompt () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`RwSignal :: new`；`String :: new`；`leptos :: task :: spawn_local`；`Closure :: < dyn FnMut (JsValue) > :: new`；`js_sys :: JSON :: stringify`；`.as_string`；`serde_json :: from_str :: < Value >`；`.as_str`；`.set`；`.into`；`wasm_bindgen_futures :: JsFuture :: from`；`listen`；`.unchecked_ref`；`.as_ref`；`.forget`；`leptos :: task :: spawn_local`；`call`；`json!`；`.as_str`；`.set`；`.into`；`js_sys :: JSON :: parse`；`.to_string`；`wasm_bindgen_futures :: JsFuture :: from`；`emit_event`；`.ok`；`leptos :: leptos_dom :: helpers :: set_interval_with_handle`；`std :: time :: Duration :: from_secs`；`on_cleanup`；`.clear`；`view!`。

### L120 · function ·  BackupControls

```rust
fn BackupControls () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`RwSignal :: new`；`Vec :: < Value > :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`leptos :: task :: spawn_local`；`call`；`json!`；`.set`；`.unwrap_or_default`；`.cloned`；`.as_array`；`.set`；`view!`。

### L143 · function ·  UpdateControls

```rust
fn UpdateControls () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`RwSignal :: new`；`RwSignal :: new`；`RwSignal :: new`；`String :: new`；`view!`。

### L156 · function ·  Settings

```rust
fn Settings () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`RwSignal :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`RwSignal :: new`；`json!`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`RwSignal :: new`；`RwSignal :: new`；`String :: new`；`leptos :: task :: spawn_local`；`Closure :: < dyn FnMut (JsValue) > :: new`；`js_sys :: JSON :: stringify`；`.as_string`；`serde_json :: from_str :: < Value >`；`.as_str`；`Some`；`.as_str`；`.get_untracked`；`.to_owned`；`.unwrap_or`；`.as_str`；`.update`；`.push_str`；`wasm_bindgen_futures :: JsFuture :: from`；`listen`；`.unchecked_ref`；`.as_ref`；`.forget`；`RwSignal :: new`；`Vec :: < Value > :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`Vec :: < Value > :: new`；`RwSignal :: new`；`Vec :: < Value > :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`leptos :: task :: spawn_local`；`call`；`json!`；`.set`；`.unwrap_or_default`；`.cloned`；`.as_array`；`.set`；`refresh`；`.set`；`.clone`；`.set`；`Vec :: new`；`.set`；`Vec :: new`；`.set`；`String :: new`；`.set`；`String :: new`；`.set`；`uuid`；`.set`；`String :: new`；`.set`；`String :: new`；`.set`；`String :: new`；`.find`；`.iter`；`.get`；`serde_json :: from_str :: < Value >`；`.unwrap_or`；`.as_str`；`.set`；`.into`；`.unwrap_or`；`.as_str`；`.set`；`.into`；`.unwrap_or`；`.as_str`；`.unwrap_or_default`；`.map`；`.as_array`；`.join`；`.collect`；`.filter_map`；`.iter`；`.set`；`join`；`.set`；`join`；`.set`；`join`；`.set`；`.into`；`.unwrap_or`；`.as_str`；`.set`；`.into`；`.unwrap_or`；`.as_str`；`.set`；`.into`；`.unwrap_or`；`.as_str`；`.set`；`.into`；`.unwrap_or`；`.as_str`；`.set`；`.unwrap_or_default`；`.map`；`.as_array`；`.join`；`.collect`；`.filter_map`；`.iter`；`leptos :: task :: spawn_local`；`call`；`json!`；`.get_untracked`；`call`；`json!`；`.as_str`；`.get_untracked`；`.set`；`fit_sprite`；`.unwrap_or`；`.as_f64`；`.unwrap_or`；`.as_f64`；`.set`；`format!`；`.get_untracked`；`call`；`json!`；`.get_untracked`；`.set`；`call`；`json!`；`.get_untracked`；`.set`；`.unwrap_or_default`；`.cloned`；`.as_array`；`.get_untracked`；`.set`；`call`；`json!`；`.get_untracked`；`.set`；`.to_owned`；`.unwrap_or`；`.as_str`；`call`；`json!`；`.get_untracked`；`.set`；`.unwrap_or_default`；`.cloned`；`.as_array`；`.get_untracked`；`.set`；`view!`。

### L354 · function ·  SuggestionPolicy

```rust
fn SuggestionPolicy (pet : RwSignal < String >) -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`RwSignal :: new`；`String :: new`；`view!`。

### L365 · function ·  FileOrganizer

```rust
fn FileOrganizer () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`RwSignal :: new`；`Vec :: < Value > :: new`；`RwSignal :: new`；`String :: new`；`leptos :: task :: spawn_local`；`call`；`json!`；`.as_str`；`.set`；`.into`；`leptos :: task :: spawn_local`；`call`；`json!`；`.set`；`.unwrap_or_default`；`.cloned`；`.as_array`；`.set`；`.into`；`.set`；`view!`。

### L431 · function ·  TrashBrowser

```rust
fn TrashBrowser () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`RwSignal :: new`；`Vec :: < Value > :: new`；`RwSignal :: new`；`String :: new`；`leptos :: task :: spawn_local`；`call`；`json!`；`.set`；`.unwrap_or_default`；`.cloned`；`.as_array`；`.set`；`view!`。

### L453 · function ·  PetArchive

```rust
fn PetArchive (pet : RwSignal < String > , changed : Callback < Option < String > >) -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`RwSignal :: new`；`RwSignal :: new`；`Vec :: < Value > :: new`；`RwSignal :: new`；`String :: new`；`leptos :: task :: spawn_local`；`call`；`json!`；`.set`；`.unwrap_or_default`；`.cloned`；`.as_array`；`.set`；`refresh`；`view!`。

### L475 · function ·  PetOnly

```rust
fn PetOnly () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`RwSignal :: new`；`json!`；`RwSignal :: new`；`.update`；`.wrapping_add`；`.get_untracked`；`.set`；`String :: new`；`leptos :: task :: spawn_local`；`call`；`json!`；`.as_str`；`call`；`json!`；`.as_str`；`.get_untracked`；`.set`；`fit_sprite`；`.unwrap_or`；`.as_f64`；`.unwrap_or`；`.as_f64`；`.set`；`format!`；`call`；`json!`；`.get_untracked`；`.set`；`refresh`；`leptos :: task :: spawn_local`；`Closure :: < dyn FnMut (JsValue) > :: new`；`refresh`；`wasm_bindgen_futures :: JsFuture :: from`；`listen`；`.unchecked_ref`；`.as_ref`；`.forget`；`view!`。

### L519 · function ·  App

```rust
fn App () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`.as_deref`；`.and_then`；`.ok`；`js_sys :: Reflect :: get`；`current_window`；`.into`；`.as_string`；`Some`；`.into_any`；`view!`；`.into_any`；`view!`。

### L531 · function ·  main

```rust
fn main ()
```

可見性／屬性：``；``。

函式內呼叫：`leptos :: mount :: mount_to_body`。

### L535 · module ·  tests

```rust
 mod tests (inline=True)
```

### L537 · use ·

```rust
use super :: * ;
```

### L539 · function · tests nonsquare_sheets_keep_aspect_inside_fixed_viewport_and_invalid_dimensions_fall_back

```rust
fn nonsquare_sheets_keep_aspect_inside_fixed_viewport_and_invalid_dimensions_fall_back ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`。

## copilot_use/apps/desktop_pet/src/main.rs · P04

職責：Pet Tauri 入口。判定：40 frontend command names 對應44 backend；非參考 bridge/DTO 等同；詳列呼叫端與簽章

### L2 · module ·  updater

```rust
 mod updater (inline=False)
```

### L3 · use ·

```rust
use day9_pet :: { profile :: PetProfile , state :: { Card , Memory , PetItem , PetStore } , } ;
```

### L7 · use ·

```rust
use tauri :: { Emitter , Manager , State } ;
```

### L8 · use ·

```rust
use uuid :: Uuid ;
```

### L9 · struct ·  DesktopState

```rust
struct DesktopState { store : PetStore , ai_slots : std :: sync :: Arc < tokio :: sync :: Semaphore > , organizer : std :: sync :: Mutex < Option < std :: sync :: Arc < day9_pet :: AuthorizedRoot > > > , previews : std :: sync :: Mutex < std :: collections :: HashMap < Uuid , day9_pet :: FilePreview > > , }
```

### L16 · function · impl DesktopState authorized_root

```rust
fn authorized_root (& self) -> Result < std :: sync :: Arc < day9_pet :: AuthorizedRoot > , String >
```

可見性／屬性：``；``。

函式內呼叫：`.ok_or_else`；`.clone`；`.map_err`；`.lock`；`.into`。

### L23 · function · impl DesktopState confirmed_preview

```rust
fn confirmed_preview (& self , id : Uuid ,) -> Result < (std :: sync :: Arc < day9_pet :: AuthorizedRoot > , day9_pet :: FilePreview ,) , String , >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`.lock`；`.ok_or`；`.clone`；`.ok_or`；`.remove`；`.map_err`；`.lock`；`Ok`。

### L45 · function ·  authorized_directory

```rust
fn authorized_directory (state : State < '_ , DesktopState >) -> Result < Option < String > , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`Ok`；`.map`；`.as_ref`；`.map_err`；`.lock`；`.into_owned`；`.to_string_lossy`；`.authorized_path`。

### L54 · function ·  authorize_directory

```rust
fn authorize_directory (state : State < '_ , DesktopState > , path : String , confirmed : bool ,) -> Result < String , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.len`；`.is_absolute`；`std :: path :: Path :: new`；`Err`；`.into`；`.map_err`；`day9_pet :: AuthorizedRoot :: new`；`.into_owned`；`.to_string_lossy`；`.authorized_path`；`.map_err`；`.lock`；`.clear`；`.map_err`；`.lock`；`Some`；`std :: sync :: Arc :: new`；`Ok`。

### L71 · function ·  revoke_directory

```rust
fn revoke_directory (state : State < '_ , DesktopState >) -> Result < () , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.lock`；`.clear`；`.map_err`；`.lock`；`Ok`。

### L77 · function ·  analytics_client

```rust
fn analytics_client () -> Result < (reqwest :: Client , String , String) , String >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`std :: env :: var`；`.to_owned`；`.ok_or`；`.strip_suffix`；`.map_err`；`std :: env :: var`；`.map_err`；`.build`；`.timeout`；`.redirect`；`.no_proxy`；`reqwest :: Client :: builder`；`reqwest :: redirect :: Policy :: none`；`std :: time :: Duration :: from_secs`；`Ok`。

### L93 · function ·  apply_analytics_suggestion

```rust
async fn apply_analytics_suggestion (state : State < '_ , DesktopState > , pet : Uuid , id : Uuid , confirmed : bool ,) -> Result < serde_json :: Value , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`Err`；`.into`；`analytics_client`；`.map_err`；`.json`；`.map_err`；`.error_for_status`；`.map_err`；`.send`；`.bearer_auth`；`.get`；`format!`；`.map_err`；`.apply_suggestion`；`.is_ok_and`；`.send`；`.json`；`.bearer_auth`；`.post`；`format!`；`serde_json :: json!`；`.is_success`；`.status`；`serde_json :: json!`；`Ok`。

### L126 · function ·  rollback_analytics_suggestion

```rust
async fn rollback_analytics_suggestion (state : State < '_ , DesktopState > , pet : Uuid , id : Uuid , confirmed : bool ,) -> Result < serde_json :: Value , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`Err`；`.into`；`analytics_client`；`.map_err`；`.rollback_suggestion`；`.is_ok_and`；`.send`；`.json`；`.bearer_auth`；`.post`；`format!`；`serde_json :: json!`；`.is_success`；`.status`；`serde_json :: json!`；`Ok`。

### L149 · function ·  create_snapshot

```rust
async fn create_snapshot (state : State < '_ , DesktopState > ,) -> Result < day9_pet :: backup :: Snapshot , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.create_snapshot`；`.into`。

### L159 · function ·  list_snapshots

```rust
fn list_snapshots (state : State < '_ , DesktopState > ,) -> Result < Vec < day9_pet :: backup :: Snapshot > , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.snapshots`；`.into`。

### L168 · function ·  restore_snapshot

```rust
async fn restore_snapshot (app : tauri :: AppHandle , state : State < '_ , DesktopState > , id : Uuid , confirmed : bool ,) -> Result < day9_pet :: backup :: Snapshot , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.restore_snapshot`；`.to_string`；`.to_owned`；`.to_owned`；`.emit`；`Ok`。

### L189 · function ·  interaction

```rust
async fn interaction (state : State < '_ , DesktopState > , pet : Uuid ,) -> Result < day9_pet :: interaction :: Interaction , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.interaction`；`.into`。

### L200 · function ·  set_interaction

```rust
async fn set_interaction (app : tauri :: AppHandle , state : State < '_ , DesktopState > , pet : Uuid , preferences : day9_pet :: interaction :: Interaction ,) -> Result < () , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.set_interaction`；`.to_owned`；`.map_err`；`.emit`；`.into`。

### L215 · function ·  learning_summary

```rust
async fn learning_summary (state : State < '_ , DesktopState > , pet : Uuid ,) -> Result < day9_pet :: interaction :: LearningSummary , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.learning_summary`；`.into`。

### L226 · function ·  recall_prompt

```rust
async fn recall_prompt (state : State < '_ , DesktopState > ,) -> Result < Option < serde_json :: Value > , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.recall_prompt`；`.into`。

### L236 · function ·  list_pets

```rust
async fn list_pets (state : State < '_ , DesktopState >) -> Result < Vec < PetItem > , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.pets`；`.into`。

### L240 · function ·  save_profile

```rust
async fn save_profile (state : State < '_ , DesktopState > , id : Uuid , profile : PetProfile ,) -> Result < () , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`day9_pet :: application :: save_profile`；`.into`；`.into`；`.into`。

### L256 · function ·  archived_pets

```rust
async fn archived_pets (state : State < '_ , DesktopState >) -> Result < Vec < PetItem > , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.archived_pets`；`.into`。

### L264 · function ·  set_pet_archived

```rust
async fn set_pet_archived (app : tauri :: AppHandle , state : State < '_ , DesktopState > , pet : Uuid , archived : bool , confirmed : bool ,) -> Result < serde_json :: Value , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.set_pet_archived`；`.to_owned`；`.map_err`；`.emit`；`serde_json :: json!`；`Ok`。

### L281 · function ·  profile_file

```rust
async fn profile_file (state : State < '_ , DesktopState > , pet : Uuid) -> Result < String , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.is_nil`；`.any`；`.iter`；`.map_err`；`.pets`；`.to_string`；`Err`；`.into`；`Ok`；`.into_owned`；`.to_string_lossy`；`.join`；`.join`；`.join`；`.ok_or`；`.parent`；`.database_path`；`.to_string`。

### L305 · function ·  import_pet_profile

```rust
async fn import_pet_profile (state : State < '_ , DesktopState > , pet : Uuid , confirmed : bool ,) -> Result < () , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.import_pet_profile`；`.into`。

### L317 · function ·  list_memories

```rust
async fn list_memories (state : State < '_ , DesktopState > , pet : Uuid) -> Result < Vec < Memory > , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.memories`；`.into`。

### L325 · function ·  save_memory

```rust
async fn save_memory (state : State < '_ , DesktopState > , pet : Uuid , id : Uuid , content : String ,) -> Result < () , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.save_memory`；`.into`。

### L338 · function ·  delete_memory

```rust
async fn delete_memory (state : State < '_ , DesktopState > , pet : Uuid , id : Uuid) -> Result < () , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.delete_memory`；`.into`。

### L346 · function ·  list_cards

```rust
async fn list_cards (state : State < '_ , DesktopState > , pet : Uuid) -> Result < Vec < Card > , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.cards`；`.into`。

### L354 · function ·  add_card

```rust
async fn add_card (state : State < '_ , DesktopState > , pet : Uuid , question : String , answer : String ,) -> Result < () , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.add_card`；`.into`。

### L367 · function ·  review_card

```rust
async fn review_card (state : State < '_ , DesktopState > , pet : Uuid , id : Uuid , quality : u8 ,) -> Result < Card , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`copilot_telemetry :: observe`；`.map_err`；`.review`；`.into`。

### L383 · function ·  pet_talk

```rust
async fn pet_talk (app : tauri :: AppHandle , state : State < '_ , DesktopState > , pet : Uuid , request : Uuid , text : String ,) -> Result < String , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.try_acquire_owned`；`.clone`；`.map_err`；`copilot_telemetry :: observe`；`.talk`；`.emit`；`serde_json :: json!`；`.to_string`。

### L408 · function ·  search_memory_tool

```rust
async fn search_memory_tool (state : State < '_ , DesktopState > , pet : Uuid , query : String ,) -> Result < serde_json :: Value , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.len`；`.is_empty`；`.trim`；`Err`；`.into`；`.map_err`；`std :: env :: current_exe`；`.map_err`；`.stdio_memory_search`；`.into`。

### L424 · function ·  pet_agent

```rust
async fn pet_agent (state : State < '_ , DesktopState > , pet : Uuid , text : String ,) -> Result < serde_json :: Value , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.try_acquire_owned`；`.clone`；`.map_err`；`std :: env :: current_exe`；`.map_err`；`copilot_telemetry :: observe`；`.agent_talk`；`.into`。

### L439 · function ·  mcp_once

```rust
fn mcp_once () -> Result < () , Box < dyn std :: error :: Error > >
```

可見性／屬性：``；``。

函式內呼叫：`.ok_or`；`std :: env :: var_os`；`.parse`；`std :: env :: var`；`Vec :: new`；`.read_until`；`.take`；`.lock`；`std :: io :: stdin`；`.len`；`Err`；`.into`；`serde_json :: from_slice`；`.build`；`.enable_all`；`tokio :: runtime :: Builder :: new_current_thread`；`.block_on`；`PetStore :: open_readonly`；`std :: path :: Path :: new`；`Ok :: < _ , Box < dyn std :: error :: Error > >`；`.mcp_request`；`.write_all`；`.lock`；`std :: io :: stdout`；`.as_bytes`；`format!`；`Ok`。

### L465 · function ·  generate_asset

```rust
async fn generate_asset (app : tauri :: AppHandle , state : State < '_ , DesktopState > , pet : Uuid ,) -> Result < serde_json :: Value , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.try_acquire_owned`；`.clone`；`.map_err`；`.generate_asset`；`.to_string`；`.emit`；`Ok`。

### L484 · function ·  load_asset

```rust
async fn load_asset (state : State < '_ , DesktopState > , pet : Uuid ,) -> Result < serde_json :: Value , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.asset`；`.into`。

### L495 · function ·  scan_file_preview

```rust
fn scan_file_preview (state : State < '_ , DesktopState >) -> Result < Vec < serde_json :: Value > , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.lock`；`.ok_or`；`.as_ref`；`.map_err`；`.scan`；`.map_err`；`.lock`；`.clear`；`Vec :: new`；`Uuid :: new_v4`；`.push`；`serde_json :: json!`；`.insert`；`Ok`。

### L511 · function ·  confirm_cleanup

```rust
async fn confirm_cleanup (state : State < '_ , DesktopState > , id : Uuid , confirmed : bool ,) -> Result < day9_pet :: CleanupReceipt , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`copilot_telemetry :: observe`；`Err`；`.into`；`.confirmed_preview`；`.map_err`；`.confirm_quarantine`；`.to_string`；`.event`；`.map_err`；`.record_cleanup`；`format!`；`Ok`。

### L540 · function ·  recover_cleanup_receipts

```rust
async fn recover_cleanup_receipts (state : State < '_ , DesktopState > ,) -> Result < Vec < day9_pet :: CleanupReceipt > , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`copilot_telemetry :: observe`；`.authorized_root`；`.map_err`；`.recovered_receipts`；`.map_err`；`.record_cleanup`；`.event`；`Ok`。

### L560 · function ·  list_trash

```rust
fn list_trash (state : State < '_ , DesktopState >) -> Result < Vec < day9_pet :: TrashEntry > , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.trash`；`.authorized_root`；`.into`。

### L567 · function ·  restore_file

```rust
async fn restore_file (state : State < '_ , DesktopState > , id : Uuid , confirmed : bool ,) -> Result < day9_pet :: RestoreReceipt , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`Err`；`.into`；`copilot_telemetry :: observe`；`.authorized_root`；`.ok_or`；`.find`；`.into_iter`；`.map_err`；`.trash`；`format!`；`.map_err`；`.record_cleanup`；`.event`；`.map_err`；`.confirm_restore`；`.to_string`；`.map_err`；`.record_restoration`；`format!`；`Ok`。

### L604 · function ·  recover_restore_receipts

```rust
async fn recover_restore_receipts (state : State < '_ , DesktopState > ,) -> Result < Vec < day9_pet :: RestoreReceipt > , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.authorized_root`；`.map_err`；`.restored_receipts`；`.map_err`；`.record_restoration`；`Ok`。

### L621 · function ·  outbox_status

```rust
async fn outbox_status (state : State < '_ , DesktopState >) -> Result < serde_json :: Value , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.outbox_status`；`.into`。

### L629 · function ·  requeue_failed_events

```rust
async fn requeue_failed_events (state : State < '_ , DesktopState >) -> Result < u64 , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.requeue_blocked_events`；`.into`。

### L637 · function ·  select_pet

```rust
async fn select_pet (app : tauri :: AppHandle , state : State < '_ , DesktopState > , pet : Uuid ,) -> Result < () , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.select_pet`；`.map_err`；`.emit`；`Ok`。

### L652 · function ·  active_pet

```rust
async fn active_pet (state : State < '_ , DesktopState >) -> Result < Option < String > , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.active_pet`；`.into`。

### L659 · function ·  gui_acceptance_enabled

```rust
fn gui_acceptance_enabled () -> bool
```

可見性／屬性：``；``。

函式內呼叫：`.as_deref`；`std :: env :: var`；`Ok`；`cfg!`；`.any`；`std :: env :: args`。

### L663 · function ·  validate_acceptance_fixture

```rust
fn validate_acceptance_fixture () -> Result < () , String >
```

可見性／屬性：``；``。

函式內呼叫：`gui_acceptance_enabled`；`Ok`；`std :: path :: PathBuf :: from`；`.ok_or`；`std :: env :: var_os`；`std :: path :: PathBuf :: from`；`.ok_or`；`std :: env :: var_os`；`std :: path :: PathBuf :: from`；`.ok_or`；`std :: env :: var_os`；`.ok_or`；`.parent`；`.exists`；`.exists`；`.parent`；`Some`；`.parent`；`Some`；`.is_symlink`；`.file_type`；`.map_err`；`std :: fs :: symlink_metadata`；`.map_err`；`std :: fs :: read`；`.join`；`.map_err`；`std :: fs :: read`；`.join`；`Err`；`.into`；`Ok`。

### L693 · function ·  gui_test_progress

```rust
fn gui_test_progress (phase : String) -> Result < () , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`gui_acceptance_enabled`；`.len`；`Err`；`.into`；`.map_err`；`std :: env :: var`；`.map_err`；`std :: fs :: write`；`format!`；`.to_owned`。

### L701 · function ·  gui_test_result

```rust
fn gui_test_result (app : tauri :: AppHandle , success : bool , message : String) -> Result < () , String >
```

可見性／屬性：``；`# [tauri :: command]`。

函式內呼叫：`gui_acceptance_enabled`；`Err`；`.into`；`.ok_or`；`.get_webview_window`；`.map_err`；`.inner_size`；`.map_err`；`.scale_factor`；`.abs`；`.abs`；`Err`；`format!`；`.map_err`；`std :: env :: var`；`.map_err`；`std :: fs :: write`；`.to_string`；`serde_json :: json!`；`.exit`；`Ok`。

### L727 · function ·  main

```rust
fn main ()
```

可見性／屬性：``；``。

函式內呼叫：`.any`；`std :: env :: args`；`.is_err`；`mcp_once`；`std :: process :: exit`；`tauri :: generate_context!`；`.any`；`std :: env :: args`；`println!`；`validate_acceptance_fixture`；`eprintln!`；`std :: process :: exit`；`.expect`；`copilot_telemetry :: init`；`libc :: geteuid`；`eprintln!`；`std :: process :: exit`；`.expect`；`.run`；`.invoke_handler`；`.setup`；`.on_page_load`；`.manage`；`.plugin`；`tauri :: Builder :: default`；`.build`；`tauri_plugin_updater :: Builder :: new`；`updater :: Updates :: new`；`.label`；`gui_acceptance_enabled`；`matches!`；`.as_deref`；`std :: env :: var`；`Ok`；`.eval`；`.as_deref`；`std :: env :: var`；`Ok`；`.eval`；`.as_deref`；`std :: env :: var`；`Ok`；`.eval`；`.as_deref`；`std :: env :: var`；`Ok`；`.eval`；`.as_deref`；`std :: env :: var`；`Ok`；`.eval`；`.and_then`；`.ok`；`std :: env :: var`；`.ok`；`Uuid :: parse_str`；`.eval`；`format!`；`.eval`；`include_str!`；`.get_webview_window`；`.set_titlebar`；`.gtk_window`；`.set_size`；`tauri :: LogicalSize :: new`；`.unwrap_or`；`.map`；`std :: env :: var_os`；`.app_data_dir`；`.path`；`std :: fs :: create_dir_all`；`std :: fs :: set_permissions`；`std :: fs :: Permissions :: from_mode`；`.map_err`；`day9_pet :: files :: prepare_data_directory`；`std :: io :: Error :: other`；`tauri :: async_runtime :: block_on`；`PetStore :: open`；`.join`；`.map_err`；`tauri :: async_runtime :: block_on`；`.reconcile_pet_files_startup`；`std :: io :: Error :: other`；`std :: env :: var`；`std :: env :: var`；`.len`；`.starts_with`；`.starts_with`；`.starts_with`；`Err`；`.into`；`.clone`；`tauri :: async_runtime :: spawn`；`.dispatch_once`；`tokio :: time :: sleep`；`std :: time :: Duration :: from_millis`；`.manage`；`std :: sync :: Arc :: new`；`tokio :: sync :: Semaphore :: new`；`std :: sync :: Mutex :: new`；`.map`；`.and_then`；`std :: env :: var_os`；`.ok`；`day9_pet :: AuthorizedRoot :: new`；`std :: sync :: Mutex :: new`；`std :: collections :: HashMap :: new`；`Ok`；`tauri :: generate_handler!`。

## copilot_use/apps/desktop_pet/src/updater.rs · P13

職責：Pet 更新/平台回歸。判定：無參考 counterpart；Windows/macOS runtime 本輪未重跑；vendor列獨立清單

### L2 · use ·

```rust
use base64 :: Engine ;
```

### L3 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L4 · use ·

```rust
use std :: { sync :: Mutex , time :: Duration } ;
```

### L5 · use ·

```rust
use tauri :: { Emitter , State } ;
```

### L6 · use ·

```rust
use tauri_plugin_updater :: { Update , UpdaterExt } ;
```

### L7 · use ·

```rust
use uuid :: Uuid ;
```

### L8 · struct ·  Pending

```rust
struct Pending { id : Uuid , update : Update , key : String , bytes : Option < Vec < u8 > > , }
```

### L14 · struct ·  Updates

```rust
pub struct Updates { pending : Mutex < Option < Pending > > , slots : tokio :: sync :: Semaphore , }
```

### L19 · function · impl Updates new

```rust
fn new () -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：`Mutex :: new`；`tokio :: sync :: Semaphore :: new`。

### L26 · function ·  local

```rust
fn local (url : & url :: Url) -> bool
```

可見性／屬性：``；``。

函式內呼叫：`matches!`；`.is_empty`；`.username`；`.is_none`；`.password`；`.is_none`；`.fragment`；`.is_none`；`.query`；`.host`；`.is_loopback`；`.is_loopback`。

### L38 · function ·  verify

```rust
fn verify (bytes : & [u8] , signature : & str , key : & str , version : & str) -> Result < () , String >
```

可見性／屬性：``；``。

函式內呼叫：`.ok_or`；`.and_then`；`.ok`；`.decode`；`.ok`；`String :: from_utf8`；`.to_owned`；`.map_err`；`minisign_verify :: PublicKey :: decode`；`decode`；`.map_err`；`minisign_verify :: Signature :: decode`；`decode`；`.map_err`；`.verify`；`.ok_or`；`.find_map`；`.split`；`.trusted_comment`；`.strip_prefix`；`.map_err`；`semver :: Version :: parse`；`.trim_start_matches`；`parse`；`parse`；`Err`；`.into`；`Ok`。

### L64 · function ·  check_local_update

```rust
async fn check_local_update (app : tauri :: AppHandle , state : State < '_ , Updates > ,) -> Result < Value , String >
```

可見性／屬性：`pub`；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.try_acquire`；`.map_err`；`.lock`；`.map_err`；`url :: Url :: parse`；`.map_err`；`std :: env :: var`；`local`；`Err`；`.into`；`.map_err`；`std :: env :: var`；`.is_empty`；`.len`；`Err`；`.into`；`.configure_client`；`.timeout`；`.clear_headers`；`.no_proxy`；`.pubkey`；`.map_err`；`.endpoints`；`.updater_builder`；`vec!`；`Duration :: from_secs`；`.redirect`；`updater_http :: redirect :: Policy :: none`；`.map_err`；`std :: env :: current_exe`；`std :: ffi :: OsString :: from`；`.push`；`.ok_or`；`.parent`；`.installer_arg`；`.installer_arg`；`.clear_installer_args`；`.map_err`；`.build`；`.map_err`；`.check`；`Ok`；`json!`；`local`；`.origin`；`.origin`；`Err`；`.into`；`Uuid :: new_v4`；`json!`；`.map_err`；`.lock`；`Some`；`Ok`。

### L120 · function ·  verify_local_update

```rust
async fn verify_local_update (state : State < '_ , Updates > , id : Uuid) -> Result < Value , String >
```

可見性／屬性：`pub`；`# [tauri :: command]`。

函式內呼叫：`.map_err`；`.try_acquire`；`.map_err`；`.lock`；`.ok_or`；`.filter`；`.as_ref`；`.clone`；`.clone`；`.map_err`；`.error_for_status`；`.map_err`；`.send`；`.get`；`.map_err`；`.build`；`.timeout`；`.redirect`；`.no_proxy`；`updater_http :: Client :: builder`；`updater_http :: redirect :: Policy :: none`；`Duration :: from_secs`；`.clone`；`.is_some_and`；`.content_length`；`Err`；`.into`；`Vec :: new`；`.map_err`；`.chunk`；`.saturating_add`；`.len`；`.len`；`Err`；`.into`；`.extend_from_slice`；`verify`；`.len`；`.map_err`；`.lock`；`.ok_or`；`.filter`；`.as_mut`；`Some`；`Ok`；`json!`。

### L164 · function ·  install_local_update

```rust
async fn install_local_update (app : tauri :: AppHandle , state : State < '_ , Updates > , desktop : State < '_ , super :: DesktopState > , id : Uuid , confirmed : bool ,) -> Result < Value , String >
```

可見性／屬性：`pub`；`# [tauri :: command]`。

函式內呼叫：`Err`；`.into`；`.map_err`；`.try_acquire`；`.ok_or`；`.filter`；`.take`；`.map_err`；`.lock`；`.is_some`；`.map_err`；`.create_snapshot`；`.clone`；`.ok_or`；`.take`；`verify`；`super :: gui_acceptance_enabled`；`.as_deref`；`std :: env :: var`；`Ok`；`.map_err`；`std :: env :: var`；`.map_err`；`std :: fs :: write`；`.to_string`；`json!`；`.restart_after_install`；`.map_err`；`.map_err`；`tokio :: task :: spawn_blocking`；`.install`；`.emit_to`；`json!`；`Ok`；`json!`。

### L207 · module ·  tests

```rust
 mod tests (inline=True)
```

### L209 · use ·

```rust
use super :: * ;
```

### L211 · function · tests only_numeric_loopback_urls_are_accepted

```rust
fn only_numeric_loopback_urls_are_accepted ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`；`assert!`；`assert!`。

## copilot_use/apps/desktop_pet/tests/glib_variant_safety.rs · P13

職責：Pet 更新/平台回歸。判定：無參考 counterpart；Windows/macOS runtime 本輪未重跑；vendor列獨立清單

### L2 · use ·

```rust
use glib :: variant :: ToVariant ;
```

### L4 · function ·  optimized_variant_strings_use_mutable_out_pointer

```rust
fn optimized_variant_strings_use_mutable_out_pointer ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.to_variant`；`vec!`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`。

## copilot_use/apps/server/frontend/src/main.rs · C05

職責：Chat 入口/HTTP/SSR。判定：API 不相容；產品入口保留，教學需明示 adapter/URL/DTO

### L1 · function ·  main

```rust
fn main ()
```

可見性／屬性：``；``。

函式內呼叫：`chat_ui :: hydrate`。

## copilot_use/apps/server/src/agent.rs · C09

職責：Chat RAG/MCP/Agent。判定：產品擴充；Agent 外部呼叫後有授權複核，不可概括到所有 AI 入口

### L2 · use ·

```rust
use super :: { StateData , auth } ;
```

### L3 · use ·

```rust
use axum :: { Json , extract :: State , http :: { HeaderMap , StatusCode } , } ;
```

### L8 · use ·

```rust
use serde :: Deserialize ;
```

### L9 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L10 · use ·

```rust
use std :: sync :: { Arc , Mutex } ;
```

### L11 · struct ·  Input

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct Input { room_id : String , question : String , }
```

### L17 · function ·  reply

```rust
async fn reply (State (state) : State < StateData > , headers : HeaderMap , Json (input) : Json < Input > ,) -> Result < Json < Value > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`auth :: principal`；`auth :: room_allowed`；`.is_empty`；`.trim`；`.len`；`.contains`；`Err`；`.map_err`；`.try_acquire_owned`；`.clone`；`HeaderMap :: new`；`.get`；`.insert`；`.clone`；`.insert`；`http :: HeaderValue :: from_static`；`copilot_telemetry :: outgoing_headers`；`.insert`；`.map_err`；`.parse`；`.map_err`；`mcp_bridge :: HttpClient :: new`；`.map_err`；`.call`；`json!`；`.ok_or`；`.cloned`；`.as_array`；`Arc :: new`；`Mutex :: new`；`Vec :: < String > :: new`；`.clone`；`.map_err`；`.map_err`；`tokio :: time :: timeout`；`std :: time :: Duration :: from_secs`；`openai_adapter :: agent :: run`；`json!`；`.clone`；`.clone`；`.map_err`；`.call`；`json!`；`Err`；`.clone`；`.as_array`；`.map_err`；`.lock`；`.as_str`；`.push`；`.into`；`Ok`；`.map_err`；`.lock`；`.any`；`.iter`；`.ok_or`；`.as_array`；`.any`；`.iter`；`.as_str`；`Some`；`Err`；`.to_owned`；`.ok_or`；`.as_str`；`auth :: principal`；`auth :: room_allowed`；`.map_err`；`.persist_as`；`day9_chat :: ChatWriter :: new`；`.clone`；`Some`；`Ok`；`Json`；`json!`。

## copilot_use/apps/server/src/auth.rs · C08

職責：Chat 認證。判定：產品安全擴充，不刪除以追求 reference 相同

### L2 · use ·

```rust
use super :: { StateData , authorized } ;
```

### L3 · use ·

```rust
use argon2 :: { Argon2 , PasswordHasher , PasswordVerifier , password_hash :: { PasswordHash , SaltString } , } ;
```

### L7 · use ·

```rust
use axum :: { Json , extract :: State , http :: { HeaderMap , StatusCode , header } , response :: IntoResponse , } ;
```

### L13 · use ·

```rust
use serde :: Deserialize ;
```

### L14 · use ·

```rust
use sha2 :: { Digest , Sha256 } ;
```

### L15 · use ·

```rust
use uuid :: Uuid ;
```

### L16 · enum ·  Principal

```rust
# [derive (Clone , Copy)] pub (super) enum Principal { Admin , User (Uuid) , }
```

### L22 · function · impl Principal user_id

```rust
fn user_id (self) -> Option < Uuid >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Some`。

### L29 · struct ·  Credentials

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct Credentials { username : String , password : String , }
```

### L35 · function ·  csrf

```rust
fn csrf (headers : & HeaderMap) -> Result < () , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`.and_then`；`.get`；`.ok`；`.to_str`；`Some`；`Err`；`Ok`。

### L41 · function ·  principal

```rust
async fn principal (state : & StateData , headers : & HeaderMap , write : bool ,) -> Result < Principal , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`authorized`；`Ok`；`.ok_or`；`.and_then`；`.and_then`；`.get`；`.ok`；`.to_str`；`.find_map`；`.split`；`.strip_prefix`；`.trim`；`.len`；`.all`；`.bytes`；`.is_ascii_hexdigit`；`Err`；`csrf`；`.map_err`；`.fetch_optional`；`.bind`；`sqlx :: query_scalar`；`.to_vec`；`Sha256 :: digest`；`.as_bytes`；`.ok_or`；`.map`。

### L72 · function ·  room_allowed

```rust
async fn room_allowed (state : & StateData , principal : Principal , room : & str ,) -> Result < () , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`matches!`；`Ok`；`.map_err`；`.fetch_one`；`.bind`；`.bind`；`sqlx :: query_scalar`；`.user_id`；`Ok`；`Err`。

### L94 · function ·  validate

```rust
fn validate (input : & Credentials) -> Result < () , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`.len`；`.len`；`.all`；`.chars`；`.is_alphanumeric`；`.len`；`.len`；`Err`；`Ok`。

### L109 · function ·  throttle

```rust
async fn throttle (state : & StateData , operation : & str , username : & str) -> Result < () , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`.begin_with`；`.map_err`；`.execute`；`sqlx :: query`；`format!`；`format!`；`.map_err`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`.map_err`；`.commit`；`Err`；`.map_err`；`.commit`；`Ok`。

### L136 · function ·  register

```rust
async fn register (State (state) : State < StateData > , headers : HeaderMap , Json (input) : Json < Credentials > ,) -> Result < StatusCode , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`csrf`；`validate`；`.map_err`；`.try_acquire_owned`；`.clone`；`throttle`；`.map_err`；`.map_err`；`tokio :: task :: spawn_blocking`；`.map_err`；`SaltString :: encode_b64`；`.as_bytes`；`Uuid :: new_v4`；`.map_err`；`.map`；`.hash_password`；`Argon2 :: default`；`.as_bytes`；`.to_string`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`Uuid :: new_v4`；`Ok`；`.is_some_and`；`.as_database_error`；`.is_unique_violation`；`Err`；`Err`。

### L177 · function ·  login

```rust
async fn login (State (state) : State < StateData > , headers : HeaderMap , Json (input) : Json < Credentials > ,) -> Result < impl IntoResponse , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`csrf`；`validate`；`.map_err`；`.try_acquire_owned`；`.clone`；`throttle`；`.map_err`；`.fetch_optional`；`.bind`；`sqlx :: query_as`；`failed_login`；`Err`；`.map_err`；`tokio :: task :: spawn_blocking`；`.is_ok_and`；`PasswordHash :: new`；`.is_ok`；`.verify_password`；`Argon2 :: default`；`.as_bytes`；`failed_login`；`Err`；`format!`；`.map_err`；`.begin_with`；`.map_err`；`.execute`；`sqlx :: query`；`.map_err`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.to_vec`；`Sha256 :: digest`；`.as_bytes`；`auth_event`；`.map_err`；`.commit`；`.as_deref`；`std :: env :: var`；`Ok`；`Ok`；`format!`；`Json`；`serde_json :: json!`。

### L240 · function ·  logout

```rust
async fn logout (State (state) : State < StateData > , headers : HeaderMap ,) -> Result < impl IntoResponse , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`principal`；`.and_then`；`.and_then`；`.get`；`.ok`；`.to_str`；`.find_map`；`.split`；`.strip_prefix`；`.trim`；`.map_err`；`.begin_with`；`.map_err`；`.execute`；`.bind`；`sqlx :: query`；`.to_vec`；`Sha256 :: digest`；`.as_bytes`；`.rows_affected`；`auth_event`；`.map_err`；`.commit`；`Ok`。

### L278 · struct ·  Invitation

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct Invitation { room_id : String , username : String , }
```

### L284 · function ·  invite

```rust
async fn invite (State (state) : State < StateData > , headers : HeaderMap , Json (input) : Json < Invitation > ,) -> Result < StatusCode , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`principal`；`.flatten`；`.map_err`；`.fetch_optional`；`.bind`；`sqlx :: query_scalar`；`Some`；`Err`；`.ok_or`；`.map_err`；`.fetch_optional`；`.bind`；`sqlx :: query_scalar`；`.map_err`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`Ok`。

### L318 · function ·  failed_login

```rust
async fn failed_login (pool : & sqlx :: SqlitePool) -> Result < () , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`.begin_with`；`auth_event`；`.map_err`；`.commit`；`Ok`。

### L329 · function ·  auth_event

```rust
async fn auth_event (tx : & mut sqlx :: Transaction < '_ , sqlx :: Sqlite > , kind : & str ,) -> Result < () , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`Uuid :: new_v4`；`.get`；`copilot_telemetry :: outgoing_headers`；`Ok`。

## copilot_use/apps/server/src/fanout.rs · C06

職責：Chat WS/Fanout。判定：保留跨程序 durability/ACL；不能把參考 socket 當成品 client

### L2 · use ·

```rust
use super :: { StateData , auth , realtime :: RoomEvent } ;
```

### L3 · use ·

```rust
use axum :: { Json , extract :: { Path , State } , http :: { HeaderMap , StatusCode } , } ;
```

### L8 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L9 · use ·

```rust
use sqlx :: { Row , SqlitePool } ;
```

### L10 · use ·

```rust
use tokio :: sync :: broadcast ;
```

### L11 · use ·

```rust
use uuid :: Uuid ;
```

### L12 · function ·  start

```rust
async fn start (_url : & str , pool : SqlitePool , events : broadcast :: Sender < RoomEvent > ,) -> Result < tokio :: task :: JoinHandle < () > , sqlx :: Error >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`.fetch_one`；`sqlx :: query_scalar`；`Ok`；`tokio :: spawn`；`tokio :: time :: interval`；`std :: time :: Duration :: from_millis`；`.tick`；`.fetch_all`；`.bind`；`sqlx :: query`；`.send`；`.into`；`json!`；`.get`；`.send`；`.into`；`json!`；`serde_json :: from_str :: < Value >`；`.get`；`.map`；`.filter`；`.as_str`；`.is_empty`；`.len`；`.and_then`；`.as_str`；`.ok`；`Uuid :: parse_str`；`.fetch_optional`；`.bind`；`.bind`；`sqlx :: query_scalar`；`json!`；`json!`；`matches!`；`.is_some`；`.and_then`；`.as_str`；`.ok`；`Uuid :: parse_str`；`.remove`；`.unwrap`；`.as_object_mut`；`.send`；`.is_multiple_of`；`.execute`；`.bind`；`sqlx :: query`。

### L100 · function ·  publish

```rust
async fn publish (pool : & SqlitePool , room : & str , mut payload : Value ,) -> Result < () , sqlx :: Error >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`json!`；`.execute`；`.bind`；`sqlx :: query`；`.to_string`；`Ok`。

### L112 · function ·  join

```rust
async fn join (pool : & SqlitePool , room : & str , id : Uuid , user : Option < Uuid > ,) -> Result < () , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`.map_err`；`.begin_with`；`.map_err`；`.execute`；`sqlx :: query`；`.map_err`；`.execute`；`sqlx :: query`；`.map_err`；`.fetch_one`；`.bind`；`sqlx :: query_as`；`Err`；`.map_err`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.map_err`；`.commit`。

### L144 · function ·  snapshot

```rust
async fn snapshot (pool : & SqlitePool , room : & str) -> Result < Vec < Uuid > , sqlx :: Error >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`.fetch_all`；`.bind`；`sqlx :: query_scalar`。

### L147 · function ·  presence

```rust
async fn presence (State (state) : State < StateData > , headers : HeaderMap , Path (room) : Path < String > ,) -> Result < Json < Vec < Uuid > > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`auth :: principal`；`auth :: room_allowed`；`.map_err`；`.map`；`snapshot`。

## copilot_use/apps/server/src/knowledge.rs · C09

職責：Chat RAG/MCP/Agent。判定：產品擴充；Agent 外部呼叫後有授權複核，不可概括到所有 AI 入口

### L2 · use ·

```rust
use super :: { StateData , auth :: { self , Principal } , } ;
```

### L6 · use ·

```rust
use axum :: { Json , extract :: { Path , State } , http :: { HeaderMap , StatusCode } , } ;
```

### L11 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L13 · use ·

```rust
use uuid :: Uuid ;
```

### L14 · struct ·  Document

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct Document { title : String , content : String , }
```

### L20 · struct ·  Search

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct Search { query : String , }
```

### L25 · struct ·  Hit

```rust
# [derive (Serialize)] pub (super) struct Hit { pub id : Uuid , pub title : String , pub content : String , pub distance : f64 , }
```

### L32 · function ·  cosine_distance

```rust
fn cosine_distance (a : & [f32] , b : & [f32]) -> Option < f64 >
```

可見性／屬性：``；``。

函式內呼叫：`.len`；`.len`；`.any`；`.chain`；`.iter`；`.is_finite`；`.zip`；`.iter`；`f64 :: from`；`f64 :: from`；`Some`；`.clamp`；`.sqrt`；`.sqrt`。

### L49 · function ·  vector

```rust
fn vector (value : & [f32]) -> String
```

可見性／屬性：``；``。

函式內呼叫：`format!`。

### L59 · function ·  add

```rust
async fn add (State (state) : State < StateData > , headers : HeaderMap , Json (input) : Json < Document > ,) -> Result < (StatusCode , Json < serde_json :: Value >) , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`auth :: principal`；`.is_empty`；`.trim`；`.len`；`.is_empty`；`.trim`；`.len`；`Err`；`.map_err`；`.try_acquire_owned`；`.clone`；`.map_err`；`openai_adapter :: embedding`；`Uuid :: new_v4`；`.map_err`；`.begin_with`；`.map_err`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`.user_id`；`Err`；`.map_err`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.user_id`；`vector`；`.map_err`；`.commit`；`Ok`；`Json`；`serde_json :: json!`。

### L101 · function ·  list

```rust
async fn list (State (state) : State < StateData > , headers : HeaderMap ,) -> Result < Json < Vec < serde_json :: Value > > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`auth :: principal`；`.map_err`；`.fetch_all`；`.bind`；`sqlx :: query_as`；`.user_id`；`Ok`；`Json`；`.collect`；`.map`；`.into_iter`；`serde_json :: json!`。

### L113 · function ·  delete

```rust
async fn delete (State (state) : State < StateData > , headers : HeaderMap , Path (id) : Path < Uuid > ,) -> Result < StatusCode , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`auth :: principal`；`.map_err`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.user_id`；`.rows_affected`；`Ok`；`Err`。

### L131 · function ·  retrieve

```rust
async fn retrieve (state : & StateData , actor : Principal , query : & str ,) -> Result < Vec < Hit > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`.is_empty`；`.trim`；`.len`；`Err`；`.map_err`；`openai_adapter :: embedding`；`.map_err`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`.user_id`；`Err`；`.map_err`；`.fetch_all`；`.bind`；`sqlx :: query_as`；`.user_id`；`Vec :: with_capacity`；`.len`；`.map_err`；`serde_json :: from_str`；`.ok_or`；`cosine_distance`；`.push`；`.sort_by`；`.then_with`；`.total_cmp`；`.cmp`；`.truncate`；`Ok`。

### L174 · function ·  search

```rust
async fn search (State (state) : State < StateData > , headers : HeaderMap , Json (input) : Json < Search > ,) -> Result < Json < Vec < Hit > > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`auth :: principal`；`.map_err`；`.try_acquire_owned`；`.clone`；`Ok`；`Json`；`retrieve`。

### L188 · module ·  tests

```rust
 mod tests (inline=False)
```

## copilot_use/apps/server/src/knowledge/tests.rs · C09

職責：Chat RAG/MCP/Agent。判定：產品擴充；Agent 外部呼叫後有授權複核，不可概括到所有 AI 入口

### L1 · use ·

```rust
use super :: cosine_distance ;
```

### L3 · function ·  portable_cosine_retrieval_rejects_invalid_vectors_and_preserves_ranking

```rust
fn portable_cosine_retrieval_rejects_invalid_vectors_and_preserves_ranking ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`vec!`；`.clone`；`vec!`；`.collect`；`.map`；`.iter`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`.clone`；`assert_eq!`。

## copilot_use/apps/server/src/lib.rs · C05

職責：Chat 入口/HTTP/SSR。判定：API 不相容；產品入口保留，教學需明示 adapter/URL/DTO

### L2 · use ·

```rust
pub use chat_application as application ;
```

### L3 · use ·

```rust
pub use persistence :: * ;
```

## copilot_use/apps/server/src/main.rs · C05

職責：Chat 入口/HTTP/SSR。判定：API 不相容；產品入口保留，教學需明示 adapter/URL/DTO

### L2 · module ·  agent

```rust
 mod agent (inline=False)
```

### L3 · module ·  auth

```rust
 mod auth (inline=False)
```

### L4 · module ·  fanout

```rust
 mod fanout (inline=False)
```

### L5 · module ·  knowledge

```rust
 mod knowledge (inline=False)
```

### L6 · module ·  mcp

```rust
 mod mcp (inline=False)
```

### L7 · module ·  openai

```rust
 mod openai (inline=False)
```

### L8 · module ·  personas

```rust
 mod personas (inline=False)
```

### L9 · module ·  realtime

```rust
 mod realtime (inline=False)
```

### L10 · module ·  suggestions

```rust
 mod suggestions (inline=False)
```

### L11 · module ·  topic_http

```rust
 mod topic_http (inline=False)
```

### L12 · module ·  web

```rust
 mod web (inline=False)
```

### L13 · use ·

```rust
use axum :: { Json , Router , extract :: { Query , State } , http :: { HeaderMap , StatusCode , header :: AUTHORIZATION } , routing :: get , } ;
```

### L19 · use ·

```rust
use day9_chat :: { ChatWriter , run_dispatcher } ;
```

### L20 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L21 · use ·

```rust
use sqlx :: { FromRow , SqlitePool } ;
```

### L22 · use ·

```rust
use std :: { error :: Error , net :: SocketAddr , sync :: Arc } ;
```

### L24 · struct ·  StateData

```rust
# [derive (Clone)] struct StateData { pool : SqlitePool , api_token : Arc < str > , events : tokio :: sync :: broadcast :: Sender < realtime :: RoomEvent > , ai_slots : Arc < tokio :: sync :: Semaphore > , auth_slots : Arc < tokio :: sync :: Semaphore > , ws_slots : Arc < tokio :: sync :: Semaphore > , tool_slots : Arc < tokio :: sync :: Semaphore > , mcp_endpoint : Arc < str > , }
```

### L35 · struct ·  RoomRequest

```rust
# [derive (Deserialize)] struct RoomRequest { id : String , name : String , }
```

### L40 · struct ·  MessageRequest

```rust
# [derive (Deserialize)] struct MessageRequest { # [serde (default)] id : Option < uuid :: Uuid > , room_id : String , content : String , }
```

### L47 · struct ·  MessagesQuery

```rust
# [derive (Deserialize)] struct MessagesQuery { room_id : String , before : Option < uuid :: Uuid > , }
```

### L52 · struct ·  CreatedMessage

```rust
# [derive (Serialize)] struct CreatedMessage { id : String , }
```

### L56 · struct ·  RoomItem

```rust
# [derive (Serialize , FromRow)] struct RoomItem { id : String , name : String , can_manage : bool , }
```

### L62 · struct ·  MessageItem

```rust
# [derive (Serialize , FromRow)] struct MessageItem { id : uuid :: Uuid , content : String , }
```

### L68 · function ·  authorized

```rust
fn authorized (headers : & HeaderMap , token : & str) -> bool
```

可見性／屬性：``；``。

函式內呼叫：`.is_some_and`；`.and_then`；`.and_then`；`.get`；`.ok`；`.to_str`；`.strip_prefix`；`.is_empty`。

### L76 · function ·  health

```rust
async fn health (State (s) : State < StateData >) -> StatusCode
```

可見性／屬性：``；``。

函式內呼叫：`.execute`；`sqlx :: query`。

### L83 · function ·  list_rooms

```rust
async fn list_rooms (State (s) : State < StateData > , headers : HeaderMap ,) -> Result < Json < Vec < RoomItem > > , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`auth :: principal`；`.map_err`；`.fetch_all`；`.bind`；`sqlx :: query_as :: < _ , RoomItem >`；`.user_id`；`Ok`；`Json`。

### L98 · function ·  create_room

```rust
async fn create_room (State (s) : State < StateData > , headers : HeaderMap , Json (input) : Json < RoomRequest > ,) -> Result < StatusCode , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`auth :: principal`；`.is_empty`；`.len`；`.all`；`.chars`；`.is_ascii_alphanumeric`；`matches!`；`.is_empty`；`.trim`；`.len`；`.contains`；`Err`；`.map_err`；`.begin_with`；`.map_err`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.user_id`；`.is_some_and`；`.as_database_error`；`.is_unique_violation`；`.user_id`；`.map_err`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.map_err`；`.commit`；`Ok`。

### L150 · function ·  list_messages

```rust
async fn list_messages (State (s) : State < StateData > , headers : HeaderMap , Query (input) : Query < MessagesQuery > ,) -> Result < Json < Vec < MessageItem > > , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`auth :: principal`；`auth :: room_allowed`；`.is_empty`；`.len`；`Err`；`.map_err`；`.fetch_one`；`.bind`；`.bind`；`sqlx :: query_scalar`；`Err`；`.map_err`；`.fetch_all`；`.bind`；`.bind`；`sqlx :: query_as :: < _ , MessageItem >`；`Ok`；`Json`。

### L187 · function ·  send_message

```rust
async fn send_message (State (s) : State < StateData > , headers : HeaderMap , Json (input) : Json < MessageRequest > ,) -> Result < (StatusCode , Json < CreatedMessage >) , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`auth :: principal`；`auth :: room_allowed`；`.is_some_and`；`.is_nil`；`.is_empty`；`.len`；`.is_empty`；`.trim`；`.len`；`.contains`；`Err`；`.map`；`.user_id`；`.to_string`；`.map_err`；`day9_chat :: application :: persist`；`ChatWriter :: new`；`.clone`；`.unwrap_or_else`；`.is_some`；`.as_deref`；`Ok`；`Json`；`.to_string`。

### L237 · function ·  main

```rust
async fn main () -> Result < () , Box < dyn Error + Send + Sync > >
```

可見性／屬性：``；`# [tokio :: main]`。

函式內呼叫：`any_spawner :: Executor :: init_tokio`；`tokio :: task :: spawn_blocking`；`copilot_telemetry :: init`；`.unwrap_or_else`；`std :: env :: var`；`.into`；`.as_deref`；`.nth`；`std :: env :: args`；`Some`；`day9_chat :: database :: open`；`.close`；`Ok`；`.as_deref`；`.nth`；`std :: env :: args`；`Some`；`.ok_or`；`.nth`；`std :: env :: args`；`day9_chat :: database :: open`；`day9_chat :: database :: backup`；`std :: path :: Path :: new`；`.close`；`Ok`；`std :: env :: var`；`std :: env :: var`；`std :: env :: var`；`.is_ok_and`；`std :: env :: var`；`.len`；`.len`；`.starts_with`；`.starts_with`；`.starts_with`；`Err`；`.into`；`day9_chat :: database :: open`；`.clone`；`tokio :: spawn`；`run_dispatcher`；`eprintln!`；`tokio :: sync :: broadcast :: channel`；`fanout :: start`；`.clone`；`.clone`；`.parse`；`.unwrap_or_else`；`std :: env :: var`；`.into`；`tokio :: net :: TcpListener :: bind`；`.local_addr`；`.into`；`format!`；`.with_state`；`.layer`；`.layer`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.fallback`；`.nest_service`；`.route`；`Router :: new`；`get`；`tower_http :: services :: ServeDir :: new`；`.unwrap_or_else`；`std :: env :: var`；`.into`；`get`；`.post`；`get`；`axum :: routing :: post`；`axum :: routing :: post`；`axum :: routing :: post`；`axum :: routing :: post`；`axum :: routing :: post`；`get`；`get`；`.post`；`get`；`axum :: routing :: post`；`get`；`axum :: routing :: post`；`.post`；`get`；`axum :: routing :: delete`；`axum :: routing :: post`；`axum :: routing :: post`；`axum :: routing :: post`；`axum :: routing :: post`；`axum :: routing :: post`；`.post`；`get`；`.post`；`get`；`axum :: extract :: DefaultBodyLimit :: max`；`axum :: middleware :: from_fn`；`Arc :: from`；`Arc :: new`；`tokio :: sync :: Semaphore :: new`；`Arc :: new`；`tokio :: sync :: Semaphore :: new`；`Arc :: new`；`tokio :: sync :: Semaphore :: new`；`Arc :: new`；`tokio :: sync :: Semaphore :: new`；`println!`；`.with_graceful_shutdown`；`axum :: serve`；`copilot_telemetry :: shutdown_signal`；`.abort`；`.abort`；`tokio :: task :: spawn_blocking`；`drop`；`Ok`。

### L377 · module ·  tests

```rust
 mod tests (inline=True)
```

### L379 · use ·

```rust
use super :: * ;
```

### L381 · function · tests missing_bearer_token_is_denied

```rust
fn missing_bearer_token_is_denied ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`。

## copilot_use/apps/server/src/mcp.rs · C09

職責：Chat RAG/MCP/Agent。判定：產品擴充；Agent 外部呼叫後有授權複核，不可概括到所有 AI 入口

### L2 · use ·

```rust
use super :: { StateData , auth , knowledge } ;
```

### L3 · use ·

```rust
use axum :: { Json , extract :: State , http :: { HeaderMap , StatusCode } , } ;
```

### L8 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L9 · function ·  handler

```rust
async fn handler (State (state) : State < StateData > , headers : HeaderMap , Json (value) : Json < Value > ,) -> Result < (StatusCode , Json < Value >) , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`auth :: principal`；`.and_then`；`.get`；`.ok`；`.to_str`；`.map_err`；`url :: Url :: parse`；`.unwrap_or_default`；`.and_then`；`.get`；`.ok`；`.to_str`；`.unwrap_or_default`；`.map`；`.host_str`；`.port`；`format!`；`.into`；`matches!`；`Err`；`.clone`；`serde_json :: from_value`；`Ok`；`Json`；`mcp_bridge :: error`；`.validate_http`；`Ok`；`Json`；`mcp_bridge :: error`；`.as_str`；`mcp_bridge :: discover`；`json!`；`Ok`；`Json`；`mcp_bridge :: error`；`.is_none_or`；`.as_object`；`.len`；`.is_none_or`；`.as_str`；`.is_empty`；`.trim`；`.len`；`Ok`；`Json`；`mcp_bridge :: error`；`.map_err`；`.try_acquire_owned`；`.clone`；`knowledge :: retrieve`；`.unwrap`；`.as_str`；`.collect`；`.map`；`.into_iter`；`json!`；`mcp_bridge :: tool_result`；`json!`；`Ok`；`Json`；`mcp_bridge :: error`；`Ok`；`Json`；`mcp_bridge :: result`。

## copilot_use/apps/server/src/openai.rs · C10

職責：Chat AI SSE。判定：靜態缺口：produce 未攜 actor/headers，完成前未複核 session/ACL；尚未修正或實測撤權 race

### L2 · use ·

```rust
use super :: { StateData , auth } ;
```

### L3 · use ·

```rust
use axum :: { Json , extract :: State , http :: { HeaderMap , StatusCode } , response :: { IntoResponse , Response , sse :: { Event , KeepAlive , Sse } , } , } ;
```

### L12 · use ·

```rust
use day9_chat :: ChatWriter ;
```

### L13 · use ·

```rust
use serde :: Deserialize ;
```

### L14 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L15 · use ·

```rust
use std :: { convert :: Infallible , time :: Duration } ;
```

### L16 · use ·

```rust
use tokio_stream :: wrappers :: ReceiverStream ;
```

### L18 · struct ·  ReplyRequest

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct ReplyRequest { room_id : String , # [serde (default)] rag : bool , }
```

### L26 · use ·

```rust
use openai_adapter :: Decoder ;
```

### L28 · function ·  reply

```rust
async fn reply (State (state) : State < StateData > , headers : HeaderMap , Json (input) : Json < ReplyRequest > ,) -> Result < Response , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`auth :: principal`；`auth :: room_allowed`；`.is_empty`；`.len`；`Err`；`.map_err`；`std :: env :: var`；`.map_err`；`std :: env :: var`；`.is_empty`；`.is_empty`；`Err`；`.map_err`；`.try_acquire_owned`；`.clone`；`.map_err`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`.map_err`；`.fetch_all`；`.bind`；`.bind`；`sqlx :: query_as`；`.is_empty`；`Err`；`.collect`；`.map`；`.into_iter`；`json!`；`.len`；`.map_err`；`serde_json :: to_vec`；`Err`；`.to_owned`；`.unwrap_or`；`.and_then`；`.last`；`.as_str`；`super :: knowledge :: retrieve`；`.push_str`；`.push_str`；`format!`；`.len`；`Err`；`.map_err`；`.build`；`.redirect`；`.timeout`；`reqwest :: Client :: builder`；`Duration :: from_secs`；`reqwest :: redirect :: Policy :: none`；`.map_err`；`.send`；`.json`；`.bearer_auth`；`.post`；`json!`；`.is_success`；`.status`；`Err`；`tokio :: sync :: mpsc :: channel`；`tokio :: spawn`；`.is_err`；`produce`；`.send`；`Ok`；`.data`；`.event`；`Event :: default`；`Ok`；`.into_response`；`.keep_alive`；`Sse :: new`；`ReceiverStream :: new`；`KeepAlive :: default`。

### L116 · function ·  produce

```rust
async fn produce (mut response : reqwest :: Response , sender : & tokio :: sync :: mpsc :: Sender < Result < Event , Infallible > > , state : & StateData , room_id : & str , model : & str ,) -> Result < () , () >
```

可見性／屬性：``；``。

函式內呼叫：`Decoder :: default`；`String :: new`；`tokio :: select!`；`Err`；`.map_err`；`.push`；`.and_then`；`.get`；`.ok_or`；`.and_then`；`.get`；`.push_str`；`.len`；`Err`；`.map_err`；`.send`；`Ok`；`.map_err`；`.json_data`；`.event`；`Event :: default`；`json!`；`.is_empty`；`.trim`；`Err`；`.map_err`；`.persist_as`；`ChatWriter :: new`；`.clone`；`Some`；`.map_err`；`.send`；`Ok`；`.map_err`；`.json_data`；`.event`；`Event :: default`；`json!`；`Ok`；`Err`。

## copilot_use/apps/server/src/personas.rs · C11

職責：Chat personas/policy。判定：產品擴充，不能用一般 ChatMessage factory 取代

### L2 · use ·

```rust
use super :: { StateData , auth } ;
```

### L3 · use ·

```rust
use axum :: { Json , extract :: { Path , State } , http :: { HeaderMap , StatusCode } , } ;
```

### L8 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L9 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L10 · use ·

```rust
use sqlx :: FromRow ;
```

### L11 · use ·

```rust
use uuid :: Uuid ;
```

### L12 · struct ·  PersonaInput

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct PersonaInput { id : Uuid , name : String , instructions : String , memory : Option < String > , cooldown_seconds : i32 , enabled : bool , }
```

### L22 · struct ·  Persona

```rust
# [derive (Clone , Serialize , FromRow)] pub (super) struct Persona { id : Uuid , name : String , instructions : String , memory : String , cooldown_seconds : i32 , enabled : bool , revision : i64 , ready_at : i64 , }
```

### L33 · function ·  owner

```rust
async fn owner (state : & StateData , headers : & HeaderMap , room : & str , write : bool ,) -> Result < auth :: Principal , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`auth :: principal`；`.map_err`；`.fetch_one`；`.bind`；`.bind`；`sqlx :: query_scalar`；`.user_id`；`Err`；`Ok`。

### L53 · function ·  load

```rust
async fn load (state : & StateData , room : & str) -> Result < Vec < Persona > , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`.fetch_all`；`.bind`；`sqlx :: query_as`。

### L57 · function ·  list

```rust
async fn list (State (state) : State < StateData > , headers : HeaderMap , Path (room) : Path < String > ,) -> Result < Json < Vec < Persona > > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`owner`；`.map`；`load`。

### L65 · function ·  save

```rust
async fn save (State (state) : State < StateData > , headers : HeaderMap , Path (room) : Path < String > , Json (input) : Json < PersonaInput > ,) -> Result < StatusCode , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`owner`；`.is_nil`；`.is_empty`；`.trim`；`.len`；`.any`；`.chars`；`.is_empty`；`.trim`；`.len`；`.contains`；`.is_some_and`；`.as_ref`；`.len`；`.contains`；`.contains`；`Err`；`.map_err`；`.begin_with`；`.map_err`；`.execute`；`.bind`；`sqlx :: query`；`.map_err`；`.fetch_optional`；`.bind`；`sqlx :: query_scalar`；`.is_some_and`；`.as_ref`；`Err`；`.map_err`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`.is_none`；`Err`；`.map_err`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.map_err`；`.commit`；`Ok`。

### L121 · struct ·  RunInput

```rust
# [derive (Deserialize , Serialize)] # [serde (deny_unknown_fields)] pub (super) struct RunInput { id : Uuid , topic : String , max_turns : i32 , max_output_budget : i32 , tools : bool , confirmed : bool , }
```

### L131 · function ·  result

```rust
async fn result (state : & StateData , id : Uuid) -> Result < Value , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`.ok_or`；`.map_err`；`.fetch_optional`；`.bind`；`sqlx :: query_scalar`；`json!`；`Ok`。

### L137 · function ·  get

```rust
async fn get (State (state) : State < StateData > , headers : HeaderMap , Path ((room , id)) : Path < (String , Uuid) > ,) -> Result < Json < Value > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`owner`；`.map_err`；`.fetch_one`；`.bind`；`.bind`；`sqlx :: query_scalar`；`Err`；`.map`；`result`。

### L156 · struct ·  Confirmation

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct Confirmation { confirmed : bool , }
```

### L161 · function ·  cancel

```rust
async fn cancel (State (state) : State < StateData > , headers : HeaderMap , Path ((room , id)) : Path < (String , Uuid) > , Json (input) : Json < Confirmation > ,) -> Result < StatusCode , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`owner`；`Err`；`.map_err`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.rows_affected`；`Err`；`Ok`。

### L177 · function ·  run

```rust
async fn run (State (state) : State < StateData > , headers : HeaderMap , Path (room) : Path < String > , Json (input) : Json < RunInput > ,) -> Result < Json < Value > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`owner`；`.is_nil`；`.is_empty`；`.trim`；`.len`；`.contains`；`.contains`；`.contains`；`Err`；`.map_err`；`serde_json :: to_value`；`.map_err`；`.fetch_optional`；`.bind`；`sqlx :: query_as`；`Err`；`matches!`；`.map`；`result`；`.is_ok_and`；`std :: env :: var`；`.is_empty`；`.is_ok_and`；`std :: env :: var`；`.is_empty`；`Err`；`.any`；`.iter`；`load`；`Err`；`.map_err`；`.try_acquire_owned`；`.clone`；`Uuid :: new_v4`；`.map_err`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.rows_affected`；`Err`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.rows_affected`；`.unwrap_or`；`tokio :: time :: timeout`；`std :: time :: Duration :: from_secs`；`execute`；`Err`；`Err`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`i32 :: from`；`.as_u16`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.map`；`result`。

### L254 · function ·  execute

```rust
async fn execute (state : & StateData , headers : & HeaderMap , room : & str , input : & RunInput , lease : Uuid ,) -> Result < () , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`.map_or`；`.as_array`；`result`；`owner`；`result`；`Ok`；`.collect`；`.filter`；`.into_iter`；`load`；`.is_empty`；`Err`；`.len`；`.as_secs`；`.map_err`；`.duration_since`；`std :: time :: SystemTime :: now`；`tokio :: select!`；`.map_err`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.rows_affected`；`.map_err`；`.execute`；`.bind`；`sqlx :: query`；`Ok`；`.map_err`；`.fetch_all`；`.bind`；`sqlx :: query_as`；`json!`；`.to_owned`；`tokio :: select!`；`owner`；`json!`；`json!`；`json!`；`.map_err`；`.persist_persona_turn`；`day9_chat :: ChatWriter :: new`；`.clone`；`.map_err`；`.execute`；`.bind`；`sqlx :: query`；`Ok`。

### L320 · function ·  cancelled

```rust
async fn cancelled (state : & StateData , id : Uuid)
```

可見性／屬性：``；``。

函式內呼叫：`tokio :: time :: sleep`；`std :: time :: Duration :: from_millis`；`.is_ok_and`；`result`。

### L331 · function ·  answer

```rust
async fn answer (state : & StateData , headers : & HeaderMap , context : & Value , instructions : & str , tools : bool ,) -> Result < (Value , Value) , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`openai_adapter :: agent :: structured_persona`；`.clone`；`.is_none_or`；`.as_array`；`.is_empty`；`Err`；`Ok`；`json!`；`HeaderMap :: new`；`.get`；`.insert`；`.clone`；`.insert`；`http :: HeaderValue :: from_static`；`.map_err`；`mcp_bridge :: HttpClient :: new`；`.ok_or`；`.cloned`；`.as_array`；`.map_err`；`.call`；`json!`；`std :: sync :: Arc :: new`；`std :: sync :: Mutex :: new`；`Vec :: < String > :: new`；`.clone`；`.map_err`；`openai_adapter :: agent :: run_limited`；`json!`；`.clone`；`.clone`；`.map_err`；`.call`；`json!`；`Err`；`.clone`；`.map_err`；`.lock`；`.as_array`；`.as_str`；`.push`；`.into`；`Ok`；`eprintln!`；`.map_err`；`.lock`；`.any`；`.iter`；`.ok_or`；`.as_array`；`.any`；`.iter`；`.as_str`；`Some`；`.as_str`；`Err`；`Ok`；`json!`。

## copilot_use/apps/server/src/realtime.rs · C06

職責：Chat WS/Fanout。判定：保留跨程序 durability/ACL；不能把參考 socket 當成品 client

### L2 · use ·

```rust
use super :: { StateData , auth } ;
```

### L3 · use ·

```rust
use axum :: { extract :: { Path , State , WebSocketUpgrade , ws :: { Message , WebSocket } , } , http :: { HeaderMap , StatusCode } , response :: Response , } ;
```

### L11 · use ·

```rust
use serde :: Deserialize ;
```

### L12 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L13 · use ·

```rust
use std :: time :: Duration ;
```

### L14 · use ·

```rust
use tokio :: sync :: broadcast ;
```

### L15 · use ·

```rust
use uuid :: Uuid ;
```

### L17 · struct ·  RoomEvent

```rust
# [derive (Clone)] pub (super) struct RoomEvent { pub room_id : String , pub payload : Value , }
```

### L22 · struct ·  Authentication

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] struct Authentication { token : String , }
```

### L28 · function ·  upgrade

```rust
async fn upgrade (State (state) : State < StateData > , Path (room_id) : Path < String > , headers : HeaderMap , ws : WebSocketUpgrade ,) -> Result < Response , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`.is_empty`；`.len`；`Err`；`.get`；`.ok_or`；`.and_then`；`.get`；`.ok`；`.to_str`；`.map_err`；`.to_str`；`format!`；`format!`；`Err`；`.contains_key`；`auth :: principal`；`auth :: room_allowed`；`Some`；`.map_err`；`.try_acquire_owned`；`.clone`；`Ok`；`.on_upgrade`；`.max_frame_size`；`.max_message_size`；`connected`。

### L65 · function ·  send

```rust
async fn send (socket : & mut WebSocket , value : Value) -> bool
```

可見性／屬性：``；``。

函式內呼叫：`.is_ok_and`；`tokio :: time :: timeout`；`Duration :: from_secs`；`.send`；`Message :: Text`；`.into`；`.to_string`；`.is_ok`。

### L73 · function ·  connected

```rust
async fn connected (mut socket : WebSocket , state : StateData , room_id : String , cookie_actor : Option < auth :: Principal > , session_headers : HeaderMap , _permit : tokio :: sync :: OwnedSemaphorePermit ,)
```

可見性／屬性：``；``。

函式內呼叫：`tokio :: time :: timeout`；`Duration :: from_secs`；`.recv`；`.is_some`；`.is_ok_and`；`serde_json :: from_str :: < Authentication >`；`.is_empty`；`.as_ref`；`send`；`json!`；`.send`；`Message :: Close`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`matches!`；`send`；`json!`；`.send`；`Message :: Close`；`session_valid`；`.send`；`Message :: Close`；`.subscribe`；`Uuid :: new_v4`；`.is_err`；`super :: fanout :: join`；`.and_then`；`.user_id`；`send`；`json!`；`.send`；`Message :: Close`；`.unwrap_or_default`；`super :: fanout :: snapshot`；`send`；`json!`；`.execute`；`.bind`；`sqlx :: query`；`super :: fanout :: publish`；`json!`；`tokio :: time :: Instant :: now`；`Duration :: from_secs`；`tokio :: time :: interval_at`；`tokio :: time :: Instant :: now`；`Duration :: from_secs`；`Duration :: from_secs`；`.set_missed_tick_behavior`；`tokio :: select!`；`.execute`；`.bind`；`sqlx :: query`；`super :: fanout :: publish`；`json!`；`.send`；`Message :: Close`。

### L205 · function ·  session_valid

```rust
async fn session_valid (state : & StateData , headers : & HeaderMap , actor : Option < auth :: Principal > , room : & str ,) -> bool
```

可見性／屬性：``；``。

函式內呼叫：`auth :: principal`；`.user_id`；`.user_id`；`.is_ok`；`auth :: room_allowed`。

## copilot_use/apps/server/src/suggestions.rs · C11

職責：Chat personas/policy。判定：產品擴充，不能用一般 ChatMessage factory 取代

### L2 · use ·

```rust
use super :: { StateData , auth } ;
```

### L3 · use ·

```rust
use axum :: { Json , extract :: { Path , State } , http :: { HeaderMap , StatusCode } , } ;
```

### L8 · use ·

```rust
use serde :: Deserialize ;
```

### L9 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L10 · use ·

```rust
use uuid :: Uuid ;
```

### L11 · struct ·  Confirm

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct Confirm { confirmed : bool , }
```

### L16 · function ·  remote

```rust
fn remote () -> Result < (reqwest :: Client , String , String) , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`std :: env :: var`；`.to_owned`；`.ok_or`；`.strip_suffix`；`.map_err`；`std :: env :: var`；`.map_err`；`.build`；`.timeout`；`.redirect`；`.no_proxy`；`reqwest :: Client :: builder`；`reqwest :: redirect :: Policy :: none`；`std :: time :: Duration :: from_secs`；`Ok`。

### L33 · function ·  owner

```rust
async fn owner (state : & StateData , headers : & HeaderMap , room : & str) -> Result < () , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`auth :: principal`；`.map_err`；`.fetch_one`；`.bind`；`.bind`；`sqlx :: query_scalar`；`.user_id`；`Ok`；`Err`。

### L49 · function ·  apply

```rust
async fn apply (State (state) : State < StateData > , headers : HeaderMap , Path ((room , id)) : Path < (String , Uuid) > , Json (input) : Json < Confirm > ,) -> Result < Json < Value > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`owner`；`Err`；`remote`；`.map_err`；`.send`；`.bearer_auth`；`.get`；`format!`；`.is_success`；`.status`；`Err`；`.map_err`；`.json`；`matches!`；`Err`；`.ok_or`；`.filter`；`.as_i64`；`.contains`；`.map_err`；`.begin_with`；`.map_err`；`.execute`；`.bind`；`sqlx :: query`；`.map_err`；`.fetch_one`；`.bind`；`sqlx :: query_as`；`.map_err`；`.fetch_optional`；`.bind`；`.bind`；`sqlx :: query_as`；`.is_some`；`Err`；`Err`；`Uuid :: new_v4`；`.map_err`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.map_err`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.map_err`；`.commit`；`.is_ok_and`；`.send`；`.json`；`.bearer_auth`；`.post`；`format!`；`json!`；`.is_success`；`.status`；`Ok`；`Json`；`json!`。

### L131 · function ·  rollback

```rust
async fn rollback (State (state) : State < StateData > , headers : HeaderMap , Path ((room , id)) : Path < (String , Uuid) > , Json (input) : Json < Confirm > ,) -> Result < Json < Value > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`owner`；`Err`；`remote`；`.map_err`；`.begin_with`；`.map_err`；`.fetch_one`；`.bind`；`sqlx :: query_as`；`.map_err`；`.fetch_optional`；`.bind`；`.bind`；`sqlx :: query_as`；`.ok_or`；`.ok_or`；`Err`；`Uuid :: new_v4`；`.map_err`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.map_err`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.map_err`；`.commit`；`.is_ok_and`；`.send`；`.json`；`.bearer_auth`；`.post`；`format!`；`json!`；`.is_success`；`.status`；`Ok`；`Json`；`json!`。

## copilot_use/apps/server/src/topic_http.rs · C12

職責：Chat 同意撤回。判定：產品隱私擴充；category 與 topic 語意分離

### L2 · use ·

```rust
use super :: { StateData , auth } ;
```

### L3 · use ·

```rust
use axum :: { Json , extract :: { Path , State } , http :: { HeaderMap , StatusCode , header } , response :: { IntoResponse , Response } , } ;
```

### L9 · use ·

```rust
use serde :: Deserialize ;
```

### L10 · use ·

```rust
use serde_json :: Value ;
```

### L11 · struct ·  Consent

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct Consent { enabled : bool , }
```

### L16 · function ·  get

```rust
async fn get (State (state) : State < StateData > , headers : HeaderMap , Path (room) : Path < String > ,) -> Result < Response , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`auth :: principal`；`.ok_or`；`.user_id`；`auth :: room_allowed`；`.map_err`；`day9_chat :: topics :: consent`；`.into_response`；`Json`；`.insert`；`.headers_mut`；`header :: HeaderValue :: from_static`；`Ok`。

### L34 · function ·  set

```rust
async fn set (State (state) : State < StateData > , headers : HeaderMap , Path (room) : Path < String > , Json (input) : Json < Consent > ,) -> Result < Json < Value > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`auth :: principal`；`.ok_or`；`.user_id`；`auth :: room_allowed`；`.map_err`；`day9_chat :: topics :: set_consent`；`Ok`；`Json`；`.map_err`；`day9_chat :: topics :: consent`。

## copilot_use/apps/server/src/web.rs · C05

職責：Chat 入口/HTTP/SSR。判定：API 不相容；產品入口保留，教學需明示 adapter/URL/DTO

### L2 · function ·  leptos_home

```rust
async fn leptos_home (axum :: extract :: State (state) : axum :: extract :: State < super :: StateData > , request : axum :: http :: Request < axum :: body :: Body > ,) -> impl axum :: response :: IntoResponse
```

可見性／屬性：`pub`；``。

函式內呼叫：`.method`；`.into_response`；`.starts_with`；`.path`；`.uri`；`.into_response`；`.is_ok`；`super :: auth :: principal`；`.headers`；`leptos_axum :: render_app_to_stream`；`chat_ui :: shell`；`.into_response`；`handler`。

## copilot_use/apps/stats_server/src/application.rs · A01

職責：Analytics reference契約。判定：參考未實作不是成品應退回501；strict event whitelist不等同任意非空kind

### L3 · use ·

```rust
use event_contract :: ProductEvent ;
```

### L4 · use ·

```rust
use serde :: Serialize ;
```

### L5 · use ·

```rust
use std :: { future :: Future , pin :: Pin } ;
```

### L6 · use ·

```rust
use uuid :: Uuid ;
```

### L7 · enum ·  RepositoryError

```rust
# [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum RepositoryError { InvalidEvent , IdentityConflict , Unavailable , }
```

### L14 · struct ·  EventSummary

```rust
# [derive (Debug , Clone , Serialize , PartialEq , Eq)] pub struct EventSummary { pub source_app : String , pub event_type : String , pub total : i64 , }
```

### L20 · struct ·  EventRecord

```rust
# [derive (Debug , Clone , Serialize , PartialEq , Eq)] pub struct EventRecord { pub event_id : Uuid , pub source_app : String , pub event_type : String , pub received_unix_ms : i64 , }
```

### L27 · type ·  RepositoryFuture

```rust
pub type RepositoryFuture < 'a , T > = Pin < Box < dyn Future < Output = Result < T , RepositoryError > > + Send + 'a > > ;
```

### L30 · trait ·  EventRepository

```rust
# [doc = " 交易成功才回傳結果；持久冪等不由記憶體去重器取代。"] pub trait EventRepository : Send + Sync { fn insert_once < 'a > (& 'a self , event : & 'a ProductEvent) -> RepositoryFuture < 'a , bool > ; fn recent < 'a > (& 'a self , source : Option < & 'a str > , kind : Option < & 'a str > ,) -> RepositoryFuture < 'a , Vec < EventRecord > > ; fn total (& self) -> RepositoryFuture < '_ , i64 > ; fn summary (& self) -> RepositoryFuture < '_ , Vec < EventSummary > > ; fn lookup < 'a > (& 'a self , source : & 'a str , id : Uuid) -> RepositoryFuture < 'a , Option < String > > ; }
```

### L32 · function · EventRepository insert_once

```rust
fn insert_once < 'a > (& 'a self , event : & 'a ProductEvent) -> RepositoryFuture < 'a , bool >
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L33 · function · EventRepository recent

```rust
fn recent < 'a > (& 'a self , source : Option < & 'a str > , kind : Option < & 'a str > ,) -> RepositoryFuture < 'a , Vec < EventRecord > >
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L38 · function · EventRepository total

```rust
fn total (& self) -> RepositoryFuture < '_ , i64 >
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L39 · function · EventRepository summary

```rust
fn summary (& self) -> RepositoryFuture < '_ , Vec < EventSummary > >
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L40 · function · EventRepository lookup

```rust
fn lookup < 'a > (& 'a self , source : & 'a str , id : Uuid) -> RepositoryFuture < 'a , Option < String > >
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L43 · struct ·  Analytics

```rust
# [doc = " 純用例協調者，不持有 PgPool、HTTP 或 gRPC Driver。"] pub struct Analytics < R : EventRepository > { repository : R , }
```

### L48 · function · impl Analytics < R > new

```rust
fn new (repository : R) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L51 · function · impl Analytics < R > receive

```rust
async fn receive (& self , event : & ProductEvent) -> Result < bool , RepositoryError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_valid`；`Err`；`.insert_once`。

### L57 · function · impl Analytics < R > recent

```rust
async fn recent (& self , source : Option < & str > , kind : Option < & str > ,) -> Result < Vec < EventRecord > , RepositoryError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.recent`。

### L64 · function · impl Analytics < R > total

```rust
async fn total (& self) -> Result < i64 , RepositoryError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.total`。

### L67 · function · impl Analytics < R > summary

```rust
async fn summary (& self) -> Result < Vec < EventSummary > , RepositoryError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.summary`。

### L70 · function · impl Analytics < R > lookup

```rust
async fn lookup (& self , source : & str , id : Uuid) -> Result < Option < String > , RepositoryError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.lookup`。

### L74 · module ·  tests

```rust
 mod tests (inline=True)
```

### L76 · use ·

```rust
use super :: * ;
```

### L77 · use ·

```rust
use std :: sync :: { Arc , atomic :: { AtomicBool , Ordering } , } ;
```

### L81 · struct ·  Unavailable

```rust
struct Unavailable { called : Arc < AtomicBool > , }
```

### L85 · function · tests::impl EventRepository for Unavailable insert_once

```rust
fn insert_once < 'a > (& 'a self , _ : & 'a ProductEvent) -> RepositoryFuture < 'a , bool >
```

可見性／屬性：``；``。

函式內呼叫：`Box :: pin`；`.store`；`Err`。

### L91 · function · tests::impl EventRepository for Unavailable recent

```rust
fn recent < 'a > (& 'a self , _ : Option < & 'a str > , _ : Option < & 'a str > ,) -> RepositoryFuture < 'a , Vec < EventRecord > >
```

可見性／屬性：``；``。

函式內呼叫：`Box :: pin`；`Err`。

### L98 · function · tests::impl EventRepository for Unavailable total

```rust
fn total (& self) -> RepositoryFuture < '_ , i64 >
```

可見性／屬性：``；``。

函式內呼叫：`Box :: pin`；`Err`。

### L101 · function · tests::impl EventRepository for Unavailable summary

```rust
fn summary (& self) -> RepositoryFuture < '_ , Vec < EventSummary > >
```

可見性／屬性：``；``。

函式內呼叫：`Box :: pin`；`Err`。

### L104 · function · tests::impl EventRepository for Unavailable lookup

```rust
fn lookup < 'a > (& 'a self , _ : & 'a str , _ : Uuid) -> RepositoryFuture < 'a , Option < String > >
```

可見性／屬性：``；``。

函式內呼叫：`Box :: pin`；`Err`。

### L109 · function · tests malformed_event_never_reaches_repository_and_storage_failure_is_not_success

```rust
async fn malformed_event_never_reaches_repository_and_storage_failure_is_not_success ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`Arc :: new`；`AtomicBool :: new`；`Analytics :: new`；`.clone`；`ProductEvent :: chat_message_persisted`；`Uuid :: nil`；`assert_eq!`；`assert!`；`Uuid :: new_v4`；`assert_eq!`；`assert!`。

## copilot_use/apps/stats_server/src/dashboard.rs · A05

職責：Analytics Dashboard/SSE。判定：SSE非event replay或gRPC替代；aggregate自動更新、recent/trend手動

### L3 · use ·

```rust
use axum :: response :: Html ;
```

### L5 · function ·  dashboard

```rust
async fn dashboard () -> Html < & 'static str >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Html`。

## copilot_use/apps/stats_server/src/governance.rs · A06

職責：Analytics governance/topics。判定：產品擴充；archive後重送不復活，withdraw不等archive

### L2 · use ·

```rust
use super :: { AnalyticsState , authorized } ;
```

### L3 · use ·

```rust
use axum :: { Json , extract :: { Query , State } , http :: { HeaderMap , StatusCode } , } ;
```

### L8 · use ·

```rust
use chrono :: { DateTime , Utc } ;
```

### L9 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L10 · use ·

```rust
use sqlx :: FromRow ;
```

### L11 · struct ·  Range

```rust
# [derive (Deserialize)] pub (super) struct Range { from : DateTime < Utc > , to : DateTime < Utc > , }
```

### L16 · struct ·  Daily

```rust
# [derive (Serialize , FromRow)] pub (super) struct Daily { day : String , source_app : String , event_type : String , total : i64 , }
```

### L23 · function ·  timeline

```rust
async fn timeline (State (state) : State < AnalyticsState > , headers : HeaderMap , Query (range) : Query < Range > ,) -> Result < Json < Vec < Daily > > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`authorized`；`Err`；`.num_days`；`Err`；`.map_err`；`.fetch_all`；`.bind`；`.bind`；`sqlx :: query_as`；`.to_rfc3339`；`.to_rfc3339`；`Ok`；`Json`。

### L38 · struct ·  Archive

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct Archive { before : DateTime < Utc > , confirmed : bool , }
```

### L44 · function ·  set_archive

```rust
async fn set_archive (state : AnalyticsState , headers : HeaderMap , input : Archive , archived : bool ,) -> Result < Json < serde_json :: Value > , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`authorized`；`Err`；`Utc :: now`；`Err`；`.map_err`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.to_rfc3339`；`Ok`；`Json`；`serde_json :: json!`。

### L61 · function ·  archive

```rust
async fn archive (State (state) : State < AnalyticsState > , headers : HeaderMap , Json (input) : Json < Archive > ,) -> Result < Json < serde_json :: Value > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`set_archive`。

### L68 · function ·  restore

```rust
async fn restore (State (state) : State < AnalyticsState > , headers : HeaderMap , Json (input) : Json < Archive > ,) -> Result < Json < serde_json :: Value > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`set_archive`。

## copilot_use/apps/stats_server/src/grpc.rs · A04

職責：Analytics gRPC。判定：批次非全有全無；來源不可冒用；共享application非旁路SQL

### L2 · use ·

```rust
use crate :: { AnalyticsState , EventStore , application :: { Analytics , RepositoryError } , } ;
```

### L6 · use ·

```rust
use analytics_transport :: { MAX_BATCH , MAX_MESSAGE_BYTES , proto :: { self , event_ingestion_server :: { EventIngestion , EventIngestionServer } , } , } ;
```

### L13 · use ·

```rust
use tonic :: { Request , Response , Status } ;
```

### L15 · struct ·  Ingestion

```rust
# [derive (Clone)] pub struct Ingestion (pub AnalyticsState) ;
```

### L19 · function · impl EventIngestion for Ingestion ingest_events

```rust
async fn ingest_events (& self , request : Request < proto :: IngestEventsRequest > ,) -> Result < Response < proto :: IngestEventsResponse > , Status >
```

可見性／屬性：``；``。

函式內呼叫：`.and_then`；`.and_then`；`.get`；`.metadata`；`.ok`；`.to_str`；`.strip_prefix`；`.get_ref`；`.is_empty`；`.len`；`Err`；`Status :: invalid_argument`；`.as_str`；`Err`；`Status :: invalid_argument`；`.is_empty`；`Some`；`.as_ref`；`Err`；`Status :: unauthenticated`；`Analytics :: new`；`EventStore :: new`；`.clone`；`Vec :: with_capacity`；`.len`；`.into_inner`；`.clone`；`.clone`；`event_contract :: ProductEvent :: try_from`；`.receive`；`Err`；`Status :: unavailable`；`.push`；`.into`；`Ok`；`Response :: new`。

### L70 · function ·  routes

```rust
fn routes (state : AnalyticsState) -> axum :: Router
```

可見性／屬性：`pub`；``。

函式內呼叫：`.into_axum_router`；`tonic :: service :: Routes :: new`；`.max_encoding_message_size`；`.max_decoding_message_size`；`EventIngestionServer :: new`；`Ingestion`。

### L79 · module ·  tests

```rust
 mod tests (inline=False)
```

## copilot_use/apps/stats_server/src/grpc/tests.rs · A04

職責：Analytics gRPC。判定：批次非全有全無；來源不可冒用；共享application非旁路SQL

### L1 · use ·

```rust
use super :: * ;
```

### L2 · use ·

```rust
use crate :: { AnalyticsState , build_app } ;
```

### L3 · use ·

```rust
use event_contract :: ProductEvent ;
```

### L4 · use ·

```rust
use futures_util :: StreamExt ;
```

### L5 · use ·

```rust
use sqlx :: PgPool ;
```

### L6 · use ·

```rust
use std :: { sync :: Arc , time :: Duration } ;
```

### L7 · use ·

```rust
use uuid :: Uuid ;
```

### L8 · const ·  CHAT

```rust
const CHAT : & str = "grpc-chat-test-credential" ;
```

### L9 · const ·  PET

```rust
const PET : & str = "grpc-pet-test-credential" ;
```

### L10 · const ·  READ

```rust
const READ : & str = "grpc-read-test-credential" ;
```

### L11 · struct ·  Server

```rust
struct Server { url : String , task : tokio :: task :: JoinHandle < () > , }
```

### L16 · function · impl Drop for Server drop

```rust
fn drop (& mut self)
```

可見性／屬性：``；``。

函式內呼叫：`.abort`。

### L20 · function ·  server

```rust
async fn server (pool : PgPool) -> Server
```

可見性／屬性：``；``。

函式內呼叫：`Arc :: from`；`Arc :: from`；`Arc :: from`；`Arc :: from`；`Arc :: new`；`tokio :: sync :: Semaphore :: new`；`.unwrap`；`tokio :: net :: TcpListener :: bind`；`format!`；`tokio :: spawn`；`.unwrap`；`axum :: serve`；`build_app`。

### L36 · function ·  ingest

```rust
async fn ingest (url : & str , token : & str , events : Vec < proto :: ProductEvent > ,) -> Result < proto :: IngestEventsResponse , Status >
```

可見性／屬性：``；``。

函式內呼叫：`.unwrap`；`proto :: event_ingestion_client :: EventIngestionClient :: connect`；`.to_owned`；`Request :: new`；`.insert`；`.metadata_mut`；`.unwrap`；`.parse`；`format!`；`.map`；`.ingest_events`。

### L53 · function ·  revision

```rust
async fn revision (pool : & PgPool) -> i64
```

可見性／屬性：``；``。

函式內呼叫：`.unwrap`；`.fetch_one`；`sqlx :: query_scalar`。

### L60 · function ·  real_grpc_batch_auth_conflict_concurrency_and_http_query

```rust
async fn real_grpc_batch_auth_conflict_concurrency_and_http_query (pool : PgPool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`server`；`.clone`；`ProductEvent :: chat_message_persisted`；`Uuid :: new_v4`；`assert_eq!`；`ProductEvent :: pet_cleanup_succeeded`；`Uuid :: new_v4`；`assert_eq!`；`assert_eq!`；`vec!`；`vec!`；`assert_eq!`；`tokio :: join!`；`.unwrap`；`.unwrap`；`assert_ne!`；`assert_eq!`；`.into`；`.into`；`.into`；`.into`；`.to_string`；`Uuid :: nil`；`.unwrap`；`ingest`；`vec!`；`assert_eq!`；`.collect`；`.map`；`proto :: ProductEvent :: from`；`ProductEvent :: chat_message_persisted`；`Uuid :: new_v4`；`assert_eq!`；`.into`；`.repeat`；`assert!`；`.unwrap`；`.send`；`.bearer_auth`；`.get`；`reqwest :: Client :: new`；`format!`；`assert_eq!`；`drop`；`self :: server`；`.clone`；`.unwrap`；`analytics_transport :: deliver`；`assert_eq!`。

### L150 · function ·  real_sqlite_outboxes_retry_and_ack_only_committed_grpc_events

```rust
async fn real_sqlite_outboxes_retry_and_ack_only_committed_grpc_events (pool : PgPool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`server`；`.clone`；`.unwrap`；`tempfile :: tempdir`；`format!`；`.unwrap`；`persistence :: database :: open`；`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.persist`；`persistence :: ChatWriter :: new`；`.clone`；`persistence :: OutboxStore :: new`；`.clone`；`reqwest :: Client :: new`；`.unwrap`；`persistence :: dispatch_once`；`.unwrap`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`assert_eq!`；`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.insert_once`；`EventStore :: new`；`.clone`；`ProductEvent :: chat_message_persisted`；`.unwrap`；`persistence :: dispatch_once`；`.unwrap`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`assert_eq!`；`.unwrap`；`day9_pet :: state :: PetStore :: open`；`.join`；`.path`；`Uuid :: new_v4`；`.into`；`.unwrap`；`.record_cleanup`；`.event`；`.unwrap`；`.dispatch_once`；`assert_eq!`；`tokio :: time :: sleep`；`Duration :: from_secs`；`.unwrap`；`.dispatch_once`；`assert_eq!`；`assert_eq!`；`.close`。

### L217 · function ·  sse_observes_commit_not_rollback_dedup_and_reconnect_snapshot

```rust
async fn sse_observes_commit_not_rollback_dedup_and_reconnect_snapshot (pool : PgPool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`server`；`.clone`；`reqwest :: Client :: new`；`format!`；`assert_eq!`；`assert_eq!`；`.unwrap`；`.send`；`.bearer_auth`；`.get`；`assert!`；`.bytes_stream`；`.unwrap`；`.unwrap`；`.next`；`assert!`；`ProductEvent :: chat_message_persisted`；`Uuid :: new_v4`；`.unwrap`；`analytics_transport :: deliver`；`.unwrap`；`.unwrap`；`.unwrap`；`tokio :: time :: timeout`；`Duration :: from_secs`；`.next`；`assert!`；`.unwrap`；`analytics_transport :: deliver`；`assert_eq!`；`assert!`；`.unwrap`；`.begin`；`.unwrap`；`.execute`；`sqlx :: query`；`assert_eq!`；`.unwrap`；`.rollback`；`assert_eq!`；`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.unwrap`；`.unwrap`；`tokio :: time :: timeout`；`Duration :: from_secs`；`.next`；`assert!`；`drop`；`.bytes_stream`；`.unwrap`；`.send`；`.bearer_auth`；`.get`；`.unwrap`；`.unwrap`；`.next`；`assert!`。

### L292 · function ·  database_failure_is_unavailable_without_success_ack_or_sse

```rust
async fn database_failure_is_unavailable_without_success_ack_or_sse ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.unwrap`；`.connect_lazy`；`.acquire_timeout`；`sqlx :: postgres :: PgPoolOptions :: new`；`Duration :: from_millis`；`server`；`ProductEvent :: chat_message_persisted`；`Uuid :: new_v4`；`assert_eq!`；`assert_eq!`。

## copilot_use/apps/stats_server/src/insights.rs · A07

職責：Analytics Insights。判定：產品擴充；不把描述性變化當因果/uniqueusers；本輪不執行AI

### L2 · use ·

```rust
use super :: { AnalyticsState , authorized } ;
```

### L3 · use ·

```rust
use axum :: { Json , extract :: { Path , State } , http :: { HeaderMap , StatusCode } , } ;
```

### L8 · use ·

```rust
use serde :: Deserialize ;
```

### L9 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L10 · use ·

```rust
use uuid :: Uuid ;
```

### L11 · struct ·  Generate

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct Generate { pub target_app : String , }
```

### L16 · function ·  schema

```rust
fn schema () -> Value
```

可見性／屬性：``；``。

函式內呼叫：`json!`。

### L19 · function ·  facts

```rust
fn facts (snapshot : & Value) -> Option < Vec < String > >
```

可見性／屬性：``；``。

函式內呼叫：`.as_array`；`.try_fold`；`.iter`；`.checked_add`；`.as_i64`；`vec!`；`.as_i64`；`.as_str`；`.as_str`；`.push`；`format!`；`Some`。

### L38 · function ·  grounded_proposal

```rust
fn grounded_proposal (snapshot : & Value , selection : & Value) -> Option < Value >
```

可見性／屬性：``；``。

函式內呼叫：`facts`；`.as_array`；`.contains`；`.len`；`std :: collections :: HashSet :: new`；`String :: new`；`.ok`；`usize :: try_from`；`.as_u64`；`.insert`；`.push_str`；`.get`；`json!`；`.then_some`；`valid`；`.as_str`。

### L56 · function ·  valid

```rust
fn valid (target : & str , proposal : & Value) -> bool
```

可見性／屬性：``；``。

函式內呼叫：`.as_i64`；`.as_str`；`.count`；`.filter`；`.chars`；`matches!`；`.contains`；`.len`；`.is_some_and`；`.as_str`；`.is_empty`；`.trim`；`.len`；`.as_str`；`.contains`；`.contains`。

### L78 · function ·  generate

```rust
async fn generate (State (state) : State < AnalyticsState > , headers : HeaderMap , Json (input) : Json < Generate > ,) -> Result < Json < Value > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`authorized`；`Err`；`matches!`；`Err`；`.map_err`；`.try_acquire_owned`；`.clone`；`.map_err`；`.fetch_all`；`.bind`；`sqlx :: query_as`；`json!`；`.ok_or`；`facts`；`.map_err`；`openai_adapter :: agent :: structured`；`json!`；`schema`；`.ok_or`；`grounded_proposal`；`Uuid :: new_v4`；`.map_err`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.unwrap`；`.as_str`；`Ok`；`Json`；`json!`。

### L112 · function ·  source

```rust
fn source (headers : & HeaderMap , state : & AnalyticsState) -> Option < & 'static str >
```

可見性／屬性：``；``。

函式內呼叫：`authorized`；`Some`；`authorized`；`Some`。

### L121 · function ·  list

```rust
async fn list (State (state) : State < AnalyticsState > , headers : HeaderMap ,) -> Result < Json < Vec < Value > > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`authorized`；`authorized`；`Err`；`.map_err`；`.fetch_all`；`sqlx :: query_scalar`；`Ok`；`Json`。

### L131 · function ·  get

```rust
async fn get (State (state) : State < AnalyticsState > , headers : HeaderMap , Path (id) : Path < Uuid > ,) -> Result < Json < Value > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`.ok_or`；`source`；`.map_err`；`.fetch_optional`；`.bind`；`.bind`；`sqlx :: query_scalar`；`.ok_or`；`.map`。

### L140 · struct ·  Review

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct Review { decision : String , confirmed : bool , }
```

### L146 · function ·  review

```rust
async fn review (State (state) : State < AnalyticsState > , headers : HeaderMap , Path (id) : Path < Uuid > , Json (input) : Json < Review > ,) -> Result < StatusCode , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`authorized`；`Err`；`matches!`；`Err`；`.map_err`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.rows_affected`；`Err`；`Ok`。

### L164 · struct ·  Receipt

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct Receipt { pub decision : String , pub receipt_id : Uuid , pub value : i64 , }
```

### L171 · function ·  receipt

```rust
async fn receipt (State (state) : State < AnalyticsState > , headers : HeaderMap , Path (id) : Path < Uuid > , Json (input) : Json < Receipt > ,) -> Result < StatusCode , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`.ok_or`；`source`；`matches!`；`.is_nil`；`Err`；`.map_err`；`.fetch_optional`；`.bind`；`.bind`；`sqlx :: query_scalar`；`.ok_or`；`.contains`；`.contains`；`Err`；`.map_err`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.rows_affected`；`.map_err`；`.fetch_one`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query_scalar`；`Err`；`Ok`。

### L204 · function ·  evaluate

```rust
async fn evaluate (State (state) : State < AnalyticsState > , headers : HeaderMap , Path (id) : Path < Uuid > , Json (input) : Json < Evaluate > ,) -> Result < Json < Value > , StatusCode >
```

可見性／屬性：`pub (super)`；`# [doc = " Descriptive comparison only: event totals do not establish learning quality or causality."]`。

函式內呼叫：`authorized`；`Err`；`Err`；`.map_err`；`.begin`；`.map_err`；`.fetch_optional`；`.bind`；`sqlx :: query_as`；`.ok_or`；`matches!`；`Err`；`.map_err`；`.fetch_all`；`.bind`；`sqlx :: query_as`；`json!`；`json!`；`.map_err`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.map_err`；`.commit`；`Ok`；`Json`。

### L247 · struct ·  Evaluate

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct Evaluate { confirmed : bool , }
```

### L252 · module ·  tests

```rust
 mod tests (inline=True)
```

### L254 · use ·

```rust
use super :: * ;
```

### L256 · function · tests generated_numbers_and_claims_come_only_from_verified_snapshot_facts

```rust
fn generated_numbers_and_claims_come_only_from_verified_snapshot_facts ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`json!`；`json!`；`.unwrap`；`grounded_proposal`；`assert!`；`json!`；`json!`；`json!`；`json!`；`json!`；`.clone`；`assert!`；`json!`；`assert!`。

### L277 · function · tests proposal_is_bounded_and_cannot_change_target

```rust
fn proposal_is_bounded_and_cannot_change_target ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`json!`；`assert!`；`assert!`；`json!`；`assert!`。

## copilot_use/apps/stats_server/src/lib.rs · A02

職責：Analytics HTTP/入口。判定：產品擴充；四欄shape對應，驗證/status/security不相容

### L2 · module ·  application

```rust
pub mod application (inline=False)
```

### L3 · module ·  dashboard

```rust
pub mod dashboard (inline=False)
```

### L4 · module ·  grpc

```rust
pub mod grpc (inline=False)
```

### L5 · module ·  live

```rust
 mod live (inline=False)
```

### L6 · use ·

```rust
use application :: { Analytics , RepositoryError } ;
```

### L7 · module ·  governance

```rust
 mod governance (inline=False)
```

### L8 · module ·  insights

```rust
 mod insights (inline=False)
```

### L9 · module ·  topics

```rust
 mod topics (inline=False)
```

### L10 · use ·

```rust
use axum :: { Router , extract :: { Json , Path , Query , State } , http :: { HeaderMap , StatusCode } , routing :: { get , post } , } ;
```

### L16 · use ·

```rust
use event_contract :: ProductEvent ;
```

### L17 · use ·

```rust
use serde :: Serialize ;
```

### L18 · use ·

```rust
use sqlx :: PgPool ;
```

### L19 · module ·  persistence

```rust
 mod persistence (inline=False)
```

### L20 · use ·

```rust
pub use application :: EventSummary ;
```

### L21 · use ·

```rust
pub use persistence :: EventStore ;
```

### L22 · use ·

```rust
use std :: sync :: Arc ;
```

### L24 · struct ·  AnalyticsState

```rust
# [derive (Clone)] pub struct AnalyticsState { pub pool : PgPool , pub api_token : Arc < str > , pub chat_token : Arc < str > , pub pet_token : Arc < str > , pub admin_token : Arc < str > , pub ai_slots : Arc < tokio :: sync :: Semaphore > , }
```

### L34 · function ·  authorized

```rust
fn authorized (headers : & HeaderMap , expected : & str) -> bool
```

可見性／屬性：``；``。

函式內呼叫：`.is_empty`；`.is_some_and`；`.and_then`；`.and_then`；`.get`；`.ok`；`.to_str`；`.strip_prefix`。

### L44 · function ·  ingest

```rust
async fn ingest (State (state) : State < AnalyticsState > , headers : HeaderMap , Json (event) : Json < ProductEvent > ,) -> StatusCode
```

可見性／屬性：``；``。

函式內呼叫：`.as_str`；`authorized`；`.is_valid`；`.receive`；`Analytics :: new`；`EventStore :: new`。

### L71 · struct ·  RecentQuery

```rust
# [derive (serde :: Deserialize)] # [serde (deny_unknown_fields)] struct RecentQuery { source_app : Option < String > , event_type : Option < String > , }
```

### L77 · function ·  recent

```rust
async fn recent (State (state) : State < AnalyticsState > , headers : HeaderMap , Query (input) : Query < RecentQuery > ,) -> Result < Json < Vec < application :: EventRecord > > , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`authorized`；`Err`；`.is_some_and`；`.as_deref`；`matches!`；`.is_some_and`；`.as_ref`；`.is_empty`；`.len`；`Err`；`.map_err`；`.recent`；`Analytics :: new`；`EventStore :: new`；`.as_deref`；`.as_deref`；`Ok`；`Json`。

### L102 · struct ·  CountResponse

```rust
# [derive (Serialize)] struct CountResponse { total : i64 , }
```

### L107 · function ·  count

```rust
async fn count (State (state) : State < AnalyticsState > , headers : HeaderMap ,) -> Result < Json < CountResponse > , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`authorized`；`Err`；`.map_err`；`.total`；`Analytics :: new`；`EventStore :: new`；`Ok`；`Json`。

### L121 · function ·  summary

```rust
async fn summary (State (state) : State < AnalyticsState > , headers : HeaderMap ,) -> Result < Json < Vec < EventSummary > > , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`authorized`；`Err`；`.map_err`；`.summary`；`Analytics :: new`；`EventStore :: new`；`Ok`；`Json`。

### L135 · function ·  health

```rust
async fn health (State (state) : State < AnalyticsState >) -> StatusCode
```

可見性／屬性：``；``。

函式內呼叫：`.health`；`EventStore :: new`。

### L142 · function ·  event_lookup

```rust
async fn event_lookup (State (state) : State < AnalyticsState > , headers : HeaderMap , Path ((source , id)) : Path < (String , uuid :: Uuid) > ,) -> Result < Json < serde_json :: Value > , StatusCode >
```

可見性／屬性：``；``。

函式內呼叫：`authorized`；`Err`；`.map_err`；`.lookup`；`Analytics :: new`；`EventStore :: new`；`Ok`；`Json`；`serde_json :: json!`；`Err`。

### L160 · function ·  build_app

```rust
fn build_app (state : AnalyticsState) -> Router
```

可見性／屬性：`pub`；``。

函式內呼叫：`grpc :: routes`；`.clone`；`.layer`；`.merge`；`.with_state`；`.layer`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`.route`；`Router :: new`；`get`；`get`；`get`；`post`；`get`；`get`；`post`；`post`；`post`；`post`；`post`；`get`；`get`；`get`；`get`；`get`；`post`；`post`；`axum :: extract :: DefaultBodyLimit :: max`；`axum :: middleware :: from_fn`。

### L190 · module ·  tests

```rust
 mod tests (inline=True)
```

### L192 · use ·

```rust
use super :: * ;
```

### L193 · use ·

```rust
use uuid :: Uuid ;
```

### L196 · function · tests missing_auth_rejected

```rust
fn missing_auth_rejected ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`。

### L201 · function · tests same_source_event_counts_once_even_after_new_store

```rust
async fn same_source_event_counts_once_even_after_new_store (pool : PgPool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`ProductEvent :: chat_message_persisted`；`Uuid :: new_v4`；`EventStore :: new`；`.clone`；`assert!`；`EventStore :: new`；`.clone`；`assert!`；`assert_eq!`。

### L211 · function · tests concurrent_delivery_of_same_event_is_idempotent

```rust
async fn concurrent_delivery_of_same_event_is_idempotent (pool : PgPool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`EventStore :: new`；`ProductEvent :: chat_message_persisted`；`Uuid :: new_v4`；`tokio :: join!`；`assert!`；`assert_eq!`。

### L220 · function · tests summary_counts_per_source_and_event_type

```rust
async fn summary_counts_per_source_and_event_type (pool : PgPool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`EventStore :: new`；`assert!`；`assert!`；`assert!`；`.unwrap`；`.summary`；`assert_eq!`；`assert_eq!`。

### L246 · function · tests two_sources_same_event_id_count_separately

```rust
async fn two_sources_same_event_id_count_separately (pool : PgPool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`Uuid :: new_v4`；`EventStore :: new`；`assert!`；`assert!`；`assert_eq!`。

### L264 · function · tests archived_event_is_not_resurrected_by_retry_and_can_be_restored

```rust
async fn archived_event_is_not_resurrected_by_retry_and_can_be_restored (pool : PgPool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`EventStore :: new`；`.clone`；`ProductEvent :: chat_message_persisted`；`Uuid :: new_v4`；`assert!`；`.unwrap`；`.execute`；`sqlx :: query`；`assert!`；`assert_eq!`；`.unwrap`；`.execute`；`sqlx :: query`；`assert_eq!`。

### L281 · function · tests conflicting_retry_is_rejected

```rust
async fn conflicting_retry_is_rejected (pool : PgPool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`EventStore :: new`；`ProductEvent :: pet_cleanup_succeeded`；`Uuid :: new_v4`；`assert!`；`.into`；`assert!`；`assert_eq!`。

## copilot_use/apps/stats_server/src/live.rs · A05

職責：Analytics Dashboard/SSE。判定：SSE非event replay或gRPC替代；aggregate自動更新、recent/trend手動

### L2 · use ·

```rust
use crate :: { AnalyticsState , authorized } ;
```

### L3 · use ·

```rust
use axum :: { extract :: State , http :: { HeaderMap , StatusCode } , response :: sse :: { Event , KeepAlive , Sse } , } ;
```

### L8 · use ·

```rust
use std :: { convert :: Infallible , sync :: { Arc , LazyLock } , time :: Duration , } ;
```

### L13 · static ·  SLOTS

```rust
static SLOTS : LazyLock < Arc < tokio :: sync :: Semaphore > > = LazyLock :: new (| | Arc :: new (tokio :: sync :: Semaphore :: new (16))) ;
```

### L15 · function ·  revision

```rust
async fn revision (pool : & sqlx :: PgPool) -> Result < i64 , sqlx :: Error >
```

可見性／屬性：``；``。

函式內呼叫：`.fetch_one`；`sqlx :: query_scalar`。

### L20 · function ·  updates

```rust
async fn updates (State (state) : State < AnalyticsState > , headers : HeaderMap ,) -> Result < Sse < impl futures_core :: Stream < Item = Result < Event , Infallible > > > , StatusCode >
```

可見性／屬性：`pub`；``。

函式內呼叫：`authorized`；`Err`；`.map_err`；`.try_acquire_owned`；`.clone`；`.map_err`；`revision`；`async_stream :: stream!`；`Ok`；`.keep_alive`；`Sse :: new`；`.text`；`.interval`；`KeepAlive :: new`；`Duration :: from_secs`。

## copilot_use/apps/stats_server/src/main.rs · A02

職責：Analytics HTTP/入口。判定：產品擴充；四欄shape對應，驗證/status/security不相容

### L2 · use ·

```rust
use day9_analytics :: { AnalyticsState , build_app } ;
```

### L3 · use ·

```rust
use sqlx :: postgres :: PgPoolOptions ;
```

### L4 · use ·

```rust
use std :: sync :: Arc ;
```

### L7 · function ·  main

```rust
async fn main () -> Result < () , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：``；`# [tokio :: main]`。

函式內呼叫：`tokio :: task :: spawn_blocking`；`copilot_telemetry :: init`；`std :: env :: var`；`std :: env :: var`；`std :: env :: var`；`std :: env :: var`；`.len`；`.contains`；`.as_str`；`.as_str`；`.as_str`；`Err`；`.into`；`std :: env :: var`；`.any`；`.iter`；`.as_str`；`.as_str`；`.as_str`；`.len`；`Err`；`.into`；`.connect`；`.max_connections`；`.acquire_timeout`；`PgPoolOptions :: new`；`std :: time :: Duration :: from_secs`；`.run`；`sqlx :: migrate!`；`build_app`；`Arc :: from`；`Arc :: from`；`Arc :: from`；`Arc :: from`；`Arc :: new`；`tokio :: sync :: Semaphore :: new`；`.unwrap_or_else`；`std :: env :: var`；`.into`；`tokio :: net :: TcpListener :: bind`；`.with_graceful_shutdown`；`axum :: serve`；`copilot_telemetry :: shutdown_signal`；`tokio :: task :: spawn_blocking`；`drop`；`Ok`。

## copilot_use/apps/stats_server/src/persistence.rs · A03

職責：Analytics PostgreSQL。判定：產品 durable實作；参考HashSet不能替換；PGmigration非SQLite

### L2 · use ·

```rust
use crate :: application :: { EventRecord , EventRepository , EventSummary , RepositoryError , RepositoryFuture , } ;
```

### L5 · use ·

```rust
use event_contract :: ProductEvent ;
```

### L6 · use ·

```rust
use sqlx :: { FromRow , PgPool , Row } ;
```

### L8 · function · impl FromRow < 'r , sqlx :: postgres :: PgRow > for EventRecord from_row

```rust
fn from_row (row : & 'r sqlx :: postgres :: PgRow) -> Result < Self , sqlx :: Error >
```

可見性／屬性：``；``。

函式內呼叫：`Ok`；`.try_get`；`.try_get`；`.try_get`；`.try_get`。

### L18 · function · impl FromRow < 'r , sqlx :: postgres :: PgRow > for EventSummary from_row

```rust
fn from_row (row : & 'r sqlx :: postgres :: PgRow) -> Result < Self , sqlx :: Error >
```

可見性／屬性：``；``。

函式內呼叫：`Ok`；`.try_get`；`.try_get`；`.try_get`。

### L26 · function ·  mapped

```rust
fn mapped (error : sqlx :: Error) -> RepositoryError
```

可見性／屬性：``；``。

函式內呼叫：`matches!`。

### L34 · function · impl EventRepository for EventStore insert_once

```rust
fn insert_once < 'a > (& 'a self , event : & 'a ProductEvent) -> RepositoryFuture < 'a , bool >
```

可見性／屬性：``；``。

函式內呼叫：`Box :: pin`；`.map_err`；`EventStore :: insert_once`。

### L37 · function · impl EventRepository for EventStore recent

```rust
fn recent < 'a > (& 'a self , source : Option < & 'a str > , kind : Option < & 'a str > ,) -> RepositoryFuture < 'a , Vec < EventRecord > >
```

可見性／屬性：``；``。

函式內呼叫：`Box :: pin`；`.map_err`；`.fetch_all`；`.bind`；`.bind`；`sqlx :: query_as`。

### L46 · function · impl EventRepository for EventStore total

```rust
fn total (& self) -> RepositoryFuture < '_ , i64 >
```

可見性／屬性：``；``。

函式內呼叫：`Box :: pin`；`.map_err`；`EventStore :: total`。

### L49 · function · impl EventRepository for EventStore summary

```rust
fn summary (& self) -> RepositoryFuture < '_ , Vec < EventSummary > >
```

可見性／屬性：``；``。

函式內呼叫：`Box :: pin`；`.map_err`；`EventStore :: summary`。

### L52 · function · impl EventRepository for EventStore lookup

```rust
fn lookup < 'a > (& 'a self , source : & 'a str , id : uuid :: Uuid ,) -> RepositoryFuture < 'a , Option < String > >
```

可見性／屬性：``；``。

函式內呼叫：`Box :: pin`；`.map_err`；`EventStore :: lookup`。

### L61 · struct ·  EventStore

```rust
# [derive (Clone)] pub struct EventStore { pool : PgPool , }
```

### L67 · function · impl EventStore new

```rust
fn new (pool : PgPool) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L72 · function · impl EventStore insert_once

```rust
async fn insert_once (& self , event : & ProductEvent) -> Result < bool , sqlx :: Error >
```

可見性／屬性：`pub`；`# [doc = " 真正持久化成功才回 2xx；同一來源/事件的重送不再增加計數。"]`。

函式內呼叫：`.to_vec`；`Sha256 :: digest`；`.as_bytes`；`format!`；`.to_vec`；`Sha256 :: digest`；`.as_bytes`；`.begin`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.rows_affected`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`.commit`；`Ok`；`.starts_with`；`.execute`；`.bind`；`sqlx :: query`；`.fetch_one`；`.bind`；`sqlx :: query_scalar :: < _ , bool >`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.unwrap_or`；`i16 :: try_from`；`.commit`；`Ok`。

### L114 · function · impl EventStore health

```rust
async fn health (& self) -> Result < () , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.execute`；`sqlx :: query`；`Ok`。

### L118 · function · impl EventStore lookup

```rust
async fn lookup (& self , source : & str , id : uuid :: Uuid ,) -> Result < Option < String > , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_optional`；`.bind`；`.bind`；`sqlx :: query_scalar`。

### L126 · function · impl EventStore summary

```rust
async fn summary (& self) -> Result < Vec < EventSummary > , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_all`；`sqlx :: query_as :: < _ , EventSummary >`。

### L136 · function · impl EventStore total

```rust
async fn total (& self) -> Result < i64 , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_one`；`sqlx :: query`；`Ok`；`.get`。

### L144 · module ·  tests

```rust
 mod tests (inline=True)
```

### L146 · use ·

```rust
use super :: * ;
```

### L148 · function · tests uuid_domain_event_roundtrips_through_native_and_varchar_storage

```rust
async fn uuid_domain_event_roundtrips_through_native_and_varchar_storage (pool : PgPool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`EventStore :: new`；`.clone`；`ProductEvent :: chat_message_persisted`；`uuid :: Uuid :: new_v4`；`assert!`；`assert_eq!`；`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.fetch_one`；`sqlx :: query_scalar`；`assert_eq!`；`assert!`；`ProductEvent :: pet_cleanup_succeeded`；`uuid :: Uuid :: new_v4`；`tokio :: join!`；`assert!`；`assert_eq!`；`.into`；`assert!`；`.unwrap`；`.execute`；`sqlx :: query`；`assert_eq!`；`assert_eq!`。

## copilot_use/apps/stats_server/src/topics.rs · A06

職責：Analytics governance/topics。判定：產品擴充；archive後重送不復活，withdraw不等archive

### L2 · use ·

```rust
use super :: { AnalyticsState , authorized } ;
```

### L3 · use ·

```rust
use axum :: { Json , extract :: State , http :: { HeaderMap , StatusCode } , } ;
```

### L8 · use ·

```rust
use serde :: Deserialize ;
```

### L9 · use ·

```rust
use uuid :: Uuid ;
```

### L10 · struct ·  Withdrawal

```rust
# [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (super) struct Withdrawal { event_ids : Vec < Uuid > , }
```

### L15 · function ·  withdraw

```rust
async fn withdraw (State (state) : State < AnalyticsState > , headers : HeaderMap , Json (input) : Json < Withdrawal > ,) -> Result < Json < serde_json :: Value > , StatusCode >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`authorized`；`Err`；`.is_empty`；`.len`；`.any`；`.iter`；`Err`；`.map_err`；`withdraw_ids`；`Ok`；`Json`；`serde_json :: json!`。

### L34 · function ·  withdraw_ids

```rust
async fn withdraw_ids (pool : & sqlx :: PgPool , ids : & [Uuid]) -> Result < () , sqlx :: Error >
```

可見性／屬性：`pub (crate)`；``。

函式內呼叫：`.begin`；`.execute`；`.bind`；`sqlx :: query`；`.execute`；`.bind`；`sqlx :: query`；`.commit`；`Ok`。

### L46 · module ·  tests

```rust
 mod tests (inline=True)
```

### L48 · use ·

```rust
use super :: * ;
```

### L49 · use ·

```rust
use crate :: EventStore ;
```

### L51 · function · tests withdrawal_before_delivery_and_retry_stays_excluded_after_admin_restore

```rust
async fn withdrawal_before_delivery_and_retry_stays_excluded_after_admin_restore (pool : sqlx :: PgPool ,)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`EventStore :: new`；`.clone`；`Uuid :: new_v4`；`.into`；`.into`；`.unwrap`；`withdraw_ids`；`assert!`；`assert_eq!`；`.unwrap`；`.execute`；`sqlx :: query`；`assert_eq!`；`assert!`；`assert!`；`event_contract :: ProductEvent :: chat_message_persisted`；`Uuid :: new_v4`；`.unwrap`；`.insert_once`；`.unwrap`；`withdraw_ids`；`assert_eq!`。

## copilot_use/crates/ai_core/src/agent.rs · X02

職責：AI Provider。判定：具體Adapter非純Domain；參考trait抽象沒有直接實作等同保證

### L2 · use ·

```rust
use crate :: { ProviderError , client , configuration } ;
```

### L3 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L4 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L5 · struct ·  ToolStep

```rust
# [derive (Debug , Serialize , Deserialize)] pub struct ToolStep { pub tool : String , pub call_id : String , }
```

### L10 · struct ·  Outcome

```rust
# [derive (Debug , Serialize , Deserialize)] pub struct Outcome { pub value : Value , pub steps : Vec < ToolStep > , }
```

### L15 · function ·  answer_schema

```rust
fn answer_schema () -> Value
```

可見性／屬性：`pub`；``。

函式內呼叫：`json!`。

### L18 · function ·  response

```rust
async fn response (body : Value) -> Result < Value , ProviderError >
```

可見性／屬性：``；``。

函式內呼叫：`configuration`；`json!`；`json!`；`.unwrap_or`；`.as_u64`；`.contains`；`Err`；`json!`；`json!`；`json!`；`.len`；`.to_string`；`Err`；`.map_err`；`.send`；`.json`；`.bearer_auth`；`.post`；`client`；`.is_success`；`.status`；`Err`；`ProviderError :: Status`；`.as_u16`；`.status`；`Vec :: new`；`.map_err`；`.chunk`；`.extend_from_slice`；`.len`；`Err`；`.map_err`；`serde_json :: from_slice`；`Err`；`Ok`。

### L60 · function ·  structured

```rust
async fn structured (input : Value , instructions : & str , schema : Value ,) -> Result < Value , ProviderError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`response`；`json!`；`output_text`；`.map_err`；`serde_json :: from_str`。

### L69 · function ·  output_text

```rust
fn output_text (value : & Value) -> Result < String , ProviderError >
```

可見性／屬性：``；``。

函式內呼叫：`String :: new`；`.ok_or`；`.as_array`；`.ok_or`；`.as_array`；`Err`；`.push_str`；`.ok_or`；`.as_str`；`.is_empty`；`.trim`；`.len`；`Err`；`Ok`。

### L94 · struct ·  Limits

```rust
# [derive (Clone , Copy)] pub struct Limits { pub requests : usize , pub tool_calls : usize , pub output_tokens : u16 , pub memory : bool , }
```

### L102 · function · impl Default for Limits default

```rust
fn default () -> Self
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L111 · function ·  persona_schema

```rust
fn persona_schema () -> Value
```

可見性／屬性：`pub`；``。

函式內呼叫：`answer_schema`；`json!`；`.push`；`.unwrap`；`.as_array_mut`；`json!`。

### L120 · function ·  structured_persona

```rust
async fn structured_persona (input : Value , instructions : & str) -> Result < Value , ProviderError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`response`；`json!`；`.map_err`；`serde_json :: from_str`；`output_text`；`validate_persona`；`Ok`。

### L127 · function ·  validate_persona

```rust
fn validate_persona (value : & Value) -> Result < () , ProviderError >
```

可見性／屬性：``；``。

函式內呼叫：`.is_none_or`；`.as_str`；`.len`；`Err`；`.clone`；`.remove`；`.ok_or`；`.as_object_mut`；`validate_answer`。

### L138 · function ·  run

```rust
async fn run < F , Fut > (input : Value , instructions : & str , tool_catalog : Vec < Value > , execute : F ,) -> Result < Outcome , ProviderError > where F : FnMut (String , Value) -> Fut , Fut : std :: future :: Future < Output = Result < Value , ProviderError > > ,
```

可見性／屬性：`pub`；``。

函式內呼叫：`run_limited`；`Limits :: default`。

### L157 · function ·  run_limited

```rust
async fn run_limited < F , Fut > (input : Value , instructions : & str , tool_catalog : Vec < Value > , mut execute : F , limits : Limits ,) -> Result < Outcome , ProviderError > where F : FnMut (String , Value) -> Fut , Fut : std :: future :: Future < Output = Result < Value , ProviderError > > ,
```

可見性／屬性：`pub`；``。

函式內呼叫：`.contains`；`.contains`；`Err`；`.len`；`.is_empty`；`.len`；`Err`；`.collect`；`.filter_map`；`.iter`；`.map`；`.as_str`；`.len`；`.len`；`.any`；`.iter`；`.is_empty`；`.len`；`.all`；`.chars`；`.is_ascii_alphanumeric`；`matches!`；`Err`；`.collect`；`.map`；`.iter`；`json!`；`.as_array`；`.clone`；`vec!`；`Vec :: new`；`json!`；`json!`；`json!`；`persona_schema`；`answer_schema`；`response`；`json!`；`.ok_or`；`.as_array`；`.collect`；`.cloned`；`.filter`；`.iter`；`.is_empty`；`.map_err`；`serde_json :: from_str`；`output_text`；`validate_persona`；`validate_answer`；`Ok`；`.len`；`.len`；`Err`；`.extend`；`.clone`；`.ok_or`；`.as_str`；`.any`；`.iter`；`Err`；`.ok_or`；`.as_str`；`.ok_or`；`.as_str`；`.len`；`Err`；`.map_err`；`serde_json :: from_str`；`execute`；`.into`；`.len`；`.to_string`；`Err`；`.push`；`json!`；`.push`；`.into`；`.into`；`Err`。

### L267 · function ·  validate_answer

```rust
fn validate_answer (value : & Value) -> Result < () , ProviderError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_none_or`；`.as_object`；`.len`；`.is_none_or`；`.as_str`；`.is_empty`；`.trim`；`.len`；`.is_none_or`；`.as_str`；`.len`；`.is_none_or`；`.as_array`；`.len`；`.any`；`.iter`；`.is_none_or`；`.as_str`；`.len`；`Err`；`Ok`。

### L283 · module ·  tests

```rust
 mod tests (inline=True)
```

### L285 · use ·

```rust
use super :: * ;
```

### L287 · function · tests partial_or_untyped_answers_are_rejected

```rust
fn partial_or_untyped_answers_are_rejected ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`；`assert!`；`assert!`。

## copilot_use/crates/ai_core/src/lib.rs · X02

職責：AI Provider。判定：具體Adapter非純Domain；參考trait抽象沒有直接實作等同保證

### L2 · module ·  agent

```rust
pub mod agent (inline=False)
```

### L3 · use ·

```rust
use serde_json :: Value ;
```

### L5 · struct ·  Decoder

```rust
# [derive (Default)] pub struct Decoder { pending : Vec < u8 > , }
```

### L10 · function · impl Decoder push

```rust
fn push (& mut self , bytes : & [u8]) -> Result < Vec < Value > , ProviderError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.extend_from_slice`；`.len`；`Err`；`Vec :: new`；`.map`；`.position`；`.windows`；`.map`；`.position`；`.windows`；`Some`；`.or`；`.map_err`；`std :: str :: from_utf8`；`.join`；`.collect`；`.filter_map`；`.lines`；`.map`；`.strip_prefix`；`.trim_start`；`.is_empty`；`.push`；`.map_err`；`serde_json :: from_str`；`.drain`；`Ok`。

### L50 · use ·

```rust
use base64 :: Engine ;
```

### L51 · use ·

```rust
use serde_json :: json ;
```

### L52 · use ·

```rust
use std :: time :: Duration ;
```

### L53 · enum ·  ProviderError

```rust
# [derive (Debug , thiserror :: Error)] pub enum ProviderError { # [error ("OpenAI 設定未完成")] Configuration , # [error ("OpenAI 請求失敗")] Transport , # [error ("OpenAI 回應狀態：{0}")] Status (u16) , # [error ("OpenAI 回應未完成或格式不合法")] Incomplete , }
```

### L64 · function ·  client

```rust
fn client () -> Result < reqwest :: Client , ProviderError >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`.build`；`.redirect`；`.timeout`；`reqwest :: Client :: builder`；`Duration :: from_secs`；`reqwest :: redirect :: Policy :: none`。

### L71 · function ·  configuration

```rust
fn configuration () -> Result < (String , String) , ProviderError >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`std :: env :: var`；`.map_err`；`std :: env :: var`；`.is_empty`；`.is_empty`；`Err`；`Ok`。

### L79 · function ·  stream_text

```rust
async fn stream_text (input : Value , instructions : & str , mut delta : impl FnMut (& str) + Send ,) -> Result < String , ProviderError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`configuration`；`.len`；`.to_string`；`.len`；`Err`；`.map_err`；`.send`；`.json`；`.bearer_auth`；`.post`；`client`；`json!`；`.is_success`；`.status`；`Err`；`ProviderError :: Status`；`.as_u16`；`.status`；`Decoder :: default`；`String :: new`；`.map_err`；`.chunk`；`.map_err`；`.push`；`.as_str`；`.ok_or`；`.as_str`；`.push_str`；`.len`；`Err`；`delta`；`.is_empty`；`.trim`；`Ok`；`Err`；`Err`。

### L125 · function ·  generate_sprite

```rust
async fn generate_sprite (prompt : & str) -> Result < (String , u32 , u32) , ProviderError >
```

可見性／屬性：`pub`；`# [doc = " 由指定的文字模型呼叫 OpenAI 生圖工具，回傳經格式/尺寸限制的 4×4 PNG sheet。"]`。

函式內呼叫：`configuration`；`.len`；`Err`；`.map_err`；`.send`；`.json`；`.bearer_auth`；`.post`；`client`；`json!`；`.is_success`；`.status`；`Err`；`ProviderError :: Status`；`.as_u16`；`.status`；`Vec :: new`；`.map_err`；`.chunk`；`.extend_from_slice`；`.len`；`Err`；`.map_err`；`serde_json :: from_slice`；`Err`；`.ok_or`；`.and_then`；`.and_then`；`.as_array`；`.find`；`.iter`；`.as_str`；`.map_err`；`.decode`；`.len`；`Err`；`u32 :: from_be_bytes`；`.map_err`；`.try_into`；`u32 :: from_be_bytes`；`.map_err`；`.try_into`；`Err`；`Ok`；`.into`。

### L178 · function ·  embedding

```rust
async fn embedding (text : & str) -> Result < Vec < f32 > , ProviderError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_empty`；`.trim`；`.len`；`Err`；`configuration`；`.unwrap_or_else`；`std :: env :: var`；`.into`；`.map_err`；`.send`；`.json`；`.bearer_auth`；`.post`；`client`；`json!`；`.is_success`；`.status`；`Err`；`ProviderError :: Status`；`.as_u16`；`.status`；`.map_err`；`.json`；`.map_err`；`serde_json :: from_value`；`.clone`；`.len`；`.any`；`.iter`；`.is_finite`；`Err`；`Ok`。

### L207 · module ·  tests

```rust
 mod tests (inline=True)
```

### L209 · use ·

```rust
use super :: * ;
```

### L211 · function · tests utf8_and_crlf_can_split_at_every_byte

```rust
fn utf8_and_crlf_can_split_at_every_byte ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`Decoder :: default`；`Vec :: new`；`.as_bytes`；`.extend`；`.unwrap`；`.push`；`assert_eq!`；`assert_eq!`。

### L222 · function · tests incomplete_and_oversized_frames_do_not_succeed

```rust
fn incomplete_and_oversized_frames_do_not_succeed ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`Decoder :: default`；`assert!`；`assert!`。

## copilot_use/crates/analytics_transport/build.rs · X04

職責：Analytics transport。判定：產品driver，timeout<30s lease；permanentstatusesblocked；proto不存在參考

### L1 · function ·  main

```rust
fn main () -> Result < () , Box < dyn std :: error :: Error > >
```

可見性／屬性：``；``。

函式內呼叫：`tonic_prost_build :: Config :: new`；`.protoc_executable`；`protoc_bin_vendored :: protoc_bin_path`；`.compile_with_config`；`tonic_prost_build :: configure`；`println!`；`Ok`。

## copilot_use/crates/analytics_transport/src/lib.rs · X04

職責：Analytics transport。判定：產品driver，timeout<30s lease；permanentstatusesblocked；proto不存在參考

### L2 · use ·

```rust
use event_contract :: ProductEvent ;
```

### L3 · use ·

```rust
use std :: time :: Duration ;
```

### L4 · use ·

```rust
use tonic :: { Code , Request , Status , transport :: Endpoint } ;
```

### L5 · module ·  proto

```rust
pub mod proto (inline=True)
```

### L8 · const ·  MAX_BATCH

```rust
pub const MAX_BATCH : usize = 64 ;
```

### L9 · const ·  MAX_MESSAGE_BYTES

```rust
pub const MAX_MESSAGE_BYTES : usize = 65536 ;
```

### L12 · function · impl From < & ProductEvent > for proto :: ProductEvent from

```rust
fn from (event : & ProductEvent) -> Self
```

可見性／屬性：``；``。

函式內呼叫：`.into`；`.to_string`；`.clone`；`.clone`。

### L22 · associated_type · impl TryFrom < proto :: ProductEvent > for ProductEvent Error

```rust
type Error = Status ;
```

### L23 · function · impl TryFrom < proto :: ProductEvent > for ProductEvent try_from

```rust
fn try_from (event : proto :: ProductEvent) -> Result < Self , Self :: Error >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`.try_into`；`Status :: invalid_argument`；`.map_err`；`uuid :: Uuid :: parse_str`；`Status :: invalid_argument`；`.is_valid`；`Err`；`Status :: invalid_argument`；`Ok`。

### L42 · function ·  deliver

```rust
async fn deliver (endpoint : & str , token : & str , event : & ProductEvent , traceparent : Option < & str > ,) -> Result < () , Status >
```

可見性／屬性：`pub`；`# [doc = " Configuration remains an HTTP origin or the existing /api/v1/events URL."]`。

函式內呼叫：`.unwrap_or`；`.strip_suffix`；`.timeout`；`.connect_timeout`；`.map_err`；`Endpoint :: from_shared`；`.to_owned`；`Status :: invalid_argument`；`Duration :: from_secs`；`Duration :: from_secs`；`.starts_with`；`.map_err`；`.tls_config`；`.with_native_roots`；`tonic :: transport :: ClientTlsConfig :: new`；`Status :: invalid_argument`；`.map_err`；`.connect`；`Status :: unavailable`；`.max_encoding_message_size`；`.max_decoding_message_size`；`proto :: event_ingestion_client :: EventIngestionClient :: new`；`Request :: new`；`vec!`；`.set_timeout`；`Duration :: from_secs`；`.insert`；`.metadata_mut`；`.map_err`；`.parse`；`format!`；`Status :: unauthenticated`；`.parse`；`.insert`；`.metadata_mut`；`.into_inner`；`.ingest_events`；`.as_slice`；`Err`；`Status :: unavailable`；`.to_string`；`Err`；`Status :: unavailable`；`proto :: Outcome :: try_from`；`Ok`；`Err`；`Status :: invalid_argument`；`Err`；`Status :: already_exists`；`Err`；`Status :: unavailable`。

### L94 · function ·  retryable

```rust
fn retryable (status : & Status) -> bool
```

可見性／屬性：`pub`；``。

函式內呼叫：`matches!`。

## copilot_use/crates/chat_application/src/lib.rs · C02

職責：Chat Application。判定：儲存後才通知的不變量對應；無 RoomRepository 等同實作；保留 durable Adapter

### L4 · use ·

```rust
pub use chat_domain :: { Committed , MessageError , MessageInput } ;
```

### L5 · use ·

```rust
use std :: { future :: Future , pin :: Pin } ;
```

### L7 · type ·  CommitFuture

```rust
# [doc = " Repository 的非同步結果，保留精確 ID 與是否為首次寫入。"] pub type CommitFuture < 'a > = Pin < Box < dyn Future < Output = Result < Committed , MessageError > > + Send + 'a > > ;
```

### L11 · trait ·  MessageRepository

```rust
# [doc = " 已接線的寫入 Port；SQL、交易與事件投遞留在 persistence Adapter。"] pub trait MessageRepository : Send + Sync { fn commit < 'a > (& 'a self , input : MessageInput < 'a >) -> CommitFuture < 'a > ; }
```

### L13 · function · MessageRepository commit

```rust
fn commit < 'a > (& 'a self , input : MessageInput < 'a >) -> CommitFuture < 'a >
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L17 · function ·  persist

```rust
async fn persist (repository : & impl MessageRepository , input : MessageInput < '_ > ,) -> Result < Committed , MessageError >
```

可見性／屬性：`pub`；`# [doc = " 只有有效輸入能進入 Port，儲存失敗不產生可發布的成功收據。"]`。

函式內呼叫：`chat_domain :: validate`；`.commit`。

### L25 · module ·  tests

```rust
 mod tests (inline=False)
```

## copilot_use/crates/chat_application/src/tests.rs · C02

職責：Chat Application。判定：儲存後才通知的不變量對應；無 RoomRepository 等同實作；保留 durable Adapter

### L1 · use ·

```rust
use super :: * ;
```

### L2 · use ·

```rust
use uuid :: Uuid ;
```

### L3 · struct ·  Failed

```rust
struct Failed ;
```

### L5 · function · impl MessageRepository for Failed commit

```rust
fn commit < 'a > (& 'a self , _ : MessageInput < 'a >) -> CommitFuture < 'a >
```

可見性／屬性：``；``。

函式內呼叫：`Box :: pin`；`Err`。

### L10 · function ·  failed_commit_never_returns_a_publishable_message

```rust
async fn failed_commit_never_returns_a_publishable_message ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`Uuid :: new_v4`；`assert_eq!`。

## copilot_use/crates/chat_domain/src/lib.rs · C01

職責：Chat Domain。判定：概念可映射、公開型別與序列化不等同；category 不能當 topic

### L2 · use ·

```rust
use uuid :: Uuid ;
```

### L4 · enum ·  MessageError

```rust
# [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum MessageError { InvalidInput , Conflict , Unavailable , }
```

### L11 · struct ·  MessageInput

```rust
# [doc = " 作者、房間及本文的寫入輸入；由入口 Adapter 映射既有協定。"] pub struct MessageInput < 'a > { pub id : Uuid , pub room : & 'a str , pub content : & 'a str , pub author_kind : & 'a str , pub author_id : Option < & 'a str > , }
```

### L20 · struct ·  Committed

```rust
# [doc = " 持久化成功後的收據；相同訊息重送保留 inserted=false 的語意。"] # [derive (Debug , PartialEq , Eq)] pub struct Committed { pub id : Uuid , pub inserted : bool , }
```

### L28 · function ·  validate

```rust
fn validate (input : & MessageInput < '_ >) -> Result < () , MessageError >
```

可見性／屬性：`pub`；`# [doc = " 對應參考 MessageAuthor／ChatMessage 的輸入不變量；不讀 DB 或系統時鐘。"]`。

函式內呼叫：`.is_none`；`.is_some_and`；`.is_empty`；`.trim`；`.len`；`.contains`；`.is_nil`；`.is_empty`；`.len`；`.contains`；`.is_empty`；`.trim`；`.len`；`.contains`；`Err`；`Ok`。

## copilot_use/crates/mcp_bridge/src/lib.rs · X03

職責：MCP。判定：產品 Driver，不進 Domain/Application；實際server另server::mcp/runtime

### L2 · use ·

```rust
use base64 :: Engine ;
```

### L3 · use ·

```rust
use http :: HeaderMap ;
```

### L4 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L5 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L6 · const ·  PROTOCOL

```rust
pub const PROTOCOL : & str = "2026-07-28" ;
```

### L7 · struct ·  Error

```rust
# [derive (Debug , thiserror :: Error)] # [error ("MCP 請求失敗或不合法")] pub struct Error ;
```

### L10 · struct ·  Request

```rust
# [derive (Debug , Deserialize , Serialize)] # [serde (deny_unknown_fields)] pub struct Request { pub jsonrpc : String , pub id : Value , pub method : String , pub params : Value , }
```

### L19 · function · impl Request new

```rust
fn new (id : u64 , method : & str , mut params : Value) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：`json!`；`.into`；`json!`；`.into`。

### L28 · function · impl Request validate

```rust
fn validate (& self) -> Result < () , (i64 , & 'static str) >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_empty`；`.len`；`.is_some_and`；`.as_str`；`.is_empty`；`.len`；`.is_some_and`；`.as_i64`；`.unsigned_abs`；`Err`；`Err`；`.is_object`；`Err`；`Ok`。

### L51 · function · impl Request validate_http

```rust
fn validate_http (& self , headers : & HeaderMap) -> Result < () , (i64 , & 'static str) >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.and_then`；`.get`；`.ok`；`.to_str`；`header`；`.as_str`；`header`；`Some`；`.as_str`；`Err`；`matches!`；`.as_str`；`.as_str`；`.and_then`；`header`；`.is_none`；`.as_deref`；`Err`；`.unwrap_or_default`；`header`；`.contains`；`.contains`；`Err`；`.validate`。

### L80 · function ·  encode_header

```rust
fn encode_header (value : & str) -> String
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_ascii`；`.trim`；`.any`；`.bytes`；`.starts_with`；`.ends_with`；`format!`；`.into`。

### L94 · function ·  decode_header

```rust
fn decode_header (value : & str) -> Option < String >
```

可見性／屬性：``；``。

函式內呼叫：`.and_then`；`.strip_prefix`；`.strip_suffix`；`.ok`；`String :: from_utf8`；`.ok`；`.decode`；`Some`；`.into`。

### L109 · function ·  result

```rust
fn result (id : Value , mut value : Value , service : & str) -> Value
```

可見性／屬性：`pub`；``。

函式內呼叫：`json!`；`json!`；`json!`。

### L115 · function ·  error

```rust
fn error (id : Value , code : i64 , message : & str) -> Value
```

可見性／屬性：`pub`；``。

函式內呼叫：`json!`；`json!`。

### L123 · function ·  discover

```rust
fn discover () -> Value
```

可見性／屬性：`pub`；``。

函式內呼叫：`json!`。

### L126 · function ·  tool

```rust
fn tool (name : & str , description : & str) -> Value
```

可見性／屬性：`pub`；``。

函式內呼叫：`json!`。

### L129 · function ·  tool_result

```rust
fn tool_result (value : Value) -> Value
```

可見性／屬性：`pub`；``。

函式內呼叫：`json!`。

### L132 · struct ·  HttpClient

```rust
# [derive (Clone)] pub struct HttpClient { endpoint : String , client : reqwest :: Client , headers : HeaderMap , }
```

### L139 · function · impl HttpClient new

```rust
fn new (endpoint : & str , headers : HeaderMap) -> Result < Self , Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.map_err`；`url :: Url :: parse`；`.host`；`.is_loopback`；`.is_loopback`；`.scheme`；`.is_empty`；`.username`；`.is_some`；`.password`；`.is_some`；`.query`；`.is_some`；`.fragment`；`Err`；`.map_err`；`.build`；`.timeout`；`.redirect`；`.no_proxy`；`reqwest :: Client :: builder`；`reqwest :: redirect :: Policy :: none`；`std :: time :: Duration :: from_secs`；`Ok`；`.into`。

### L167 · function · impl HttpClient call

```rust
async fn call (& self , method : & str , params : Value) -> Result < Value , Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Request :: new`；`.map_err`；`.validate`；`.header`；`.header`；`.header`；`.headers`；`.post`；`.clone`；`.or`；`.as_str`；`.as_str`；`.header`；`encode_header`；`.map_err`；`.send`；`.json`；`.is_success`；`.status`；`Err`；`.is_some_and`；`.and_then`；`.get`；`.headers`；`.ok`；`.to_str`；`.starts_with`；`Vec :: new`；`openai_adapter :: Decoder :: default`；`.map_err`；`.chunk`；`.extend_from_slice`；`.len`；`Err`；`.map_err`；`.push`；`.is_some`；`.get`；`.is_some`；`.get`；`Some`；`.ok_or`；`.map_err`；`serde_json :: from_slice`；`.is_some`；`.get`；`Err`；`Ok`；`.clone`。

### L225 · module ·  tests

```rust
 mod tests (inline=True)
```

### L227 · use ·

```rust
use super :: * ;
```

### L229 · function · tests protocol_and_header_mismatch_are_rejected

```rust
fn protocol_and_header_mismatch_are_rejected ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`Request :: new`；`json!`；`HeaderMap :: new`；`.insert`；`.unwrap`；`.parse`；`.insert`；`.unwrap`；`.parse`；`.insert`；`.unwrap`；`.parse`；`.insert`；`.unwrap`；`.parse`；`assert!`；`.insert`；`.unwrap`；`.parse`；`assert_eq!`；`json!`；`assert_eq!`。

### L251 · function · tests unsafe_header_values_are_encoded_and_round_trip

```rust
fn unsafe_header_values_are_encoded_and_round_trip ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert_eq!`。

### L262 · function · tests remote_or_credential_urls_are_not_accepted

```rust
fn remote_or_credential_urls_are_not_accepted ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`；`assert!`。

## copilot_use/crates/persistence/src/database.rs · C04

職責：Chat SQLite。判定：同為 SQLite，schema/交易邊界不相同；參考 41 字元測試斷言與文字相反

### L2 · use ·

```rust
use sqlx :: { SqlitePool , sqlite :: { SqliteConnectOptions , SqliteJournalMode , SqlitePoolOptions , SqliteSynchronous } , } ;
```

### L6 · use ·

```rust
use std :: { path :: Path , str :: FromStr , time :: Duration } ;
```

### L8 · type ·  Error

```rust
type Error = Box < dyn std :: error :: Error + Send + Sync > ;
```

### L9 · function ·  open

```rust
async fn open (url : & str) -> Result < SqlitePool , Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.starts_with`；`.contains`；`Err`；`.into`；`SqliteConnectOptions :: from_str`；`.get_filename`；`.filter`；`.parent`；`.is_empty`；`.as_os_str`；`.exists`；`std :: fs :: create_dir_all`；`std :: fs :: set_permissions`；`std :: fs :: Permissions :: from_mode`；`.is_symlink`；`.file_type`；`std :: fs :: symlink_metadata`；`Err`；`.into`；`std :: fs :: metadata`；`.uid`；`libc :: getuid`；`.mode`；`.permissions`；`Err`；`.into`；`.exists`；`std :: fs :: symlink_metadata`；`.is_file`；`.is_symlink`；`.file_type`；`Err`；`.into`；`std :: fs :: OpenOptions :: new`；`.create_new`；`.write`；`.mode`；`.open`；`.uid`；`std :: fs :: metadata`；`libc :: getuid`；`.nlink`；`std :: fs :: metadata`；`Err`；`.into`；`std :: fs :: set_permissions`；`std :: fs :: Permissions :: from_mode`；`std :: path :: PathBuf :: from`；`format!`；`std :: fs :: symlink_metadata`；`.is_file`；`.is_symlink`；`.file_type`；`Err`；`.into`；`.connect_with`；`.acquire_timeout`；`.max_connections`；`SqlitePoolOptions :: new`；`Duration :: from_secs`；`.busy_timeout`；`.synchronous`；`.journal_mode`；`.foreign_keys`；`.create_if_missing`；`Duration :: from_secs`；`.run`；`sqlx :: migrate!`；`Ok`。

### L90 · function ·  backup

```rust
async fn backup (pool : & SqlitePool , destination : & Path) -> Result < () , Error >
```

可見性／屬性：`pub`；`# [doc = " SQLite VACUUM INTO is a consistent online snapshot including WAL commits; never copy a live main file alone."]`。

函式內呼叫：`std :: fs :: OpenOptions :: new`；`.create_new`；`.write`；`.mode`；`.open`；`.execute`；`.bind`；`sqlx :: query`；`.ok_or`；`.to_str`；`.sync_all`；`.connect_with`；`.max_connections`；`SqlitePoolOptions :: new`；`.read_only`；`.filename`；`SqliteConnectOptions :: new`；`.fetch_one`；`sqlx :: query_scalar`；`.fetch_all`；`sqlx :: query`；`.close`；`.is_empty`；`Err`；`.into`；`Ok`。

### L125 · module ·  tests

```rust
 mod tests (inline=False)
```

## copilot_use/crates/persistence/src/database/tests.rs · C04

職責：Chat SQLite。判定：同為 SQLite，schema/交易邊界不相同；參考 41 字元測試斷言與文字相反

### L1 · use ·

```rust
use super :: * ;
```

### L3 · function ·  snapshot_is_portable_with_wal_commits_and_never_overwrites_existing_data

```rust
async fn snapshot_is_portable_with_wal_commits_and_never_overwrites_existing_data ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`std :: fs :: create_dir`；`.unwrap`；`std :: fs :: set_permissions`；`std :: fs :: Permissions :: from_mode`；`.join`；`.join`；`.unwrap`；`open`；`format!`；`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.persist`；`crate :: ChatWriter :: new`；`.clone`；`.unwrap`；`backup`；`.unwrap`；`std :: fs :: read`；`assert!`；`assert_eq!`；`.join`；`.unwrap`；`std :: fs :: rename`；`.unwrap`；`open`；`format!`；`.unwrap`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`assert_eq!`；`.unwrap`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`assert_eq!`；`.close`；`.close`；`.unwrap`；`std :: fs :: remove_dir_all`。

### L49 · function ·  sqlite_rejects_symlink_and_hardlinked_databases

```rust
async fn sqlite_rejects_symlink_and_hardlinked_databases ()
```

可見性／屬性：``；`# [cfg (unix)] # [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`std :: fs :: create_dir`；`.unwrap`；`std :: fs :: set_permissions`；`std :: fs :: Permissions :: from_mode`；`.join`；`.unwrap`；`std :: fs :: write`；`.join`；`.unwrap`；`std :: os :: unix :: fs :: symlink`；`assert!`；`.unwrap`；`std :: fs :: remove_file`；`.unwrap`；`std :: fs :: hard_link`；`assert!`；`.unwrap`；`std :: fs :: remove_dir_all`。

## copilot_use/crates/persistence/src/lib.rs · C04

職責：Chat SQLite。判定：同為 SQLite，schema/交易邊界不相同；參考 41 字元測試斷言與文字相反

### L3 · use ·

```rust
pub use chat_application as application ;
```

### L4 · module ·  database

```rust
pub mod database (inline=False)
```

### L5 · module ·  topics

```rust
pub mod topics (inline=False)
```

### L6 · use ·

```rust
use event_contract :: ProductEvent ;
```

### L7 · use ·

```rust
use sqlx :: { Row , SqlitePool } ;
```

### L8 · use ·

```rust
use std :: time :: Duration ;
```

### L9 · use ·

```rust
use uuid :: Uuid ;
```

### L11 · struct ·  ChatWriter

```rust
# [derive (Clone)] pub struct ChatWriter { pool : SqlitePool , }
```

### L16 · function · impl application :: MessageRepository for ChatWriter commit

```rust
fn commit < 'a > (& 'a self , input : application :: MessageInput < 'a >) -> application :: CommitFuture < 'a >
```

可見性／屬性：``；``。

函式內呼叫：`Box :: pin`；`.persist_with_id`；`Ok`；`Err`；`Err`。

### L36 · function · impl ChatWriter new

```rust
fn new (pool : SqlitePool) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L41 · function · impl ChatWriter persist

```rust
async fn persist (& self , room_id : & str , content : & str) -> Result < Uuid , sqlx :: Error >
```

可見性／屬性：`pub`；`# [doc = " 必須先 commit 才能呼叫使用者程式的 WS publish。"]`。

函式內呼叫：`.persist_as`。

### L46 · function · impl ChatWriter persist_as

```rust
async fn persist_as (& self , room_id : & str , content : & str , author_kind : & str , author_id : Option < & str > ,) -> Result < Uuid , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.map`；`.persist_with_id`；`Uuid :: new_v4`。

### L60 · function · impl ChatWriter persist_with_id

```rust
async fn persist_with_id (& self , id : Uuid , room_id : & str , content : & str , author_kind : & str , author_id : Option < & str > ,) -> Result < (Uuid , bool) , sqlx :: Error >
```

可見性／屬性：`pub`；`# [doc = " 重送 ID 僅能對應相同房間、作者與本文；"] # [doc = " INSERT 衝突序列化保證並發寫入只建立一筆 Outbox。"]`。

函式內呼叫：`.is_nil`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`.begin_with`；`Self :: stage`；`.commit`；`Ok`。

### L76 · function · impl ChatWriter stage

```rust
async fn stage (tx : & mut sqlx :: Transaction < '_ , sqlx :: Sqlite > , id : Uuid , room_id : & str , content : & str , author_kind : & str , author_id : Option < & str > ,) -> Result < (Uuid , bool) , sqlx :: Error >
```

可見性／屬性：``；``。

函式內呼叫：`ProductEvent :: chat_message_persisted`；`.rows_affected`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.fetch_one`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query_scalar`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`Ok`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.unwrap`；`i16 :: try_from`；`.get`；`copilot_telemetry :: outgoing_headers`；`topics :: stage`；`.execute`；`.bind`；`sqlx :: query`；`.to_string`；`serde_json :: json!`；`Ok`。

### L112 · function · impl ChatWriter persist_persona_turn

```rust
async fn persist_persona_turn (& self , room : & str , persona : Uuid , revision : i64 , run : Uuid , lease : Uuid , turn : & serde_json :: Value ,) -> Result < Uuid , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.ok_or_else`；`.filter`；`.as_str`；`.is_empty`；`.trim`；`.len`；`sqlx :: Error :: Protocol`；`.into`；`.ok_or_else`；`.filter`；`.as_str`；`.len`；`sqlx :: Error :: Protocol`；`.into`；`.begin_with`；`.fetch_one`；`.bind`；`.bind`；`sqlx :: query_scalar`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`Uuid :: new_v4`；`.clone`；`serde_json :: json!`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.rows_affected`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.rows_affected`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`Self :: stage`；`Some`；`.to_string`；`.commit`；`Ok`。

### L151 · struct ·  LeasedEvent

```rust
# [derive (Debug , Clone)] pub struct LeasedEvent { pub event : ProductEvent , pub lease_token : Uuid , pub attempts : i32 , pub traceparent : Option < String > , }
```

### L159 · struct ·  OutboxStore

```rust
# [derive (Clone)] pub struct OutboxStore { pool : SqlitePool , }
```

### L164 · function · impl OutboxStore new

```rust
fn new (pool : SqlitePool) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L169 · function · impl OutboxStore claim_one

```rust
async fn claim_one (& self) -> Result < Option < LeasedEvent > , sqlx :: Error >
```

可見性／屬性：`pub`；`# [doc = " atomic SQLite claim 防止多 worker 併發領取同一事件，租約過期允許重新領取。"]`。

函式內呼叫：`Uuid :: new_v4`；`.fetch_optional`；`.bind`；`sqlx :: query`；`Ok`；`.map`；`.get`；`.get`；`.get`；`.get`；`.get`；`.get`。

### L190 · function · impl OutboxStore ack

```rust
async fn ack (& self , lease : & LeasedEvent) -> Result < bool , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.execute`；`.bind`；`.bind`；`sqlx :: query`；`Ok`；`.rows_affected`。

### L204 · function · impl OutboxStore fail

```rust
async fn fail (& self , lease : & LeasedEvent , error_code : & str , retryable : bool ,) -> Result < bool , sqlx :: Error >
```

可見性／屬性：`pub`；`# [doc = " 拒絕過期 token 的 ACK/重試；超過 10 次封鎖供人工處置。"]`。

函式內呼叫：`.min`；`.min`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`Ok`；`.rows_affected`。

### L230 · function ·  dispatch_once

```rust
async fn dispatch_once (store : & OutboxStore , _client : & reqwest :: Client , endpoint : & str , token : & str ,) -> Result < bool , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.claim_one`；`Ok`；`.or_else`；`.clone`；`.cloned`；`.get`；`copilot_telemetry :: outgoing_headers`；`analytics_transport :: deliver`；`.as_deref`；`.ack`；`.fail`；`analytics_transport :: retryable`；`Ok`。

### L262 · function ·  run_dispatcher

```rust
async fn run_dispatcher (pool : SqlitePool , endpoint : & str , token : & str ,) -> Result < () , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：`pub`；`# [doc = " 預期外部配置有權限的 HTTPS endpoint 與 Bearer Token。"]`。

函式內呼叫：`.build`；`.redirect`；`.timeout`；`reqwest :: Client :: builder`；`Duration :: from_secs`；`reqwest :: redirect :: Policy :: none`；`OutboxStore :: new`；`topics :: dispatch_withdrawal`；`dispatch_once`；`matches!`；`tokio :: task :: yield_now`；`tokio :: task :: yield_now`；`tokio :: time :: sleep`；`Duration :: from_millis`；`eprintln!`；`tokio :: time :: sleep`；`Duration :: from_millis`。

### L287 · module ·  tests

```rust
 mod tests (inline=False)
```

## copilot_use/crates/persistence/src/tests.rs · C04

職責：Chat SQLite。判定：同為 SQLite，schema/交易邊界不相同；參考 41 字元測試斷言與文字相反

### L1 · use ·

```rust
use super :: * ;
```

### L3 · function ·  persona_memory_message_event_and_run_commit_only_with_current_authority

```rust
async fn persona_memory_message_event_and_run_commit_only_with_current_authority (pool : SqlitePool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`Uuid :: new_v4`；`Uuid :: new_v4`；`Uuid :: new_v4`；`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.execute`；`.bind`；`sqlx :: query`；`.unwrap`；`.execute`；`.bind`；`sqlx :: query`；`.unwrap`；`.execute`；`.bind`；`sqlx :: query`；`ChatWriter :: new`；`.clone`；`serde_json :: json!`；`assert!`；`assert_eq!`；`assert_eq!`；`assert!`；`.unwrap`；`.persist_persona_turn`；`assert_eq!`；`assert_eq!`；`assert!`；`.unwrap`；`.execute`；`sqlx :: query`；`assert!`。

### L83 · function ·  event_id_is_message_id_and_stable_for_retries

```rust
fn event_id_is_message_id_and_stable_for_retries ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`Uuid :: new_v4`；`assert_eq!`。

### L88 · function ·  message_insert_has_exactly_one_outbox_event

```rust
async fn message_insert_has_exactly_one_outbox_event (pool : SqlitePool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.persist`；`ChatWriter :: new`；`.clone`；`.unwrap`；`.fetch_one`；`.bind`；`sqlx :: query_as`；`assert_eq!`。

### L105 · function ·  missing_room_rolls_back_both_message_and_event

```rust
async fn missing_room_rolls_back_both_message_and_event (pool : SqlitePool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`.persist`；`ChatWriter :: new`；`.clone`；`assert!`；`.unwrap`；`.fetch_one`；`sqlx :: query_as`；`assert_eq!`。

### L118 · function ·  stale_lease_cannot_ack

```rust
async fn stale_lease_cannot_ack (pool : SqlitePool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.persist`；`ChatWriter :: new`；`.clone`；`OutboxStore :: new`；`.clone`；`.unwrap`；`.unwrap`；`.claim_one`；`assert_eq!`；`.unwrap`；`.execute`；`.bind`；`sqlx :: query`；`.unwrap`；`.unwrap`；`.claim_one`；`assert!`；`assert!`。

### L140 · function ·  outbox_insert_failure_rolls_back_inserted_message

```rust
async fn outbox_insert_failure_rolls_back_inserted_message (pool : SqlitePool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.execute`；`sqlx :: raw_sql`；`assert!`；`.unwrap`；`.fetch_one`；`sqlx :: query_scalar`；`assert_eq!`。

### L160 · function ·  expired_lease_can_be_reclaimed_and_old_token_cannot_mutate

```rust
async fn expired_lease_can_be_reclaimed_and_old_token_cannot_mutate (pool : SqlitePool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.persist`；`ChatWriter :: new`；`.clone`；`OutboxStore :: new`；`.clone`；`.unwrap`；`.unwrap`；`.claim_one`；`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.unwrap`；`.claim_one`；`assert_eq!`；`assert_ne!`；`assert!`；`assert!`；`assert!`。

### L183 · function ·  permanent_delivery_failure_is_blocked

```rust
async fn permanent_delivery_failure_is_blocked (pool : SqlitePool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.persist`；`ChatWriter :: new`；`.clone`；`OutboxStore :: new`；`.clone`；`.unwrap`；`.unwrap`；`.claim_one`；`assert!`；`assert!`。

### L198 · function ·  concurrent_message_replay_has_one_outbox_and_conflicts_are_rejected

```rust
async fn concurrent_message_replay_has_one_outbox_and_conflicts_are_rejected (pool : SqlitePool)
```

可見性／屬性：``；`# [sqlx :: test (migrations = "./migrations")]`。

函式內呼叫：`.unwrap`；`.execute`；`sqlx :: query`；`ChatWriter :: new`；`.clone`；`Uuid :: new_v4`；`tokio :: join!`；`.unwrap`；`.unwrap`；`assert_eq!`；`assert_ne!`；`assert!`；`.unwrap`；`.fetch_one`；`sqlx :: query_as`；`assert_eq!`。

## copilot_use/crates/persistence/src/topics.rs · C12

職責：Chat 同意撤回。判定：產品隱私擴充；category 與 topic 語意分離

### L2 · use ·

```rust
use sqlx :: { Row , SqlitePool } ;
```

### L3 · use ·

```rust
use uuid :: Uuid ;
```

### L4 · function ·  consent

```rust
async fn consent (pool : & SqlitePool , user : Uuid , room : & str ,) -> Result < serde_json :: Value , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_one`；`.bind`；`.bind`；`sqlx :: query_scalar`；`.fetch_one`；`.bind`；`.bind`；`sqlx :: query_scalar`；`Ok`；`serde_json :: json!`。

### L15 · function ·  set_consent

```rust
async fn set_consent (pool : & SqlitePool , user : Uuid , room : & str , enabled : bool ,) -> Result < () , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`Ok`。

### L24 · function ·  stage

```rust
async fn stage (tx : & mut sqlx :: Transaction < '_ , sqlx :: Sqlite > , message : Uuid , room : & str , author_kind : & str , author_id : Option < & str > , content : & str ,) -> Result < () , sqlx :: Error >
```

可見性／屬性：`pub (crate)`；``。

函式內呼叫：`Ok`；`.and_then`；`.ok`；`Uuid :: parse_str`；`Ok`；`.fetch_optional`；`.bind`；`.bind`；`sqlx :: query_scalar`；`Some`；`Ok`；`event_contract :: topics :: occurrences`；`Uuid :: new_v4`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`format!`；`.get`；`copilot_telemetry :: outgoing_headers`；`Ok`。

### L63 · function ·  queue_batch

```rust
async fn queue_batch (pool : & SqlitePool) -> Result < () , sqlx :: Error >
```

可見性／屬性：``；``。

函式內呼叫：`.begin_with`；`.fetch_optional`；`sqlx :: query`；`.get`；`.get`；`.get`；`.fetch_all`；`.bind`；`.bind`；`.bind`；`sqlx :: query_scalar`；`.is_empty`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`Uuid :: new_v4`；`.map_err`；`serde_json :: to_string`；`sqlx :: Error :: Protocol`；`.to_string`；`.execute`；`.bind`；`sqlx :: query`；`.commit`；`Ok`。

### L95 · function ·  dispatch_withdrawal

```rust
async fn dispatch_withdrawal (pool : & SqlitePool , client : & reqwest :: Client , endpoint : & str , token : & str ,) -> Result < bool , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：`pub`；``。

函式內呼叫：`queue_batch`；`Uuid :: new_v4`；`.fetch_optional`；`.bind`；`sqlx :: query`；`Ok`；`.get`；`serde_json :: from_str`；`.get`；`.get`；`reqwest :: Url :: parse`；`.set_path`；`.set_query`；`.set_fragment`；`.json`；`.bearer_auth`；`.post`；`serde_json :: json!`；`copilot_telemetry :: outgoing_headers`；`.header`；`.is_ok_and`；`.send`；`.is_success`；`.status`；`.begin_with`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.rows_affected`；`.execute`；`.bind`；`sqlx :: query`；`.commit`；`.min`；`.min`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`Ok`。

## copilot_use/crates/pet_domain/src/application.rs · P02

職責：Pet Profile。判定：概念對應，格式/驗證/錯誤不等同；ProjectionPending 必須保留

### L2 · use ·

```rust
use crate :: profile :: PetProfile ;
```

### L3 · use ·

```rust
use std :: { future :: Future , pin :: Pin } ;
```

### L4 · use ·

```rust
use uuid :: Uuid ;
```

### L5 · enum ·  ProfileError

```rust
# [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum ProfileError { Invalid , Unavailable , ProjectionPending , }
```

### L12 · type ·  ProfileFuture

```rust
# [doc = " 投影尚未完成仍回報 ProjectionPending，不把部分成功視為完整成功。"] pub type ProfileFuture < 'a > = Pin < Box < dyn Future < Output = Result < () , ProfileError > > + Send + 'a > > ;
```

### L15 · trait ·  ProfileRepository

```rust
# [doc = " 實際由 PetStore 實作、Tauri 呼叫的設定 Port。"] pub trait ProfileRepository : Send + Sync { fn save < 'a > (& 'a self , id : Uuid , profile : & 'a PetProfile) -> ProfileFuture < 'a > ; }
```

### L17 · function · ProfileRepository save

```rust
fn save < 'a > (& 'a self , id : Uuid , profile : & 'a PetProfile) -> ProfileFuture < 'a >
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L21 · function ·  save_profile

```rust
async fn save_profile (repository : & impl ProfileRepository , id : Uuid , profile : & PetProfile ,) -> Result < () , ProfileError >
```

可見性／屬性：`pub`；`# [doc = " 使用既有 Profile 的驗證，通過後才委派持久化。"]`。

函式內呼叫：`.is_nil`；`.is_err`；`.validate`；`Err`；`.save`。

### L32 · module ·  tests

```rust
 mod tests (inline=False)
```

## copilot_use/crates/pet_domain/src/application/tests.rs · P02

職責：Pet Profile。判定：概念對應，格式/驗證/錯誤不等同；ProjectionPending 必須保留

### L1 · use ·

```rust
use super :: * ;
```

### L2 · use ·

```rust
use std :: sync :: atomic :: { AtomicBool , Ordering } ;
```

### L4 · struct ·  Unavailable

```rust
struct Unavailable { called : AtomicBool , }
```

### L8 · function · impl ProfileRepository for Unavailable save

```rust
fn save < 'a > (& 'a self , _ : Uuid , _ : & 'a PetProfile) -> ProfileFuture < 'a >
```

可見性／屬性：``；``。

函式內呼叫：`Box :: pin`；`.store`；`Err`。

### L17 · function ·  invalid_profile_never_reaches_adapter_and_projection_failure_is_not_success

```rust
async fn invalid_profile_never_reaches_adapter_and_projection_failure_is_not_success ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`AtomicBool :: new`；`.into`；`.into`；`vec!`；`vec!`；`vec!`；`assert_eq!`；`.clear`；`assert_eq!`；`assert!`；`.into`；`assert_eq!`；`assert!`。

## copilot_use/crates/pet_domain/src/learning.rs · P10

職責：Pet 學習/備份。判定：產品擴充；不在Domain匯SQLx，backup與reference preview不是對應功能

### L2 · struct ·  Schedule

```rust
# [derive (Debug , Clone , Copy , PartialEq)] pub struct Schedule { pub repetitions : i64 , pub interval_days : i64 , pub ease : f64 , pub due_at : i64 , }
```

### L10 · function · impl Schedule review

```rust
fn review (& mut self , quality : u8 , at : i64) -> Result < () , & 'static str >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Err`；`.clamp`；`.round`；`.clamp`；`Ok`。

### L34 · module ·  tests

```rust
 mod tests (inline=True)
```

### L36 · use ·

```rust
use super :: * ;
```

### L38 · function · tests invalid_quality_preserves_schedule_and_failure_resets_interval

```rust
fn invalid_quality_preserves_schedule_and_failure_resets_interval ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`；`assert_eq!`；`.unwrap`；`.review`；`assert_eq!`；`.unwrap`；`.review`；`assert_eq!`；`.unwrap`；`.review`；`assert_eq!`。

## copilot_use/crates/pet_domain/src/lib.rs · P01

職責：Pet FSM。判定：狀態與命令不相容；Sleep/Wake/revision 目前無等同流程，不能空造同名型別

### L2 · module ·  application

```rust
pub mod application (inline=False)
```

### L3 · module ·  learning

```rust
pub mod learning (inline=False)
```

### L4 · module ·  profile

```rust
pub mod profile (inline=False)
```

## copilot_use/crates/pet_domain/src/profile.rs · P02

職責：Pet Profile。判定：概念對應，格式/驗證/錯誤不等同；ProjectionPending 必須保留

### L3 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L4 · use ·

```rust
use std :: collections :: BTreeSet ;
```

### L5 · use ·

```rust
use thiserror :: Error ;
```

### L6 · use ·

```rust
use uuid :: Uuid ;
```

### L8 · const ·  PET_WIDTH

```rust
pub const PET_WIDTH : u32 = 200 ;
```

### L9 · const ·  PET_HEIGHT

```rust
pub const PET_HEIGHT : u32 = 200 ;
```

### L11 · enum ·  ProfileError

```rust
# [derive (Debug , Error)] pub enum ProfileError { # [error ("invalid profile field: {0}")] Invalid (& 'static str) , # [error ("invalid pet id")] InvalidId , # [error ("failed to parse profile: {0}")] Parse (# [from] toml :: de :: Error) , }
```

### L21 · struct ·  AppearanceOverride

```rust
# [derive (Debug , Clone , Serialize , Deserialize , Default , PartialEq , Eq)] # [serde (deny_unknown_fields)] pub struct AppearanceOverride { # [serde (default , rename = "顏色" , alias = "color")] pub color : Option < String > , # [serde (default , rename = "風格" , alias = "style")] pub style : Option < String > , # [serde (default , rename = "品種" , alias = "breed")] pub breed : Option < String > , }
```

### L32 · struct ·  AnimationOverride

```rust
# [derive (Debug , Clone , Serialize , Deserialize , Default , PartialEq , Eq)] # [serde (deny_unknown_fields)] pub struct AnimationOverride { # [serde (default , rename = "動作" , alias = "actions")] pub actions : Option < Vec < String > > , # [serde (default , rename = "速度" , alias = "speed")] pub speed : Option < String > , }
```

### L41 · struct ·  PetProfile

```rust
# [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq)] # [serde (deny_unknown_fields)] pub struct PetProfile { # [serde (rename = "名字" , alias = "name")] pub name : String , # [serde (rename = "種類" , alias = "species")] pub species : String , # [serde (rename = "喜好" , alias = "likes")] pub likes : Vec < String > , # [serde (rename = "個性" , alias = "personality")] pub personality : Vec < String > , # [serde (rename = "興趣" , alias = "interests")] pub interests : Vec < String > , # [serde (default , rename = "外觀" , alias = "appearance")] pub appearance : Option < AppearanceOverride > , # [serde (default , rename = "動畫" , alias = "animation")] pub animation : Option < AnimationOverride > , }
```

### L60 · function ·  valid_item

```rust
fn valid_item (text : & str) -> bool
```

可見性／屬性：``；``。

函式內呼叫：`.is_empty`；`.trim`；`.len`；`.contains`。

### L64 · function ·  valid_items

```rust
fn valid_items (items : & [String]) -> bool
```

可見性／屬性：``；``。

函式內呼叫：`.len`；`.all`；`.iter`；`valid_item`。

### L69 · function · impl PetProfile parse

```rust
fn parse (input : & str) -> Result < Self , ProfileError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`toml :: from_str`；`.validate`；`Ok`。

### L76 · function · impl PetProfile user_toml

```rust
fn user_toml (& self) -> Result < String , serde_json :: Error >
```

可見性／屬性：`pub`；`# [doc = " TOML 1.0-compatible keys, including quoted Chinese table names."]`。

函式內呼叫：`.push_str`；`format!`；`Ok`；`String :: new`；`line`；`line`；`line`；`line`；`line`；`.push_str`；`line`；`.push_str`；`line`；`line`；`Ok`。

### L118 · function · impl PetProfile validate

```rust
fn validate (& self) -> Result < () , ProfileError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`valid_item`；`Err`；`ProfileError :: Invalid`；`valid_item`；`Err`；`ProfileError :: Invalid`；`valid_items`；`Err`；`ProfileError :: Invalid`；`valid_items`；`Err`；`ProfileError :: Invalid`；`valid_items`；`Err`；`ProfileError :: Invalid`；`.is_some_and`；`.as_ref`；`valid_item`；`Err`；`ProfileError :: Invalid`；`.is_some_and`；`.as_ref`；`valid_items`；`.is_some_and`；`.as_ref`；`valid_item`；`Err`；`ProfileError :: Invalid`；`Ok`。

### L150 · function · impl PetProfile requested_actions

```rust
fn requested_actions (& self) -> BTreeSet < String >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.unwrap_or_default`；`.map`；`.and_then`；`.as_ref`；`.as_ref`；`.collect`；`.cloned`；`.iter`。

### L159 · struct ·  AssetManifest

```rust
# [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq)] pub struct AssetManifest { pub schema_version : u16 , pub pet_id : Uuid , pub original_image_width : u32 , pub original_image_height : u32 , pub display_width : u32 , pub display_height : u32 , pub asset_version : u32 , pub generated_frames : Vec < String > , pub profile_hash : String , pub sha256 : String , pub file : String , pub columns : u8 , pub rows : Vec < String > , pub frames_per_row : u8 , }
```

### L178 · function · impl AssetManifest generated

```rust
fn generated (mut self , version : u32 , profile_hash : String , sha256 : String) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：`format!`。

### L185 · function · impl AssetManifest new

```rust
fn new (pet_id : Uuid , image_width : u32 , image_height : u32) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：`.collect`；`.flat_map`；`.into_iter`；`.map`；`format!`；`String :: new`；`String :: new`；`String :: new`；`.collect`；`.map`；`.into_iter`。

### L211 · module ·  tests

```rust
 mod tests (inline=True)
```

### L213 · use ·

```rust
use super :: * ;
```

### L214 · const ·  MINIMAL

```rust
const MINIMAL : & str = r#""名字" = "小啾"
"種類" = "鳥"
"喜好" = ["唱歌", "水果"]
"個性" = ["吵雜"]
"興趣" = ["音樂", "探索"]
"# ;
```

### L222 · function · tests five_fields_are_enough_without_advanced_settings

```rust
fn five_fields_are_enough_without_advanced_settings ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.unwrap`；`PetProfile :: parse`；`assert_eq!`；`assert!`；`assert!`。

### L230 · function · tests optional_override_has_priority_without_forcing_defaults

```rust
fn optional_override_has_priority_without_forcing_defaults ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.unwrap`；`PetProfile :: parse`；`format!`；`assert_eq!`；`assert!`。

### L243 · function · tests image_size_is_independent_from_display_bounds

```rust
fn image_size_is_independent_from_display_bounds ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`AssetManifest :: new`；`Uuid :: new_v4`；`assert_eq!`；`assert_eq!`。

### L253 · function · tests missing_required_fields_fail

```rust
fn missing_required_fields_fail ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`；`assert!`。

## copilot_use/crates/pet_runtime/src/agent.rs · P11

職責：Pet Chat/Agent/生圖。判定：參考 mock 不等同成品AI；真實AI本輪未執行，readonlytool限制保留

### L2 · use ·

```rust
use crate :: state :: { PetStore , now } ;
```

### L3 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L4 · use ·

```rust
use std :: { path :: Path , sync :: { Arc , Mutex } , } ;
```

### L8 · use ·

```rust
use tokio :: io :: { AsyncReadExt , AsyncWriteExt } ;
```

### L9 · use ·

```rust
use uuid :: Uuid ;
```

### L11 · function · impl PetStore mcp_request

```rust
async fn mcp_request (& self , pet : Uuid , value : Value) -> Value
```

可見性／屬性：`pub`；``。

函式內呼叫：`.clone`；`serde_json :: from_value`；`mcp_bridge :: error`；`.validate`；`mcp_bridge :: error`；`.unwrap_or`；`.fetch_optional`；`.bind`；`sqlx :: query_scalar`；`.to_string`；`.is_none`；`mcp_bridge :: error`；`.as_str`；`mcp_bridge :: discover`；`json!`；`.is_none_or`；`.as_object`；`.len`；`.is_none_or`；`.as_str`；`.is_empty`；`.trim`；`.len`；`mcp_bridge :: error`；`.to_lowercase`；`.unwrap`；`.as_str`；`.memories`；`mcp_bridge :: error`；`.collect`；`.take`；`.split_whitespace`；`.collect`；`.map`；`.take`；`.filter`；`.iter`；`.any`；`.iter`；`.contains`；`.to_lowercase`；`json!`；`.is_empty`；`.collect`；`.map`；`.take`；`.iter`；`json!`；`mcp_bridge :: tool_result`；`json!`；`mcp_bridge :: error`；`mcp_bridge :: result`。

### L72 · function · impl PetStore stdio_memory_search

```rust
async fn stdio_memory_search (& self , pet : Uuid , query : & str , executable : & Path ,) -> Result < Value , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：`pub`；``。

函式內呼叫：`stdio_call`；`.database_path`；`json!`；`Err`；`.into`；`Ok`；`.clone`。

### L91 · function · impl PetStore agent_talk

```rust
async fn agent_talk (& self , pet : Uuid , question : & str , executable : & Path ,) -> Result < Value , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_empty`；`.trim`；`.len`；`Err`；`.into`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`.to_string`；`stdio_call`；`.database_path`；`json!`；`.ok_or`；`.cloned`；`.as_array`；`.to_owned`；`.to_owned`；`.database_path`；`Arc :: new`；`Mutex :: new`；`Vec :: < String > :: new`；`.clone`；`format!`；`tokio :: time :: timeout`；`std :: time :: Duration :: from_secs`；`openai_adapter :: agent :: run`；`json!`；`.clone`；`.clone`；`.clone`；`.map_err`；`stdio_call`；`json!`；`Err`；`.clone`；`.as_array`；`.map_err`；`.lock`；`.as_str`；`.push`；`.into`；`Ok`；`.map_err`；`.lock`；`.any`；`.iter`；`.ok_or`；`.as_array`；`.any`；`.iter`；`.as_str`；`Some`；`Err`；`.into`；`.ok_or`；`.as_str`；`.begin`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`Uuid :: new_v4`；`.to_string`；`now`；`Uuid :: new_v4`；`.into`；`.into`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`serde_json :: to_string`；`.get`；`copilot_telemetry :: outgoing_headers`；`.commit`；`Ok`；`json!`。

### L202 · function ·  stdio_call

```rust
async fn stdio_call (executable : & Path , database : & Path , pet : Uuid , method : & str , params : Value ,) -> Result < Value , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：``；``。

函式內呼叫：`tokio :: process :: Command :: new`；`.env_clear`；`.arg`；`std :: env :: var_os`；`.env`；`.kill_on_drop`；`.stderr`；`.stdout`；`.stdin`；`.env`；`.env`；`.to_string`；`std :: process :: Stdio :: piped`；`std :: process :: Stdio :: piped`；`std :: process :: Stdio :: null`；`.spawn`；`.ok_or`；`.take`；`serde_json :: to_vec`；`mcp_bridge :: Request :: new`；`.push`；`.write_all`；`drop`；`.ok_or`；`.take`；`Vec :: new`；`tokio :: time :: timeout`；`std :: time :: Duration :: from_secs`；`.read_to_end`；`.take`；`.len`；`.success`；`.wait`；`Err`；`.into`；`serde_json :: from_slice`；`.is_some`；`.get`；`Err`；`.into`；`Ok`；`.clone`。

### L255 · module ·  tests

```rust
 mod tests (inline=True)
```

### L257 · use ·

```rust
use super :: * ;
```

### L259 · function · tests mcp_memory_scope_cannot_be_overridden_by_tool_arguments

```rust
async fn mcp_memory_scope_cannot_be_overridden_by_tool_arguments ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`PetStore :: open`；`Uuid :: new_v4`；`Uuid :: new_v4`；`.unwrap`；`crate :: profile :: PetProfile :: parse`；`include_str!`；`.unwrap`；`.save_profile`；`.unwrap`；`.save_profile`；`.unwrap`；`.save_memory`；`Uuid :: new_v4`；`.unwrap`；`.save_memory`；`Uuid :: new_v4`；`mcp_bridge :: Request :: new`；`json!`；`.mcp_request`；`.unwrap`；`serde_json :: to_value`；`.unwrap`；`.as_array`；`assert_eq!`；`assert_eq!`；`mcp_bridge :: Request :: new`；`json!`；`assert!`；`.close`；`.unwrap`；`std :: fs :: remove_file`。

## copilot_use/crates/pet_runtime/src/ai.rs · P11

職責：Pet Chat/Agent/生圖。判定：參考 mock 不等同成品AI；真實AI本輪未執行，readonlytool限制保留

### L2 · use ·

```rust
use crate :: { profile :: PetProfile , state :: { PetStore , now } , } ;
```

### L6 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L7 · use ·

```rust
use sha2 :: { Digest , Sha256 } ;
```

### L8 · use ·

```rust
use uuid :: Uuid ;
```

### L10 · function · impl PetStore talk

```rust
async fn talk (& self , pet : Uuid , text : & str , delta : impl FnMut (& str) + Send ,) -> Result < String , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_empty`；`.trim`；`.len`；`Err`；`.into`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`.to_string`；`.memories`；`serde_json :: to_string`；`.learning_summary`；`format!`；`.fetch_all`；`.bind`；`sqlx :: query_as`；`.to_string`；`.collect`；`.map`；`.into_iter`；`json!`；`.push`；`json!`；`openai_adapter :: stream_text`；`json!`；`.begin`；`.as_str`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`Uuid :: new_v4`；`.to_string`；`now`；`Uuid :: new_v4`；`.into`；`.into`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`serde_json :: to_string`；`.get`；`copilot_telemetry :: outgoing_headers`；`.commit`；`Ok`。

### L72 · function · impl PetStore generate_asset

```rust
async fn generate_asset (& self , pet : Uuid ,) -> Result < Value , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`.to_string`；`serde_json :: from_str`；`.validate`；`format!`；`openai_adapter :: generate_sprite`；`.save_generated_asset`；`i64 :: from`；`i64 :: from`；`.sync_pet_files`；`Ok`；`json!`。

### L92 · function · impl PetStore save_generated_asset

```rust
async fn save_generated_asset (& self , pet : Uuid , profile : & str , png : & str , width : i64 , height : i64 ,) -> Result < () , sqlx :: Error >
```

可見性／屬性：``；``。

函式內呼叫：`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`now`；`format!`；`.rows_affected`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`Ok`。

### L109 · function · impl PetStore asset

```rust
async fn asset (& self , pet : Uuid) -> Result < Value , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_optional`；`.bind`；`sqlx :: query_as`；`.to_string`；`Ok`；`.unwrap_or`；`.map`；`.filter`；`format!`；`json!`。

### L114 · module ·  tests

```rust
 mod tests (inline=True)
```

### L116 · use ·

```rust
use super :: * ;
```

### L118 · function · tests stale_images_never_replace_or_render_a_changed_profile

```rust
async fn stale_images_never_replace_or_render_a_changed_profile ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.unwrap`；`tempfile :: tempdir`；`.join`；`.path`；`.unwrap`；`PetStore :: open`；`Uuid :: new_v4`；`.unwrap`；`serde_json :: from_str`；`.unwrap`；`.save_profile`；`.unwrap`；`serde_json :: to_string`；`.unwrap`；`.save_generated_asset`；`assert_eq!`；`.into`；`.unwrap`；`.save_profile`；`assert!`；`assert!`；`assert_eq!`；`.close`；`.unwrap`；`std :: fs :: remove_file`。

## copilot_use/crates/pet_runtime/src/backup.rs · P10

職責：Pet 學習/備份。判定：產品擴充；不在Domain匯SQLx，backup與reference preview不是對應功能

### L3 · use ·

```rust
use crate :: state :: PetStore ;
```

### L4 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L5 · use ·

```rust
use sha2 :: { Digest , Sha256 } ;
```

### L6 · use ·

```rust
use std :: { io :: Read , path :: PathBuf } ;
```

### L7 · use ·

```rust
use uuid :: Uuid ;
```

### L8 · type ·  Error

```rust
type Error = Box < dyn std :: error :: Error + Send + Sync > ;
```

### L9 · struct ·  Snapshot

```rust
# [derive (Serialize , Deserialize)] # [serde (deny_unknown_fields)] pub struct Snapshot { pub id : Uuid , pub sha256 : String , pub created_at : i64 , pub bytes : u64 , }
```

### L18 · function · impl PetStore backups

```rust
fn backups (& self) -> Result < PathBuf , Error >
```

可見性／屬性：``；``。

函式內呼叫：`Ok`；`.join`；`.ok_or`；`.parent`；`.database_path`。

### L25 · function · impl PetStore create_snapshot

```rust
async fn create_snapshot (& self) -> Result < Snapshot , Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.backups`；`std :: fs :: create_dir_all`；`.is_symlink`；`.file_type`；`std :: fs :: symlink_metadata`；`Err`；`.into`；`Uuid :: new_v4`；`.join`；`.to_string`；`std :: fs :: create_dir`；`std :: fs :: set_permissions`；`std :: fs :: Permissions :: from_mode`；`.join`；`.execute`；`.bind`；`sqlx :: query`；`.ok_or`；`.to_str`；`.len`；`std :: fs :: metadata`；`Err`；`.into`；`std :: fs :: read`；`format!`；`crate :: state :: now`；`.len`；`std :: fs :: write`；`.join`；`serde_json :: to_vec`；`.sync_all`；`.open`；`.write`；`.read`；`std :: fs :: OpenOptions :: new`；`.sync_all`；`.open`；`.write`；`.read`；`std :: fs :: OpenOptions :: new`；`.join`；`.sync_all`；`std :: fs :: File :: open`；`Ok`。

### L73 · function · impl PetStore snapshots

```rust
fn snapshots (& self) -> Result < Vec < Snapshot > , Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.backups`；`.exists`；`Ok`；`vec!`；`Vec :: new`；`std :: fs :: read_dir`；`.is_dir`；`.file_type`；`Uuid :: parse_str`；`.to_string_lossy`；`.file_name`；`.join`；`.path`；`.exists`；`.is_symlink`；`.file_type`；`std :: fs :: symlink_metadata`；`Vec :: new`；`.read_to_end`；`.take`；`std :: fs :: File :: open`；`.len`；`serde_json :: from_slice`；`.push`；`.sort_by_key`；`std :: cmp :: Reverse`；`.truncate`；`Ok`。

### L110 · function · impl PetStore restore_snapshot

```rust
async fn restore_snapshot (& self , id : Uuid , confirmed : bool) -> Result < Snapshot , Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Err`；`.into`；`.join`；`.backups`；`.to_string`；`.is_symlink`；`.file_type`；`std :: fs :: symlink_metadata`；`Err`；`.into`；`.join`；`.is_symlink`；`.file_type`；`std :: fs :: symlink_metadata`；`Err`；`.into`；`serde_json :: from_slice`；`std :: fs :: read`；`Err`；`.into`；`.join`；`std :: fs :: OpenOptions :: new`；`.read`；`.custom_flags`；`.open`；`.is_file`；`.metadata`；`Err`；`.into`；`Vec :: new`；`.read_to_end`；`.take`；`.len`；`format!`；`Err`；`.into`；`.create_snapshot`；`.join`；`.backups`；`format!`；`std :: fs :: OpenOptions :: new`；`.create_new`；`.write`；`.mode`；`.open`；`.write_all`；`.sync_all`；`PetStore :: open`；`.close`；`.restore_verified`；`std :: fs :: remove_file`；`.map_err`；`.sync_all_pet_files`；`Ok`。

### L183 · function · impl PetStore restore_verified

```rust
async fn restore_verified (& self , path : & std :: path :: Path) -> Result < () , Error >
```

可見性／屬性：``；``。

函式內呼叫：`.acquire`；`.execute`；`.bind`；`sqlx :: query`；`.ok_or`；`.to_str`；`.fetch_one`；`sqlx :: query_scalar`；`Err`；`.into`；`.fetch_one`；`sqlx :: query_scalar`；`Err`；`.into`；`.execute`；`sqlx :: query`；`.execute`；`sqlx :: query`；`.execute`；`sqlx :: query`；`.execute`；`sqlx :: query`；`.execute`；`sqlx :: query`；`.execute`；`sqlx :: query`；`Ok :: < () , sqlx :: Error >`；`.execute`；`sqlx :: query`；`.execute`；`sqlx :: query`；`Err`；`.into`；`Ok :: < () , Error >`；`.execute`；`sqlx :: query`；`Ok`。

### L220 · module ·  tests

```rust
 mod tests (inline=True)
```

### L222 · use ·

```rust
use super :: * ;
```

### L224 · function · tests older_snapshot_is_upgraded_in_private_copy_without_changing_original

```rust
async fn older_snapshot_is_upgraded_in_private_copy_without_changing_original ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`std :: fs :: create_dir`；`.unwrap`；`PetStore :: open`；`.join`；`Uuid :: new_v4`；`.unwrap`；`serde_json :: from_str`；`.unwrap`；`.save_profile`；`Uuid :: new_v4`；`.unwrap`；`.save_memory`；`.unwrap`；`.create_snapshot`；`.join`；`.unwrap`；`.backups`；`.to_string`；`.join`；`.unwrap`；`sqlx :: SqlitePool :: connect_with`；`.filename`；`sqlx :: sqlite :: SqliteConnectOptions :: new`；`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.execute`；`sqlx :: query`；`.close`；`.unwrap`；`std :: fs :: read`；`format!`；`.len`；`.unwrap`；`std :: fs :: write`；`.join`；`.unwrap`；`serde_json :: to_vec`；`.unwrap`；`.save_memory`；`.unwrap`；`.restore_snapshot`；`assert_eq!`；`assert_eq!`；`.unwrap`；`sqlx :: SqlitePool :: connect_with`；`.read_only`；`.filename`；`sqlx :: sqlite :: SqliteConnectOptions :: new`；`assert_eq!`；`.close`；`.close`；`.unwrap`；`std :: fs :: remove_dir_all`。

### L285 · function · tests restore_write_failure_rolls_back_all_user_data_and_preserves_safety_snapshot

```rust
async fn restore_write_failure_rolls_back_all_user_data_and_preserves_safety_snapshot ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`std :: fs :: create_dir`；`.unwrap`；`PetStore :: open`；`.join`；`Uuid :: new_v4`；`.unwrap`；`serde_json :: from_str`；`.unwrap`；`.save_profile`；`Uuid :: new_v4`；`.unwrap`；`.save_memory`；`.unwrap`；`.create_snapshot`；`.unwrap`；`.save_memory`；`.unwrap`；`.execute`；`sqlx :: query`；`assert!`；`assert_eq!`；`assert_eq!`；`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.restore_snapshot`；`assert_eq!`；`.close`；`.unwrap`；`std :: fs :: remove_dir_all`。

### L312 · function · tests snapshots_restore_profiles_memories_and_assets_without_losing_delivery_or_new_pets

```rust
async fn snapshots_restore_profiles_memories_and_assets_without_losing_delivery_or_new_pets ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`std :: fs :: create_dir`；`.unwrap`；`PetStore :: open`；`.join`；`Uuid :: new_v4`；`.unwrap`；`serde_json :: from_str`；`.unwrap`；`.save_profile`；`Uuid :: new_v4`；`.unwrap`；`.save_memory`；`.unwrap`；`.add_card`；`.unwrap`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`.unwrap`；`.create_snapshot`；`assert_eq!`；`.unwrap`；`.save_memory`；`Uuid :: new_v4`；`.unwrap`；`.save_profile`；`.unwrap`；`.execute`；`sqlx :: query`；`assert!`；`.unwrap`；`.restore_snapshot`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`.unwrap`；`.restore_snapshot`；`assert_eq!`；`.unwrap`；`std :: fs :: write`；`.join`；`.join`；`.join`；`.to_string`；`assert!`；`assert_eq!`；`.close`；`.unwrap`；`std :: fs :: remove_dir_all`。

## copilot_use/crates/pet_runtime/src/files.rs · P09

職責：Pet Store/Profile 投影。判定：產品擴充；對照Profile與五欄概念；資料隔離／filepath不進publicevent

### L3 · use ·

```rust
use crate :: { profile :: { AssetManifest , PetProfile } , state :: PetStore , } ;
```

### L7 · use ·

```rust
use base64 :: Engine ;
```

### L8 · use ·

```rust
use serde_json :: json ;
```

### L9 · use ·

```rust
use sha2 :: { Digest , Sha256 } ;
```

### L10 · use ·

```rust
use std :: { fs , io :: { Read , Write } , path :: { Path , PathBuf } , time :: { Duration , Instant } , } ;
```

### L16 · use ·

```rust
use uuid :: Uuid ;
```

### L17 · type ·  Error

```rust
type Error = Box < dyn std :: error :: Error + Send + Sync > ;
```

### L19 · function ·  private_directory

```rust
fn private_directory (path : & Path) -> Result < () , Error >
```

可見性／屬性：``；``。

函式內呼叫：`fs :: create_dir`；`.kind`；`Err`；`.into`；`fs :: symlink_metadata`；`.is_dir`；`.is_symlink`；`.file_type`；`Err`；`.into`；`.uid`；`fs :: metadata`；`libc :: geteuid`；`Err`；`.into`；`fs :: set_permissions`；`fs :: Permissions :: from_mode`；`.open`；`.custom_flags`；`.access_mode`；`fs :: OpenOptions :: new`；`crate :: windows_fs :: make_private`；`Ok`。

### L50 · function ·  prepare_data_directory

```rust
fn prepare_data_directory (path : & Path) -> Result < () , Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`private_directory`。

### L53 · function ·  options

```rust
fn options () -> fs :: OpenOptions
```

可見性／屬性：``；``。

函式內呼叫：`fs :: OpenOptions :: new`；`.write`；`.read`；`.custom_flags`；`.mode`；`.custom_flags`；`.access_mode`。

### L72 · function ·  lock

```rust
async fn lock (root : & Path) -> Result < fs :: File , Error >
```

可見性／屬性：``；``。

函式內呼叫：`.open`；`.truncate`；`.create`；`options`；`.join`；`.is_file`；`.metadata`；`Err`；`.into`；`Instant :: now`；`Duration :: from_millis`；`.try_lock`；`Ok`；`Instant :: now`；`tokio :: time :: sleep`；`Duration :: from_millis`；`Err`；`.into`。

### L91 · function ·  atomic

```rust
fn atomic (path : & Path , bytes : & [u8]) -> Result < () , Error >
```

可見性／屬性：``；``。

函式內呼叫：`fs :: symlink_metadata`；`.is_file`；`.is_symlink`；`.file_type`；`Err`；`.into`；`.ok_or`；`.parent`；`.join`；`format!`；`(| | -> Result < () , Error > { let mut file = options () . create_new (true) . open (& temporary) ? ; # [cfg (windows)] crate :: windows_fs :: make_private (& file) ? ; file . write_all (bytes) ? ; file . sync_all () ? ; drop (file) ; fs :: rename (& temporary , path) ? ; # [cfg (unix)] fs :: File :: open (parent) ? . sync_all () ? ; Ok (()) })`；`.open`；`.create_new`；`options`；`crate :: windows_fs :: make_private`；`.write_all`；`.sync_all`；`drop`；`fs :: rename`；`.sync_all`；`fs :: File :: open`；`Ok`；`.is_err`；`fs :: remove_file`。

### L116 · function ·  read_profile

```rust
fn read_profile (root : & Path , pet : Uuid) -> Result < PetProfile , Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_nil`；`Err`；`.into`；`.to_path_buf`；`.join`；`.to_string`；`fs :: symlink_metadata`；`.is_dir`；`.is_symlink`；`.file_type`；`Err`；`.into`；`.join`；`.join`；`.to_string`；`fs :: symlink_metadata`；`.is_file`；`.is_symlink`；`.file_type`；`.len`；`Err`；`.into`；`String :: new`；`.read_to_string`；`.take`；`fs :: File :: open`；`Ok`；`PetProfile :: parse`。

### L138 · function · impl PetStore files_root

```rust
fn files_root (& self) -> Result < PathBuf , Error >
```

可見性／屬性：``；``。

函式內呼叫：`.join`；`.ok_or`；`.parent`；`.database_path`；`private_directory`；`Ok`。

### L148 · function · impl PetStore sync_pet_files

```rust
async fn sync_pet_files (& self , pet : Uuid) -> Result < () , Error >
```

可見性／屬性：`pub`；`# [doc = " A UUID-derived path only; this never grants FileOrganizer authorization."]`。

函式內呼叫：`.sync_pet_files_mode`。

### L151 · function · impl PetStore sync_pet_files_mode

```rust
async fn sync_pet_files_mode (& self , pet : Uuid , preserve_manual : bool) -> Result < () , Error >
```

可見性／屬性：``；``。

函式內呼叫：`.is_nil`；`Err`；`.into`；`.lock`；`.files_root`；`lock`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`.to_string`；`serde_json :: from_str`；`.validate`；`.join`；`.to_string`；`private_directory`；`format!`；`.join`；`.join`；`.is_ok_and`；`fs :: symlink_metadata`；`.is_file`；`.is_symlink`；`.file_type`；`.len`；`.is_ok_and`；`fs :: symlink_metadata`；`.is_file`；`.is_symlink`；`.file_type`；`.len`；`serde_json :: from_slice`；`fs :: read`；`.to_string`；`atomic`；`.join`；`.as_bytes`；`.user_toml`；`atomic`；`.join`；`serde_json :: to_vec`；`json!`；`.join`；`private_directory`；`.fetch_optional`；`.bind`；`sqlx :: query_as`；`.to_string`；`.filter`；`Err`；`.into`；`.decode`；`.len`；`.starts_with`；`Err`；`.into`；`.join`；`private_directory`；`.join`；`.to_string`；`private_directory`；`atomic`；`.join`；`.generated`；`AssetManifest :: new`；`u32 :: try_from`；`u32 :: try_from`；`u32 :: try_from`；`format!`；`atomic`；`.join`；`serde_json :: to_vec`；`atomic`；`.join`；`serde_json :: to_vec`；`json!`；`Ok`。

### L238 · function · impl PetStore reconcile_pet_files_startup

```rust
async fn reconcile_pet_files_startup (& self) -> Result < () , Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.pets`；`.sync_pet_files_mode`；`Uuid :: parse_str`；`Ok`。

### L245 · function · impl PetStore sync_all_pet_files

```rust
async fn sync_all_pet_files (& self) -> Result < () , Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.pets`；`.sync_pet_files`；`Uuid :: parse_str`；`Ok`。

### L251 · function · impl PetStore import_pet_profile

```rust
async fn import_pet_profile (& self , pet : Uuid , confirmed : bool) -> Result < () , Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_nil`；`Err`；`.into`；`.files_root`；`read_profile`；`.save_profile`。

### L261 · module ·  tests

```rust
 mod tests (inline=True)
```

### L263 · use ·

```rust
use super :: * ;
```

### L265 · function · tests concurrent_first_projection_creation_preserves_each_pet

```rust
async fn concurrent_first_projection_creation_preserves_each_pet ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.unwrap`；`tempfile :: tempdir`；`.unwrap`；`PetStore :: open`；`.join`；`.path`；`std :: sync :: Arc :: new`；`tokio :: sync :: Barrier :: new`；`Vec :: new`；`.clone`；`.clone`；`.push`；`tokio :: spawn`；`Uuid :: new_v4`；`.unwrap`；`serde_json :: from_value`；`json!`；`.wait`；`.save_profile`；`.sync_pet_files`；`panic!`；`.wait`；`.unwrap`；`assert!`；`.close`。

### L301 · function · tests actual_per_pet_files_are_isolated_and_confirmed_import_updates_only_selected_pet

```rust
async fn actual_per_pet_files_are_isolated_and_confirmed_import_updates_only_selected_pet ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`fs :: create_dir`；`.unwrap`；`PetStore :: open`；`.join`；`.unwrap`；`serde_json :: from_value`；`json!`；`Uuid :: new_v4`；`Uuid :: new_v4`；`.unwrap`；`.save_profile`；`.unwrap`；`.save_profile`；`.join`；`.join`；`.join`；`.to_string`；`.join`；`.join`；`.join`；`.to_string`；`.unwrap`；`fs :: read`；`.unwrap`；`toml :: from_str`；`.unwrap`；`fs :: read_to_string`；`assert_eq!`；`.clone`；`.into`；`.unwrap`；`fs :: write`；`.unwrap`；`toml :: to_string`；`.unwrap`；`.reconcile_pet_files_startup`；`assert!`；`assert!`；`.unwrap`；`.import_pet_profile`；`assert_eq!`；`assert!`；`.unwrap`；`serde_json :: from_slice`；`.unwrap`；`fs :: read`；`.join`；`.unwrap`；`.parent`；`assert_eq!`；`.close`；`.unwrap`；`fs :: remove_dir_all`。

### L358 · function · tests gui_optional_empty_overrides_roundtrip_through_toml

```rust
async fn gui_optional_empty_overrides_roundtrip_through_toml ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`fs :: create_dir`；`.unwrap`；`PetStore :: open`；`.join`；`Uuid :: new_v4`；`.unwrap`；`serde_json :: from_value`；`json!`；`.unwrap`；`.save_profile`；`.unwrap`；`.import_pet_profile`；`.close`；`.unwrap`；`fs :: remove_dir_all`。

### L371 · function · tests unsafe_projection_never_writes_outside_and_can_be_repaired_from_canonical_database

```rust
async fn unsafe_projection_never_writes_outside_and_can_be_repaired_from_canonical_database ()
```

可見性／屬性：``；`# [cfg (unix)] # [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`fs :: create_dir`；`.unwrap`；`PetStore :: open`；`.join`；`.unwrap`；`serde_json :: from_value`；`json!`；`Uuid :: new_v4`；`.unwrap`；`.save_profile`；`.join`；`.join`；`.join`；`.to_string`；`.join`；`.unwrap`；`fs :: write`；`.unwrap`；`fs :: remove_file`；`.unwrap`；`std :: os :: unix :: fs :: symlink`；`.into`；`assert!`；`assert_eq!`；`.unwrap`；`fs :: remove_file`；`.unwrap`；`.sync_pet_files`；`assert!`；`.close`；`.unwrap`；`fs :: remove_dir_all`。

## copilot_use/crates/pet_runtime/src/interaction.rs · P01

職責：Pet FSM。判定：狀態與命令不相容；Sleep/Wake/revision 目前無等同流程，不能空造同名型別

### L2 · use ·

```rust
use crate :: state :: { PetStore , now } ;
```

### L3 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L4 · use ·

```rust
use uuid :: Uuid ;
```

### L5 · struct ·  Interaction

```rust
# [derive (Clone , Debug , Default , Serialize , Deserialize , PartialEq)] # [serde (deny_unknown_fields)] pub struct Interaction { pub action : Action , pub speed : Speed , pub proactive : bool , }
```

### L12 · enum ·  Action

```rust
# [derive (Clone , Copy , Debug , Default , Serialize , Deserialize , PartialEq)] # [serde (rename_all = "snake_case")] pub enum Action { # [default] Idle , Moving , Happy , Learning , }
```

### L21 · enum ·  Speed

```rust
# [derive (Clone , Copy , Debug , Default , Serialize , Deserialize , PartialEq)] # [serde (rename_all = "snake_case")] pub enum Speed { Slow , # [default] Normal , Fast , }
```

### L29 · struct ·  LearningSummary

```rust
# [derive (Serialize)] pub struct LearningSummary { pub total : i64 , pub due : i64 , pub needs_practice : i64 , pub established : i64 , }
```

### L37 · function · impl PetStore interaction

```rust
async fn interaction (& self , pet : Uuid) -> Result < Interaction , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_optional`；`.bind`；`sqlx :: query_scalar`；`format!`；`.map`；`.transpose`；`.map`；`.map_err`；`serde_json :: from_str`；`sqlx :: Error :: Protocol`；`.into`；`.unwrap_or_default`。

### L50 · function · impl PetStore set_interaction

```rust
async fn set_interaction (& self , pet : Uuid , preferences : & Interaction ,) -> Result < () , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_one`；`.bind`；`sqlx :: query_scalar :: < _ , bool >`；`.to_string`；`Err`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`format!`；`.map_err`；`serde_json :: to_string`；`sqlx :: Error :: Protocol`；`.to_string`；`Ok`。

### L67 · function · impl PetStore learning_summary

```rust
async fn learning_summary (& self , pet : Uuid) -> Result < LearningSummary , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_one`；`.bind`；`.bind`；`sqlx :: query_as`；`now`；`.to_string`；`Ok`。

### L77 · function · impl PetStore recall_prompt

```rust
async fn recall_prompt (& self) -> Result < Option < serde_json :: Value > , sqlx :: Error >
```

可見性／屬性：`pub`；`# [doc = " Atomic one-hour cooldown across windows/processes; only current active pet, explicitly opted in."]`。

函式內呼叫：`.active_pet`；`Ok`；`.map_err`；`Uuid :: parse_str`；`sqlx :: Error :: Protocol`；`.into`；`.interaction`；`Ok`；`.begin`；`.fetch_optional`；`.bind`；`.bind`；`sqlx :: query_as`；`now`；`Ok`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`format!`；`.to_string`；`now`；`now`；`.rows_affected`；`Ok`；`.commit`；`Ok`；`Some`；`serde_json :: json!`。

### L107 · module ·  tests

```rust
 mod tests (inline=True)
```

### L109 · use ·

```rust
use super :: * ;
```

### L111 · function · tests local_reminders_require_opt_in_and_are_scoped_durable_and_rate_limited

```rust
async fn local_reminders_require_opt_in_and_are_scoped_durable_and_rate_limited ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`std :: fs :: create_dir`；`.join`；`.unwrap`；`PetStore :: open`；`Uuid :: new_v4`；`.unwrap`；`serde_json :: from_str`；`.unwrap`；`.save_profile`；`.unwrap`；`.select_pet`；`.unwrap`；`.add_card`；`assert!`；`.unwrap`；`.set_interaction`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`assert!`；`.close`；`.unwrap`；`PetStore :: open`；`assert_eq!`；`assert!`；`.close`；`.unwrap`；`std :: fs :: remove_dir_all`；`assert!`。

## copilot_use/crates/pet_runtime/src/lib.rs · P08

職責：Pet 真 OS quarantine/restore。判定：產品安全擴充；FS與DB非單一交易，明確部分成功與恢復，不假稱零窗口

### L3 · module ·  agent

```rust
pub mod agent (inline=False)
```

### L4 · module ·  ai

```rust
pub mod ai (inline=False)
```

### L5 · use ·

```rust
pub use pet_domain :: application ;
```

### L6 · module ·  backup

```rust
pub mod backup (inline=False)
```

### L7 · module ·  files

```rust
pub mod files (inline=False)
```

### L8 · module ·  interaction

```rust
pub mod interaction (inline=False)
```

### L9 · module ·  outbox

```rust
pub mod outbox (inline=False)
```

### L10 · use ·

```rust
pub use pet_domain :: profile ;
```

### L11 · module ·  registry

```rust
pub mod registry (inline=False)
```

### L12 · module ·  state

```rust
pub mod state (inline=False)
```

### L13 · module ·  suggestions

```rust
pub mod suggestions (inline=False)
```

### L14 · module ·  unix_fs

```rust
 mod unix_fs (inline=False)
```

### L16 · module ·  windows_fs

```rust
 mod windows_fs (inline=False)
```

### L18 · use ·

```rust
use event_contract :: ProductEvent ;
```

### L19 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L20 · use ·

```rust
use std :: { fs , io , path :: { Path , PathBuf } , } ;
```

### L24 · use ·

```rust
use thiserror :: Error ;
```

### L25 · use ·

```rust
use uuid :: Uuid ;
```

### L26 · use ·

```rust
# [cfg (windows)] use windows_fs as unix_fs ;
```

### L29 · enum ·  CleanupError

```rust
# [derive (Debug , Error)] pub enum CleanupError { # [error ("invalid or unauthorized file name")] InvalidName , # [error ("refuse symbolic link or non-regular file")] UnsafeItem , # [error ("item changed after preview")] Changed , # [error ("operation requires recovery: {0}")] RecoveryRequired (Uuid) , # [error ("IO: {0}")] Io (# [from] io :: Error) , }
```

### L43 · struct ·  FilePreview

```rust
# [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq)] pub struct FilePreview { pub file_name : String , pub size_bytes : u64 , pub last_modified_nanos : u128 , # [serde (default)] pub identity : Option < [u64 ; 3] > , }
```

### L52 · struct ·  CleanupReceipt

```rust
# [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq)] pub struct CleanupReceipt { pub operation_id : Uuid , pub quarantined_file : String , pub size_bytes : u64 , }
```

### L59 · struct ·  TrashEntry

```rust
# [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq)] pub struct TrashEntry { pub operation_id : Uuid , pub file_name : String , pub size_bytes : u64 , pub restored : bool , }
```

### L66 · struct ·  RestoreReceipt

```rust
# [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq)] pub struct RestoreReceipt { pub operation_id : Uuid , pub quarantine_operation_id : Uuid , pub size_bytes : u64 , }
```

### L73 · function · impl RestoreReceipt event

```rust
fn event (& self) -> ProductEvent
```

可見性／屬性：`pub`；``。

函式內呼叫：`.into`；`.into`。

### L84 · function · impl CleanupReceipt event

```rust
fn event (& self) -> ProductEvent
```

可見性／屬性：`pub`；`# [doc = " 真實 rename 成功後才建立產品事件；預覽不產生成功事件。"]`。

函式內呼叫：`ProductEvent :: pet_cleanup_succeeded`。

### L89 · struct ·  AuthorizedRoot

```rust
pub struct AuthorizedRoot { # [cfg_attr (any (target_os = "linux" , target_os = "macos" , target_os = "windows") , allow (dead_code))] root : PathBuf , # [cfg (any (target_os = "linux" , target_os = "macos" , target_os = "windows"))] pinned : unix_fs :: PinnedRoot , }
```

### L99 · function ·  validate_name

```rust
fn validate_name (name : & str) -> Result < () , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`.is_empty`；`.len`；`.starts_with`；`.ends_with`；`.ends_with`；`.any`；`.chars`；`.is_control`；`matches!`；`Err`；`Ok`。

### L114 · function ·  modified_nanos

```rust
fn modified_nanos (meta : & fs :: Metadata) -> Result < u128 , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`Ok`；`.as_nanos`；`.map_err`；`.duration_since`；`.modified`；`io :: Error :: other`。

### L123 · function · impl AuthorizedRoot new

```rust
fn new (path : impl AsRef < Path >) -> Result < Self , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.next`；`.components`；`.as_ref`；`matches!`；`Err`；`fs :: canonicalize`；`.is_dir`；`Err`；`unix_fs :: PinnedRoot :: open`；`Ok`。

### L146 · function · impl AuthorizedRoot authorized_path

```rust
fn authorized_path (& self) -> & Path
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L150 · function · impl AuthorizedRoot scan

```rust
fn scan (& self) -> Result < Vec < FilePreview > , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.scan`；`Vec :: new`；`.take`；`fs :: read_dir`；`.to_str`；`.file_name`；`.preview`；`.push`；`Ok`。

### L169 · function · impl AuthorizedRoot preview

```rust
fn preview (& self , name : & str) -> Result < FilePreview , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.preview`；`validate_name`；`.join`；`fs :: symlink_metadata`；`.is_file`；`.file_type`；`.is_symlink`；`.file_type`；`Err`；`Ok`；`.to_string`；`.len`；`modified_nanos`。

### L193 · function · impl AuthorizedRoot confirm_quarantine

```rust
fn confirm_quarantine (& self , preview : & FilePreview , confirmed : bool ,) -> Result < CleanupReceipt , CleanupError >
```

可見性／屬性：`pub`；`# [doc = " 必須由 UI 明確要求確認；只把一般檔案移至同一個授權根的 quarantine。"] # [doc = " Linux 平台以固定目錄 descriptor、NOREPLACE 與捕獲後 inode 驗證完成操作。"]`。

函式內呼叫：`Err`；`.quarantine`；`Err`；`CleanupError :: Io`；`io :: Error :: new`。

### L214 · function · impl AuthorizedRoot trash

```rust
fn trash (& self) -> Result < Vec < TrashEntry > , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.trash`；`Err`；`.into`；`io :: Error :: new`。

### L224 · function · impl AuthorizedRoot confirm_restore

```rust
fn confirm_restore (& self , id : Uuid , confirmed : bool ,) -> Result < RestoreReceipt , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Err`；`.restore`；`Err`；`.into`；`io :: Error :: new`。

### L242 · function · impl AuthorizedRoot restored_receipts

```rust
fn restored_receipts (& self) -> Result < Vec < RestoreReceipt > , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.restored_receipts`；`Err`；`.into`；`io :: Error :: new`。

### L252 · function · impl AuthorizedRoot recovered_receipts

```rust
fn recovered_receipts (& self) -> Result < Vec < CleanupReceipt > , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.recovered_receipts`；`Err`；`CleanupError :: Io`；`io :: Error :: new`。

### L267 · module ·  tests

```rust
 mod tests (inline=True)
```

### L269 · use ·

```rust
use super :: * ;
```

### L270 · function · tests private_directory

```rust
fn private_directory (path : impl AsRef < Path >)
```

可見性／屬性：``；``。

函式內呼叫：`.unwrap`；`fs :: create_dir_all`；`.unwrap`；`fs :: set_permissions`；`fs :: Permissions :: from_mode`。

### L278 · function · tests private_fixture

```rust
fn private_fixture (path : impl AsRef < Path > , contents : & [u8])
```

可見性／屬性：``；``。

函式內呼叫：`.unwrap`；`fs :: write`；`.unwrap`；`fs :: set_permissions`；`fs :: Permissions :: from_mode`。

### L289 · function · tests preview_is_readonly_confirmation_is_required

```rust
fn preview_is_readonly_confirmation_is_required ()
```

可見性／屬性：``；`# [cfg (any (target_os = "linux" , target_os = "macos"))] # [test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`private_directory`；`private_fixture`；`.join`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`.preview`；`assert!`；`assert!`；`assert!`；`.unwrap`；`.confirm_quarantine`；`assert!`；`assert!`；`assert_eq!`；`.unwrap`；`fs :: remove_dir_all`。

### L310 · function · tests reject_relative_escape_and_changed_file

```rust
fn reject_relative_escape_and_changed_file ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`private_directory`；`private_fixture`；`.join`；`.unwrap`；`AuthorizedRoot :: new`；`assert!`；`.unwrap`；`.preview`；`.unwrap`；`fs :: write`；`.join`；`assert!`；`assert!`；`assert_eq!`；`drop`；`.unwrap`；`fs :: remove_dir_all`。

### L333 · function · tests unverified_platform_cannot_move_file

```rust
fn unverified_platform_cannot_move_file ()
```

可見性／屬性：``；`# [cfg (not (any (target_os = "linux" , target_os = "macos" , target_os = "windows")))] # [test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`private_directory`；`private_fixture`；`.join`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`.preview`；`assert!`；`assert_eq!`；`.unwrap`；`fs :: remove_dir_all`。

### L347 · function · tests restore_process_child

```rust
fn restore_process_child ()
```

可見性／屬性：``；`# [cfg (any (target_os = "linux" , target_os = "macos"))] # [cfg (any (target_os = "linux" , target_os = "macos"))] # [test] # [ignore = "Only invoked by the parent with its own disposable fixture"]`。

函式內呼叫：`PathBuf :: from`；`.unwrap`；`std :: env :: var`；`.unwrap`；`Uuid :: parse_str`；`.unwrap`；`std :: env :: var`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`fs :: write`；`.join`；`format!`；`std :: time :: Instant :: now`；`std :: time :: Duration :: from_secs`；`.exists`；`.join`；`assert!`；`std :: thread :: sleep`；`std :: time :: Duration :: from_millis`；`.confirm_restore`；`.kind`；`panic!`。

### L365 · function · tests two_processes_restore_with_one_durable_receipt

```rust
fn two_processes_restore_with_one_durable_receipt ()
```

可見性／屬性：``；`# [cfg (any (target_os = "linux" , target_os = "macos"))] # [test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`private_directory`；`private_fixture`；`.join`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`.confirm_quarantine`；`.unwrap`；`.preview`；`.unwrap`；`.spawn`；`.env`；`.env`；`.env_clear`；`.args`；`std :: process :: Command :: new`；`.unwrap`；`std :: env :: current_exe`；`.to_string`；`spawn`；`spawn`；`std :: time :: Instant :: now`；`std :: time :: Duration :: from_secs`；`.exists`；`.join`；`format!`；`.exists`；`.join`；`format!`；`assert!`；`std :: thread :: sleep`；`std :: time :: Duration :: from_millis`；`.unwrap`；`fs :: write`；`.join`；`assert!`；`assert!`；`assert_eq!`；`assert_eq!`；`.unwrap`；`fs :: remove_dir_all`。

### L400 · function · tests restore_is_confirmed_idempotent_and_does_not_replace_a_new_file

```rust
fn restore_is_confirmed_idempotent_and_does_not_replace_a_new_file ()
```

可見性／屬性：``；`# [cfg (any (target_os = "linux" , target_os = "macos"))] # [test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`private_directory`；`private_fixture`；`.join`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`.confirm_quarantine`；`.unwrap`；`.preview`；`assert!`；`assert!`；`private_fixture`；`.join`；`assert!`；`assert_eq!`；`.unwrap`；`fs :: remove_file`；`.join`；`.unwrap`；`.confirm_restore`；`assert_eq!`；`assert_eq!`；`assert!`；`drop`；`.unwrap`；`AuthorizedRoot :: new`；`assert_eq!`；`.unwrap`；`fs :: remove_dir_all`。

### L428 · function · tests reject_source_and_quarantine_symlinks_without_touching_target

```rust
fn reject_source_and_quarantine_symlinks_without_touching_target ()
```

可見性／屬性：``；`# [cfg (unix)] # [test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.join`；`std :: env :: temp_dir`；`format!`；`private_directory`；`.unwrap`；`fs :: create_dir`；`.unwrap`；`fs :: write`；`.join`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`symlink`；`.join`；`.join`；`assert!`；`private_fixture`；`.join`；`.unwrap`；`.preview`；`.unwrap`；`symlink`；`.join`；`assert!`；`assert!`；`assert_eq!`；`.unwrap`；`fs :: remove_dir_all`；`.unwrap`；`fs :: remove_dir_all`。

### L449 · function · tests replacing_authorized_root_path_cannot_redirect_operation

```rust
fn replacing_authorized_root_path_cannot_redirect_operation ()
```

可見性／屬性：``；`# [cfg (any (target_os = "linux" , target_os = "macos"))] # [test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.with_extension`；`private_directory`；`private_fixture`；`.join`；`.unwrap`；`AuthorizedRoot :: new`；`.unwrap`；`.preview`；`.unwrap`；`fs :: rename`；`.unwrap`；`fs :: create_dir`；`.unwrap`；`fs :: write`；`.join`；`.unwrap`；`.confirm_quarantine`；`assert_eq!`；`assert_eq!`；`.unwrap`；`fs :: remove_dir_all`；`.unwrap`；`fs :: remove_dir_all`。

### L470 · function · tests journal_recovers_after_restart_but_never_for_replaced_file

```rust
fn journal_recovers_after_restart_but_never_for_replaced_file ()
```

可見性／屬性：``；`# [cfg (any (target_os = "linux" , target_os = "macos"))] # [test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`private_directory`；`private_fixture`；`.join`；`.unwrap`；`AuthorizedRoot :: new`；`assert_eq!`；`assert_eq!`；`.unwrap`；`.confirm_quarantine`；`.unwrap`；`.preview`；`drop`；`.unwrap`；`AuthorizedRoot :: new`；`assert_eq!`；`.join`；`.join`；`.unwrap`；`fs :: remove_file`；`private_fixture`；`assert!`；`.unwrap`；`fs :: remove_dir_all`。

### L491 · function · tests externally_writable_root_is_not_authorized

```rust
fn externally_writable_root_is_not_authorized ()
```

可見性／屬性：``；`# [cfg (any (target_os = "linux" , target_os = "macos"))] # [test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`private_directory`；`.unwrap`；`fs :: set_permissions`；`fs :: Permissions :: from_mode`；`assert!`；`.unwrap`；`fs :: remove_dir_all`。

## copilot_use/crates/pet_runtime/src/main.rs · P11

職責：Pet Chat/Agent/生圖。判定：參考 mock 不等同成品AI；真實AI本輪未執行，readonlytool限制保留

### L2 · use ·

```rust
use day9_pet :: { AuthorizedRoot , files :: read_profile } ;
```

### L3 · use ·

```rust
use std :: { error :: Error , path :: Path } ;
```

### L4 · use ·

```rust
use uuid :: Uuid ;
```

### L6 · function ·  usage

```rust
fn usage () -> & 'static str
```

可見性／屬性：``；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L10 · function ·  main

```rust
fn main () -> Result < () , Box < dyn Error + Send + Sync > >
```

可見性／屬性：``；``。

函式內呼叫：`.skip`；`std :: env :: args`；`.as_deref`；`.next`；`std :: env :: var`；`.ok_or_else`；`.next`；`.to_owned`；`usage`；`AuthorizedRoot :: new`；`Path :: new`；`.preview`；`println!`；`std :: env :: var`；`.parse`；`.ok_or_else`；`.next`；`.to_owned`；`usage`；`read_profile`；`Path :: new`；`println!`；`.parse`；`.ok_or_else`；`.next`；`.to_owned`；`usage`；`std :: env :: var`；`read_profile`；`Path :: new`；`.join`；`.join`；`Path :: new`；`.to_string`；`std :: fs :: symlink_metadata`；`.is_file`；`.is_symlink`；`.file_type`；`.len`；`Err`；`.into`；`serde_json :: from_slice`；`std :: fs :: read`；`println!`；`Err`；`.into`；`usage`；`Ok`。

## copilot_use/crates/pet_runtime/src/outbox.rs · P12

職責：Pet Outbox/policy。判定：產品可靠投遞擴充；At-least-once delivery+durable dedup，非無限 exactly-once保證

### L2 · use ·

```rust
use crate :: state :: { PetStore , now } ;
```

### L3 · use ·

```rust
use sqlx :: FromRow ;
```

### L4 · use ·

```rust
use uuid :: Uuid ;
```

### L5 · struct ·  Lease

```rust
# [derive (Debug , FromRow)] pub struct Lease { pub event_id : String , pub payload : String , pub attempts : i64 , pub traceparent : Option < String > , pub lease_token : String , }
```

### L14 · function · impl PetStore claim_event

```rust
async fn claim_event (& self) -> Result < Option < Lease > , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_optional`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query_as`；`.to_string`；`Uuid :: new_v4`；`now`；`now`；`now`。

### L18 · function · impl PetStore ack_event

```rust
async fn ack_event (& self , lease : & Lease) -> Result < bool , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Ok`；`.rows_affected`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`now`。

### L22 · function · impl PetStore fail_event

```rust
async fn fail_event (& self , lease : & Lease , code : & str , permanent : bool ,) -> Result < bool , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Ok`；`.rows_affected`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`now`；`.min`；`now`。

### L38 · function · impl PetStore outbox_status

```rust
async fn outbox_status (& self) -> Result < serde_json :: Value , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_one`；`.bind`；`.bind`；`.bind`；`sqlx :: query_as`；`now`；`now`；`now`；`Ok`；`serde_json :: json!`。

### L45 · function · impl PetStore requeue_blocked_events

```rust
async fn requeue_blocked_events (& self) -> Result < u64 , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Ok`；`.rows_affected`；`.execute`；`.bind`；`sqlx :: query`；`now`。

### L49 · function · impl PetStore dispatch_once

```rust
async fn dispatch_once (& self , endpoint : & str , token : & str ,) -> Result < bool , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.claim_event`；`Ok`；`serde_json :: from_str :: < event_contract :: ProductEvent >`；`.is_valid`；`.to_string`；`.fail_event`；`Ok`；`analytics_transport :: deliver`；`.as_deref`；`.ack_event`；`analytics_transport :: retryable`；`.fail_event`；`Ok`。

### L94 · module ·  tests

```rust
 mod tests (inline=True)
```

### L96 · use ·

```rust
use super :: * ;
```

### L98 · function · tests expired_lease_is_reclaimed_and_stale_worker_cannot_mutate

```rust
async fn expired_lease_is_reclaimed_and_stale_worker_cannot_mutate ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`PetStore :: open`；`Uuid :: new_v4`；`.into`；`.unwrap`；`.record_cleanup`；`.event`；`.unwrap`；`.unwrap`；`.claim_event`；`assert!`；`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.unwrap`；`.claim_event`；`assert_eq!`；`assert_ne!`；`assert!`；`assert!`；`assert!`；`assert!`；`assert_eq!`；`.unwrap`；`.unwrap`；`.claim_event`；`assert!`；`assert!`；`.close`；`.unwrap`；`std :: fs :: remove_file`。

## copilot_use/crates/pet_runtime/src/registry.rs · P09

職責：Pet Store/Profile 投影。判定：產品擴充；對照Profile與五欄概念；資料隔離／filepath不進publicevent

### L2 · use ·

```rust
use crate :: state :: { PetItem , PetStore , now } ;
```

### L3 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L4 · use ·

```rust
use uuid :: Uuid ;
```

### L5 · type ·  Error

```rust
type Error = Box < dyn std :: error :: Error + Send + Sync > ;
```

### L7 · function · impl PetStore archived_pets

```rust
async fn archived_pets (& self) -> Result < Vec < PetItem > , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_all`；`sqlx :: query_as`。

### L12 · function · impl PetStore set_pet_archived

```rust
async fn set_pet_archived (& self , pet : Uuid , archived : bool , confirmed : bool ,) -> Result < Value , Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_nil`；`Err`；`.into`；`.begin`；`.fetch_one`；`.bind`；`sqlx :: query_scalar :: < _ , bool >`；`.to_string`；`Err`；`.into`；`.rows_affected`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`now`；`.to_string`；`.execute`；`.bind`；`sqlx :: query`；`.to_string`；`.execute`；`.bind`；`sqlx :: query`；`.to_string`；`Uuid :: new_v4`；`.into`；`.into`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`serde_json :: to_string`；`.get`；`copilot_telemetry :: outgoing_headers`；`.commit`；`Ok`；`json!`。

### L72 · module ·  tests

```rust
 mod tests (inline=True)
```

### L74 · use ·

```rust
use super :: * ;
```

### L76 · function · tests removal_is_confirmed_recoverable_idempotent_and_keeps_other_pet_and_memory

```rust
async fn removal_is_confirmed_recoverable_idempotent_and_keeps_other_pet_and_memory ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`std :: fs :: create_dir`；`.unwrap`；`PetStore :: open`；`.join`；`.unwrap`；`serde_json :: from_value`；`json!`；`Uuid :: new_v4`；`Uuid :: new_v4`；`.unwrap`；`.save_profile`；`.unwrap`；`.save_profile`；`.unwrap`；`.select_pet`；`.unwrap`；`.save_memory`；`Uuid :: new_v4`；`assert!`；`assert!`；`assert!`；`assert!`；`assert_eq!`；`assert_eq!`；`assert!`；`assert!`；`assert!`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`.close`；`.unwrap`；`std :: fs :: remove_dir_all`。

## copilot_use/crates/pet_runtime/src/state.rs · P09

職責：Pet Store/Profile 投影。判定：產品擴充；對照Profile與五欄概念；資料隔離／filepath不進publicevent

### L2 · use ·

```rust
use crate :: profile :: PetProfile ;
```

### L3 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L4 · use ·

```rust
use sqlx :: { FromRow , SqlitePool , sqlite :: { SqliteConnectOptions , SqlitePoolOptions } , } ;
```

### L8 · use ·

```rust
use std :: { path :: Path , time :: { SystemTime , UNIX_EPOCH } , } ;
```

### L12 · use ·

```rust
use uuid :: Uuid ;
```

### L14 · function ·  now

```rust
fn now () -> i64
```

可見性／屬性：`pub`；``。

函式內呼叫：`.as_secs`；`.unwrap_or_default`；`.duration_since`；`SystemTime :: now`。

### L20 · struct ·  PetStore

```rust
# [derive (Clone)] pub struct PetStore { pub pool : SqlitePool , database_path : std :: path :: PathBuf , pub (crate) projection_queue : std :: sync :: Arc < tokio :: sync :: Mutex < () > > , }
```

### L26 · struct ·  PetItem

```rust
# [derive (Serialize , Deserialize , FromRow)] pub struct PetItem { pub id : String , pub profile : String , }
```

### L31 · struct ·  Memory

```rust
# [derive (Serialize , Deserialize , FromRow)] pub struct Memory { pub id : String , pub pet_id : String , pub content : String , pub created_at : i64 , }
```

### L38 · struct ·  Card

```rust
# [derive (Debug , Serialize , Deserialize , FromRow)] pub struct Card { pub id : String , pub pet_id : String , pub question : String , pub answer : String , pub repetitions : i64 , pub interval_days : i64 , pub ease : f64 , pub due_at : i64 , }
```

### L50 · function · impl Card review

```rust
fn review (& mut self , quality : u8 , at : i64) -> Result < () , & 'static str >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.review`；`Ok`。

### L66 · function · impl crate :: application :: ProfileRepository for PetStore save

```rust
fn save < 'a > (& 'a self , id : Uuid , profile : & 'a PetProfile ,) -> crate :: application :: ProfileFuture < 'a >
```

可見性／屬性：``；``。

函式內呼叫：`Box :: pin`；`.map_err`；`PetStore :: save_profile`；`.to_string`。

### L85 · function · impl PetStore database_path

```rust
fn database_path (& self) -> & Path
```

可見性／屬性：`pub`；``。

函式內呼叫：無可解析直接呼叫／trait 無 body。

### L88 · function · impl PetStore open_readonly

```rust
async fn open_readonly (path : & Path) -> Result < Self , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.create_if_missing`；`.read_only`；`.filename`；`SqliteConnectOptions :: new`；`.connect_with`；`.max_connections`；`SqlitePoolOptions :: new`；`Ok`；`.to_owned`；`std :: sync :: Arc :: new`；`tokio :: sync :: Mutex :: new`。

### L103 · function · impl PetStore open

```rust
async fn open (path : & Path) -> Result < Self , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.foreign_keys`；`.create_if_missing`；`.filename`；`SqliteConnectOptions :: new`；`.connect_with`；`.max_connections`；`SqlitePoolOptions :: new`；`.run`；`sqlx :: migrate!`；`Ok`；`.to_owned`；`std :: sync :: Arc :: new`；`tokio :: sync :: Mutex :: new`。

### L119 · function · impl PetStore select_pet

```rust
async fn select_pet (& self , pet : Uuid) -> Result < () , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`.to_string`；`Err`；`.execute`；`.bind`；`sqlx :: query`；`.to_string`；`Ok`。

### L132 · function · impl PetStore active_pet

```rust
async fn active_pet (& self) -> Result < Option < String > , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_optional`；`sqlx :: query_scalar`。

### L137 · function · impl PetStore pets

```rust
async fn pets (& self) -> Result < Vec < PetItem > , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_all`；`sqlx :: query_as`。

### L142 · function · impl PetStore save_profile

```rust
async fn save_profile (& self , id : Uuid , profile : & PetProfile ,) -> Result < () , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_nil`；`Err`；`.into`；`.validate`；`serde_json :: to_string`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`.rows_affected`；`Err`；`.into`；`.map_err`；`.sync_pet_files`；`Ok`。

### L161 · function · impl PetStore memories

```rust
async fn memories (& self , id : Uuid) -> Result < Vec < Memory > , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_all`；`.bind`；`sqlx :: query_as`；`.to_string`。

### L164 · function · impl PetStore save_memory

```rust
async fn save_memory (& self , pet : Uuid , id : Uuid , content : & str) -> Result < () , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_empty`；`.trim`；`.len`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`.to_string`；`now`；`Ok`。

### L172 · function · impl PetStore delete_memory

```rust
async fn delete_memory (& self , pet : Uuid , id : Uuid) -> Result < () , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`.to_string`；`Ok`。

### L180 · function · impl PetStore cards

```rust
async fn cards (& self , pet : Uuid) -> Result < Vec < Card > , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.fetch_all`；`.bind`；`sqlx :: query_as`；`.to_string`。

### L186 · function · impl PetStore add_card

```rust
async fn add_card (& self , pet : Uuid , question : & str , answer : & str ,) -> Result < () , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.any`；`.iter`；`.is_empty`；`.trim`；`.len`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`Uuid :: new_v4`；`.to_string`；`now`；`Ok`。

### L208 · function · impl PetStore record_cleanup

```rust
async fn record_cleanup (& self , receipt : & crate :: CleanupReceipt , event : & event_contract :: ProductEvent ,) -> Result < () , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.event`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`.map_err`；`serde_json :: to_string`；`sqlx :: Error :: Protocol`；`.to_string`；`.map_err`；`serde_json :: to_string`；`sqlx :: Error :: Protocol`；`.to_string`；`.begin`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`now`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`.to_string`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`.get`；`copilot_telemetry :: outgoing_headers`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`.to_string`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`.commit`；`Ok`。

### L252 · function · impl PetStore record_restoration

```rust
async fn record_restoration (& self , receipt : & crate :: RestoreReceipt ,) -> Result < () , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.event`；`.is_valid`；`.is_nil`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`.map_err`；`serde_json :: to_string`；`sqlx :: Error :: Protocol`；`.to_string`；`.map_err`；`serde_json :: to_string`；`sqlx :: Error :: Protocol`；`.to_string`；`.begin`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`now`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`.to_string`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`.get`；`copilot_telemetry :: outgoing_headers`；`.fetch_one`；`.bind`；`sqlx :: query_scalar`；`.to_string`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`.commit`；`Ok`。

### L285 · function · impl PetStore review

```rust
async fn review (& self , pet : Uuid , id : Uuid , quality : u8) -> Result < Card , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.begin`；`.fetch_one`；`.bind`；`.bind`；`sqlx :: query_as`；`.to_string`；`.to_string`；`.map_err`；`.review`；`now`；`sqlx :: Error :: Protocol`；`.into`；`.fetch_optional`；`.bind`；`sqlx :: query_scalar`；`.to_string`；`.max`；`.unwrap_or`；`now`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`.to_string`；`Uuid :: new_v4`；`.into`；`.into`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`.map_err`；`serde_json :: to_string`；`sqlx :: Error :: Protocol`；`.to_string`；`.get`；`copilot_telemetry :: outgoing_headers`；`.commit`；`Ok`。

### L321 · module ·  tests

```rust
 mod tests (inline=True)
```

### L323 · use ·

```rust
use super :: * ;
```

### L325 · function · tests restoration_receipt_and_event_roll_back_together_and_replay_once

```rust
async fn restoration_receipt_and_event_roll_back_together_and_replay_once ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`PetStore :: open`；`Uuid :: new_v4`；`Uuid :: new_v4`；`.unwrap`；`.execute`；`sqlx :: query`；`assert!`；`.unwrap`；`.fetch_one`；`sqlx :: query_scalar`；`assert_eq!`；`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.record_restoration`；`.unwrap`；`.record_restoration`；`.unwrap`；`.fetch_one`；`sqlx :: query_scalar`；`assert_eq!`；`assert!`；`.close`；`.unwrap`；`std :: fs :: remove_file`。

### L358 · function · tests failed_recall_resets_schedule_and_ease_is_bounded

```rust
fn failed_recall_resets_schedule_and_ease_is_bounded ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.to_string`；`Uuid :: new_v4`；`.to_string`；`Uuid :: new_v4`；`.into`；`.into`；`.unwrap`；`.review`；`assert_eq!`；`.unwrap`；`.review`；`assert_eq!`；`assert_eq!`；`assert!`；`assert!`。

### L380 · function · tests pet_memory_isolation_and_restart_persistence

```rust
async fn pet_memory_isolation_and_restart_persistence ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`PetStore :: open`；`Uuid :: new_v4`；`Uuid :: new_v4`；`.unwrap`；`PetProfile :: parse`；`include_str!`；`.unwrap`；`.save_profile`；`.unwrap`；`.save_profile`；`Uuid :: new_v4`；`.unwrap`；`.save_memory`；`.unwrap`；`.delete_memory`；`assert!`；`assert_eq!`；`.close`；`.unwrap`；`PetStore :: open`；`assert_eq!`；`.unwrap`；`.delete_memory`；`assert!`；`.close`；`.unwrap`；`std :: fs :: remove_file`。

### L402 · function · tests recovered_receipt_and_event_are_atomic_and_idempotent

```rust
async fn recovered_receipt_and_event_are_atomic_and_idempotent ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`PetStore :: open`；`Uuid :: new_v4`；`.into`；`.unwrap`；`.execute`；`sqlx :: query`；`assert!`；`.unwrap`；`.fetch_one`；`sqlx :: query_scalar`；`assert_eq!`；`.unwrap`；`.execute`；`sqlx :: query`；`.unwrap`；`.record_cleanup`；`.event`；`.unwrap`；`.record_cleanup`；`.event`；`.clone`；`assert!`；`.close`；`.unwrap`；`PetStore :: open`；`.unwrap`；`.record_cleanup`；`.event`；`.unwrap`；`.fetch_one`；`sqlx :: query_as`；`assert_eq!`；`.close`；`.unwrap`；`std :: fs :: remove_file`。

## copilot_use/crates/pet_runtime/src/suggestions.rs · P12

職責：Pet Outbox/policy。判定：產品可靠投遞擴充；At-least-once delivery+durable dedup，非無限 exactly-once保證

### L2 · use ·

```rust
use crate :: state :: PetStore ;
```

### L3 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L4 · use ·

```rust
use uuid :: Uuid ;
```

### L6 · function · impl PetStore apply_suggestion

```rust
async fn apply_suggestion (& self , pet : Uuid , id : Uuid , proposal : & Value , confirmed : bool ,) -> Result < Value , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.ok_or_else`；`.filter`；`.as_i64`；`.contains`；`sqlx :: Error :: Protocol`；`.into`；`matches!`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`.begin`；`.execute`；`.bind`；`sqlx :: query`；`.to_string`；`.fetch_one`；`.bind`；`sqlx :: query_as`；`.to_string`；`.fetch_optional`；`.bind`；`.bind`；`sqlx :: query_as`；`.to_string`；`.to_string`；`.is_some`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`.to_string`；`Uuid :: new_v4`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`.execute`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`.to_string`；`.commit`；`Ok`；`json!`。

### L57 · function · impl PetStore rollback_suggestion

```rust
async fn rollback_suggestion (& self , pet : Uuid , id : Uuid , confirmed : bool ,) -> Result < Value , sqlx :: Error >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Err`；`sqlx :: Error :: Protocol`；`.into`；`.begin`；`.fetch_one`；`.bind`；`.bind`；`sqlx :: query_as`；`.to_string`；`.to_string`；`.ok_or_else`；`sqlx :: Error :: Protocol`；`.into`；`.fetch_one`；`.bind`；`sqlx :: query_as`；`.to_string`；`Err`；`sqlx :: Error :: Protocol`；`.into`；`.to_string`；`Uuid :: new_v4`；`.execute`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`.execute`；`.bind`；`.bind`；`.bind`；`sqlx :: query`；`.to_string`；`.to_string`；`.commit`；`Ok`；`json!`。

### L96 · module ·  tests

```rust
 mod tests (inline=True)
```

### L98 · use ·

```rust
use super :: * ;
```

### L100 · function · tests confirmed_application_is_idempotent_and_rollback_preserves_newer_settings

```rust
async fn confirmed_application_is_idempotent_and_rollback_preserves_newer_settings ()
```

可見性／屬性：``；`# [tokio :: test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`PetStore :: open`；`Uuid :: new_v4`；`.unwrap`；`.save_profile`；`.unwrap`；`crate :: profile :: PetProfile :: parse`；`include_str!`；`Uuid :: new_v4`；`json!`；`assert!`；`.unwrap`；`.apply_suggestion`；`assert_eq!`；`.unwrap`；`.add_card`；`.remove`；`.unwrap`；`.cards`；`.unwrap`；`.review`；`.unwrap`；`Uuid :: parse_str`；`assert_eq!`；`.unwrap`；`.rollback_suggestion`；`assert_eq!`；`Uuid :: new_v4`；`.unwrap`；`.apply_suggestion`；`.unwrap`；`.execute`；`sqlx :: query`；`assert!`；`.close`；`.unwrap`；`std :: fs :: remove_file`。

## copilot_use/crates/pet_runtime/src/unix_fs.rs · P08

職責：Pet 真 OS quarantine/restore。判定：產品安全擴充；FS與DB非單一交易，明確部分成功與恢復，不假稱零窗口

### L2 · use ·

```rust
use crate :: { CleanupError , CleanupReceipt , FilePreview , modified_nanos , validate_name } ;
```

### L3 · use ·

```rust
use std :: { ffi :: CString , fs :: { File , Metadata } , io :: { self , Read , Write } , os :: { fd :: { AsRawFd , FromRawFd , IntoRawFd } , unix :: fs :: { MetadataExt , OpenOptionsExt } , } , path :: Path , } ;
```

### L13 · use ·

```rust
use uuid :: Uuid ;
```

### L14 · struct ·  PinnedRoot

```rust
pub struct PinnedRoot { directory : File , }
```

### L17 · struct ·  PreparedOperation

```rust
# [derive (serde :: Serialize , serde :: Deserialize)] # [serde (deny_unknown_fields)] struct PreparedOperation { operation_id : Uuid , preview : FilePreview , quarantined_file : String , }
```

### L24 · struct ·  RestoreMarker

```rust
# [derive (serde :: Serialize , serde :: Deserialize)] # [serde (deny_unknown_fields)] struct RestoreMarker { restore_id : Uuid , restoring : bool , }
```

### L30 · function ·  directory_names

```rust
fn directory_names (directory : & File) -> Result < Vec < String > , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`open_at`；`.into_raw_fd`；`libc :: fdopendir`；`.is_null`；`libc :: close`；`Err`；`.into`；`io :: Error :: last_os_error`；`libc :: closedir`；`Directory`；`Vec :: new`；`libc :: readdir`；`.is_null`；`std :: ffi :: CStr :: from_ptr`；`.as_ptr`；`.to_str`；`.push`；`.to_owned`；`Ok`。

### L59 · function ·  name

```rust
fn name (value : & str) -> Result < CString , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`.map_err`；`CString :: new`。

### L62 · function ·  open_at

```rust
fn open_at (directory : & File , value : & str , flags : i32) -> Result < File , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`name`；`libc :: openat`；`.as_raw_fd`；`.as_ptr`；`io :: Error :: last_os_error`；`.raw_os_error`；`Some`；`Err`；`Err`；`.into`；`Ok`；`File :: from_raw_fd`。

### L80 · function ·  rename

```rust
fn rename (from : & File , source : & str , to : & File , destination : & str) -> Result < () , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`name`；`name`；`libc :: renameat2`；`.as_raw_fd`；`.as_ptr`；`.as_raw_fd`；`.as_ptr`；`libc :: renameatx_np`；`.as_raw_fd`；`.as_ptr`；`.as_raw_fd`；`.as_ptr`；`Err`；`.into`；`io :: Error :: last_os_error`；`Ok`。

### L108 · function ·  preview

```rust
fn preview (metadata : Metadata , file_name : & str) -> Result < FilePreview , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`.is_file`；`.nlink`；`.uid`；`libc :: geteuid`；`.mode`；`Err`；`Ok`；`.into`；`.len`；`modified_nanos`；`Some`；`.dev`；`.ino`。

### L124 · function · impl PinnedRoot trash_directory

```rust
fn trash_directory (& self) -> Result < File , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`open_at`；`.metadata`；`.uid`；`libc :: geteuid`；`.mode`；`Err`；`Ok`。

### L136 · function · impl PinnedRoot journal

```rust
fn journal (& self , directory : & File , id : Uuid ,) -> Result < (PreparedOperation , Option < Uuid >) , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`.is_nil`；`Err`；`open_at`；`format!`；`.metadata`；`.is_file`；`.nlink`；`.uid`；`libc :: geteuid`；`.mode`；`.len`；`Err`；`Vec :: new`；`.read_to_end`；`.take`；`.len`；`Err`；`.split`；`.map_err`；`serde_json :: from_slice`；`.unwrap_or_default`；`.next`；`format!`；`Err`；`validate_name`；`serde_json :: from_slice :: < RestoreMarker >`；`.is_nil`；`Err`；`.is_some_and`；`Err`；`Some`；`Ok`。

### L180 · function · impl PinnedRoot is_restored

```rust
fn is_restored (& self , folder : & File , prepared : & PreparedOperation) -> bool
```

可見性／屬性：``；``。

函式內呼叫：`matches!`；`.is_ok_and`；`.preview`。

### L186 · function · impl PinnedRoot trash

```rust
fn trash (& self) -> Result < Vec < crate :: TrashEntry > , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.trash_directory`；`.kind`；`Ok`；`Vec :: new`；`Err`；`Vec :: new`；`directory_names`；`.and_then`；`.strip_suffix`；`.ok`；`Uuid :: parse_str`；`.journal`；`.is_ok_and`；`.and_then`；`open_at`；`preview`；`.metadata`；`.is_some`；`.is_restored`；`.push`；`Ok`。

### L220 · function · impl PinnedRoot restore

```rust
fn restore (& self , id : Uuid) -> Result < crate :: RestoreReceipt , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.trash_directory`；`std :: time :: Instant :: now`；`std :: time :: Duration :: from_millis`；`libc :: flock`；`.as_raw_fd`；`io :: Error :: last_os_error`；`.kind`；`std :: time :: Instant :: now`；`Err`；`.into`；`std :: thread :: sleep`；`std :: time :: Duration :: from_millis`；`.journal`；`.is_restored`；`Ok`；`open_at`；`preview`；`.metadata`；`Err`；`.unwrap_or_else`；`open_at`；`format!`；`.metadata`；`.is_file`；`.nlink`；`.uid`；`libc :: geteuid`；`.mode`；`Err`；`.is_none`；`.write_all`；`.map_err`；`serde_json :: to_vec`；`.write_all`；`.sync_all`；`rename`；`.is_ok_and`；`.preview`；`.is_err`；`rename`；`Err`；`CleanupError :: RecoveryRequired`；`Err`；`.sync_all`；`.sync_all`；`.write_all`；`.sync_all`；`Ok`。

### L306 · function · impl PinnedRoot restored_receipts

```rust
fn restored_receipts (& self) -> Result < Vec < crate :: RestoreReceipt > , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.trash_directory`；`.kind`；`Ok`；`Vec :: new`；`Err`；`Vec :: new`；`.trash`；`.journal`；`.push`；`Ok`。

### L331 · function · impl PinnedRoot scan

```rust
fn scan (& self) -> Result < Vec < FilePreview > , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Vec :: new`；`directory_names`；`.preview`；`.push`；`Ok`。

### L340 · function · impl PinnedRoot open

```rust
fn open (path : & Path) -> Result < Self , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.open`；`.custom_flags`；`.read`；`std :: fs :: OpenOptions :: new`；`.metadata`；`.uid`；`libc :: geteuid`；`.mode`；`Err`；`Ok`。

### L351 · function · impl PinnedRoot preview

```rust
fn preview (& self , value : & str) -> Result < FilePreview , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`validate_name`；`open_at`；`preview`；`.metadata`。

### L356 · function · impl PinnedRoot quarantine

```rust
fn quarantine (& self , expected : & FilePreview) -> Result < CleanupReceipt , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`validate_name`；`.preview`；`Err`；`name`；`libc :: mkdirat`；`.as_raw_fd`；`.as_ptr`；`.raw_os_error`；`io :: Error :: last_os_error`；`Some`；`Err`；`.into`；`io :: Error :: last_os_error`；`open_at`；`.metadata`；`.uid`；`libc :: geteuid`；`.mode`；`Err`；`Uuid :: new_v4`；`format!`；`.clone`；`.clone`；`name`；`format!`；`libc :: openat`；`.as_raw_fd`；`.as_ptr`；`Err`；`.into`；`io :: Error :: last_os_error`；`File :: from_raw_fd`；`.map_err`；`serde_json :: to_vec`；`.write_all`；`.write_all`；`.sync_all`；`.sync_all`；`rename`；`.is_ok_and`；`.and_then`；`open_at`；`preview`；`.metadata`；`.is_err`；`rename`；`Err`；`CleanupError :: RecoveryRequired`；`Err`；`.sync_all`；`.sync_all`；`.write_all`；`.sync_all`；`Ok`。

### L426 · function · impl PinnedRoot recovered_receipts

```rust
fn recovered_receipts (& self) -> Result < Vec < CleanupReceipt > , CleanupError >
```

可見性／屬性：`pub`；`# [doc = " Recover only an actually captured file with the original identity. A crash"] # [doc = " before rename, a torn completion marker or a replaced file never fabricates success."]`。

函式內呼叫：`open_at`；`.kind`；`Ok`；`Vec :: new`；`Err`；`.metadata`；`.uid`；`libc :: geteuid`；`.mode`；`Err`；`Vec :: new`；`directory_names`；`.ends_with`；`open_at`；`.metadata`；`.is_file`；`.nlink`；`.uid`；`libc :: geteuid`；`.mode`；`.len`；`Vec :: new`；`.read_to_end`；`.take`；`.len`；`.unwrap_or_default`；`.next`；`.split`；`serde_json :: from_slice`；`format!`；`format!`；`.is_err`；`validate_name`；`.and_then`；`open_at`；`preview`；`.metadata`；`.is_ok_and`；`.push`；`Ok`。

## copilot_use/crates/pet_runtime/src/windows_fs.rs · P08

職責：Pet 真 OS quarantine/restore。判定：產品安全擴充；FS與DB非單一交易，明確部分成功與恢復，不假稱零窗口

### L5 · use ·

```rust
use crate :: { CleanupError , CleanupReceipt , FilePreview , RestoreReceipt , TrashEntry , modified_nanos , validate_name , } ;
```

### L9 · use ·

```rust
use std :: { fs :: File , io :: { self , Read , Write } , os :: windows :: io :: { AsRawHandle , FromRawHandle } , path :: { Path , PathBuf } , } ;
```

### L15 · use ·

```rust
use uuid :: Uuid ;
```

### L17 · use ·

```rust
use windows_sys :: Win32 :: Security :: ACE_HEADER ;
```

### L18 · use ·

```rust
use windows_sys :: { Wdk :: { Foundation :: OBJECT_ATTRIBUTES , Storage :: FileSystem :: * } , Win32 :: { Foundation :: * , Security :: { Authorization :: * , * } , Storage :: FileSystem :: { BY_HANDLE_FILE_INFORMATION , CreateFileW , FILE_ADD_FILE , FILE_APPEND_DATA , FILE_ATTRIBUTE_DIRECTORY , FILE_ATTRIBUTE_NORMAL , FILE_ATTRIBUTE_REPARSE_POINT , FILE_FLAG_BACKUP_SEMANTICS , FILE_FLAG_OPEN_REPARSE_POINT , FILE_READ_ATTRIBUTES , FILE_READ_DATA , FILE_SHARE_DELETE , FILE_SHARE_READ , FILE_SHARE_WRITE , FILE_WRITE_DATA , GetFileInformationByHandle , GetFinalPathNameByHandleW , OPEN_EXISTING , } , System :: { IO :: IO_STATUS_BLOCK , Threading :: { GetCurrentProcess , OpenProcessToken } , } , } , } ;
```

### L36 · struct ·  PinnedRoot

```rust
pub struct PinnedRoot { directory : File , }
```

### L39 · struct ·  Local

```rust
struct Local (* mut core :: ffi :: c_void) ;
```

### L41 · function · impl Drop for Local drop

```rust
fn drop (& mut self)
```

可見性／屬性：``；``。

函式內呼叫：`LocalFree`。

### L47 · function ·  nt

```rust
fn nt (status : i32) -> Result < () , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`Err`；`.into`；`io :: Error :: from_raw_os_error`；`RtlNtStatusToDosError`；`Ok`。

### L54 · function ·  user

```rust
fn user () -> Result < Vec < usize > , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`std :: ptr :: null_mut`；`OpenProcessToken`；`GetCurrentProcess`；`Err`；`.into`；`io :: Error :: last_os_error`；`File :: from_raw_handle`；`GetTokenInformation`；`.as_raw_handle`；`std :: ptr :: null_mut`；`Err`；`vec!`；`GetTokenInformation`；`.as_raw_handle`；`.cast`；`.as_mut_ptr`；`Err`；`.into`；`io :: Error :: last_os_error`；`Ok`。

### L88 · function ·  user_sid

```rust
fn user_sid (data : & [usize]) -> PSID
```

可見性／屬性：``；``。

函式內呼叫：`.cast`；`.as_ptr`。

### L91 · function ·  trusted

```rust
fn trusted (sid : PSID , current : PSID) -> bool
```

可見性／屬性：``；``。

函式內呼叫：`EqualSid`；`IsWellKnownSid`；`IsWellKnownSid`。

### L98 · function ·  secure

```rust
fn secure (file : & File , private : bool) -> Result < () , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`std :: ptr :: null_mut`；`std :: ptr :: null_mut`；`std :: ptr :: null_mut`；`GetSecurityInfo`；`.as_raw_handle`；`std :: ptr :: null_mut`；`std :: ptr :: null_mut`；`Err`；`.into`；`io :: Error :: from_raw_os_error`；`Local`；`user`；`user_sid`；`.is_null`；`.is_null`；`trusted`；`eprintln!`；`Err`；`std :: ptr :: null_mut`；`GetAce`；`u32 :: from`；`Err`；`.cast`；`u32 :: from`；`.cast`；`.cast`；`.cast_mut`；`std :: ptr :: addr_of!`；`IsWellKnownSid`；`trusted`；`eprintln!`；`Err`；`eprintln!`；`Err`；`Ok`。

### L174 · function ·  make_private

```rust
fn make_private (file : & File) -> Result < () , CleanupError >
```

可見性／屬性：`pub (super)`；``。

函式內呼叫：`set_descriptor`；`private_descriptor`。

### L177 · function ·  private_descriptor

```rust
fn private_descriptor () -> Result < Local , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`user`；`std :: ptr :: null_mut`；`ConvertSidToStringSidW`；`user_sid`；`Err`；`.into`；`io :: Error :: last_os_error`；`Local`；`.cast`；`.add`；`Err`；`.map_err`；`String :: from_utf16`；`std :: slice :: from_raw_parts`；`.collect`；`.chain`；`.encode_utf16`；`format!`；`Some`；`std :: ptr :: null_mut`；`ConvertStringSecurityDescriptorToSecurityDescriptorW`；`.as_ptr`；`std :: ptr :: null_mut`；`Err`；`.into`；`io :: Error :: last_os_error`；`Ok`；`Local`。

### L211 · function ·  security_text

```rust
fn security_text (file : & File) -> Result < String , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`std :: ptr :: null_mut`；`GetSecurityInfo`；`.as_raw_handle`；`std :: ptr :: null_mut`；`std :: ptr :: null_mut`；`std :: ptr :: null_mut`；`std :: ptr :: null_mut`；`Err`；`.into`；`io :: Error :: from_raw_os_error`；`Local`；`std :: ptr :: null_mut`；`ConvertSecurityDescriptorToStringSecurityDescriptorW`；`Err`；`.into`；`io :: Error :: last_os_error`；`Local`；`.cast`；`Err`；`.ok_or`；`.find`；`.add`；`std :: slice :: from_raw_parts`；`.map_err`；`String :: from_utf16`。

### L256 · function ·  set_descriptor

```rust
fn set_descriptor (file : & File , descriptor : & Local) -> Result < () , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`std :: ptr :: null_mut`；`GetSecurityDescriptorDacl`；`.is_null`；`GetSecurityDescriptorControl`；`eprintln!`；`Err`；`SetSecurityInfo`；`.as_raw_handle`；`std :: ptr :: null_mut`；`std :: ptr :: null_mut`；`std :: ptr :: null`；`Err`；`.into`；`io :: Error :: from_raw_os_error`；`Ok`。

### L296 · function ·  restore_security

```rust
fn restore_security (file : & File , text : & str) -> Result < () , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`.len`；`.contains`；`Err`；`.collect`；`.chain`；`.encode_utf16`；`Some`；`std :: ptr :: null_mut`；`ConvertStringSecurityDescriptorToSecurityDescriptorW`；`.as_ptr`；`std :: ptr :: null_mut`；`Err`；`.into`；`io :: Error :: last_os_error`；`Local`；`set_descriptor`；`secure`。

### L317 · function ·  open_at

```rust
fn open_at (directory : & File , name : & str , folder : bool , create : bool , write : bool , exclusive : bool ,) -> Result < File , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`.contains`；`.is_empty`；`Err`；`.collect`；`.encode_utf16`；`.map_err`；`u16 :: try_from`；`.len`；`.as_mut_ptr`；`Some`；`private_descriptor`；`std :: mem :: size_of :: < OBJECT_ATTRIBUTES >`；`.as_raw_handle`；`.map_or`；`.as_ref`；`std :: ptr :: null`；`.cast`；`std :: ptr :: null_mut`；`std :: ptr :: null_mut`；`std :: mem :: zeroed`；`nt`；`NtCreateFile`；`std :: ptr :: null`；`.as_slice`；`.as_slice`；`.collect`；`.encode_utf16`；`std :: ptr :: null`；`File :: from_raw_handle`；`check_kind`；`secure`；`.first`；`Some`；`Ok`。

### L405 · function ·  information

```rust
fn information (file : & File) -> Result < BY_HANDLE_FILE_INFORMATION , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`std :: mem :: zeroed`；`GetFileInformationByHandle`；`.as_raw_handle`；`Err`；`.into`；`io :: Error :: last_os_error`；`Ok`。

### L412 · function ·  check_kind

```rust
fn check_kind (file : & File , directory : bool) -> Result < () , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`information`；`Err`；`Ok`。

### L422 · function ·  preview

```rust
fn preview (file : & File , name : & str) -> Result < FilePreview , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`check_kind`；`secure`；`information`；`.metadata`；`Ok`；`.into`；`.len`；`modified_nanos`；`Some`；`u64 :: from`；`u64 :: from`；`u64 :: from`；`u64 :: from`；`u64 :: from`。

### L439 · function ·  names

```rust
fn names (directory : & File) -> Result < Vec < String > , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`vec!`；`GetFinalPathNameByHandleW`；`.as_raw_handle`；`.as_mut_ptr`；`.len`；`.len`；`Err`；`.into`；`io :: Error :: last_os_error`；`PathBuf :: from`；`.map_err`；`String :: from_utf16`；`Ok`；`.collect`；`.filter_map`；`.take`；`std :: fs :: read_dir`；`.ok`；`.into_string`；`.file_name`；`.ok`。

### L459 · function ·  rename

```rust
fn rename (file : & File , to : & File , name : & str) -> Result < () , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`.collect`；`.encode_utf16`；`std :: mem :: offset_of!`；`.len`；`vec!`；`.cast`；`.as_mut_ptr`；`.as_raw_handle`；`.len`；`std :: ptr :: copy_nonoverlapping`；`.as_ptr`；`.cast`；`std :: ptr :: addr_of_mut!`；`.len`；`std :: mem :: zeroed`；`nt`；`NtSetInformationFile`；`.as_raw_handle`；`.cast`。

### L486 · struct ·  Journal

```rust
# [derive (serde :: Serialize , serde :: Deserialize)] # [serde (deny_unknown_fields)] struct Journal { operation_id : Uuid , preview : FilePreview , quarantined_file : String , original_security : String , }
```

### L494 · struct ·  Marker

```rust
# [derive (serde :: Serialize , serde :: Deserialize)] # [serde (deny_unknown_fields)] struct Marker { restore_id : Uuid , }
```

### L499 · function ·  journal

```rust
fn journal (directory : & File , id : Uuid) -> Result < (Journal , Option < Uuid >) , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`.is_nil`；`Err`；`open_at`；`format!`；`secure`；`Vec :: new`；`.read_to_end`；`.take`；`.len`；`Err`；`.split`；`.map_err`；`serde_json :: from_slice`；`.ok_or`；`.next`；`validate_name`；`format!`；`.is_none`；`Err`；`.filter`；`.is_empty`；`.map_err`；`serde_json :: from_slice`；`.is_nil`；`.is_some_and`；`Err`；`Some`；`Ok`。

### L538 · function · impl PinnedRoot open

```rust
fn open (path : & Path) -> Result < Self , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.collect`；`.chain`；`.encode_wide`；`.as_os_str`；`Some`；`CreateFileW`；`.as_ptr`；`std :: ptr :: null`；`std :: ptr :: null_mut`；`Err`；`.into`；`io :: Error :: last_os_error`；`File :: from_raw_handle`；`check_kind`；`secure`；`Ok`。

### L564 · function · impl PinnedRoot trash_directory

```rust
fn trash_directory (& self) -> Result < File , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`open_at`。

### L567 · function · impl PinnedRoot lock

```rust
fn lock (& self , directory : & File) -> Result < File , CleanupError >
```

可見性／屬性：``；``。

函式內呼叫：`std :: time :: Instant :: now`；`std :: time :: Duration :: from_millis`；`open_at`；`.raw_os_error`；`Some`；`std :: time :: Instant :: now`；`std :: thread :: sleep`；`std :: time :: Duration :: from_millis`。

### L580 · function · impl PinnedRoot preview

```rust
fn preview (& self , name : & str) -> Result < FilePreview , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`validate_name`；`preview`；`open_at`。

### L587 · function · impl PinnedRoot scan

```rust
fn scan (& self) -> Result < Vec < FilePreview > , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Ok`；`.collect`；`.filter_map`；`.into_iter`；`names`；`.ok`；`.preview`。

### L593 · function · impl PinnedRoot quarantine

```rust
fn quarantine (& self , expected : & FilePreview) -> Result < CleanupReceipt , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`validate_name`；`.trash_directory`；`.lock`；`open_at`；`preview`；`Err`；`Uuid :: new_v4`；`format!`；`.clone`；`.clone`；`security_text`；`open_at`；`format!`；`.map_err`；`serde_json :: to_vec`；`.len`；`Err`；`.write_all`；`.write_all`；`.sync_all`；`set_descriptor`；`private_descriptor`；`rename`；`.map_err`；`restore_security`；`eprintln!`；`CleanupError :: RecoveryRequired`；`Err`；`Ok`。

### L645 · function · impl PinnedRoot matches

```rust
fn matches (& self , directory : & File , name : & str , expected : & FilePreview) -> bool
```

可見性／屬性：``；``。

函式內呼叫：`.is_ok_and`；`.and_then`；`open_at`；`preview`。

### L650 · function · impl PinnedRoot restore

```rust
fn restore (& self , id : Uuid) -> Result < RestoreReceipt , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.trash_directory`；`.lock`；`journal`；`.unwrap_or_else`；`.is_some`；`.matches`；`open_at`；`.map_err`；`restore_security`；`eprintln!`；`CleanupError :: RecoveryRequired`；`Ok`；`open_at`；`preview`；`Err`；`.is_none`；`open_at`；`format!`；`.seek`；`SeekFrom :: End`；`.write_all`；`.map_err`；`serde_json :: to_vec`；`.write_all`；`.sync_all`；`rename`；`.map_err`；`restore_security`；`eprintln!`；`CleanupError :: RecoveryRequired`；`Ok`。

### L716 · function · impl PinnedRoot trash

```rust
fn trash (& self) -> Result < Vec < TrashEntry > , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.trash_directory`；`.lock`；`Vec :: new`；`names`；`.and_then`；`.strip_suffix`；`.ok`；`Uuid :: parse_str`；`journal`；`.matches`；`.is_some`；`.matches`；`.push`；`Ok`。

### L743 · function · impl PinnedRoot recovered_receipts

```rust
fn recovered_receipts (& self) -> Result < Vec < CleanupReceipt > , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Ok`；`.collect`；`.map`；`.into_iter`；`.trash`；`format!`。

### L754 · function · impl PinnedRoot restored_receipts

```rust
fn restored_receipts (& self) -> Result < Vec < RestoreReceipt > , CleanupError >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.collect`；`.map`；`.filter`；`.into_iter`；`.trash`；`.collect`；`.map`；`.into_iter`；`.restore`。

### L764 · module ·  tests

```rust
 mod tests (inline=True)
```

### L766 · use ·

```rust
use super :: * ;
```

### L768 · function · tests windows_owner_rights_private_acl_is_allowed_but_world_write_is_rejected

```rust
fn windows_owner_rights_private_acl_is_allowed_but_world_write_is_rejected ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`.open`；`.access_mode`；`.create_new`；`.write`；`.read`；`std :: fs :: OpenOptions :: new`；`.collect`；`.chain`；`.encode_utf16`；`Some`；`std :: ptr :: null_mut`；`assert_ne!`；`Local`；`.unwrap`；`set_descriptor`；`assert_eq!`；`assert_eq!`；`drop`；`.unwrap`；`std :: fs :: remove_file`。

### L804 · function · tests windows_handle_capture_restore_and_conflict_are_idempotent

```rust
fn windows_handle_capture_restore_and_conflict_are_idempotent ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`std :: fs :: create_dir`；`.unwrap`；`std :: fs :: write`；`.join`；`.unwrap`；`PinnedRoot :: open`；`.unwrap`；`.preview`；`.unwrap`；`security_text`；`.unwrap`；`open_at`；`assert!`；`.unwrap`；`std :: fs :: write`；`.join`；`assert!`；`.unwrap`；`std :: fs :: write`；`.join`；`.unwrap`；`.preview`；`.unwrap`；`.quarantine`；`assert!`；`assert_eq!`；`.unwrap`；`std :: fs :: write`；`.join`；`assert!`；`assert_eq!`；`.unwrap`；`std :: fs :: remove_file`；`.join`；`.unwrap`；`.restore`；`assert_eq!`；`assert_eq!`；`assert_eq!`；`.unwrap`；`security_text`；`.unwrap`；`open_at`；`.expect`；`.split_once`；`format!`；`assert_eq!`；`drop`；`.unwrap`；`std :: fs :: remove_dir_all`。

### L846 · function · tests windows_rejects_links_and_writers_and_pins_renamed_root

```rust
fn windows_rejects_links_and_writers_and_pins_renamed_root ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.join`；`std :: env :: temp_dir`；`format!`；`.unwrap`；`std :: fs :: create_dir`；`.unwrap`；`std :: fs :: write`；`.join`；`.unwrap`；`PinnedRoot :: open`；`.unwrap`；`.preview`；`.unwrap`；`std :: fs :: hard_link`；`.join`；`.join`；`assert!`；`.unwrap`；`std :: fs :: remove_file`；`.join`；`.unwrap`；`.open`；`.write`；`std :: fs :: OpenOptions :: new`；`.join`；`assert!`；`drop`；`.with_extension`；`.unwrap`；`std :: fs :: rename`；`.unwrap`；`std :: fs :: create_dir`；`.unwrap`；`std :: fs :: write`；`.join`；`.unwrap`；`.quarantine`；`assert_eq!`；`assert_eq!`；`assert!`；`assert!`；`drop`；`.unwrap`；`std :: fs :: remove_dir_all`；`.unwrap`；`std :: fs :: remove_dir_all`。

## copilot_use/crates/shared/src/lib.rs · C03

職責：Chat 事件。判定：四欄事件子集可適配，不能宣稱所有事件／發送方式相同

### L4 · module ·  topics

```rust
pub mod topics (inline=False)
```

### L5 · use ·

```rust
use serde :: { Deserialize , Serialize } ;
```

### L6 · use ·

```rust
use uuid :: Uuid ;
```

### L8 · struct ·  ProductEvent

```rust
# [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq)] # [serde (deny_unknown_fields)] pub struct ProductEvent { pub schema_version : u16 , pub event_id : Uuid , pub source_app : String , pub event_type : String , }
```

### L18 · function · impl ProductEvent chat_message_persisted

```rust
fn chat_message_persisted (message_id : Uuid) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：`.into`；`.into`。

### L27 · function · impl ProductEvent pet_cleanup_succeeded

```rust
fn pet_cleanup_succeeded (operation_id : Uuid) -> Self
```

可見性／屬性：`pub`；``。

函式內呼叫：`.into`；`.into`。

### L36 · function · impl ProductEvent is_valid

```rust
fn is_valid (& self) -> bool
```

可見性／屬性：`pub`；``。

函式內呼叫：`.is_nil`；`matches!`；`.is_some_and`；`.strip_prefix`；`.contains`。

### L61 · module ·  tests

```rust
 mod tests (inline=True)
```

### L63 · use ·

```rust
use super :: * ;
```

### L66 · function · tests chat_event_has_only_four_non_sensitive_fields

```rust
fn chat_event_has_only_four_non_sensitive_fields ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`Uuid :: new_v4`；`.unwrap`；`serde_json :: to_value`；`ProductEvent :: chat_message_persisted`；`assert_eq!`；`assert_eq!`；`assert!`。

### L75 · function · tests only_allow_known_sources_and_schema

```rust
fn only_allow_known_sources_and_schema ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`ProductEvent :: chat_message_persisted`；`Uuid :: new_v4`；`assert!`；`.into`；`assert!`。

### L82 · function · tests unknown_or_cross_source_event_types_are_rejected

```rust
fn unknown_or_cross_source_event_types_are_rejected ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`ProductEvent :: chat_message_persisted`；`Uuid :: new_v4`；`.into`；`assert!`；`.into`；`assert!`；`.into`；`assert!`；`Uuid :: nil`；`assert!`。

## copilot_use/crates/shared/src/topics.rs · X01

職責：Shared wire/type。判定：同名crate不同職責；禁止把 WS Message本文加入low-sensitivity analytics schema

### L2 · const ·  VOCABULARY

```rust
pub const VOCABULARY : & [& str] = & ["rust" , "ownership" , "borrow" , "lifetime" , "async" , "trait" , "sql" , "postgresql" , "leptos" , "tauri" ,] ;
```

### L14 · function ·  occurrences

```rust
fn occurrences (text : & str) -> Vec < String >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.to_lowercase`；`.any`；`.iter`；`.contains`；`.count`；`.filter`；`.chars`；`Vec :: new`；`.collect`；`.map`；`.take`；`.filter`；`.split`；`.is_ascii_alphabetic`；`.contains`。

### L40 · module ·  tests

```rust
 mod tests (inline=True)
```

### L42 · use ·

```rust
use super :: * ;
```

### L44 · function · tests only_known_words_are_counted_and_secret_contact_and_identifier_messages_are_excluded

```rust
fn only_known_words_are_counted_and_secret_contact_and_identifier_messages_are_excluded ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert_eq!`；`assert!`；`assert_eq!`。

## copilot_use/crates/telemetry/src/lib.rs · X05

職責：Telemetry。判定：產品擴充；沒有空模組功能等同；不以AST說明取代部署collector驗證

### L2 · use ·

```rust
use opentelemetry :: { KeyValue , global , logs :: { LogRecord , Logger , LoggerProvider , Severity } , propagation :: { Extractor , Injector } , trace :: { FutureExt , Span , Status , TraceContextExt , Tracer } , } ;
```

### L8 · use ·

```rust
use opentelemetry_otlp :: { Protocol , WithExportConfig , WithHttpConfig } ;
```

### L9 · use ·

```rust
use opentelemetry_sdk :: { Resource , logs :: SdkLoggerProvider , metrics :: SdkMeterProvider , propagation :: TraceContextPropagator , trace :: SdkTracerProvider , } ;
```

### L13 · use ·

```rust
use std :: { collections :: HashMap , sync :: OnceLock , time :: Duration } ;
```

### L14 · static ·  LOGS

```rust
static LOGS : OnceLock < SdkLoggerProvider > = OnceLock :: new () ;
```

### L15 · struct ·  Telemetry

```rust
pub struct Telemetry { traces : Option < SdkTracerProvider > , metrics : Option < SdkMeterProvider > , logs : Option < SdkLoggerProvider > , }
```

### L21 · function · impl Drop for Telemetry drop

```rust
fn drop (& mut self)
```

可見性／屬性：``；``。

函式內呼叫：`.shutdown`；`.shutdown`；`.shutdown`。

### L33 · function ·  local_endpoint

```rust
fn local_endpoint (value : & str) -> Result < String , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：``；``。

函式內呼叫：`url :: Url :: parse`；`.host`；`.is_loopback`；`.is_loopback`；`matches!`；`.is_empty`；`.username`；`.is_some`；`.password`；`.is_some`；`.query`；`.is_some`；`.fragment`；`Err`；`.into`；`Ok`；`.into`；`.trim_end_matches`。

### L51 · function ·  local_http_client

```rust
fn local_http_client () -> Result < reqwest :: blocking :: Client , reqwest :: Error >
```

可見性／屬性：``；``。

函式內呼叫：`.build`；`.timeout`；`.redirect`；`.no_proxy`；`reqwest :: blocking :: Client :: builder`；`reqwest :: redirect :: Policy :: none`；`Duration :: from_secs`。

### L58 · struct ·  LocalHttpClient

```rust
# [derive (Debug , Clone)] struct LocalHttpClient { inner : reqwest :: blocking :: Client , allowed : std :: collections :: HashSet < String > , }
```

### L65 · function · impl opentelemetry_http :: HttpClient for LocalHttpClient send_bytes

```rust
async fn send_bytes (& self , mut request : http :: Request < bytes :: Bytes > ,) -> Result < http :: Response < bytes :: Bytes > , opentelemetry_http :: HttpError >
```

可見性／屬性：``；``。

函式內呼叫：`.method`；`.contains`；`.to_string`；`.uri`；`Err`；`Box :: new`；`std :: io :: Error :: other`；`.clear`；`.headers_mut`；`.insert`；`.headers_mut`；`http :: HeaderValue :: from_static`；`opentelemetry_http :: HttpClient :: send_bytes`。

### L86 · function ·  init

```rust
fn init (service : & 'static str) -> Result < Telemetry , Box < dyn std :: error :: Error + Send + Sync > >
```

可見性／屬性：`pub`；``。

函式內呼叫：`global :: set_text_map_propagator`；`TraceContextPropagator :: new`；`std :: env :: var`；`Ok`；`local_endpoint`；`local_http_client`；`.collect`；`.map`；`.iter`；`format!`；`.build`；`.with_attributes`；`.with_service_name`；`Resource :: builder_empty`；`KeyValue :: new`；`env!`；`.build`；`.with_timeout`；`.with_http_client`；`.with_headers`；`.with_endpoint`；`.with_protocol`；`.with_http`；`opentelemetry_otlp :: SpanExporter :: builder`；`format!`；`HashMap :: new`；`.clone`；`Duration :: from_secs`；`.build`；`.with_batch_exporter`；`.with_resource`；`SdkTracerProvider :: builder`；`.clone`；`.build`；`.with_timeout`；`.with_http_client`；`.with_headers`；`.with_endpoint`；`.with_protocol`；`.with_http`；`opentelemetry_otlp :: MetricExporter :: builder`；`format!`；`HashMap :: new`；`.clone`；`Duration :: from_secs`；`.build`；`.with_periodic_exporter`；`.with_resource`；`SdkMeterProvider :: builder`；`.clone`；`.build`；`.with_timeout`；`.with_http_client`；`.with_headers`；`.with_endpoint`；`.with_protocol`；`.with_http`；`opentelemetry_otlp :: LogExporter :: builder`；`format!`；`HashMap :: new`；`.clone`；`Duration :: from_secs`；`.build`；`.with_batch_exporter`；`.with_resource`；`SdkLoggerProvider :: builder`；`global :: set_tracer_provider`；`.clone`；`global :: set_meter_provider`；`.clone`；`.set`；`.clone`；`Ok`；`Some`；`Some`；`Some`。

### L153 · struct ·  Headers

```rust
struct Headers < 'a > (& 'a axum :: http :: HeaderMap) ;
```

### L155 · function · impl Extractor for Headers < '_ > get

```rust
fn get (& self , key : & str) -> Option < & str >
```

可見性／屬性：``；``。

函式內呼叫：`.and_then`；`.get`；`.ok`；`.to_str`。

### L158 · function · impl Extractor for Headers < '_ > keys

```rust
fn keys (& self) -> Vec < & str >
```

可見性／屬性：``；``。

函式內呼叫：`.collect`；`.map`；`.keys`；`.as_str`。

### L162 · struct ·  Carrier

```rust
struct Carrier (HashMap < String , String >) ;
```

### L164 · function · impl Injector for Carrier set

```rust
fn set (& mut self , key : & str , value : String)
```

可見性／屬性：``；``。

函式內呼叫：`.insert`；`.into`。

### L168 · function ·  outgoing_headers

```rust
fn outgoing_headers () -> HashMap < String , String >
```

可見性／屬性：`pub`；``。

函式內呼叫：`Carrier`；`HashMap :: new`；`global :: get_text_map_propagator`；`.inject_context`；`opentelemetry :: Context :: current`。

### L175 · function ·  event

```rust
fn event (operation : & 'static str , status : u16)
```

可見性／屬性：`pub`；``。

函式內呼叫：`KeyValue :: new`；`KeyValue :: new`；`i64 :: from`；`.add`；`.build`；`.u64_counter`；`global :: meter`；`.get`；`.logger`；`.create_log_record`；`.set_body`；`.into`；`.set_severity_number`；`.add_attribute`；`i64 :: from`；`.emit`。

### L193 · function ·  http

```rust
async fn http (request : axum :: extract :: Request , next : axum :: middleware :: Next ,) -> axum :: response :: Response
```

可見性／屬性：`pub`；``。

函式內呼叫：`.unwrap_or_else`；`.map`；`.get`；`.extensions`；`.to_owned`；`.as_str`；`.into`；`global :: get_text_map_propagator`；`.extract`；`Headers`；`.headers`；`.start_with_context`；`global :: tracer`；`.set_attribute`；`KeyValue :: new`；`.set_attribute`；`KeyValue :: new`；`.to_string`；`.method`；`.with_span`；`.with_context`；`.run`；`.clone`；`.as_u16`；`.status`；`.set_attribute`；`.span`；`KeyValue :: new`；`i64 :: from`；`.set_status`；`.span`；`Status :: error`；`.end`；`.span`；`.attach`；`event`。

### L224 · function ·  shutdown_signal

```rust
async fn shutdown_signal ()
```

可見性／屬性：`pub`；``。

函式內呼叫：`.expect`；`tokio :: signal :: unix :: signal`；`tokio :: signal :: unix :: SignalKind :: terminate`；`tokio :: select!`；`tokio :: signal :: ctrl_c`。

### L237 · function ·  observe

```rust
async fn observe < T , E > (operation : & 'static str , future : impl std :: future :: Future < Output = Result < T , E > > ,) -> Result < T , E >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.start`；`global :: tracer`；`.with_span`；`opentelemetry :: Context :: current`；`.with_context`；`.clone`；`.is_ok`；`.set_status`；`.span`；`Status :: error`；`.end`；`.span`；`.attach`；`event`。

### L256 · module ·  tests

```rust
 mod tests (inline=True)
```

### L258 · use ·

```rust
use super :: * ;
```

### L260 · function · tests no_remote_or_secret_endpoint

```rust
fn no_remote_or_secret_endpoint ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`assert!`；`assert!`；`assert!`。

### L266 · function · tests local_exporter_client_never_follows_redirects

```rust
fn local_exporter_client_never_follows_redirects ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.unwrap`；`TcpListener :: bind`；`.unwrap`；`.set_nonblocking`；`.unwrap`；`TcpListener :: bind`；`.unwrap`；`.local_addr`；`.unwrap`；`.local_addr`；`std :: thread :: spawn`；`.unwrap`；`.accept`；`.unwrap`；`.read`；`.unwrap`；`write!`；`.unwrap`；`.send`；`.body`；`.post`；`.unwrap`；`local_http_client`；`format!`；`assert_eq!`；`.unwrap`；`.join`；`assert!`。

### L293 · function · tests transport_strips_environment_credentials_and_rejects_other_destinations

```rust
fn transport_strips_environment_credentials_and_rejects_other_destinations ()
```

可見性／屬性：``；`# [test]`。

函式內呼叫：`.unwrap`；`TcpListener :: bind`；`format!`；`std :: thread :: spawn`；`.unwrap`；`.accept`；`.unwrap`；`.read`；`String :: from_utf8_lossy`；`assert!`；`assert!`；`.unwrap`；`.write_all`；`.unwrap`；`local_http_client`；`.collect`；`.into_iter`；`.clone`；`.unwrap`；`.body`；`.header`；`.uri`；`.method`；`http :: Request :: builder`；`bytes :: Bytes :: new`；`assert!`；`.unwrap`；`.join`；`.unwrap`；`.body`；`.uri`；`.method`；`http :: Request :: builder`；`bytes :: Bytes :: new`；`assert!`。

## copilot_use/crates/ui/src/browser.rs · C07

職責：Chat UI。判定：映射頁面／生命週期，非逐檔移植；參考 hardcoded localhost WS 不適用部署

### L2 · use ·

```rust
use leptos :: prelude :: * ;
```

### L3 · use ·

```rust
use send_wrapper :: SendWrapper ;
```

### L4 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L5 · use ·

```rust
use std :: { cell :: { Cell , RefCell } , rc :: Rc , } ;
```

### L9 · use ·

```rust
use wasm_bindgen :: { JsCast , closure :: Closure } ;
```

### L10 · use ·

```rust
use web_sys :: { CloseEvent , MessageEvent , WebSocket } ;
```

### L11 · function ·  api

```rust
async fn api (path : & str , method : & str , body : Option < Value >) -> Result < Value , String >
```

可見性／屬性：`pub`；``。

函式內呼叫：`.header`；`.header`；`.method`；`gloo_net :: http :: RequestBuilder :: new`；`.map_err`；`.try_into`；`.map_err`；`.send`；`.map_err`；`.body`；`.to_string`；`.send`；`.ok`；`Err`；`format!`；`.status`；`.status`；`.status`；`Ok`；`.map_err`；`.json`；`.into`。

### L37 · struct ·  SocketGuard

```rust
struct SocketGuard { active : Rc < Cell < bool > > , socket : WebSocket , _open : Closure < dyn FnMut () > , _message : Closure < dyn FnMut (MessageEvent) > , _close : Closure < dyn FnMut (CloseEvent) > , retry : Rc < RefCell < Option < gloo_timers :: callback :: Timeout > > > , _presence_poll : gloo_timers :: callback :: Interval , typing_timer : Rc < RefCell < Option < gloo_timers :: callback :: Timeout > > > , }
```

### L48 · function · impl Drop for SocketGuard drop

```rust
fn drop (& mut self)
```

可見性／屬性：``；``。

函式內呼叫：`.set`；`.set_onopen`；`.set_onmessage`；`.set_onclose`；`.take`；`.borrow_mut`；`.take`；`.borrow_mut`；`.close`。

### L58 · function ·  observe_room

```rust
fn observe_room (room : RwSignal < String > , messages : RwSignal < Vec < Value > > , error : RwSignal < String > , presence : RwSignal < Vec < Value > > , typing : RwSignal < String > , typing_signal : RwSignal < u32 > ,)
```

可見性／屬性：`pub`；``。

函式內呼叫：`Rc :: new`；`RefCell :: new`；`.clone`；`Rc :: new`；`Cell :: new`；`Effect :: new`；`.get`；`js_sys :: Date :: now`；`.get`；`.set`；`.as_ref`；`.borrow`；`.ready_state`；`.send_with_str`；`RwSignal :: new`；`Rc :: new`；`Cell :: new`；`Effect :: new`；`.get`；`.get`；`.set`；`Vec :: new`；`.set`；`String :: new`；`.is_empty`；`.location`；`.unwrap`；`web_sys :: window`；`.unwrap_or_default`；`.host`；`.as_deref`；`.protocol`；`Ok`；`WebSocket :: new`；`format!`；`.set`；`.into`；`.borrow_mut`；`Some`；`.clone`；`.clone`；`Closure :: < dyn FnMut () > :: new`；`.send_with_str`；`.to_string`；`json!`；`.set_onopen`；`Some`；`.unchecked_ref`；`.as_ref`；`.clone`；`Rc :: new`；`Cell :: new`；`.clone`；`.clone`；`Rc :: new`；`RefCell :: new`；`.clone`；`Rc :: new`；`RefCell :: new`；`String :: new`；`Closure :: < dyn FnMut (MessageEvent) > :: new`；`.get`；`.get_untracked`；`.as_string`；`.data`；`serde_json :: from_str :: < Value >`；`.as_str`；`.as_str`；`Some`；`.as_str`；`.borrow`；`.set`；`.into`；`.clone`；`.borrow_mut`；`Some`；`gloo_timers :: callback :: Timeout :: new`；`.get`；`.set`；`String :: new`；`.update`；`.clone`；`.contains`；`.push`；`.retain`；`.update`；`.any`；`.iter`；`.push`；`.clone`；`.len`；`.remove`；`.borrow_mut`；`.into`；`.unwrap_or`；`.as_str`；`.set`；`.unwrap_or_default`；`.cloned`；`.as_array`；`.set`；`.contains`；`.as_str`；`.get_untracked`；`.set`；`String :: new`；`.clone`；`.clone`；`format!`；`super :: spawn`；`api`；`.get`；`.get_untracked`；`.set`；`.unwrap_or_default`；`.cloned`；`.as_array`；`.get`；`.set`；`.set`；`.into`；`.set_onmessage`；`Some`；`.unchecked_ref`；`.as_ref`；`Rc :: new`；`RefCell :: new`；`.clone`；`.clone`；`Closure :: < dyn FnMut (CloseEvent) > :: new`；`.code`；`.get`；`.set`；`.into`；`.pow`；`.min`；`.get`；`.set`；`.get`；`.set`；`.into`；`.borrow_mut`；`Some`；`gloo_timers :: callback :: Timeout :: new`；`.update`；`.wrapping_add`；`.set_onclose`；`Some`；`.unchecked_ref`；`.as_ref`；`.clone`；`.clone`；`gloo_timers :: callback :: Interval :: new`；`.clone`；`.clone`；`super :: spawn`；`api`；`format!`；`.get`；`.get_untracked`；`.set`；`.unwrap_or_default`；`.cloned`；`.as_array`；`SendWrapper :: new`；`on_cleanup`；`drop`。

### L247 · function ·  delivery_queue

```rust
fn delivery_queue (room : RwSignal < String > , pending : RwSignal < Vec < Value > > , messages : RwSignal < Vec < Value > > , error : RwSignal < String > ,)
```

可見性／屬性：`pub`；`# [doc = " Bounded in-memory queue: retries keep the exact same UUID and payload."] # [doc = " Page reload intentionally does not retain private message content in browser storage."]`。

函式內呼叫：`Rc :: new`；`Cell :: new`；`Rc :: new`；`Cell :: new`；`SendWrapper :: new`；`.clone`；`on_cleanup`；`.set`；`Effect :: new`；`.get`；`.is_empty`；`.replace`；`.clone`；`.clone`；`super :: spawn`；`.get`；`.cloned`；`.first`；`.get_untracked`；`api`；`Some`；`.clone`；`.get`；`.update`；`.retain`；`.get_untracked`；`.unwrap_or`；`.as_str`；`.update`；`.any`；`.iter`；`.push`；`.clone`；`.len`；`.remove`；`.get_untracked`；`.set`；`String :: new`；`.get`；`.set`；`.clone`；`.ends_with`；`.ends_with`；`.ends_with`；`.ends_with`；`.pow`；`.min`；`gloo_timers :: future :: TimeoutFuture :: new`；`.set`。

### L315 · function ·  ai_stream

```rust
async fn ai_stream (body : Value , output : RwSignal < String >) -> Result < () , String >
```

可見性／屬性：`pub`；``。

函式內呼叫：`js_sys :: Object :: new`；`.map_err`；`js_sys :: Reflect :: set`；`.into`；`.into`；`.map_err`；`js_sys :: JSON :: parse`；`.map_err`；`js_sys :: Reflect :: set`；`.into`；`.map_err`；`js_sys :: Reflect :: set`；`.into`；`.into`；`.to_string`；`.unchecked_into`；`.map_err`；`js_sys :: Reflect :: get`；`.unwrap`；`web_sys :: window`；`.into`；`.unchecked_into`；`.map_err`；`.call2`；`.unwrap`；`web_sys :: window`；`.into`；`.map_err`；`wasm_bindgen_futures :: JsFuture :: from`；`.unwrap_or`；`.and_then`；`.ok`；`js_sys :: Reflect :: get`；`.into`；`.as_bool`；`Err`；`.into`；`.map_err`；`js_sys :: Reflect :: get`；`.into`；`.unchecked_into`；`.map_err`；`js_sys :: Reflect :: get`；`.into`；`.map_err`；`.call0`；`.unchecked_into`；`.map_err`；`js_sys :: Reflect :: get`；`.into`；`Vec :: < u8 > :: new`；`.map_err`；`wasm_bindgen_futures :: JsFuture :: from`；`.unchecked_into`；`.map_err`；`.call0`；`.and_then`；`.ok`；`js_sys :: Reflect :: get`；`.into`；`.as_bool`；`Some`；`.to_vec`；`js_sys :: Uint8Array :: new`；`.map_err`；`js_sys :: Reflect :: get`；`.into`；`.extend`；`.len`；`Err`；`.into`；`.position`；`.windows`；`.map_err`；`std :: str :: from_utf8`；`.find_map`；`.lines`；`.map`；`.strip_prefix`；`.join`；`.collect`；`.filter_map`；`.lines`；`.map`；`.strip_prefix`；`.map_err`；`serde_json :: from_str`；`.update`；`.push_str`；`.unwrap_or`；`.as_str`；`Err`；`.drain`；`Err`；`.into`；`Ok`。

## copilot_use/crates/ui/src/lib.rs · C07

職責：Chat UI。判定：映射頁面／生命週期，非逐檔移植；參考 hardcoded localhost WS 不適用部署

### L2 · use ·

```rust
use leptos :: prelude :: * ;
```

### L3 · use ·

```rust
use leptos_router :: { components :: { A , Outlet , ParentRoute , ProtectedRoute , Route , Router , Routes } , hooks :: use_navigate , path , } ;
```

### L8 · use ·

```rust
use serde_json :: { Value , json } ;
```

### L9 · struct ·  Session

```rust
# [derive (Clone , Copy)] struct Session { signed_in : RwSignal < bool > , error : RwSignal < String > , }
```

### L14 · module ·  browser

```rust
 mod browser (inline=False)
```

### L16 · use ·

```rust
# [cfg (target_arch = "wasm32")] use browser :: api ;
```

### L19 · function ·  api

```rust
async fn api (_ : & str , _ : & str , _ : Option < Value >) -> Result < Value , String >
```

可見性／屬性：``；`# [cfg (not (target_arch = "wasm32"))]`。

函式內呼叫：`Err`；`.into`。

### L22 · function ·  session

```rust
fn session () -> Session
```

可見性／屬性：``；``。

函式內呼叫：`.expect`；`use_context`。

### L25 · function ·  spawn

```rust
fn spawn (task : impl std :: future :: Future < Output = () > + 'static)
```

可見性／屬性：``；``。

函式內呼叫：`leptos :: task :: spawn_local`。

### L29 · function ·  App

```rust
fn App (signed_in : bool) -> impl IntoView
```

可見性／屬性：`pub`；`# [component]`。

函式內呼叫：`RwSignal :: new`；`RwSignal :: new`；`String :: new`；`provide_context`；`view!`。

### L45 · function ·  Layout

```rust
fn Layout () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`session`；`view!`。

### L50 · function ·  Login

```rust
fn Login () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`session`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`use_navigate`；`json!`；`.clone`；`.set`；`spawn`；`api`；`Some`；`.set`；`String :: new`；`.set`；`navigate`；`Default :: default`；`.set`；`.set`；`view!`。

### L76 · function ·  Chat

```rust
fn Chat () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`session`；`RwSignal :: new`；`Vec :: < Value > :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`Vec :: < Value > :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`RwSignal :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`spawn`；`api`；`.set`；`.unwrap_or_default`；`.cloned`；`.as_array`；`.set`；`Effect :: new`；`refresh`；`.set`；`.clone`；`.set`；`Vec :: new`；`format!`；`spawn`；`api`；`.get_untracked`；`.set`；`.unwrap_or_default`；`.cloned`；`.as_array`；`.set`；`RwSignal :: new`；`Vec :: < Value > :: new`；`RwSignal :: new`；`Vec :: < Value > :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`browser :: observe_room`；`browser :: delivery_queue`；`view!`。

### L183 · function ·  TopicControls

```rust
fn TopicControls (room : RwSignal < String >) -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`RwSignal :: new`；`RwSignal :: new`；`RwSignal :: new`；`RwSignal :: new`；`RwSignal :: new`；`RwSignal :: new`；`String :: new`；`.get`；`.get_untracked`；`.is_empty`；`spawn`；`api`；`format!`；`.get_untracked`；`.get_untracked`；`.set`；`.set`；`.set`；`.unwrap_or`；`.as_i64`；`.set`；`String :: new`；`.get_untracked`；`.get_untracked`；`.set`；`Effect :: new`；`.get`；`.update`；`.wrapping_add`；`.set`；`.set`；`.set`；`.set`；`.set`；`String :: new`；`.is_empty`；`refresh`；`view!`。

### L237 · function ·  PersonaControls

```rust
fn PersonaControls (room : RwSignal < String > , can_manage : Signal < bool >) -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`RwSignal :: new`；`Vec :: < Value > :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`RwSignal :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`RwSignal :: new`；`RwSignal :: new`；`RwSignal :: new`；`RwSignal :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`session`；`.get`；`.is_empty`；`.get`；`spawn`；`api`；`format!`；`.get_untracked`；`.set`；`.unwrap_or_default`；`.cloned`；`.as_array`；`.set`；`Effect :: new`；`.get`；`.get`；`.set`；`Vec :: new`；`.set`；`String :: new`；`.set`；`String :: new`；`.set`；`String :: new`；`.set`；`String :: new`；`.set`；`String :: new`；`.set`；`.get_untracked`；`.set`；`String :: new`；`.set`；`String :: new`；`refresh`；`view!`。

### L307 · function ·  Knowledge

```rust
fn Knowledge () -> impl IntoView
```

可見性／屬性：``；`# [component]`。

函式內呼叫：`session`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`String :: new`；`RwSignal :: new`；`Vec :: < Value > :: new`；`spawn`；`api`；`.set`；`.unwrap_or_default`；`.cloned`；`.as_array`；`.set`；`Effect :: new`；`refresh`；`view!`。

### L325 · function ·  uuid

```rust
fn uuid () -> String
```

可見性／屬性：``；`# [cfg (target_arch = "wasm32")]`。

函式內呼叫：`.unwrap_or_default`；`.and_then`；`.ok`；`.and_then`；`js_sys :: Reflect :: get`；`.unwrap`；`web_sys :: window`；`.into`；`.and_then`；`js_sys :: Reflect :: get`；`.into`；`.call0`；`.unchecked_into`；`.as_string`。

### L338 · function ·  uuid

```rust
fn uuid () -> String
```

可見性／屬性：``；`# [cfg (not (target_arch = "wasm32"))]`。

函式內呼叫：`String :: new`。

### L341 · function ·  shell

```rust
fn shell (signed_in : bool) -> impl IntoView
```

可見性／屬性：`pub`；``。

函式內呼叫：`view!`。

### L345 · function ·  hydrate

```rust
fn hydrate ()
```

可見性／屬性：`pub`；`# [cfg (feature = "hydrate")]`。

函式內呼叫：`.as_deref`；`.and_then`；`.and_then`；`.and_then`；`web_sys :: window`；`.document`；`.flatten`；`.ok`；`.query_selector`；`.get_attribute`；`Some`；`leptos :: mount :: hydrate_body`；`view!`。
