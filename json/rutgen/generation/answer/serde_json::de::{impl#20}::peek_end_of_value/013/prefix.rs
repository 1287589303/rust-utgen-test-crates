// Answer 0

#[test]
fn test_peek_end_of_value_space() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'static> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }
    }

    let read = MockRead { data: vec![b' '], pos: 0 };
    let mut deserializer = StreamDeserializer::new(read);
    let _ = deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_newline() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'static> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }
    }

    let read = MockRead { data: vec![b'\n'], pos: 0 };
    let mut deserializer = StreamDeserializer::new(read);
    let _ = deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_tab() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'static> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }
    }

    let read = MockRead { data: vec![b'\t'], pos: 0 };
    let mut deserializer = StreamDeserializer::new(read);
    let _ = deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_carriage_return() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'static> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }
    }

    let read = MockRead { data: vec![b'\r'], pos: 0 };
    let mut deserializer = StreamDeserializer::new(read);
    let _ = deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_double_quote() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'static> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }
    }

    let read = MockRead { data: vec![b'"'], pos: 0 };
    let mut deserializer = StreamDeserializer::new(read);
    let _ = deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_open_bracket() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'static> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }
    }

    let read = MockRead { data: vec![b'['], pos: 0 };
    let mut deserializer = StreamDeserializer::new(read);
    let _ = deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_close_bracket() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'static> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }
    }

    let read = MockRead { data: vec![b']'], pos: 0 };
    let mut deserializer = StreamDeserializer::new(read);
    let _ = deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_open_brace() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'static> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }
    }

    let read = MockRead { data: vec![b'{'], pos: 0 };
    let mut deserializer = StreamDeserializer::new(read);
    let _ = deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_close_brace() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'static> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }
    }

    let read = MockRead { data: vec![b'}'], pos: 0 };
    let mut deserializer = StreamDeserializer::new(read);
    let _ = deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_comma() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'static> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }
    }

    let read = MockRead { data: vec![b','], pos: 0 };
    let mut deserializer = StreamDeserializer::new(read);
    let _ = deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_colon() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'static> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }
    }

    let read = MockRead { data: vec![b':'], pos: 0 };
    let mut deserializer = StreamDeserializer::new(read);
    let _ = deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_none() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl Read<'static> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }
    }

    let read = MockRead { data: Vec::new(), pos: 0 };
    let mut deserializer = StreamDeserializer::new(read);
    let _ = deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_err() {
    struct MockRead {
        pos: usize,
    }

    impl Read<'static> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            Err(Error::syntax(ErrorCode::TrailingCharacters, 0, 0))
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }
    }

    let read = MockRead { pos: 0 };
    let mut deserializer = StreamDeserializer::new(read);
    let _ = deserializer.peek_end_of_value();
}

