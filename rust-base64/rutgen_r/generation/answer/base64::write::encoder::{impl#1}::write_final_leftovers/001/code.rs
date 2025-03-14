// Answer 0

#[derive(Default)]
struct TestWriter {
    data: Vec<u8>,
}

impl TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.extend_from_slice(buf);
        Ok(buf.len())
    }
}

struct Encoder {
    delegate: Option<TestWriter>,
    extra_input: Vec<u8>,
    extra_input_occupied_len: usize,
    output: Vec<u8>,
    output_occupied_len: usize,
}

impl Encoder {
    fn new() -> Self {
        Encoder {
            delegate: None,
            extra_input: vec![0; 10],
            extra_input_occupied_len: 0,
            output: vec![0; 10],
            output_occupied_len: 0,
        }
    }

    fn write_all_encoded_output(&mut self) -> Result<()> {
        Ok(())
    }

    fn write_final_leftovers(&mut self) -> Result<()> {
        if self.delegate.is_none() {
            return Ok(());
        }

        self.write_all_encoded_output()?;

        if self.extra_input_occupied_len > 0 {
            let encoded_len = self.engine.encode_slice(
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

    // Dummy engine implementation for test purposes
    struct Engine;

    impl Engine {
        fn encode_slice(&self, input: &[u8], output: &mut [u8]) -> Result<usize> {
            output[..input.len()].copy_from_slice(input);
            Ok(input.len())
        }
    }

    // Dummy engine instance used in Encoder
    fn engine(&self) -> Engine {
        Engine
    }
}

#[test]
fn test_write_final_leftovers_delegate_none() {
    let mut encoder = Encoder::new();
    encoder.delegate = None;

    let result = encoder.write_final_leftovers();

    assert_eq!(result, Ok(()));
}

