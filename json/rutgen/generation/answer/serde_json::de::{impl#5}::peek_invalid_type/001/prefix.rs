// Answer 0

#[test]
fn test_peek_invalid_type_with_n() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }
    
    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                self.pos += 1;
                Ok(Some(self.data[self.pos - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.next().ok();
        }

        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
    }

    let mut read = MockRead {
        data: vec![b'n', b'u', b'l', b'l'],
        pos: 0,
    };

    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 0 };

    deserializer.peek_invalid_type(&Expected::new("unit"));
}

#[test]
fn test_peek_invalid_type_with_t() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                self.pos += 1;
                Ok(Some(self.data[self.pos - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.next().ok();
        }

        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
    }

    let mut read = MockRead {
        data: vec![b't', b'r', b'u', b'e'],
        pos: 0,
    };

    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 0 };

    deserializer.peek_invalid_type(&Expected::new("boolean"));
}

#[test]
fn test_peek_invalid_type_with_f() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                self.pos += 1;
                Ok(Some(self.data[self.pos - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.next().ok();
        }

        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
    }

    let mut read = MockRead {
        data: vec![b'f', b'a', b'l', b's', b'e'],
        pos: 0,
    };

    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 0 };

    deserializer.peek_invalid_type(&Expected::new("boolean"));
}

#[test]
fn test_peek_invalid_type_with_square_bracket() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                self.pos += 1;
                Ok(Some(self.data[self.pos - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.next().ok();
        }

        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
    }

    let mut read = MockRead {
        data: vec![b'['],
        pos: 0,
    };

    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 0 };

    deserializer.peek_invalid_type(&Expected::new("sequence"));
}

#[test]
fn test_peek_invalid_type_with_minus() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                self.pos += 1;
                Ok(Some(self.data[self.pos - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.next().ok();
        }

        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
    }

    let mut read = MockRead {
        data: vec![b'-', b'1'],
        pos: 0,
    };

    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 0 };

    deserializer.peek_invalid_type(&Expected::new("number"));
}

#[test]
fn test_peek_invalid_type_with_double_quote() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                self.pos += 1;
                Ok(Some(self.data[self.pos - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.next().ok();
        }

        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
    }

    let mut read = MockRead {
        data: vec![b'"', b'h', b'e', b'l', b'l', b'o'],
        pos: 0,
    };

    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 0 };

    deserializer.peek_invalid_type(&Expected::new("string"));
}

#[test]
fn test_peek_invalid_type_with_curly_brace() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                self.pos += 1;
                Ok(Some(self.data[self.pos - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.next().ok();
        }

        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
    }

    let mut read = MockRead {
        data: vec![b'{'],
        pos: 0,
    };

    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 0 };

    deserializer.peek_invalid_type(&Expected::new("map"));
}

#[test]
fn test_peek_invalid_type_with_numeric() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                self.pos += 1;
                Ok(Some(self.data[self.pos - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.next().ok();
        }

        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
    }

    let mut read = MockRead {
        data: vec![b'1', b'2', b'3'],
        pos: 0,
    };

    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 0 };
    
    deserializer.peek_invalid_type(&Expected::new("number"));
}

