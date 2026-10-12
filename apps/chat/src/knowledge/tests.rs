use super::cosine_distance;
#[test]
fn portable_cosine_retrieval_rejects_invalid_vectors_and_preserves_ranking() {
    let mut query = vec![0.0; 1536];
    query[0] = 1.0;
    let same = query.clone();
    let mut orthogonal = vec![0.0; 1536];
    orthogonal[1] = 1.0;
    let opposite = query.iter().map(|v| -v).collect::<Vec<_>>();
    assert_eq!(cosine_distance(&query, &same), Some(0.0));
    assert_eq!(cosine_distance(&query, &orthogonal), Some(1.0));
    assert_eq!(cosine_distance(&query, &opposite), Some(2.0));
    assert_eq!(cosine_distance(&query, &vec![0.0; 1536]), None);
    assert_eq!(cosine_distance(&query, &[1.0]), None);
    let mut invalid = query.clone();
    invalid[0] = f32::NAN;
    assert_eq!(cosine_distance(&query, &invalid), None);
}
