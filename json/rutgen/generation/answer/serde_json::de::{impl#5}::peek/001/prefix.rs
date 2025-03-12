// Answer 0

#[test]
fn test_peek_empty_input() {
    struct EmptyReader;
    
    impl<'de> Read<'de> for EmptyReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::new(0) }

        fn peek_position(&self) -> Position { Position::new(0) }

        fn byte_offset(&self) -> usize { 0 }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }

        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = EmptyReader;
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.peek().unwrap();
}

#[test]
fn test_peek_single_byte_input() {
    struct SingleByteReader {
        byte: Option<u8>,
    }

    impl<'de> Read<'de> for SingleByteReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            self.byte.take().map(|b| Ok(Some(b))).unwrap_or(Ok(None))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(self.byte)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::new(0) }

        fn peek_position(&self) -> Position { Position::new(0) }

        fn byte_offset(&self) -> usize { 0 }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }

        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = SingleByteReader { byte: Some(42) };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.peek().unwrap();
}

#[test]
fn test_peek_max_byte_input() {
    struct MaxByteReader {
        byte: Option<u8>,
    }

    impl<'de> Read<'de> for MaxByteReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            self.byte.take().map(|b| Ok(Some(b))).unwrap_or(Ok(None))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(self.byte)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::new(0) }

        fn peek_position(&self) -> Position { Position::new(0) }

        fn byte_offset(&self) -> usize { 0 }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }

        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MaxByteReader { byte: Some(u8::MAX) };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.peek().unwrap();
}

