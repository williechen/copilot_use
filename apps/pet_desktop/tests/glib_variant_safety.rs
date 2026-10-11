#![cfg(target_os = "linux")]
use glib::variant::ToVariant;
#[test]
fn optimized_variant_strings_use_mutable_out_pointer() {
    let source = vec!["alpha", "βeta", "gamma", "delta"].to_variant();
    assert_eq!(
        source.array_iter_str().unwrap().collect::<Vec<_>>(),
        vec!["alpha", "βeta", "gamma", "delta"]
    );
    assert_eq!(source.array_iter_str().unwrap().last(), Some("delta"));
    assert_eq!(source.array_iter_str().unwrap().nth(1), Some("βeta"));
    assert_eq!(source.array_iter_str().unwrap().next_back(), Some("delta"));
    assert_eq!(source.array_iter_str().unwrap().nth_back(1), Some("gamma"));
}
