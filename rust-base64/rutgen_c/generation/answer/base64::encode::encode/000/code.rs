// Answer 0

#[test]
fn test_encode_empty_input() {
    let input = b"";
    let result = encode(input);
    assert_eq!(result, "");
}

#[test]
fn test_encode_single_byte() {
    let input = b"A"; // ASCII for 'A'
    let result = encode(input);
    assert_eq!(result, "QQ==");
}

#[test]
fn test_encode_multiple_bytes() {
    let input = b"Hello, World!";
    let result = encode(input);
    assert_eq!(result, "SGVsbG8sIFdvcmxkIQ==");
}

#[test]
fn test_encode_non_ascii_input() {
    let input = b"\xF0\x9F\x98\x81"; // U+1F601 (grinning face with smiling eyes)
    let result = encode(input);
    assert_eq!(result, "8J+YgPCfkaI=");
}

