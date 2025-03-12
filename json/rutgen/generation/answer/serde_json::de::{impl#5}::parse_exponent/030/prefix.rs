// Answer 0

#[test]
fn test_parse_exponent_positive_large_number() {
    struct DummyRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for DummyRead {
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

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
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

    let data = b"e+5";
    let mut reader = DummyRead { data: data.to_vec(), position: 0 };
    let mut scratch = Vec::new();
    let mut deserializer = Deserializer { read: reader, scratch, remaining_depth: 0 };

    let result = deserializer.parse_exponent(true, 1, -100);
}

#[test]
fn test_parse_exponent_negative_large_number() {
    struct DummyRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for DummyRead {
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

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
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

    let data = b"e-10";
    let mut reader = DummyRead { data: data.to_vec(), position: 0 };
    let mut scratch = Vec::new();
    let mut deserializer = Deserializer { read: reader, scratch, remaining_depth: 0 };

    let result = deserializer.parse_exponent(false, 1, -100);
}

#[test]
fn test_parse_exponent_zero_value() {
    struct DummyRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for DummyRead {
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

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
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

    let data = b"e0";
    let mut reader = DummyRead { data: data.to_vec(), position: 0 };
    let mut scratch = Vec::new();
    let mut deserializer = Deserializer { read: reader, scratch, remaining_depth: 0 };

    let result = deserializer.parse_exponent(true, 0, 0);
}

