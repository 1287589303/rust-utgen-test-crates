// Answer 0

#[test]
fn test_ignore_str_success_case() {
    struct TestRead {
        data: Vec<u8>,
        position: usize,
        escaped: bool,
    }

    impl TestRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0, escaped: false }
        }
    }

    impl Read<'static> for TestRead {
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

        fn position(&self) -> Position {
            Position { line: 1, column: 1, byte_offset: self.position }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Err(Error::from(ErrorCode::ExpectedSomeValue))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Err(Error::from(ErrorCode::ExpectedSomeValue))
        }

        fn ignore_str(&mut self) -> Result<()> {
            loop {
                let ch = tri!(self.next());
                if !is_escape(ch, true) {
                    continue;
                }
                match ch {
                    b'"' => {
                        return Ok(());
                    }
                    b'\\' => {
                        tri!(self.next());
                    }
                    _ => {
                        return Err(Error::from(ErrorCode::ControlCharacterWhileParsingString));
                    }
                }
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::from(ErrorCode::ExpectedSomeValue))
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestRead::new(vec![b'\\', b'"']);
    let _ = reader.ignore_str();
}

#[test]
#[should_panic]
fn test_ignore_str_error_case() {
    struct TestRead {
        data: Vec<u8>,
        position: usize,
    }

    impl TestRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Read<'static> for TestRead {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                if byte < 0x20 {
                    return Err(Error::from(ErrorCode::ControlCharacterWhileParsingString));
                }
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
            Position { line: 1, column: 1, byte_offset: self.position }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Err(Error::from(ErrorCode::ExpectedSomeValue))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Err(Error::from(ErrorCode::ExpectedSomeValue))
        }

        fn ignore_str(&mut self) -> Result<()> {
            loop {
                let ch = tri!(self.next());
                if !is_escape(ch, true) {
                    continue;
                }
                match ch {
                    b'"' => {
                        return Ok(());
                    }
                    b'\\' => {
                        tri!(self.next());
                    }
                    _ => {
                        return Err(Error::from(ErrorCode::ControlCharacterWhileParsingString));
                    }
                }
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::from(ErrorCode::ExpectedSomeValue))
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestRead::new(vec![b'\\', b'\x01']);
    let _ = reader.ignore_str();
}

