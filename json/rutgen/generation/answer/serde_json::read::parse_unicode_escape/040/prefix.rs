// Answer 0

#[test]
fn test_parse_unicode_escape_with_leading_surrogate_pair() {
    struct MockRead {
        index: usize,
        hex_values: Vec<u16>,
        peek_values: Vec<u8>,
    }

    impl MockRead {
        fn new(hex_values: Vec<u16>, peek_values: Vec<u8>) -> Self {
            Self { index: 0, hex_values, peek_values }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.hex_values.len() {
                let value = self.hex_values[self.index];
                self.index += 1;
                Ok(value)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.peek_values.len() {
                Ok(Some(self.peek_values[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }
    }

    impl Read<'_> for MockRead {}

    let mut scratch = Vec::new();
    let mut reader = MockRead::new(vec![0xD800, 0xDC00], vec![b'A', b'\\', b'u']);
    let validate = false;

    let _ = parse_unicode_escape(&mut reader, validate, &mut scratch);
} 

#[test]
fn test_parse_unicode_escape_with_trailing_surrogate() {
    struct MockRead {
        index: usize,
        hex_values: Vec<u16>,
        peek_values: Vec<u8>,
    }

    impl MockRead {
        fn new(hex_values: Vec<u16>, peek_values: Vec<u8>) -> Self {
            Self { index: 0, hex_values, peek_values }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.hex_values.len() {
                let value = self.hex_values[self.index];
                self.index += 1;
                Ok(value)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.peek_values.len() {
                Ok(Some(self.peek_values[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }
    }

    impl Read<'_> for MockRead {}

    let mut scratch = Vec::new();
    let mut reader = MockRead::new(vec![0xDBFF, 0xD800], vec![b'A', b'\\', b'u']);
    let validate = false;

    let _ = parse_unicode_escape(&mut reader, validate, &mut scratch);
} 

#[test]
fn test_parse_unicode_escape_with_direct_appending() {
    struct MockRead {
        index: usize,
        hex_values: Vec<u16>,
        peek_values: Vec<u8>,
    }

    impl MockRead {
        fn new(hex_values: Vec<u16>, peek_values: Vec<u8>) -> Self {
            Self { index: 0, hex_values, peek_values }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.hex_values.len() {
                let value = self.hex_values[self.index];
                self.index += 1;
                Ok(value)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.peek_values.len() {
                Ok(Some(self.peek_values[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }
    }

    impl Read<'_> for MockRead {}

    let mut scratch = Vec::new();
    let mut reader = MockRead::new(vec![0xD800, 0xDC01], vec![b'A', b'\\', b'u']);
    let validate = false;

    let _ = parse_unicode_escape(&mut reader, validate, &mut scratch);
} 

#[test]
fn test_parse_unicode_escape_with_invalid_next() {
    struct MockRead {
        index: usize,
        hex_values: Vec<u16>,
        peek_values: Vec<u8>,
    }

    impl MockRead {
        fn new(hex_values: Vec<u16>, peek_values: Vec<u8>) -> Self {
            Self { index: 0, hex_values, peek_values }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.hex_values.len() {
                let value = self.hex_values[self.index];
                self.index += 1;
                Ok(value)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.peek_values.len() {
                Ok(Some(self.peek_values[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }
    }

    impl Read<'_> for MockRead {}

    let mut scratch = Vec::new();
    let mut reader = MockRead::new(vec![0xD800, 0xDACC], vec![b'A', b'\\', b'/']);
    let validate = false;

    let _ = parse_unicode_escape(&mut reader, validate, &mut scratch);
}

