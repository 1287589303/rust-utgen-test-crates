// Answer 0

#[test]
fn test_parse_unicode_escape_invalid_hex() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }
    
    impl MockRead {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<i16> {
            // Simulate decoding an invalid hex escape
            Err(Error::new(ErrorCode::InvalidEscape))
        }
        
        fn peek(&self) -> Option<u8> {
            if self.index < self.input.len() {
                Some(self.input[self.index])
            } else {
                None
            }
        }

        fn discard(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }
    }

    let mut read = MockRead::new(vec![b'u']); // Simulate a \u input scenario
    let validate = true;
    let mut scratch = Vec::new();

    let _ = parse_unicode_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_invalid_hex_out_of_range() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }
    
    impl MockRead {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            // Simulate decoding a hex escape that is out of range
            Ok(0xFFFF + 1) // Out of valid Unicode range for a hex escape
        }
        
        fn peek(&self) -> Option<u8> {
            if self.index < self.input.len() {
                Some(self.input[self.index])
            } else {
                None
            }
        }

        fn discard(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }
    }
    
    let mut read = MockRead::new(vec![b'u']);
    let validate = true;
    let mut scratch = Vec::new();

    let _ = parse_unicode_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_unexpected_end() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }
    
    impl MockRead {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<i16> {
            // Simulate a proper hex escape decoding
            Ok(0xD800) // Leading surrogate
        }
        
        fn peek(&self) -> Option<u8> {
            if self.index < self.input.len() {
                Some(self.input[self.index])
            } else {
                None
            }
        }

        fn discard(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }
    }

    let mut read = MockRead::new(vec![b'u']);
    let validate = true;
    let mut scratch = Vec::new();

    let _ = parse_unicode_escape(&mut read, validate, &mut scratch);
}

