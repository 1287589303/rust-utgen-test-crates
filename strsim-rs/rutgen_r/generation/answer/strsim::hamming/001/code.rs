// Answer 0

#[test]
fn test_hamming_equal_strings() {
    assert_eq!(Ok(0), hamming("test", "test"));
}

#[test]
fn test_hamming_different_strings() {
    assert_eq!(Ok(3), hamming("hamming", "hammers"));
}

#[test]
fn test_hamming_different_length_strings() {
    use strsim::StrSimError::DifferentLengthArgs;
    assert_eq!(Err(DifferentLengthArgs), hamming("hamming", "ham"));
}

#[test]
fn test_hamming_empty_strings() {
    assert_eq!(Ok(0), hamming("", ""));
}

#[test]
fn test_hamming_one_char_different() {
    assert_eq!(Ok(1), hamming("a", "b"));
}

#[test]
fn test_hamming_one_char_length_diff() {
    use strsim::StrSimError::DifferentLengthArgs;
    assert_eq!(Err(DifferentLengthArgs), hamming("a", ""));
}

