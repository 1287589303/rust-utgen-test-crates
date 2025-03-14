// Answer 0

#[test]
fn test_decode_standard() {
    use base64::{engine::{self, general_purpose}, alphabet};

    let engine = general_purpose::STANDARD;
    let input = "aGVsbG8gd29ybGR+Cg==";
    let result = engine.decode(input).unwrap();
    assert_eq!(result, b"hello world\n");
}

#[test]
fn test_decode_custom_url_safe() {
    use base64::{engine::{self, general_purpose}, alphabet};

    let engine = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
    let input = "aGVsbG8gaW50ZXJuZXR-Cg";
    let result = engine.decode(input).unwrap();
    assert_eq!(result, b"hello internet\n");
}

#[test]
fn test_decode_empty_input() {
    use base64::{engine::{self, general_purpose}};

    let engine = general_purpose::STANDARD;
    let input = "";
    let result = engine.decode(input).unwrap();
    assert_eq!(result, b"");
}

#[test]
#[should_panic(expected = "DecodeError")]
fn test_decode_invalid_input() {
    use base64::{engine::{self, general_purpose}};

    let engine = general_purpose::STANDARD;
    let input = "invalid_base64";
    let _result = engine.decode(input).unwrap(); // This should panic
}

