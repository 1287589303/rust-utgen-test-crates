// Answer 0

#[test]
fn test_unicode_escape_leading_surrogate() {
    struct MockRead {
        hex_value: u16,
        next_char: u8,
    }
    
    impl MockRead {
        fn decode_hex_escape(&self) -> Result<u16> {
            Ok(self.hex_value)
        }

        fn peek(&self) -> Result<Option<u8>> {
            Ok(Some(self.next_char))
        }

        fn discard(&mut self) {}
    }
    
    let mut scratch = Vec::new();

    let mut read = MockRead {
        hex_value: 0xD800, // Leading surrogate
        next_char: b'\\',   // Valid escape character
    };

    let _ = parse_unicode_escape(&mut read, false, &mut scratch);
}

#[test]
fn test_unicode_escape_trailing_surrogate() {
    struct MockRead {
        hex_value: u16,
        next_char: u8,
    }
    
    impl MockRead {
        fn decode_hex_escape(&self) -> Result<u16> {
            Ok(self.hex_value)
        }

        fn peek(&self) -> Result<Option<u8>> {
            Ok(Some(self.next_char))
        }

        fn discard(&mut self) {}
    }
    
    let mut scratch = Vec::new();

    let mut read = MockRead {
        hex_value: 0xDBFF, // Trailing surrogate
        next_char: b'\\',   // Valid escape character
    };

    let _ = parse_unicode_escape(&mut read, false, &mut scratch);
}

