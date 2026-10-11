//! Desktop Pet CLI：檢查五項設定及安全預覽；預設絕不進行不可逆檔案操作。
use day9_pet::{
    AuthorizedRoot,
    profile::{AssetManifest, ProfileStore},
};
use std::{error::Error, path::Path};
use uuid::Uuid;

fn usage() -> &'static str {
    "Usage:\n  PET_AUTHORIZED_ROOT=/safe/root cargo run -p day9-pet -- preview file.txt\n  PET_PROFILE_ROOT=/safe/profiles cargo run -p day9-pet -- profile <pet-uuid>\n  cargo run -p day9-pet -- manifest <pet-uuid> <image-width> <image-height>"
}

fn main() -> Result<(), Box<dyn Error>> {
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
            let profile = ProfileStore::new(Path::new(&root))?.load(id)?;
            println!("{}", serde_json::to_string_pretty(&profile)?);
        }
        Some("manifest") => {
            let id: Uuid = args.next().ok_or_else(|| usage().to_owned())?.parse()?;
            let image_width: u32 = args.next().ok_or_else(|| usage().to_owned())?.parse()?;
            let image_height: u32 = args.next().ok_or_else(|| usage().to_owned())?.parse()?;
            if image_width == 0 || image_height == 0 {
                return Err("image dimensions must be positive".into());
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&AssetManifest::new(id, image_width, image_height))?
            );
        }
        _ => return Err(usage().into()),
    }
    Ok(())
}
