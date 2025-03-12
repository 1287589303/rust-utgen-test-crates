// Answer 0

#[test]
fn test_peek_end_of_value_ok_space() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }
    
    impl read::Read<'static> for TestReader {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                return Ok(Some(self.data[self.index]));
            }
            Ok(None)
        }
        
        fn byte_offset(&self) -> usize {
            self.index
        }
    }
    
    let mut reader = TestReader { data: vec![b' '], index: 0 };
    let mut deserializer = Deserializer::new(reader);
    let mut stream_deserializer = StreamDeserializer::new(&mut deserializer);
    
    let _ = stream_deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_ok_newline() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }
    
    impl read::Read<'static> for TestReader {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                return Ok(Some(self.data[self.index]));
            }
            Ok(None)
        }
        
        fn byte_offset(&self) -> usize {
            self.index
        }
    }
    
    let mut reader = TestReader { data: vec![b'\n'], index: 0 };
    let mut deserializer = Deserializer::new(reader);
    let mut stream_deserializer = StreamDeserializer::new(&mut deserializer);
    
    let _ = stream_deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_err_invalid_character() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }
    
    impl read::Read<'static> for TestReader {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                return Ok(Some(b'x'));
            }
            Ok(None)
        }
        
        fn byte_offset(&self) -> usize {
            self.index
        }
    }
    
    let mut reader = TestReader { data: vec![b'x'], index: 0 };
    let mut deserializer = Deserializer::new(reader);
    let mut stream_deserializer = StreamDeserializer::new(&mut deserializer);
    
    let _ = stream_deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_err_trailing_characters() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }
    
    impl read::Read<'static> for TestReader {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                return Ok(Some(b'a'));
            }
            Ok(None)
        }
        
        fn byte_offset(&self) -> usize {
            self.index
        }
    }
    
    let mut reader = TestReader { data: vec![b'a'], index: 0 };
    let mut deserializer = Deserializer::new(reader);
    let mut stream_deserializer = StreamDeserializer::new(&mut deserializer);
    
    let _ = stream_deserializer.peek_end_of_value();
}

