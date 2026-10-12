//! Portable preference use case; DB, file projection and Tauri stay outside this Port.
use crate::profile::PetProfile;
use std::{future::Future, pin::Pin};
use uuid::Uuid;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileError {
    Invalid,
    Unavailable,
    ProjectionPending,
}
pub type ProfileFuture<'a> = Pin<Box<dyn Future<Output = Result<(), ProfileError>> + Send + 'a>>;
pub trait ProfileRepository: Send + Sync {
    fn save<'a>(&'a self, id: Uuid, profile: &'a PetProfile) -> ProfileFuture<'a>;
}
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
