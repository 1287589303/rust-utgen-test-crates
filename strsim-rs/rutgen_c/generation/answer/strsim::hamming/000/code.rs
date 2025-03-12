// Answer 0

#[test]
fn test_hamming_equal_length_diff_characters() {
    assert_eq!(Ok(3), hamming("hamming", "hammers"));
}

#[test]
fn test_hamming_equal_length_no_diff() {
    assert_eq!(Ok(0), hamming("hello", "hello"));
}

#[test]
fn test_hamming_different_length() {
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("hamming", "ham"));
}

#[test]
fn test_hamming_empty_strings() {
    assert_eq!(Ok(0), hamming("", ""));
}

#[test]
fn test_hamming_one_empty_string() {
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("hamming", ""));
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("", "ham"));
}

