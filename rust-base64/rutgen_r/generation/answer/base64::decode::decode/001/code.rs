// Answer 0

#[test]
fn test_decode_valid_base64() {
    let input = "SGVsbG8sIFdvcmxkIQ==";
    let result = base64::decode(input).unwrap();
    assert_eq!(result, b"Hello, World!");
}

#[test]
fn test_decode_invalid_base64() {
    let input = "!!!invalid_base64!!!";
    let result = base64::decode(input);
    assert!(result.is_err());
}

#[test]
fn test_decode_empty_string() {
    let input = "";
    let result = base64::decode(input).unwrap();
    assert_eq!(result, b"");
}

#[test]
fn test_decode_padding_base64() {
    let input = "U29tZSBkYXRhLg=="; // "Some data."
    let result = base64::decode(input).unwrap();
    assert_eq!(result, b"Some data.");
}

