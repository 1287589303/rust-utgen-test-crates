// Answer 0

#[test]
fn test_encode_slice_success() {
    struct MockEngine;

    impl base64::Engine for MockEngine {
        fn config(&self) -> &base64::Config {
            &base64::Config::new(base64::CharacterSet::Standard, true)
        }
    }

    let engine = MockEngine;
    let input = b"hello internet!";
    let mut buf = vec![0u8; input.len() * 4 / 3 + 4]; // Allocate buffer
    let result = engine.encode_slice(input, &mut buf).unwrap();
    
    buf.truncate(result); // Resize to fit the actual size written
    
    assert_eq!(input, base64::engine::general_purpose::STANDARD.decode(&buf).unwrap().as_slice());
}

#[test]
fn test_encode_slice_output_buffer_too_small() {
    struct MockEngine;

    impl base64::Engine for MockEngine {
        fn config(&self) -> &base64::Config {
            &base64::Config::new(base64::CharacterSet::Standard, true)
        }
    }

    let engine = MockEngine;
    let input = b"hello internet!";
    let mut buf = vec![0u8; 5]; // Intentionally too small
    
    let result = engine.encode_slice(input, &mut buf);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), base64::EncodeSliceError::OutputSliceTooSmall);
}

#[test]
fn test_encode_slice_empty_input() {
    struct MockEngine;

    impl base64::Engine for MockEngine {
        fn config(&self) -> &base64::Config {
            &base64::Config::new(base64::CharacterSet::Standard, true)
        }
    }

    let engine = MockEngine;
    let input: &[u8] = &[];
    let mut buf = vec![0u8; 4]; // Buffer for base64 encoding of 0 bytes is 0 bytes + padding
    
    let result = engine.encode_slice(input, &mut buf).unwrap();
    assert_eq!(result, 0);
    assert_eq!(&buf[..result], &[] as &[u8]); // Buffer should be empty
}

