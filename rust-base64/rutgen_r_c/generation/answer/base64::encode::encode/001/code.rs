// Answer 0

#[test]
fn test_encode_empty_input() {
    let input = b"";
    let result = encode(input);
    assert_eq!(result, "");
}

#[test]
fn test_encode_single_character() {
    let input = b"a";
    let result = encode(input);
    assert_eq!(result, "YQ==");
}

#[test]
fn test_encode_two_characters() {
    let input = b"ab";
    let result = encode(input);
    assert_eq!(result, "YWJ=");
}

#[test]
fn test_encode_three_characters() {
    let input = b"abc";
    let result = encode(input);
    assert_eq!(result, "YWJj");
}

#[test]
fn test_encode_full_block() {
    let input = b"abcdef";
    let result = encode(input);
    assert_eq!(result, "YWJjZGU=");
}

#[test]
fn test_encode_full_input() {
    let input = b"Hello, World!";
    let result = encode(input);
    assert_eq!(result, "SGVsbG8sIFdvcmxkIQ==");
}

