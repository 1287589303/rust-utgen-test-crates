// Answer 0

#[test]
fn test_parse_unicode_escape_leading_surrogate() {
    struct MockRead {
        hex_value: Option<u16>,
        can_peek: bool,
    }
    
    impl MockRead {
        fn new(hex_value: u16, can_peek: bool) -> Self {
            Self {
                hex_value: Some(hex_value),
                can_peek,
            }
        }
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            self.hex_value.take().ok_or(Error::new(ErrorCode::InvalidNumber))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.can_peek {
                Ok(Some(b'\\'))
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn discard(&mut self) {}
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(0xD800, false);
    let result = parse_unicode_escape(&mut read, false, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_trailing_surrogate() {
    struct MockRead {
        hex_value: Option<u16>,
        can_peek: bool,
    }
    
    impl MockRead {
        fn new(hex_value: u16, can_peek: bool) -> Self {
            Self {
                hex_value: Some(hex_value),
                can_peek,
            }
        }
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            self.hex_value.take().ok_or(Error::new(ErrorCode::InvalidNumber))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.can_peek {
                Ok(Some(b'\\'))
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn discard(&mut self) {}
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(0xDBFF, false);
    let result = parse_unicode_escape(&mut read, false, &mut scratch);
}

