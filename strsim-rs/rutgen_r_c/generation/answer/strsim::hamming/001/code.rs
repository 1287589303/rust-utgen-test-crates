// Answer 0

#[test]
fn test_hamming_equal_length_strings() {
    assert_eq!(Ok(3), hamming("hamming", "hammers"));
}

#[test]
fn test_hamming_different_length_strings() {
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("hamming", "ham"));
}

#[test]
fn test_hamming_empty_strings() {
    assert_eq!(Ok(0), hamming("", ""));
}

#[test]
fn test_hamming_one_empty_string() {
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("hamming", ""));
}

#[test]
fn test_hamming_identical_strings() {
    assert_eq!(Ok(0), hamming("test", "test"));
}

#[test]
fn test_hamming_different_characters() {
    assert_eq!(Ok(4), hamming("abcd", "efgh"));
}

