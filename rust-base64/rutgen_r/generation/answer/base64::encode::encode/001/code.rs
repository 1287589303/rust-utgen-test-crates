// Answer 0

#[test]
fn test_encode_empty_input() {
    let input = b"";
    let output = base64::encode(input);
    assert_eq!(output, "");
}

#[test]
fn test_encode_single_byte() {
    let input = b"A";
    let output = base64::encode(input);
    assert_eq!(output, "QQ==");
}

#[test]
fn test_encode_two_bytes() {
    let input = b"AB";
    let output = base64::encode(input);
    assert_eq!(output, "QUI=");
}

#[test]
fn test_encode_three_bytes() {
    let input = b"ABC";
    let output = base64::encode(input);
    assert_eq!(output, "QUJD");
}

#[test]
fn test_encode_four_bytes() {
    let input = b"ABCD";
    let output = base64::encode(input);
    assert_eq!(output, "QUJDRA==");
}

#[test]
fn test_encode_non_ascii() {
    let input = b"\xF0\x9F\x98\x81"; // 😀
    let output = base64::encode(input);
    assert_eq!(output, "8J+YgI==");
}

