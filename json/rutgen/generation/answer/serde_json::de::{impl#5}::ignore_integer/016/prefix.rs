// Answer 0

#[test]
fn test_ignore_integer_leading_zero_invalid() {
    struct MockRead {
        chars: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.chars.len() {
                let val = self.chars[self.pos];
                self.pos += 1;
                Ok(Some(val))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.chars.len() {
                Ok(Some(self.chars[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.pos += 1;
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.pos as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.pos as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock_read = MockRead { chars: vec![b'0', b'0'], pos: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: Vec::new(), remaining_depth: 0 };

    let _result = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_valid() {
    struct MockRead {
        chars: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.chars.len() {
                let val = self.chars[self.pos];
                self.pos += 1;
                Ok(Some(val))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.chars.len() {
                Ok(Some(self.chars[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.pos += 1;
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.pos as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.pos as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock_read = MockRead { chars: vec![b'1', b'2', b'.'], pos: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: Vec::new(), remaining_depth: 0 };

    let _result = deserializer.ignore_integer();
}

