use pet_server::pet_store::PetStore;
#[sqlx::test(migrations="../migrations")]
async fn new_pet_has_two_inactive_base_activities(pool:sqlx::SqlitePool) {
    let store=PetStore{pool:pool.clone()};let pet=store.create("驗收寵物").await.unwrap();
    let activities:Vec<(String,i64)>=sqlx::query_as("SELECT activity_type,is_activity FROM pet_activity WHERE pet_id=?1 AND owner_id='local-owner' ORDER BY activity_type")
        .bind(&pet.id).fetch_all(&pool).await.unwrap();
    assert_eq!(activities,vec![("坐下".into(),0),("散步".into(),0)]);
}
