// Answer 0

#[test]
fn test_write_final_leftovers_no_delegate() {
    struct DummyWriter {
        data: Vec<u8>,
    }

    impl DummyWriter {
        fn new() -> Self {
            Self { data: Vec::new() }
        }
    }

    struct Encoder {
        delegate: Option<DummyWriter>,
        extra_input_occupied_len: usize,
        output: Vec<u8>,
        output_occupied_len: usize,
        engine: DummyEngine,
        extra_input: Vec<u8>,
    }

    struct DummyEngine;

    impl DummyEngine {
        fn encode_slice(&self, input: &[u8], output: &mut [u8]) -> Result<usize, &'static str> {
            // Simulate encoding
            let bytes_written = input.len().min(output.len());
            output[..bytes_written].copy_from_slice(&input[..bytes_written]);
            Ok(bytes_written)
        }
    }

    impl Encoder {
        fn write_all_encoded_output(&mut self) -> Result<(), &'static str> {
            // Simulate writing output
            if let Some(writer) = &mut self.delegate {
                writer.data.extend_from_slice(&self.output[..self.output_occupied_len]);
            }
            Ok(())
        }

        fn write_final_leftovers(&mut self) -> Result<(), &'static str> {
            if self.delegate.is_none() {
                return Ok(());
            }

            self.write_all_encoded_output()?;

            if self.extra_input_occupied_len > 0 {
                let encoded_len = self
                    .engine
                    .encode_slice(
                        &self.extra_input[..self.extra_input_occupied_len],
                        &mut self.output[..],
                    )
                    .expect("buffer is large enough");

                self.output_occupied_len = encoded_len;
                self.write_all_encoded_output()?;
                self.extra_input_occupied_len = 0;
            }

            Ok(())
        }
    }

    let mut encoder = Encoder {
        delegate: None,
        extra_input_occupied_len: 0,
        output: vec![0; 10],
        output_occupied_len: 0,
        engine: DummyEngine,
        extra_input: vec![],
    };

    let result = encoder.write_final_leftovers();
    assert!(result.is_ok());
}

#[test]
fn test_write_final_leftovers_with_delegate() {
    struct DummyWriter {
        data: Vec<u8>,
    }

    impl DummyWriter {
        fn new() -> Self {
            Self { data: Vec::new() }
        }
    }

    struct Encoder {
        delegate: Option<DummyWriter>,
        extra_input_occupied_len: usize,
        output: Vec<u8>,
        output_occupied_len: usize,
        engine: DummyEngine,
        extra_input: Vec<u8>,
    }

    struct DummyEngine;

    impl DummyEngine {
        fn encode_slice(&self, input: &[u8], output: &mut [u8]) -> Result<usize, &'static str> {
            let bytes_written = input.len().min(output.len());
            output[..bytes_written].copy_from_slice(&input[..bytes_written]);
            Ok(bytes_written)
        }
    }

    impl Encoder {
        fn write_all_encoded_output(&mut self) -> Result<(), &'static str> {
            if let Some(writer) = &mut self.delegate {
                writer.data.extend_from_slice(&self.output[..self.output_occupied_len]);
            }
            Ok(())
        }

        fn write_final_leftovers(&mut self) -> Result<(), &'static str> {
            if self.delegate.is_none() {
                return Ok(());
            }

            self.write_all_encoded_output()?;

            if self.extra_input_occupied_len > 0 {
                let encoded_len = self
                    .engine
                    .encode_slice(
                        &self.extra_input[..self.extra_input_occupied_len],
                        &mut self.output[..],
                    )
                    .expect("buffer is large enough");

                self.output_occupied_len = encoded_len;
                self.write_all_encoded_output()?;
                self.extra_input_occupied_len = 0;
            }

            Ok(())
        }
    }

    let mut writer = DummyWriter::new();
    let mut encoder = Encoder {
        delegate: Some(writer),
        extra_input_occupied_len: 3,
        output: vec![0; 10],
        output_occupied_len: 0,
        engine: DummyEngine,
        extra_input: vec![1, 2, 3],
    };

    encoder.delegate = Some(writer);
    let result = encoder.write_final_leftovers();
    assert!(result.is_ok());
    assert!(!encoder.delegate.as_ref().unwrap().data.is_empty());
}

