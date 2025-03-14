// Answer 0

#[derive(Default)]
struct FakeSink {
    written: Vec<u8>,
    error: Option<Box<dyn std::error::Error>>,
}

impl FakeSink {
    fn new() -> Self {
        Self::default()
    }

    fn set_error<E: std::error::Error + 'static>(&mut self, error: E) {
        self.error = Some(Box::new(error));
    }
}

impl Sink for FakeSink {
    type Error = Box<dyn std::error::Error>;

    fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        if let Some(ref error) = self.error {
            Err(error.clone())
        } else {
            self.written.extend_from_slice(bytes);
            Ok(())
        }
    }
}

struct Encoder {
    engine: Engine,
}

struct Engine {
    config: Config,
}

struct Config {
    padding: bool,
}

impl Engine {
    fn internal_encode(&self, chunk: &[u8], buf: &mut [u8]) -> usize {
        // Sample encoding for testing purposes.
        // Normally you would have actual encoding logic.
        buf[..chunk.len()].copy_from_slice(chunk);
        chunk.len()
    }

    fn config(&self) -> &Config {
        &self.config
    }
}

fn add_padding(len: usize, buf: &mut [u8]) -> usize {
    let padding_needed = (4 - (len % 4)) % 4;
    for i in 0..padding_needed {
        buf[i] = b'=';  // Use '=' padding
    }
    padding_needed
}

#[test]
fn test_encode_full_chunk_no_error() {
    let engine = Engine { config: Config { padding: false } };
    let encoder = Encoder { engine };
    let mut sink = FakeSink::new();
    let data = vec![1u8; 1024]; // Full buffer
    let result = encoder.encode(&data, &mut sink);
    assert!(result.is_ok());
}

#[test]
fn test_encode_partial_chunk_with_error() {
    let engine = Engine { config: Config { padding: true } };
    let mut encoder = Encoder { engine };
    let mut sink = FakeSink::new();
    sink.set_error("write error");

    let data = vec![1u8; 1023]; // One byte less than a full buffer
    let result = encoder.encode(&data, &mut sink);
    assert!(result.is_err());
}

