// Answer 0

#[test]
fn test_ignore_integer_valid() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position { /* ... */ }
        fn peek_position(&self) -> Position { /* ... */ }
        fn byte_offset(&self) -> usize {
            self.index
        }
        
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { /* ... */ }
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { /* ... */ }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
    }

    let reader = MockReader { data: vec![b'1', b'2', b'3', b'.'], index: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };

    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_leading_zero_error() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position { /* ... */ }
        fn peek_position(&self) -> Position { /* ... */ }
        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { /* ... */ }
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { /* ... */ }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
    }

    let reader = MockReader { data: vec![b'0', b'1'], index: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.ignore_integer();
    assert!(result.is_err());
}

#[test]
fn test_ignore_integer_invalid_characters() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position { /* ... */ }
        fn peek_position(&self) -> Position { /* ... */ }
        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { /* ... */ }
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { /* ... */ }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
    }

    let reader = MockReader { data: vec![b'2', b'3', b'#'], index: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.ignore_integer();
    assert!(result.is_err());
}

