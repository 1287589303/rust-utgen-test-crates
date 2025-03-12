// Answer 0

#[test]
fn test_decode_to_string_empty() {
    let result = decode_to_string("");
    let expected: Option<String> = Some(String::new());
    assert_eq!(result, expected);
}

#[test]
fn test_decode_to_string_ascii_only() {
    let result = decode_to_string("hello");
    let expected: Option<String> = Some(String::from("hello"));
    assert_eq!(result, expected);
}

#[test]
fn test_decode_to_string_valid_non_ascii() {
    let result = decode_to_string("xn--espaol-zc0b");
    let expected: Option<String> = Some(String::from("español"));
    assert_eq!(result, expected);
}

#[test]
fn test_decode_to_string_boundary_length_1() {
    let result = decode_to_string("a");
    let expected: Option<String> = Some(String::from("a"));
    assert_eq!(result, expected);
}

#[test]
fn test_decode_to_string_boundary_length_63() {
    let input = "a".repeat(63);
    let result = decode_to_string(&input);
    let expected: Option<String> = Some(input);
    assert_eq!(result, expected);
}

#[test]
fn test_decode_to_string_invalid_sequence() {
    let result = decode_to_string("xn--invalid-");
    assert!(result.is_none());
}

#[test]
fn test_decode_to_string_overflow_length() {
    let long_input = "a".repeat(64);
    let result = decode_to_string(&long_input);
    assert!(result.is_none());
}

#[test]
fn test_decode_to_string_unicode_characters() {
    let result = decode_to_string("xn--bcher-kva");
    let expected: Option<String> = Some(String::from("bücher"));
    assert_eq!(result, expected);
}

