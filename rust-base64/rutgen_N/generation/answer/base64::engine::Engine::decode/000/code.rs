// Answer 0

#[test]
fn test_decode_standard() {
    use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
    
    let engine = general_purpose::STANDARD;
    let input = "aGVsbG8gd29ybGR+Cg==";
    let result = engine.decode(input).unwrap();
    assert_eq!(result, b"hello world\n".to_vec());
}

#[test]
fn test_decode_url_safe() {
    use base64::{Engine as _, alphabet, engine::{self, GeneralPurpose}};
    
    let engine = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
    let input = "aGVsbG8gaW50ZXJuZXR-Cg";
    let result = engine.decode(input).unwrap();
    assert_eq!(result, b"hello internet\n".to_vec());
}

#[test]
#[should_panic(expected = "DecodeError")] // Assuming that DecodeError is the type used for panicking
fn test_decode_invalid_input() {
    use base64::{Engine as _, engine::general_purpose};

    let engine = general_purpose::STANDARD;
    let input = "invalid_base64";
    let _ = engine.decode(input).unwrap(); // This should panic
}

