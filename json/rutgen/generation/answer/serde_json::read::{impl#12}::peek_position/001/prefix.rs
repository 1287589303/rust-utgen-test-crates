// Answer 0

#[test]
fn test_peek_position_initial() {
    struct TestReader {
        position: Position,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;
        
        fn next(&mut self) -> Result<Option<u8>> {
            unimplemented!()
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            unimplemented!()
        }
        
        fn discard(&mut self) {}
        
        fn position(&self) -> Position {
            self.position
        }
        
        fn peek_position(&self) -> Position {
            self.position
        }
        
        fn byte_offset(&self) -> usize {
            unimplemented!()
        }
        
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }
        
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
        
        fn begin_raw_buffering(&mut self) {}
        
        fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value> 
        where V: Visitor<'static> {
            unimplemented!()
        }
        
        fn set_failed(&mut self, failed: &mut bool) {}
    }

    let mut test_reader = TestReader {
        position: Position { line: 0, column: 0 }
    };
    
    let _position = test_reader.peek_position();
}

#[test]
fn test_peek_position_middle() {
    struct TestReader {
        position: Position,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;
        
        fn next(&mut self) -> Result<Option<u8>> {
            unimplemented!()
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            unimplemented!()
        }
        
        fn discard(&mut self) {}
        
        fn position(&self) -> Position {
            self.position
        }
        
        fn peek_position(&self) -> Position {
            self.position
        }
        
        fn byte_offset(&self) -> usize {
            unimplemented!()
        }
        
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }
        
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
        
        fn begin_raw_buffering(&mut self) {}
        
        fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value> 
        where V: Visitor<'static> {
            unimplemented!()
        }
        
        fn set_failed(&mut self, failed: &mut bool) {}
    }

    let mut test_reader = TestReader {
        position: Position { line: 10, column: 5 }
    };
    
    let _position = test_reader.peek_position();
}

#[test]
fn test_peek_position_end() {
    struct TestReader {
        position: Position,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;
        
        fn next(&mut self) -> Result<Option<u8>> {
            unimplemented!()
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            unimplemented!()
        }
        
        fn discard(&mut self) {}
        
        fn position(&self) -> Position {
            self.position
        }
        
        fn peek_position(&self) -> Position {
            self.position
        }
        
        fn byte_offset(&self) -> usize {
            unimplemented!()
        }
        
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }
        
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
        
        fn begin_raw_buffering(&mut self) {}
        
        fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value> 
        where V: Visitor<'static> {
            unimplemented!()
        }
        
        fn set_failed(&mut self, failed: &mut bool) {}
    }

    let mut test_reader = TestReader {
        position: Position { line: 50, column: 20 }
    };
    
    let _position = test_reader.peek_position();
}

