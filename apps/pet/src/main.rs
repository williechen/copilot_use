//! Desktop Pet CLI：檢查五項設定及安全預覽；預設絕不進行不可逆檔案操作。
use day9_pet::{AuthorizedRoot, files::read_profile};
use std::{error::Error, path::Path};
use uuid::Uuid;

fn usage() -> &'static str {
    "Usage:\n  PET_AUTHORIZED_ROOT=/safe/root cargo run -p day9-pet -- preview file.txt\n  PET_PROFILE_ROOT=/safe/profiles cargo run -p day9-pet -- profile <pet-uuid>\n  PET_PROFILE_ROOT=/safe/profiles cargo run -p day9-pet -- manifest <pet-uuid>"
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("preview") => {
            let path = std::env::var("PET_AUTHORIZED_ROOT")?;
            let filename = args.next().ok_or_else(|| usage().to_owned())?;
            let root = AuthorizedRoot::new(Path::new(&path))?;
            let preview = root.preview(&filename)?;
            println!("{}", serde_json::to_string_pretty(&preview)?);
        }
        Some("profile") => {
            let root = std::env::var("PET_PROFILE_ROOT")?;
            let id: Uuid = args.next().ok_or_else(|| usage().to_owned())?.parse()?;
            let profile = read_profile(Path::new(&root), id)?;
            println!("{}", serde_json::to_string_pretty(&profile)?);
        }
        Some("manifest") => {
            let id: Uuid = args.next().ok_or_else(|| usage().to_owned())?.parse()?;
            let root = std::env::var("PET_PROFILE_ROOT")?;
            read_profile(Path::new(&root), id)?;
            let path = Path::new(&root)
                .join(id.to_string())
                .join("assets/manifest.json");
            let metadata = std::fs::symlink_metadata(&path)?;
            if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > 8192 {
                return Err("unsafe asset manifest".into());
            }
            let manifest: serde_json::Value = serde_json::from_slice(&std::fs::read(path)?)?;
            println!("{}", serde_json::to_string_pretty(&manifest)?);
        }
        _ => return Err(usage().into()),
    }
    Ok(())
}
