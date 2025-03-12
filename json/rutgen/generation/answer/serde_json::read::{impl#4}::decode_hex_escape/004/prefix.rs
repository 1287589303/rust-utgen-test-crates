// Answer 0

#[test]
fn test_decode_hex_escape_success() {
    struct MockIoRead {
        data: Vec<u8>,
        position: usize,
    }
    
    impl<'de> Read<'de> for MockIoRead {
        const should_early_return_if_failed: bool = true;
        
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
        
        fn position(&self) -> Position { unimplemented!() }
        
        fn peek_position(&self) -> Position { unimplemented!() }
        
        fn byte_offset(&self) -> usize { self.position }
        
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            let a = tri!(self.next()?);
            let b = tri!(self.next()?);
            let c = tri!(self.next()?);
            let d = tri!(self.next()?);
            match decode_four_hex_digits(a, b, c, d) {
                Some(val) => Ok(val),
                None => error(self, ErrorCode::InvalidEscape),
            }
        }
        
        fn set_failed(&mut self, _failed: &mut bool) {}
    }
    
    let mut mock_reader = MockIoRead {
        data: vec![b'1', b'a', b'F', b'0'],
        position: 0,
    };
    let _ = mock_reader.decode_hex_escape();
}

#[test]
#[should_panic]
fn test_decode_hex_escape_failure() {
    struct MockIoRead {
        data: Vec<u8>,
        position: usize,
    }
    
    impl<'de> Read<'de> for MockIoRead {
        const should_early_return_if_failed: bool = true;

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
        
        fn position(&self) -> Position { unimplemented!() }
        
        fn peek_position(&self) -> Position { unimplemented!() }
        
        fn byte_offset(&self) -> usize { self.position }
        
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            let a = tri!(self.next()?);
            let b = tri!(self.next()?);
            let c = tri!(self.next()?);
            let d = tri!(self.next()?);
            match decode_four_hex_digits(a, b, c, d) {
                Some(val) => Ok(val),
                None => error(self, ErrorCode::InvalidEscape),
            }
        }
        
        fn set_failed(&mut self, _failed: &mut bool) {}
    }
    
    let mut mock_reader = MockIoRead {
        data: vec![b'1', b'G'], // 'G' is not a valid hex digit
        position: 0,
    };
    let _ = mock_reader.decode_hex_escape();
}

