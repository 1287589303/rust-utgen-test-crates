// Answer 0

#[test]
fn test_normalized_levenshtein_equal_strings() {
    assert!((normalized_levenshtein("string", "string") - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_non_empty_strings() {
    assert!((normalized_levenshtein("kitten", "sitting") - 0.57142).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_first_empty() {
    assert!(normalized_levenshtein("", "second").abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_second_empty() {
    assert!(normalized_levenshtein("first", "").abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_empty_strings() {
    assert!((normalized_levenshtein("", "") - 1.0).abs() < 0.00001);
}

