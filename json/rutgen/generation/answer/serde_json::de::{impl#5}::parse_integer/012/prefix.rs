// Answer 0

#[test]
fn test_parse_integer_leading_zero() {
    struct MockReader {
        next_chars: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(chars: Vec<u8>) -> Self {
            Self { next_chars: chars, position: 0 }
        }
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.next_chars.len() {
                let ch = self.next_chars[self.position];
                self.position += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.next_chars.len() {
                Ok(Some(self.next_chars[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mut reader = MockReader::new(vec![b'0', b'0']);
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let result = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_valid_overflow() {
    struct MockReader {
        next_chars: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(chars: Vec<u8>) -> Self {
            Self { next_chars: chars, position: 0 }
        }
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.next_chars.len() {
                let ch = self.next_chars[self.position];
                self.position += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.next_chars.len() {
                Ok(Some(self.next_chars[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mut reader = MockReader::new(vec![b'9', b'9', b'9', b'9', b'9', b'9', b'9', b'9', b'9', b'9']); // Input generating overflow
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let result = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_invalid_non_digit() {
    struct MockReader {
        next_chars: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(chars: Vec<u8>) -> Self {
            Self { next_chars: chars, position: 0 }
        }
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.next_chars.len() {
                let ch = self.next_chars[self.position];
                self.position += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.next_chars.len() {
                Ok(Some(self.next_chars[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mut reader = MockReader::new(vec![b'1', b'a']); // Input generates invalid number
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let result = deserializer.parse_integer(true);
}

