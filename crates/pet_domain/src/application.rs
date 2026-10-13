//! 設定儲存用例；Profile 規則在 Domain，DB／檔案投影／Tauri 留在 Adapter。
use crate::profile::PetProfile;
use std::{future::Future, pin::Pin};
use uuid::Uuid;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileError {
    Invalid,
    Unavailable,
    ProjectionPending,
}

/// 投影尚未完成仍回報 ProjectionPending，不把部分成功視為完整成功。
pub type ProfileFuture<'a> = Pin<Box<dyn Future<Output = Result<(), ProfileError>> + Send + 'a>>;

/// 實際由 PetStore 實作、Tauri 呼叫的設定 Port。
pub trait ProfileRepository: Send + Sync {
    fn save<'a>(&'a self, id: Uuid, profile: &'a PetProfile) -> ProfileFuture<'a>;
}

/// 使用既有 Profile 的驗證，通過後才委派持久化。
pub async fn save_profile(
    repository: &impl ProfileRepository,
    id: Uuid,
    profile: &PetProfile,
) -> Result<(), ProfileError> {
    if id.is_nil() || profile.validate().is_err() {
        return Err(ProfileError::Invalid);
    }
    repository.save(id, profile).await
}

#[cfg(test)]
mod tests;
