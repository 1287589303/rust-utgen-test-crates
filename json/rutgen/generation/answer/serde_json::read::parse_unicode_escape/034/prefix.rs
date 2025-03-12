// Answer 0

#[test]
fn test_parse_unicode_escape_with_bound_n_d800() {
    struct MockRead {
        hex_escape_val: Result<u16, Error>,
        peek_val: Result<u8, Error>,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            self.hex_escape_val.clone()
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            self.peek_val.clone().map(Some)
        }

        fn discard(&mut self) { /* no-op */ }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead {
        hex_escape_val: Ok(0xD800), // n == 0xD800
        peek_val: Err(Error::from(ErrorCode::EofWhileParsingString)), // EOF on peek
    };

    let _ = parse_unicode_escape(&mut read, false, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_with_bound_n_dbff() {
    struct MockRead {
        hex_escape_val: Result<u16, Error>,
        peek_val: Result<u8, Error>,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            self.hex_escape_val.clone()
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            self.peek_val.clone().map(Some)
        }

        fn discard(&mut self) { /* no-op */ }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead {
        hex_escape_val: Ok(0xDBFF), // n == 0xDBFF
        peek_val: Err(Error::from(ErrorCode::EofWhileParsingString)), // EOF on peek
    };

    let _ = parse_unicode_escape(&mut read, false, &mut scratch);
}

