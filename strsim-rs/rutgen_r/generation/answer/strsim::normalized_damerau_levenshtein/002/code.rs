// Answer 0

#[test]
fn test_normalized_damerau_levenshtein_b_empty() {
    let result = strsim::normalized_damerau_levenshtein("", "flower");
    assert!(result.abs() < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_b_non_empty() {
    let result = strsim::normalized_damerau_levenshtein("", "test");
    assert!(result.abs() < 0.00001);
}

