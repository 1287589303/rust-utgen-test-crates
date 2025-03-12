// Answer 0

#[test]
fn test_error_invalid_number() {
    struct TestRead {
        position: Position,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { self.position }
        fn peek_position(&self) -> Position { self.position }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut read = TestRead { position: Position { line: 1, column: 0 } };
    let deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };

    deserializer.error(ErrorCode::InvalidNumber);
}

#[test]
fn test_error_expected_some_value() {
    struct TestRead {
        position: Position,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { self.position }
        fn peek_position(&self) -> Position { self.position }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut read = TestRead { position: Position { line: 1, column: 0 } };
    let deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };

    deserializer.error(ErrorCode::ExpectedSomeValue);
}

#[test]
fn test_error_number_out_of_range() {
    struct TestRead {
        position: Position,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { self.position }
        fn peek_position(&self) -> Position { self.position }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut read = TestRead { position: Position { line: 1, column: 0 } };
    let deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };

    deserializer.error(ErrorCode::NumberOutOfRange);
}

