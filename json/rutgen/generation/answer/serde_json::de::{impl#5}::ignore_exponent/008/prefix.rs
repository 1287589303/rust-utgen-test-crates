// Answer 0

#[test]
fn test_ignore_exponent_positive_zero() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for TestReader {
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
            Position { line: 1, column: self.position as u64 }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn begin_raw_buffering(&mut self) { unimplemented!() }
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value> where V: Visitor<'static> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader { data: vec![b'e', b'+', b'0', b'1', b'2'], position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.ignore_exponent();
}

#[test]
fn test_ignore_exponent_negative() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for TestReader {
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
            Position { line: 1, column: self.position as u64 }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn begin_raw_buffering(&mut self) { unimplemented!() }
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value> where V: Visitor<'static> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader { data: vec![b'e', b'-', b'0', b'0', b'1'], position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.ignore_exponent();
}

#[test]
#[should_panic(expected = "InvalidNumber")]
fn test_ignore_exponent_invalid() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for TestReader {
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
            Position { line: 1, column: self.position as u64 }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn begin_raw_buffering(&mut self) { unimplemented!() }
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value> where V: Visitor<'static> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader { data: vec![b'e', b'0', b'0', b'0'], position: 0 }; // No '+' or '-' before digits
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.ignore_exponent();
}

