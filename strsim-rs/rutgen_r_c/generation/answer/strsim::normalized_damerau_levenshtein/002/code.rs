// Answer 0

#[test]
fn test_normalized_damerau_levenshtein_empty_a_non_empty_b() {
    let result = normalized_damerau_levenshtein("", "flower");
    assert!((result - 0.0).abs() < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_empty_a_non_empty_b_another_string() {
    let result = normalized_damerau_levenshtein("", "hello");
    assert!((result - 0.0).abs() < 0.00001);
}

