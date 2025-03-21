// Answer 0

#[test]
fn test_ignore_decimal_with_digits() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for TestReader {
        const should_early_return_if_failed: bool = false;

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

        fn position(&self) -> Position {
            // Dummy implementation, not relevant for this test
            Position::default()
        }

        fn peek_position(&self) -> Position {
            // Dummy implementation, not relevant for this test
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            self.index
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

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = TestReader {
        data: b"123e".to_vec(), // Valid digits to trigger the logic
        index: 0,
    };

    let mut deserializer = Deserializer {
        read: reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    // First consume the digit '1', then '2', and '3'
    deserializer.ignore_decimal().unwrap();

    // Now we will peek and get an error
    let result = deserializer.ignore_decimal();
    assert!(result.is_err());
}

#[test]
fn test_ignore_decimal_no_digits() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for TestReader {
        const should_early_return_if_failed: bool = false;

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

        fn position(&self) -> Position {
            Position::default()  // Dummy implementation
        }

        fn peek_position(&self) -> Position {
            Position::default()  // Dummy implementation
        }

        fn byte_offset(&self) -> usize {
            self.index
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

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = TestReader {
        data: b"abc".to_vec(), // No digits for invalid case
        index: 0,
    };

    let mut deserializer = Deserializer {
        read: reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.ignore_decimal();
    // The expectation is that the function returns an error
    assert!(result.is_err());
}

