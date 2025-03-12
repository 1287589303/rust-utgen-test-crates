// Answer 0

#[test]
fn test_parse_unicode_escape_valid_lower_range() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            if self.position < self.input.len() {
                let val = self.input[self.position] as i16;
                self.position += 1;
                Ok(val)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn peek(&self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }
    }

    impl Read<'_> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<i16> {
            self.decode_hex_escape()
        }

        fn peek(&self) -> Result<Option<u8>> {
            self.peek()
        }

        fn discard(&mut self) {
            self.discard();
        }
    }

    let mut scratch = Vec::new();
    let mut mock_reader = MockRead::new(vec![0x7A, b'u']); // 0x7A is valid (n < 0xD800)

    let result = parse_unicode_escape(&mut mock_reader, false, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_valid_surrogate() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            if self.position < self.input.len() {
                let val = self.input[self.position] as i16;
                self.position += 1;
                Ok(val)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn peek(&self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }
    }

    impl Read<'_> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<i16> {
            self.decode_hex_escape()
        }

        fn peek(&self) -> Result<Option<u8>> {
            self.peek()
        }

        fn discard(&mut self) {
            self.discard();
        }
    }

    let mut scratch = Vec::new();
    let mut mock_reader = MockRead::new(vec![0x7A, b'\\', b'u']); // 0x7A valid, then an escape

    let result = parse_unicode_escape(&mut mock_reader, false, &mut scratch);
}

