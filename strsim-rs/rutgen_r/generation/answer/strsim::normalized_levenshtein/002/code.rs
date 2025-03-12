// Answer 0

#[test]
fn test_normalized_levenshtein_empty_a_non_empty_b() {
    let a = "";
    let b = "second";
    let result = normalized_levenshtein(a, b);
    assert!(result.abs() < 0.00001); // Expecting result to be close to 0.0
}

#[test]
fn test_normalized_levenshtein_empty_a_non_empty_b_different_lengths() {
    let a = "";
    let b = "example";
    let result = normalized_levenshtein(a, b);
    assert!(result.abs() < 0.00001); // Expecting result to be close to 0.0
}

#[test]
fn test_normalized_levenshtein_empty_a_non_empty_b_another_case() {
    let a = "";
    let b = "test";
    let result = normalized_levenshtein(a, b);
    assert!(result.abs() < 0.00001); // Expecting result to be close to 0.0
}

