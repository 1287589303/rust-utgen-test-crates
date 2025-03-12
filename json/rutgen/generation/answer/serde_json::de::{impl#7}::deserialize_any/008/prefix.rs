// Answer 0

#[test]
fn test_deserialize_any_unit() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
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

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            let result = String::from_utf8(scratch.clone()).map_err(|_| Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))?;
            Ok(Reference::Borrowed(&result))
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(scratch))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
    }

    let mut mock_read = MockRead {
        data: b"true".to_vec(),
        position: 0,
    };
    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: vec![],
        remaining_depth: 0,
    };

    let visitor = MockVisitor {};
    deserializer.deserialize_any(visitor).unwrap();
}

#[test]
fn test_deserialize_any_bool_true() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
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

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            let result = String::from_utf8(scratch.clone()).map_err(|_| Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))?;
            Ok(Reference::Borrowed(&result))
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(scratch))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
    }

    let mut mock_read = MockRead {
        data: b"false".to_vec(),
        position: 0,
    };
    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: vec![],
        remaining_depth: 0,
    };

    let visitor = MockVisitor {};
    deserializer.deserialize_any(visitor).unwrap();
}

#[test]
fn test_deserialize_any_array() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
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

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            let result = String::from_utf8(scratch.clone()).map_err(|_| Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))?;
            Ok(Reference::Borrowed(&result))
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(scratch))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
    }

    let mut mock_read = MockRead {
        data: b"[1, 2, 3]".to_vec(),
        position: 0,
    };
    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: vec![],
        remaining_depth: 0,
    };

    let visitor = MockVisitor {};
    deserializer.deserialize_any(visitor).unwrap();
}

#[test]
fn test_deserialize_any_string() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
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

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            let result = String::from_utf8(scratch.clone()).map_err(|_| Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))?;
            Ok(Reference::Borrowed(&result))
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(scratch))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
    }

    let mut mock_read = MockRead {
        data: b"\"hello\"".to_vec(),
        position: 0,
    };
    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: vec![],
        remaining_depth: 0,
    };

    let visitor = MockVisitor {};
    deserializer.deserialize_any(visitor).unwrap();
}

#[test]
fn test_deserialize_any_negative_number() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
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

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            let result = String::from_utf8(scratch.clone()).map_err(|_| Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))?;
            Ok(Reference::Borrowed(&result))
        }

        fn parse_str_raw<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(scratch))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
    }

    let mut mock_read = MockRead {
        data: b"-42".to_vec(),
        position: 0,
    };
    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: vec![],
        remaining_depth: 0,
    };

    let visitor = MockVisitor {};
    deserializer.deserialize_any(visitor).unwrap();
}

