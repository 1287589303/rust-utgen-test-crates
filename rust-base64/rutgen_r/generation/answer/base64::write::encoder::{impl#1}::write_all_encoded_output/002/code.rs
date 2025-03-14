// Answer 0

#[derive(Debug)]
struct MockWriter {
    error_kind: Option<std::io::ErrorKind>,
}

impl MockWriter {
    fn new() -> Self {
        MockWriter { error_kind: None }
    }

    fn write_to_delegate(&mut self, _len: usize) -> Result<(), std::io::Error> {
        if let Some(kind) = self.error_kind {
            Err(std::io::Error::new(kind, ""))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
struct Encoder {
    output_occupied_len: usize,
    writer: MockWriter,
}

impl Encoder {
    fn new() -> Self {
        Encoder {
            output_occupied_len: 0,
            writer: MockWriter::new(),
        }
    }

    fn write_all_encoded_output(&mut self) -> Result<()> {
        while self.output_occupied_len > 0 {
            let remaining_len = self.output_occupied_len;
            match self.writer.write_to_delegate(remaining_len) {
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
                Ok(()) => {}
            };
        }
        debug_assert_eq!(0, self.output_occupied_len);
        Ok(())
    }
}

#[test]
fn test_write_all_encoded_output_interrupted_error() {
    let mut encoder = Encoder::new();
    encoder.output_occupied_len = 10; // Precondition: self.output_occupied_len > 0
    encoder.writer.error_kind = Some(std::io::ErrorKind::Interrupted);

    let result = encoder.write_all_encoded_output();

    assert!(result.is_ok());
    assert_eq!(encoder.output_occupied_len, 10); // Still > 0 after interrupted
}

#[test]
fn test_write_all_encoded_output_other_error() {
    let mut encoder = Encoder::new();
    encoder.output_occupied_len = 10; // Precondition: self.output_occupied_len > 0
    encoder.writer.error_kind = Some(std::io::ErrorKind::Other);

    let result = encoder.write_all_encoded_output();

    assert!(result.is_err());
    assert_eq!(encoder.output_occupied_len, 10); // Still > 0 after error
}

#[test]
fn test_write_all_encoded_output_success() {
    let mut encoder = Encoder::new();
    encoder.output_occupied_len = 10; // Precondition: self.output_occupied_len > 0
    encoder.writer.error_kind = None; // No error

    let result = encoder.write_all_encoded_output();

    assert!(result.is_ok());
    assert_eq!(encoder.output_occupied_len, 0); // Precondition: now = 0
}

