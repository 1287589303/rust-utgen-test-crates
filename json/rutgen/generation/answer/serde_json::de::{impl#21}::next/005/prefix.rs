// Answer 0

#[test]
fn test_next_with_whitespace_none() {
    struct MockReader {
        should_early_return_if_failed: bool,
    }

    impl read::Read<'static> for MockReader {
        fn byte_offset(&self) -> usize { 0 }
        fn set_failed(&mut self, _: &mut bool) {}
        fn peek_position(&self) -> Position { Position { line: 1, column: 1 } } 
    }

    struct MockDeserializer;

    impl de::Deserialize<'static> for MockDeserializer {
        fn deserialize<D>(deserializer: D) -> Result<Self>
        where
            D: Deserializer<'static>,
        {
            Ok(MockDeserializer)
        }
    }

    let mock_reader = MockReader { should_early_return_if_failed: true };
    let mut deserializer = StreamDeserializer::new(mock_reader);
    deserializer.failed = false;

    let result = deserializer.next();
}

#[test]
fn test_next_with_whitespace_some_bracket() {
    struct MockReader {
        should_early_return_if_failed: bool,
    }

    impl read::Read<'static> for MockReader {
        fn byte_offset(&self) -> usize { 0 }
        fn set_failed(&mut self, _: &mut bool) {}
        fn peek_position(&self) -> Position { Position { line: 1, column: 1 } } 
    }

    struct MockDeserializer;

    impl de::Deserialize<'static> for MockDeserializer {
        fn deserialize<D>(deserializer: D) -> Result<Self>
        where
            D: Deserializer<'static>,
        {
            Ok(MockDeserializer)
        }
    }

    let mock_reader = MockReader { should_early_return_if_failed: true };
    let mut deserializer = StreamDeserializer::new(mock_reader);
    deserializer.failed = false;

    deserializer.de.read.set_failed(&mut deserializer.failed);
    
    let result = deserializer.next();
}

#[test]
fn test_next_with_whitespace_some_quote() {
    struct MockReader {
        should_early_return_if_failed: bool,
    }

    impl read::Read<'static> for MockReader {
        fn byte_offset(&self) -> usize { 0 }
        fn set_failed(&mut self, _: &mut bool) {}
        fn peek_position(&self) -> Position { Position { line: 1, column: 1 } } 
    }

    struct MockDeserializer;

    impl de::Deserialize<'static> for MockDeserializer {
        fn deserialize<D>(deserializer: D) -> Result<Self>
        where
            D: Deserializer<'static>,
        {
            Ok(MockDeserializer)
        }
    }

    let mock_reader = MockReader { should_early_return_if_failed: true };
    let mut deserializer = StreamDeserializer::new(mock_reader);
    deserializer.failed = false;

    let result = deserializer.next();
}

#[test]
fn test_next_with_whitespace_some_curly() {
    struct MockReader {
        should_early_return_if_failed: bool,
    }

    impl read::Read<'static> for MockReader {
        fn byte_offset(&self) -> usize { 0 }
        fn set_failed(&mut self, _: &mut bool) {}
        fn peek_position(&self) -> Position { Position { line: 1, column: 1 } } 
    }

    struct MockDeserializer;

    impl de::Deserialize<'static> for MockDeserializer {
        fn deserialize<D>(deserializer: D) -> Result<Self>
        where
            D: Deserializer<'static>,
        {
            Ok(MockDeserializer)
        }
    }

    let mock_reader = MockReader { should_early_return_if_failed: true };
    let mut deserializer = StreamDeserializer::new(mock_reader);
    deserializer.failed = false;

    let result = deserializer.next();
}

#[test]
fn test_next_with_error_return() {
    struct MockReader {
        should_early_return_if_failed: bool,
    }

    impl read::Read<'static> for MockReader {
        fn byte_offset(&self) -> usize { 0 }
        fn set_failed(&mut self, _: &mut bool) {}
        fn peek_position(&self) -> Position { Position { line: 1, column: 1 } } 
    }

    struct MockDeserializer;

    impl de::Deserialize<'static> for MockDeserializer {
        fn deserialize<D>(deserializer: D) -> Result<Self>
        where
            D: Deserializer<'static>,
        {
            Err(Error::syntax(ErrorCode::TrailingCharacters, 1, 1))
        }
    }

    let mock_reader = MockReader { should_early_return_if_failed: true };
    let mut deserializer = StreamDeserializer::new(mock_reader);
    deserializer.failed = false;

    let result = deserializer.next();
}

