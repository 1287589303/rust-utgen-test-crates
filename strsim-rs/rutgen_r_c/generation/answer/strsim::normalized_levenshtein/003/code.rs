// Answer 0

#[test]
fn test_normalized_levenshtein_different_strings() {
    let result = normalized_levenshtein("kitten", "sitting");
    assert!((result - 0.57142).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_empty_strings() {
    let result = normalized_levenshtein("", "");
    assert!((result - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_empty_first_string() {
    let result = normalized_levenshtein("", "second");
    assert!(result.abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_empty_second_string() {
    let result = normalized_levenshtein("first", "");
    assert!(result.abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_same_strings() {
    let result = normalized_levenshtein("string", "string");
    assert!((result - 1.0).abs() < 0.00001);
}

