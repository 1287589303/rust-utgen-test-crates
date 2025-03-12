// Answer 0

#[test]
fn test_next_parse_whitespace_ok_none() {
    struct MockRead {
        has_failed: bool,
    }
    impl read::Read<'_> for MockRead {
        fn byte_offset(&self) -> usize {
            0
        }
        fn set_failed(&mut self, _: &mut bool) {}
        fn peek_position(&self) -> read::Position {
            read::Position { line: 0, column: 0 }
        }
        fn should_early_return_if_failed() -> bool {
            false
        }
    }

    let read = MockRead { has_failed: false };
    let mut deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    let result = stream_deserializer.next();
}

#[test]
fn test_next_parse_whitespace_ok_some_b() {
    struct MockRead {
        empty_data: bool,
    }
    impl read::Read<'_> for MockRead {
        fn byte_offset(&self) -> usize {
            1
        }
        fn set_failed(&mut self, _: &mut bool) {}
        fn peek_position(&self) -> read::Position {
            read::Position { line: 0, column: 1 }
        }
        fn should_early_return_if_failed() -> bool {
            false
        }
    }

    struct MockDeserialize;
    impl de::Deserialize<'_> for MockDeserialize {
        fn deserialize<D>(_: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'_>,
        {
            Err(Error::syntax(ErrorCode::TrailingCharacters, 0, 0))
        }
    }

    let read = MockRead { empty_data: false };
    let mut deserializer = Deserializer::new(read);
    let mut stream_deserializer: StreamDeserializer<_, MockDeserialize> = StreamDeserializer::new(deserializer);
    let result = stream_deserializer.next();
}

#[test]
fn test_next_parse_whitespace_err() {
    struct MockRead {
        error_on_read: bool,
    }
    impl read::Read<'_> for MockRead {
        fn byte_offset(&self) -> usize {
            0
        }
        fn set_failed(&mut self, _: &mut bool) {}
        fn peek_position(&self) -> read::Position {
            read::Position { line: 0, column: 0 }
        }
        fn should_early_return_if_failed() -> bool {
            false
        }
    }

    let read = MockRead { error_on_read: true };
    let mut deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    let result = stream_deserializer.next();
}

#[test]
fn test_next_parse_whitespace_ok_some_bracket() {
    struct MockRead {
        valid_data: bool,
    }
    impl read::Read<'_> for MockRead {
        fn byte_offset(&self) -> usize {
            5
        }
        fn set_failed(&mut self, _: &mut bool) {}
        fn peek_position(&self) -> read::Position {
            read::Position { line: 1, column: 5 }
        }
        fn should_early_return_if_failed() -> bool {
            false
        }
    }

    struct MockDeserialize;
    impl de::Deserialize<'_> for MockDeserialize {
        fn deserialize<D>(_: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'_>,
        {
            Err(Error::syntax(ErrorCode::TrailingCharacters, 1, 5))
        }
    }

    let read = MockRead { valid_data: true };
    let mut deserializer = Deserializer::new(read);
    let mut stream_deserializer: StreamDeserializer<_, MockDeserialize> = StreamDeserializer::new(deserializer);
    let result = stream_deserializer.next();
}

