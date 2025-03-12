// Answer 0

#[test]
fn test_scan_integer128_valid_input() {
    struct TestRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }
        
        fn position(&self) -> Position {
            // Stub implementation.
            Position::default()
        }
        
        fn peek_position(&self) -> Position {
            // Stub implementation.
            Position::default()
        }
        
        fn byte_offset(&self) -> usize {
            self.index
        }
        
        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }
        
        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            todo!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            todo!()
        }
        
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let input = TestRead { input: vec![b'1', b'2', b'3', b'4'], index: 0 };
    let mut deserializer = Deserializer { read: input, scratch: vec![], remaining_depth: 0 };
    let mut buf = String::new();
    let _ = deserializer.scan_integer128(&mut buf);
}

#[test]
fn test_scan_integer128_invalid_leading_zero() {
    struct TestRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            todo!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            todo!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let input = TestRead { input: vec![b'0', b'1'], index: 0 };
    let mut deserializer = Deserializer { read: input, scratch: vec![], remaining_depth: 0 };
    let mut buf = String::new();
    let _ = deserializer.scan_integer128(&mut buf);
}

#[test]
fn test_scan_integer128_invalid_character() {
    struct TestRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            todo!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            todo!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let input = TestRead { input: vec![b'a'], index: 0 };
    let mut deserializer = Deserializer { read: input, scratch: vec![], remaining_depth: 0 };
    let mut buf = String::new();
    let _ = deserializer.scan_integer128(&mut buf);
}

