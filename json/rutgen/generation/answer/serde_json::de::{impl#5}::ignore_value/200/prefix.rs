// Answer 0

#[test]
fn test_ignore_value_with_true() {
    struct TestRead {
        input: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn position(&self) -> Position {
            Position::new(0, self.pos)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.pos)
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut test_read = TestRead { input: b" true ".to_vec(), pos: 0 };
    let mut deserializer = Deserializer { read: test_read, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.ignore_value().unwrap();
}

#[test]
fn test_ignore_value_with_false() {
    struct TestRead {
        input: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn position(&self) -> Position {
            Position::new(0, self.pos)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.pos)
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut test_read = TestRead { input: b" false ".to_vec(), pos: 0 };
    let mut deserializer = Deserializer { read: test_read, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.ignore_value().unwrap();
}

#[test]
fn test_ignore_value_with_null() {
    struct TestRead {
        input: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn position(&self) -> Position {
            Position::new(0, self.pos)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.pos)
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut test_read = TestRead { input: b" null ".to_vec(), pos: 0 };
    let mut deserializer = Deserializer { read: test_read, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.ignore_value().unwrap();
}

#[test]
fn test_ignore_value_with_negative_integer() {
    struct TestRead {
        input: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn position(&self) -> Position {
            Position::new(0, self.pos)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.pos)
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut test_read = TestRead { input: b" -42 ".to_vec(), pos: 0 };
    let mut deserializer = Deserializer { read: test_read, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.ignore_value().unwrap();
}

#[test]
fn test_ignore_value_with_positive_integer() {
    struct TestRead {
        input: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn position(&self) -> Position {
            Position::new(0, self.pos)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.pos)
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut test_read = TestRead { input: b" 42 ".to_vec(), pos: 0 };
    let mut deserializer = Deserializer { read: test_read, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.ignore_value().unwrap();
}

#[test]
fn test_ignore_value_with_string() {
    struct TestRead {
        input: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn position(&self) -> Position {
            Position::new(0, self.pos)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.pos)
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut test_read = TestRead { input: b" \"Hello\" ".to_vec(), pos: 0 };
    let mut deserializer = Deserializer { read: test_read, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.ignore_value().unwrap();
}

#[test]
fn test_ignore_value_with_unexpected_character() {
    struct TestRead {
        input: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn position(&self) -> Position {
            Position::new(0, self.pos)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.pos)
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut test_read = TestRead { input: b" @ ".to_vec(), pos: 0 };
    let mut deserializer = Deserializer { read: test_read, scratch: Vec::new(), remaining_depth: 0 };
    let result = deserializer.ignore_value();
    match result {
        Err(_) => {}
        _ => panic!("Test failed, expected error."),
    }
}

