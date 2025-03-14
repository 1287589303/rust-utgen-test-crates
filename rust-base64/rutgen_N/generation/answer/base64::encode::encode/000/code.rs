// Answer 0

#[test]
fn test_encode_empty_input() {
    let input = b"";
    let result = base64::encode(input);
    assert_eq!(result, "");
}

#[test]
fn test_encode_single_byte() {
    let input = b"A";
    let result = base64::encode(input);
    assert_eq!(result, "QQ==");
}

#[test]
fn test_encode_two_bytes() {
    let input = b"AB";
    let result = base64::encode(input);
    assert_eq!(result, "QUE=");
}

#[test]
fn test_encode_three_bytes() {
    let input = b"ABC";
    let result = base64::encode(input);
    assert_eq!(result, "QUJD");
}

#[test]
fn test_encode_four_bytes() {
    let input = b"ABCD";
    let result = base64::encode(input);
    assert_eq!(result, "QUJDRA==");
}

#[test]
fn test_encode_multiple_bytes() {
    let input = b"Hello, World!";
    let result = base64::encode(input);
    assert_eq!(result, "SGVsbG8sIFdvcmxkIQ==");
}

