// Answer 0

#[test]
fn test_normalized_damerau_levenshtein() {
    assert!((normalized_damerau_levenshtein("levenshtein", "löwenbräu") - 0.27272).abs() < 0.00001);
    assert!((normalized_damerau_levenshtein("", "") - 1.0).abs() < 0.00001);
    assert!(normalized_damerau_levenshtein("", "flower").abs() < 0.00001);
    assert!(normalized_damerau_levenshtein("tree", "").abs() < 0.00001);
    assert!((normalized_damerau_levenshtein("sunglasses", "sunglasses") - 1.0).abs() < 0.00001);
}

