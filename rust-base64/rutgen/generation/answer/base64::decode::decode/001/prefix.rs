// Answer 0

#[test]
fn test_decode_valid_input_no_padding() {
    let input = "SGVsbG8sIHdvcmxkIQ=="; // "Hello, world!" in base64
    let _ = decode(input);
}

#[test]
fn test_decode_valid_input_with_padding() {
    let input = "SGVsbG8="; // "Hello" in base64
    let _ = decode(input);
}

#[test]
fn test_decode_valid_input_exact_length() {
    let input = "SGVsbG8sIFdvcmxkIQ"; // "Hello, world!" truncated (valid for base64 length)
    let _ = decode(input);
}

#[test]
fn test_decode_input_with_invalid_character() {
    let input = "SGVsbG8sIHdvcmxkIQ@"; // Invalid character '@'
    let _ = decode(input);
}

#[test]
fn test_decode_input_with_invalid_length() {
    let input = "SGVsbG"; // Invalid length for base64
    let _ = decode(input);
}

#[test]
fn test_decode_input_with_extra_padding() {
    let input = "SGVsbG8=="; // Invalid padding with extra '='
    let _ = decode(input);
}

#[test]
fn test_decode_empty_input() {
    let input = ""; // Empty input should yield an error
    let _ = decode(input);
}

#[test]
fn test_decode_input_only_padding() {
    let input = "==="; // Only padding characters
    let _ = decode(input);
}

