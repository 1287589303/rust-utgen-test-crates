// Answer 0


#[cfg(test)]
mod tests {
    use super::*;
    
    struct MockEngine;

    impl Config for MockEngine {
        fn encode_padding(&self) -> bool {
            false
        }
    }

    struct MockSink {
        data: Vec<u8>,
        error: Option<Box<dyn std::error::Error>>,
    }

    impl MockSink {
        fn new() -> Self {
            MockSink { data: Vec::new(), error: None }
        }
    }

    impl Sink for MockSink {
        type Error = Box<dyn std::error::Error>;

        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
            if self.error.is_none() {
                self.data.extend_from_slice(bytes);
                Ok(())
            } else {
                Err(self.error.clone().unwrap())
            }
        }
    }

    #[test]
    fn test_encode_multiple_full_chunks() -> Result<(), Box<dyn std::error::Error>> {
        let engine = MockEngine;
        let encoder = ChunkedEncoder::new(&engine);
        let mut sink = MockSink::new();
        let input = vec![0u8; 1024]; // Full buffer size, guarantees full chunks

        encoder.encode(&input, &mut sink)?;

        assert_eq!(sink.data.len(), 1024); // Check that the total data length is as expected
        Ok(())
    }

    #[test]
    fn test_encode_partial_chunk() -> Result<(), Box<dyn std::error::Error>> {
        let engine = MockEngine;
        let encoder = ChunkedEncoder::new(&engine);
        let mut sink = MockSink::new();
        let input = vec![0u8; 1000]; // Size guarantees one partial chunk

        encoder.encode(&input, &mut sink)?;

        assert_eq!(sink.data.len(), 1000); // Ensure the correct length of data was encoded
        Ok(())
    }

    #[test]
    fn test_encode_empty_input() -> Result<(), Box<dyn std::error::Error>> {
        let engine = MockEngine;
        let encoder = ChunkedEncoder::new(&engine);
        let mut sink = MockSink::new();
        let input: Vec<u8> = Vec::new(); // Empty input

        encoder.encode(&input, &mut sink)?;

        assert!(sink.data.is_empty()); // Check that no data was written
        Ok(())
    }
}


