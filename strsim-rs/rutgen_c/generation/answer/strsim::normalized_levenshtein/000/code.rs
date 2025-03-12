// Answer 0

#[test]
fn test_normalized_levenshtein() {
    assert!((normalized_levenshtein("kitten", "sitting") - 0.57142).abs() < 0.00001);
    assert!((normalized_levenshtein("", "") - 1.0).abs() < 0.00001);
    assert!(normalized_levenshtein("", "second").abs() < 0.00001);
    assert!(normalized_levenshtein("first", "").abs() < 0.00001);
    assert!((normalized_levenshtein("string", "string") - 1.0).abs() < 0.00001);
}

