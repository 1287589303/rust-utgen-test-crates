// Answer 0

#[test]
fn test_peek_invalid_type_with_n() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) { self.position += 1; }

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.position }
        
        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Ok(Reference::Borrowed("mock"))
        }
        
        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Ok(Reference::Borrowed(b"mock"))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input = vec![b'n'];
    let mut reader = TestRead { input, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.peek_invalid_type(&Expected::Any)
}


#[test]
fn test_peek_invalid_type_with_t() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) { self.position += 1; }

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Ok(Reference::Borrowed("mock"))
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Ok(Reference::Borrowed(b"mock"))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input = vec![b't'];
    let mut reader = TestRead { input, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.peek_invalid_type(&Expected::Any)
}


#[test]
fn test_peek_invalid_type_with_f() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) { self.position += 1; }

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Ok(Reference::Borrowed("mock"))
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Ok(Reference::Borrowed(b"mock"))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input = vec![b'f'];
    let mut reader = TestRead { input, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.peek_invalid_type(&Expected::Any)
}


#[test]
fn test_peek_invalid_type_with_minus() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) { self.position += 1; }

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Ok(Reference::Borrowed("mock"))
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Ok(Reference::Borrowed(b"mock"))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input = vec![b'-'];
    let mut reader = TestRead { input, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.peek_invalid_type(&Expected::Any)
}


#[test]
fn test_peek_invalid_type_with_quote() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) { self.position += 1; }

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Ok(Reference::Borrowed("mock"))
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Ok(Reference::Borrowed(b"mock"))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input = vec![b'"'];
    let mut reader = TestRead { input, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.peek_invalid_type(&Expected::Any)
}


#[test]
fn test_peek_invalid_type_with_left_bracket() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) { self.position += 1; }

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Ok(Reference::Borrowed("mock"))
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Ok(Reference::Borrowed(b"mock"))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input = vec![b'['];
    let mut reader = TestRead { input, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.peek_invalid_type(&Expected::Any)
}


#[test]
fn test_peek_invalid_type_with_left_brace() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) { self.position += 1; }

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Ok(Reference::Borrowed("mock"))
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Ok(Reference::Borrowed(b"mock"))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input = vec![b'{'];
    let mut reader = TestRead { input, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };
    deserializer.peek_invalid_type(&Expected::Any)
}

