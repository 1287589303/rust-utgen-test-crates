// Answer 0

#[test]
fn test_discard_empty_buffer() {
    struct TestReader {
        buffer: Vec<u8>,
    }
    
    impl<'de> Read<'de> for TestReader {
        const should_early_return_if_failed: bool = false;
        
        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) { self.buffer.clear(); }
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, failed: &mut bool) { *failed = false; }
    }

    let mut reader = TestReader { buffer: Vec::new() };
    reader.discard();
}

#[test]
fn test_discard_filled_buffer() {
    struct TestReader {
        buffer: Vec<u8>,
    }
    
    impl<'de> Read<'de> for TestReader {
        const should_early_return_if_failed: bool = false;
        
        fn next(&mut self) -> Result<Option<u8>> { Ok(self.buffer.pop()) }
        fn peek(&mut self) -> Result<Option<u8>> { self.buffer.last().cloned().map(Ok).unwrap_or_else(|| Ok(None)) }
        fn discard(&mut self) { self.buffer.clear(); }
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.buffer.len() }
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, failed: &mut bool) { *failed = false; }
    }

    let mut reader = TestReader { buffer: vec![1, 2, 3] };
    reader.discard();
}

#[test]
fn test_discard_large_buffer() {
    struct TestReader {
        buffer: Vec<u8>,
    }
    
    impl<'de> Read<'de> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(self.buffer.pop()) }
        fn peek(&mut self) -> Result<Option<u8>> { self.buffer.last().cloned().map(Ok).unwrap_or_else(|| Ok(None)) }
        fn discard(&mut self) { self.buffer.clear(); }
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.buffer.len() }
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, failed: &mut bool) { *failed = false; }
    }

    let mut reader = TestReader { buffer: (0..10_000).collect() };
    reader.discard();
}

