// Answer 0

#[test]
fn test_parse_unicode_escape_valid_case() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.position < self.input.len() {
                let value = self.input[self.position];
                self.position += 1;
                Ok(value as u32)
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }
    }

    // Instantiate the mock read device
    let mut read = MockRead::new(vec![0xD800, b'\\', b'u', 0xDC00].into_iter().map(|v| v as u8).collect());
    let validate = false;
    let mut scratch = Vec::new();

    // Call the function under test
    let _ = parse_unicode_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_lone_surrogate() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.position < self.input.len() {
                let value = self.input[self.position];
                self.position += 1;
                Ok(value as u32)
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }
    }

    // Instantiate the mock read device
    let mut read = MockRead::new(vec![0xD800, b'\\', b'u', 0xDFFF].into_iter().map(|v| v as u8).collect());
    let validate = false;
    let mut scratch = Vec::new();

    // Call the function under test
    let _ = parse_unicode_escape(&mut read, validate, &mut scratch);
}

