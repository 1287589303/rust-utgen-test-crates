// Answer 0

#[test]
fn test_jaro_identical_strings() {
    let similarity = jaro("test", "test");
    assert!((1.0 - similarity).abs() < 0.001);
}

#[test]
fn test_jaro_completely_different_strings() {
    let similarity = jaro("abc", "def");
    assert!((0.0 - similarity).abs() < 0.001);
}

#[test]
fn test_jaro_partial_similarity() {
    let similarity = jaro("feline", "felony");
    assert!((0.833 - similarity).abs() < 0.001);
}

#[test]
fn test_jaro_empty_strings() {
    let similarity = jaro("", "");
    assert!((1.0 - similarity).abs() < 0.001);
}

#[test]
fn test_jaro_one_empty_string() {
    let similarity_a = jaro("nonempty", "");
    let similarity_b = jaro("", "nonempty");
    assert!((0.0 - similarity_a).abs() < 0.001);
    assert!((0.0 - similarity_b).abs() < 0.001);
}

#[test]
fn test_jaro_case_insensitive() {
    let similarity = jaro("Test", "test");
    assert!((0.875 - similarity).abs() < 0.001);
}

#[test]
fn test_jaro_long_strings() {
    let similarity = jaro("abcdefghijklmnopqrstuvwxyz", "zyxwvutsrqponmlkjihgfedcba");
    assert!((0.0 - similarity).abs() < 0.001);
}

