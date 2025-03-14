// Answer 0

#[test]
fn test_encode_standard() {
    struct MockEngine;
    
    impl base64::Engine for MockEngine {
        fn config(&self) -> &base64::engine::Config {
            &base64::engine::general_purpose::NO_PAD
        }
    }
    
    let engine = MockEngine;
    let encoded = engine.encode(b"hello world~");
    assert_eq!(encoded, "aGVsbG8gd29ybGQ~");
}

#[test]
fn test_encode_url_safe() {
    struct MockEngine;
    
    impl base64::Engine for MockEngine {
        fn config(&self) -> &base64::engine::Config {
            &base64::engine::general_purpose::URL_SAFE
        }
    }
    
    let engine = MockEngine;
    let encoded = engine.encode(b"hello internet~");
    assert_eq!(encoded, "aGVsbG8gaW50ZXJuZXQ~");
}

#[test]
fn test_encode_empty_input() {
    struct MockEngine;

    impl base64::Engine for MockEngine {
        fn config(&self) -> &base64::engine::Config {
            &base64::engine::general_purpose::NO_PAD
        }
    }
    
    let engine = MockEngine;
    let encoded = engine.encode(b"");
    assert_eq!(encoded, "");
}

#[should_panic]
#[test]
fn test_encode_invalid_utf8() {
    struct MockEngine;

    impl base64::Engine for MockEngine {
        fn config(&self) -> &base64::engine::Config {
            &base64::engine::general_purpose::NO_PAD
        }
    }

    let engine = MockEngine;
    let _ = engine.encode(b"\xFF\xFF"); // This should panic due to invalid UTF-8
}

