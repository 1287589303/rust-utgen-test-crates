// Answer 0

#[test]
fn test_empty_a_empty_b() {
    let result = generic_damerau_levenshtein(&[], &[]);
}

#[test]
fn test_empty_a_non_empty_b() {
    let result = generic_damerau_levenshtein(&[], &[1]);
}

#[test]
fn test_non_empty_a_empty_b() {
    let result = generic_damerau_levenshtein(&[1], &[]);
}

#[test]
fn test_empty_a_two_elements_b() {
    let result = generic_damerau_levenshtein(&[], &[1, 2]);
}

#[test]
fn test_two_elements_a_empty_b() {
    let result = generic_damerau_levenshtein(&[1, 2], &[]);
}

