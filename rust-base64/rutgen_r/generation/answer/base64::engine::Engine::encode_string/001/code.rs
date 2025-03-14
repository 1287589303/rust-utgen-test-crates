// Answer 0

#[test]
fn test_encode_string_standard() {
    use base64::{Engine as _, engine::{self, general_purpose}};

    let mut output_buf = String::new();
    general_purpose::STANDARD.encode_string(b"test", &mut output_buf);
    
    assert_eq!(output_buf, "dGVzdA==");
}

#[test]
fn test_encode_string_custom() {
    use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

    const CUSTOM_ENGINE: engine::GeneralPurpose =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

    let mut output_buf = String::new();
    CUSTOM_ENGINE.encode_string(b"test url", &mut output_buf);
    
    assert_eq!(output_buf, "dGVzdCB1cmw"); // base64 encoded URL-safe
}

#[test]
fn test_encode_string_empty() {
    use base64::{Engine as _, engine::{self, general_purpose}};

    let mut output_buf = String::new();
    general_purpose::STANDARD.encode_string(b"", &mut output_buf);
    
    assert_eq!(output_buf, ""); // empty input should produce empty output
}

#[test]
fn test_encode_string_special_characters() {
    use base64::{Engine as _, engine::{self, general_purpose}};

    let mut output_buf = String::new();
    general_purpose::STANDARD.encode_string(b"hello, world!", &mut output_buf);
    
    assert_eq!(output_buf, "aGVsbG8sIHdvcmxkIQ==");
}

