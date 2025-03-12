// Answer 0

#[test]
fn test_end_map_with_closing_brace() {
    struct MockReader;
    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> { Ok(Some(b'}')) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(Some(b'}')) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer { read: MockReader, scratch: Vec::new(), remaining_depth: 0 };
    let _ = deserializer.end_map();
}

#[test]
fn test_end_map_with_trailing_comma() {
    struct MockReader;
    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> { Ok(Some(b',')) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(Some(b',')) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer { read: MockReader, scratch: Vec::new(), remaining_depth: 0 };
    let _ = deserializer.end_map();
}

#[test]
fn test_end_map_with_other_byte() {
    struct MockReader;
    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> { Ok(Some(b'x')) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(Some(b'x')) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer { read: MockReader, scratch: Vec::new(), remaining_depth: 0 };
    let _ = deserializer.end_map();
}

#[test]
fn test_end_map_with_eof() {
    struct MockReader;
    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer { read: MockReader, scratch: Vec::new(), remaining_depth: 0 };
    let _ = deserializer.end_map();
}

