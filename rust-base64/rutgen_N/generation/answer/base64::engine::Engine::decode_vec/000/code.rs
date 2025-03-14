// Answer 0

#[test]
fn test_decode_vec_standard_engine() {
    use base64::{Engine as _, engine::general_purpose};

    let mut buffer = Vec::<u8>::new();
    let result = general_purpose::STANDARD.decode_vec("aGVsbG8gd29ybGR+Cg==", &mut buffer);
    
    assert!(result.is_ok());
    assert_eq!(buffer, b"hello world\n");
}

#[test]
fn test_decode_vec_custom_engine() {
    use base64::{Engine as _, alphabet, engine::{self, general_purpose}};

    const CUSTOM_ENGINE: engine::GeneralPurpose =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::PAD);

    let mut buffer = Vec::<u8>::new();
    let result = CUSTOM_ENGINE.decode_vec("aGVsbG8gaW50ZXJuZXR-Cg==", &mut buffer);

    assert!(result.is_ok());
    assert_eq!(buffer, b"hello internets\n");
}

#[test]
fn test_decode_vec_empty_input() {
    use base64::{Engine as _, engine::general_purpose};

    let mut buffer = Vec::<u8>::new();
    let result = general_purpose::STANDARD.decode_vec("", &mut buffer);
    
    assert!(result.is_ok());
    assert_eq!(buffer.len(), 0);
}

#[test]
#[should_panic(expected = "Overflow when calculating output buffer length")]
fn test_decode_vec_overflow() {
    use base64::{Engine as _, engine::general_purpose};

    let mut buffer = vec![0; usize::MAX]; // Create a buffer that will cause overflow
    let _result = general_purpose::STANDARD.decode_vec("aGVsbG8gd29ybGR+Cg==", &mut buffer);
}

