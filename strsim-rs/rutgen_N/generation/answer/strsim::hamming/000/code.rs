// Answer 0

#[test]
fn test_hamming_different_strings_same_length() {
    let result = hamming("hamming", "hammers");
    assert_eq!(Ok(3), result);
}

#[test]
fn test_hamming_shorter_string() {
    let result = hamming("hamming", "ham");
    assert_eq!(Err(StrSimError::DifferentLengthArgs), result);
}

#[test]
fn test_hamming_empty_strings() {
    let result = hamming("", "");
    assert_eq!(Ok(0), result);
}

#[test]
fn test_hamming_one_empty_string() {
    let result = hamming("hamming", "");
    assert_eq!(Err(StrSimError::DifferentLengthArgs), result);
}

#[test]
fn test_hamming_identical_strings() {
    let result = hamming("identical", "identical");
    assert_eq!(Ok(0), result);
}

