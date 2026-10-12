use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

struct Unavailable {
    called: AtomicBool,
}
impl ProfileRepository for Unavailable {
    fn save<'a>(&'a self, _: Uuid, _: &'a PetProfile) -> ProfileFuture<'a> {
        Box::pin(async move {
            self.called.store(true, Ordering::SeqCst);
            Err(ProfileError::ProjectionPending)
        })
    }
}

#[tokio::test]
async fn invalid_profile_never_reaches_adapter_and_projection_failure_is_not_success() {
    let repository = Unavailable {
        called: AtomicBool::new(false),
    };
    let mut profile = PetProfile {
        name: "小啾".into(),
        species: "鳥".into(),
        likes: vec![],
        personality: vec![],
        interests: vec![],
        appearance: None,
        animation: None,
    };
    assert_eq!(
        save_profile(&repository, Uuid::nil(), &profile).await,
        Err(ProfileError::Invalid)
    );
    profile.name.clear();
    assert_eq!(
        save_profile(&repository, Uuid::from_u128(1), &profile).await,
        Err(ProfileError::Invalid)
    );
    assert!(!repository.called.load(Ordering::SeqCst));
    profile.name = "小啾".into();
    assert_eq!(
        save_profile(&repository, Uuid::from_u128(1), &profile).await,
        Err(ProfileError::ProjectionPending)
    );
    assert!(repository.called.load(Ordering::SeqCst));
}
