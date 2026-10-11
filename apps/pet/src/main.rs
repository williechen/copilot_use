//! 檔案掃描唯讀 CLI；實際隔離操作必須由具確認與授權的 GUI 呼叫。
use day9_pet::AuthorizedRoot;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let path=std::env::var("PET_AUTHORIZED_ROOT")?;
    let filename=std::env::args().nth(1).ok_or("用法：PET_AUTHORIZED_ROOT=/授權目錄 cargo run -p day9-pet -- file.txt")?;
    let root=AuthorizedRoot::new(&path)?;
    let preview=root.preview(&filename)?;
    println!("預覽（未執行檔案異動）: {}",serde_json::to_string_pretty(&preview)?);
    Ok(())
}