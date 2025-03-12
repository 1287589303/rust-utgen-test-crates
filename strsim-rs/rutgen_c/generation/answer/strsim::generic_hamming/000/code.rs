// Answer 0

#[test]
fn test_generic_hamming_equal_sequences() {
    let result = generic_hamming("abc", "abc");
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_different_sequences() {
    let result = generic_hamming("abc", "abd");
    assert_eq!(result, Ok(1));
}

#[test]
fn test_generic_hamming_different_length_sequences() {
    let result = generic_hamming("abc", "ab");
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
}

#[test]
fn test_generic_hamming_empty_sequences() {
    let result = generic_hamming("", "");
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_empty_vs_non_empty() {
    let result = generic_hamming("", "a");
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
}

