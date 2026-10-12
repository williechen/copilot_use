//! Opt-in vocabulary counting; never emit arbitrary words, original text or identifiers.
pub const VOCABULARY: &[&str] = &[
    "rust",
    "ownership",
    "borrow",
    "lifetime",
    "async",
    "trait",
    "sql",
    "postgresql",
    "leptos",
    "tauri",
];
pub fn occurrences(text: &str) -> Vec<String> {
    let lower = text.to_lowercase();
    if [
        "sk-",
        "-----begin",
        "@",
        "http://",
        "https://",
        "password",
        "api_key",
        "token=",
        "secret=",
    ]
    .iter()
    .any(|s| lower.contains(s))
        || text.chars().filter(char::is_ascii_digit).count() >= 9
    {
        return Vec::new();
    }
    lower
        .split(|c: char| !c.is_ascii_alphabetic())
        .filter(|word| VOCABULARY.contains(word))
        .take(32)
        .map(str::to_owned)
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_known_words_are_counted_and_secret_contact_and_identifier_messages_are_excluded() {
        assert_eq!(
            occurrences("Rust rust，ownership 與 private-family-name"),
            vec!["rust", "rust", "ownership"]
        );
        for text in [
            "Rust sk-synthetic-key",
            "Rust user@example.test",
            "Rust token=fixture",
            "Rust 0912345678",
            "Rust https://example.test",
        ] {
            assert!(occurrences(text).is_empty());
        }
        assert_eq!(occurrences(&"rust ".repeat(100)).len(), 32);
    }
}
