// Answer 0

#[test]
fn test_parse_object_colon_with_colon() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let result = Some(self.data[self.position]);
                self.position += 1;
                Ok(result)
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 0, column: 0 } }
        fn peek_position(&self) -> Position { Position { line: 0, column: 0 } }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { todo!() }
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { todo!() }
        fn ignore_str(&mut self) -> Result<()> { todo!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { todo!() }
    }

    let read = MockRead { data: vec![b' ', b':'], position: 0 };
    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 0 };

    let _ = deserializer.parse_object_colon();
}

#[test]
fn test_parse_object_colon_with_expected_colon_error() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let result = Some(self.data[self.position]);
                self.position += 1;
                Ok(result)
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 0, column: 0 } }
        fn peek_position(&self) -> Position { Position { line: 0, column: 0 } }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { todo!() }
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { todo!() }
        fn ignore_str(&mut self) -> Result<()> { todo!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { todo!() }
    }

    let read = MockRead { data: vec![b' ', b'a'], position: 0 };
    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.parse_object_colon();
    assert!(result.is_err());
}

#[test]
fn test_parse_object_colon_with_eof() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let result = Some(self.data[self.position]);
                self.position += 1;
                Ok(result)
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 0, column: 0 } }
        fn peek_position(&self) -> Position { Position { line: 0, column: 0 } }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { todo!() }
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { todo!() }
        fn ignore_str(&mut self) -> Result<()> { todo!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { todo!() }
    }

    let read = MockRead { data: vec![], position: 0 };
    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.parse_object_colon();
    assert!(result.is_err());
}

