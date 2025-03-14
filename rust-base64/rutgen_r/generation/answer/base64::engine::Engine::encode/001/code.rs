// Answer 0

#[test]
fn test_encode_standard_engine() {
    use base64::engine::{self, general_purpose};

    let engine = general_purpose::STANDARD;
    let input = b"hello world~";
    let result = engine.encode(input);
    assert_eq!(result, "aGVsbG8gd29ybGQ~");
}

#[test]
fn test_encode_url_safe_engine() {
    use base64::engine::{self, general_purpose, alphabet};

    struct CustomEngine {
        inner: engine::GeneralPurpose,
    }

    let engine = CustomEngine {
        inner: engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD),
    };
    let input = b"hello internet~";
    let result = engine.inner.encode(input);
    assert_eq!(result, "aGVsbG8gaW50ZXJuZXQ~");
}

#[test]
fn test_encode_empty_input() {
    use base64::engine::{self, general_purpose};

    let engine = general_purpose::STANDARD;
    let input = b"";
    let result = engine.encode(input);
    assert_eq!(result, "");
}

#[test]
fn test_encode_single_byte() {
    use base64::engine::{self, general_purpose};

    let engine = general_purpose::STANDARD;
    let input = b"A";
    let result = engine.encode(input);
    assert_eq!(result, "QQ==");
}

#[should_panic]
fn test_encode_invalid_utf8() {
    use base64::engine::{self, general_purpose};

    let engine = general_purpose::STANDARD;
    let input = b"\xFF"; // Invalid UTF-8 sequence
    let _result = engine.encode(input); // Should panic on invalid UTF-8
}

