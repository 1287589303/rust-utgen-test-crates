// Answer 0

#[test]
fn test_write_encoded_bytes_success() {
    struct MockEncoder {
        string: String,
    }

    impl MockEncoder {
        fn new() -> Self {
            MockEncoder {
                string: String::new(),
            }
        }
    }

    impl base64::WriteEncoded for MockEncoder {
        type Error = std::string::FromUtf8Error;

        fn write_encoded_bytes(&mut self, s: &[u8]) -> Result<(), Self::Error> {
            self.string.push_str(std::str::from_utf8(s).unwrap());
            Ok(())
        }
    }

    let mut encoder = MockEncoder::new();
    let result = encoder.write_encoded_bytes(b"Hello, World!");
    
    assert!(result.is_ok());
    assert_eq!(encoder.string, "Hello, World!");
}

#[test]
#[should_panic(expected = "invalid utf-8 sequence")]
fn test_write_encoded_bytes_invalid_utf8() {
    struct MockEncoder {
        string: String,
    }

    impl MockEncoder {
        fn new() -> Self {
            MockEncoder {
                string: String::new(),
            }
        }
    }

    impl base64::WriteEncoded for MockEncoder {
        type Error = std::string::FromUtf8Error;

        fn write_encoded_bytes(&mut self, s: &[u8]) -> Result<(), Self::Error> {
            self.string.push_str(std::str::from_utf8(s).unwrap());
            Ok(())
        }
    }

    let mut encoder = MockEncoder::new();
    encoder.write_encoded_bytes(&[0, 159, 146, 150]).unwrap(); // Invalid UTF-8
}

