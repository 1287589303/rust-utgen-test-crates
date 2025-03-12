// Answer 0

#[test]
fn test_parse_unicode_escape_with_leading_surrogate() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            MockRead { data, pos: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.pos + 4 <= self.data.len() {
                let hex = str::from_utf8(&self.data[self.pos..self.pos + 4]).unwrap();
                self.pos += 4;
                u16::from_str_radix(hex, 16).map_err(|_| Error::from(ErrorCode::InvalidEscape)).map(|v| v as u16)
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&self) -> Option<u8> {
            self.data.get(self.pos).copied()
        }

        fn discard(&mut self) {
            self.pos += 1;
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(b"DC00\\uD8FF".to_vec()); // Input containing 0xDC00 as first hexadecimal escape
    let validate = true;

    let _ = parse_unicode_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_with_trailing_surrogate() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            MockRead { data, pos: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.pos + 4 <= self.data.len() {
                let hex = str::from_utf8(&self.data[self.pos..self.pos + 4]).unwrap();
                self.pos += 4;
                u16::from_str_radix(hex, 16).map_err(|_| Error::from(ErrorCode::InvalidEscape)).map(|v| v as u16)
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&self) -> Option<u8> {
            self.data.get(self.pos).copied()
        }

        fn discard(&mut self) {
            self.pos += 1;
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(b"DC00\\uDFFF".to_vec()); // Input containing 0xDFFF as second hexadecimal escape
    let validate = true;

    let _ = parse_unicode_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_with_invalid_leading_surrogate() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            MockRead { data, pos: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.pos + 4 <= self.data.len() {
                let hex = str::from_utf8(&self.data[self.pos..self.pos + 4]).unwrap();
                self.pos += 4;
                u16::from_str_radix(hex, 16).map_err(|_| Error::from(ErrorCode::InvalidEscape)).map(|v| v as u16)
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&self) -> Option<u8> {
            self.data.get(self.pos).copied()
        }

        fn discard(&mut self) {
            self.pos += 1;
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(b"DC00\\uD800".to_vec()); // Improper pair, leading surrogate
    let validate = true;

    let _ = parse_unicode_escape(&mut read, validate, &mut scratch);
}

