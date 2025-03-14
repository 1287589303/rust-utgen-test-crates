// Answer 0

#[derive(Default)]
struct MockSink {
    written_bytes: Vec<u8>,
    error: Option<String>,
}

impl MockSink {
    fn new() -> Self {
        Self {
            written_bytes: Vec::new(),
            error: None,
        }
    }

    fn set_error(&mut self, error: &str) {
        self.error = Some(error.to_string());
    }
}

trait Sink {
    type Error;
    fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;
}

impl Sink for MockSink {
    type Error = String;

    fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        if let Some(ref err) = self.error {
            Err(err.clone())
        } else {
            self.written_bytes.extend_from_slice(bytes);
            Ok(())
        }
    }
}

#[derive(Default)]
struct MockEngine {
    padding: bool,
}

impl MockEngine {
    fn internal_encode(&self, chunk: &[u8], buf: &mut [u8]) -> usize {
        chunk.len() // Simplified encoding for testing purpose
    }

    fn config(&self) -> &MockConfig {
        &MockConfig { padding: self.padding }
    }
}

struct MockConfig {
    padding: bool,
}

impl MockConfig {
    fn encode_padding(&self) -> bool {
        self.padding
    }
}

struct Encoder {
    engine: MockEngine,
}

impl Encoder {
    fn new(engine: MockEngine) -> Self {
        Self { engine }
    }
}

#[test]
fn test_encode_with_partial_chunk_no_padding_error() {
    let mut sink = MockSink::new();
    sink.set_error("Write error");

    let engine = MockEngine { padding: false };
    let encoder = Encoder::new(engine);
    let bytes = vec![1, 2, 3, 4, 5]; // Total 5 bytes, creates a partial chunk

    let result = encoder.encode(&bytes, &mut sink);
    assert!(result.is_err());
}

#[test]
fn test_encode_with_partial_chunk_no_padding_success() {
    let mut sink = MockSink::new();
    
    let engine = MockEngine { padding: false };
    let encoder = Encoder::new(engine);
    let bytes = vec![1, 2, 3]; // Total 3 bytes, creates a partial chunk

    let result = encoder.encode(&bytes, &mut sink);
    assert!(result.is_ok());
    assert_eq!(sink.written_bytes, vec![1, 2, 3]); // Check if the bytes written are correct
} 

#[test]
fn test_encode_with_multiple_partial_chunks_no_padding() {
    let mut sink = MockSink::new();
    
    let engine = MockEngine { padding: false };
    let encoder = Encoder::new(engine);
    let bytes = vec![1, 2, 3, 4, 5, 6]; // Total 6 bytes, creates two partial chunks

    let result = encoder.encode(&bytes, &mut sink);
    assert!(result.is_ok());
    assert_eq!(sink.written_bytes, vec![1, 2, 3, 4, 5, 6]); // Check written bytes
}

