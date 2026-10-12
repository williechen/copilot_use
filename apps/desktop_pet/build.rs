fn main() {
    let git = |args: &[&str]| {
        std::process::Command::new("git")
            .args(args)
            .output()
            .ok()
            .filter(|o| o.status.success())
            .and_then(|o| String::from_utf8(o.stdout).ok())
    };
    let sha = git(&["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".into());
    let dirty = git(&["status", "--porcelain", "--untracked-files=normal"])
        .map(|s| !s.trim().is_empty())
        .unwrap_or(true);
    println!("cargo:rustc-env=COPILOT_SOURCE_SHA={}", sha.trim());
    println!("cargo:rustc-env=COPILOT_SOURCE_DIRTY={dirty}");
    for item in ["HEAD", "index"] {
        if let Some(path) = git(&["rev-parse", "--git-path", item]) {
            println!("cargo:rerun-if-changed={}", path.trim());
        }
    }
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=../../crates/pet_domain/src");
    tauri_build::build()
}
