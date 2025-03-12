// Answer 0

#[test]
fn test_next_with_ok_none() {
    struct TestRead {
        byte_offset: usize,
    }

    impl<'de> read::Read<'de> for TestRead {
        fn byte_offset(&self) -> usize {
            self.byte_offset
        }
        
        fn set_failed(&mut self, _: &mut bool) {}
        
        fn peek_position(&self) -> read::Position {
            read::Position { line: 1, column: 1 }
        }

        const should_early_return_if_failed: bool = false;
    }

    struct TestType;

    impl<'de> de::Deserialize<'de> for TestType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            Ok(TestType)
        }
    }

    let read = TestRead { byte_offset: 0 };
    let mut deserializer = StreamDeserializer::new(read);
    let result = deserializer.next();
}

#[test]
fn test_next_with_ok_some() {
    struct TestRead {
        byte_offset: usize,
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> read::Read<'de> for TestRead {
        fn byte_offset(&self) -> usize {
            self.byte_offset
        }
        
        fn set_failed(&mut self, _: &mut bool) {}

        fn peek_position(&self) -> read::Position {
            read::Position { line: 1, column: self.position }
        }

        const should_early_return_if_failed: bool = false;
    }

    struct TestType;

    impl<'de> de::Deserialize<'de> for TestType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            Ok(TestType)
        }
    }

    let read = TestRead { byte_offset: 0, data: vec![b'{'], position: 1 };
    let mut deserializer = StreamDeserializer::new(read);
    let result = deserializer.next();
}

