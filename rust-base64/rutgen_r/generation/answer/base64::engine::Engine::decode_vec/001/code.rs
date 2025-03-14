// Answer 0

#[test]
fn test_decode_vec_standard_engine() {
    use base64::{Engine as _, engine::general_purpose};

    let mut buffer = Vec::new();
    let result = general_purpose::STANDARD.decode_vec("aGVsbG8gd29ybGR+Cg==", &mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, b"hello world\n");
}

#[test]
fn test_decode_vec_custom_engine() {
    use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
    
    const CUSTOM_ENGINE: engine::GeneralPurpose = 
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::PAD);

    let mut buffer = Vec::new();
    let result = CUSTOM_ENGINE.decode_vec("aGVsbG8gaW50ZXJuZXR-Cg==", &mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, b"hello internet\n");
}

#[test]
fn test_decode_vec_empty_input() {
    use base64::{Engine as _, engine::general_purpose};

    let mut buffer = Vec::new();
    let result = general_purpose::STANDARD.decode_vec("", &mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer.is_empty(), true);
}

#[should_panic]
fn test_decode_vec_invalid_input() {
    use base64::{Engine as _, engine::general_purpose};

    let mut buffer = Vec::new();
    let _ = general_purpose::STANDARD.decode_vec("invalid_base64", &mut buffer).unwrap();
}

