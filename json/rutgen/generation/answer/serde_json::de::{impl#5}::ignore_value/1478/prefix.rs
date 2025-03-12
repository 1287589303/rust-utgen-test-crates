// Answer 0

#[test]
fn test_ignore_value_valid_null() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
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
        fn discard(&mut self) {
            self.position += 1;
        }
        fn position(&self) -> Position {
            Position::default()
        }
        fn peek_position(&self) -> Position {
            Position::default()
        }
        fn byte_offset(&self) -> usize {
            self.position
        }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut read = MockRead { data: b"null".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };

    let _ = deserializer.ignore_value();
}

#[test]
fn test_ignore_value_valid_true() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
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
        fn discard(&mut self) {
            self.position += 1;
        }
        fn position(&self) -> Position {
            Position::default()
        }
        fn peek_position(&self) -> Position {
            Position::default()
        }
        fn byte_offset(&self) -> usize {
            self.position
        }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut read = MockRead { data: b"true".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };

    let _ = deserializer.ignore_value();
}

#[test]
fn test_ignore_value_invalid_character() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
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
        fn discard(&mut self) {
            self.position += 1;
        }
        fn position(&self) -> Position {
            Position::default()
        }
        fn peek_position(&self) -> Position {
            Position::default()
        }
        fn byte_offset(&self) -> usize {
            self.position
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

    let mut read = MockRead { data: b"[invalid".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };

    let _ = deserializer.ignore_value();
}

