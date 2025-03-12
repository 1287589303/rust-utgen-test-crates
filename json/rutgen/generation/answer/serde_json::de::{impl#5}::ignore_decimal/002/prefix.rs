// Answer 0

#[test]
fn test_ignore_decimal_with_valid_number() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
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

        fn position(&self) -> Position {
            Position::new(self.position, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position, 0)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, failed: &mut bool) {}
    }

    let mut mock_reader = MockReader {
        data: b"123e456".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let _ = deserializer.ignore_decimal();
}

#[test]
#[should_panic]
fn test_ignore_decimal_with_no_digits() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
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

        fn position(&self) -> Position {
            Position::new(self.position, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position, 0)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, failed: &mut bool) {}
    }

    let mut mock_reader = MockReader {
        data: b"ee456".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let _ = deserializer.ignore_decimal();
}

