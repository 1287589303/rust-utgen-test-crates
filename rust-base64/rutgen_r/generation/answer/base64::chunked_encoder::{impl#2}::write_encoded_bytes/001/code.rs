// Answer 0

#[test]
fn test_write_encoded_bytes_empty() {
    struct ChunkedEncoder {
        string: String,
    }

    impl ChunkedEncoder {
        fn new() -> Self {
            ChunkedEncoder { string: String::new() }
        }

        fn write_encoded_bytes(&mut self, s: &[u8]) -> Result<(), ()> {
            self.string.push_str(std::str::from_utf8(s).unwrap());
            Ok(())
        }
    }

    let mut encoder = ChunkedEncoder::new();
    let result = encoder.write_encoded_bytes(b"");
    assert_eq!(result, Ok(()));
}

#[test]
fn test_write_encoded_bytes_simple_string() {
    struct ChunkedEncoder {
        string: String,
    }

    impl ChunkedEncoder {
        fn new() -> Self {
            ChunkedEncoder { string: String::new() }
        }

        fn write_encoded_bytes(&mut self, s: &[u8]) -> Result<(), ()> {
            self.string.push_str(std::str::from_utf8(s).unwrap());
            Ok(())
        }
    }

    let mut encoder = ChunkedEncoder::new();
    let result = encoder.write_encoded_bytes(b"Hello");
    assert_eq!(result, Ok(()));
}

#[test]
fn test_write_encoded_bytes_complex_string() {
    struct ChunkedEncoder {
        string: String,
    }

    impl ChunkedEncoder {
        fn new() -> Self {
            ChunkedEncoder { string: String::new() }
        }

        fn write_encoded_bytes(&mut self, s: &[u8]) -> Result<(), ()> {
            self.string.push_str(std::str::from_utf8(s).unwrap());
            Ok(())
        }
    }

    let mut encoder = ChunkedEncoder::new();
    let result = encoder.write_encoded_bytes(b"Base64 Encoding");
    assert_eq!(result, Ok(()));
}

#[test]
fn test_write_encoded_bytes_unicode() {
    struct ChunkedEncoder {
        string: String,
    }

    impl ChunkedEncoder {
        fn new() -> Self {
            ChunkedEncoder { string: String::new() }
        }

        fn write_encoded_bytes(&mut self, s: &[u8]) -> Result<(), ()> {
            self.string.push_str(std::str::from_utf8(s).unwrap());
            Ok(())
        }
    }

    let mut encoder = ChunkedEncoder::new();
    let result = encoder.write_encoded_bytes("こんにちは".as_bytes());
    assert_eq!(result, Ok(()));
}

