// Answer 0

#[derive(Debug)]
struct MockWriter {
    data: Vec<u8>,
}

impl MockWriter {
    fn new() -> Self {
        MockWriter { data: Vec::new() }
    }

    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.data.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn finish(self) -> Result<MockWriter, std::io::Error> {
        Ok(self)
    }
}

struct EncoderWriter<W> {
    delegate: Option<W>,
}

impl<W> EncoderWriter<W> {
    pub fn new(delegate: W) -> Self {
        EncoderWriter {
            delegate: Some(delegate),
        }
    }

    pub fn finish(self) -> Result<W, std::io::Error> {
        self.delegate.take().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Already finished"))
    }
    
    pub fn into_inner(mut self) -> W {
        self.delegate
            .take()
            .expect("Encoder has already had finish() called")
    }
}

#[test]
fn test_into_inner_after_finish() {
    let writer = MockWriter::new();
    let encoder = EncoderWriter::new(writer);
    let finished_writer = encoder.finish().expect("Failed to finish encoder");
    let inner_writer = encoder.into_inner(); // This should panic
}

#[test]
fn test_into_inner_functionality() {
    let writer = MockWriter::new();
    let encoder = EncoderWriter::new(writer);
    let inner_writer = encoder.into_inner();
    assert!(inner_writer.finish().is_ok());
}

