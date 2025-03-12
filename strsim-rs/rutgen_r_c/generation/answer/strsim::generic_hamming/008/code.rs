// Answer 0

#[test]
fn test_generic_hamming_equal() {
    let result = generic_hamming("abc", "abc");
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_empty_strings() {
    let result = generic_hamming("", "");
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_single_character_equal() {
    let result = generic_hamming("a", "a");
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_multiple_characters_equal() {
    let result = generic_hamming("hello", "hello");
    assert_eq!(result, Ok(0));
}

