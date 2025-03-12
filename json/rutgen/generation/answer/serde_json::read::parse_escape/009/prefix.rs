// Answer 0

#[test]
fn test_parse_escape_double_quote() {
    struct TestReader {
        input: Vec<u8>,
        cursor: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, cursor: 0 }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            // Simulate decoding a hex escape (not used in this case).
            Ok(0)
        }

        fn next(&mut self) -> Option<u8> {
            if self.cursor < self.input.len() {
                let byte = self.input[self.cursor];
                self.cursor += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn discard(&mut self) {}
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader::new(vec![b'"']);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
}

#[test]
fn test_parse_escape_backslash() {
    struct TestReader {
        input: Vec<u8>,
        cursor: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, cursor: 0 }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn next(&mut self) -> Option<u8> {
            if self.cursor < self.input.len() {
                let byte = self.input[self.cursor];
                self.cursor += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn discard(&mut self) {}
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader::new(vec![b'\\']);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
}

#[test]
fn test_parse_escape_forward_slash() {
    struct TestReader {
        input: Vec<u8>,
        cursor: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, cursor: 0 }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn next(&mut self) -> Option<u8> {
            if self.cursor < self.input.len() {
                let byte = self.input[self.cursor];
                self.cursor += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn discard(&mut self) {}
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader::new(vec![b'/']);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
}

