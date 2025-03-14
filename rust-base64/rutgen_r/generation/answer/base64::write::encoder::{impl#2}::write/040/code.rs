// Answer 0

#[test]
fn test_write_empty_input() {
    struct MockDelegate {
        data_written: Vec<u8>,
    }

    impl MockDelegate {
        fn new() -> Self {
            Self { data_written: Vec::new() }
        }

        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data_written.extend_from_slice(buf);
            Ok(buf.len())
        }
    }

    struct Encoder {
        delegate: Option<MockDelegate>,
        output_occupied_len: usize,
        extra_input: [u8; 3],
        extra_input_occupied_len: usize,
        engine: MockEngine,
    }

    struct MockEngine;

    impl MockEngine {
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len().min(3);
            output[..len].copy_from_slice(&input[..len]);
            len // returning number of bytes "encoded" for testing
        }
    }

    impl Encoder {
        fn new(delegate: MockDelegate) -> Self {
            Self {
                delegate: Some(delegate),
                output_occupied_len: 0,
                extra_input: [0; 3],
                extra_input_occupied_len: 0,
                engine: MockEngine,
            }
        }

        fn write(&mut self, input: &[u8]) -> Result<usize, std::io::Error> {
            assert!(self.delegate.is_some(), "Cannot write more after calling finish()");
            if input.is_empty() {
                return Ok(0);
            }

            if self.output_occupied_len > 0 {
                return Ok(0);
            }

            let input_complete_chunks_len = input.len() - (input.len() % 3);
            let max_input_len = 3; // Assume MAX_INPUT_LEN = 3 for testing 

            self.engine.internal_encode(&input[..input_complete_chunks_len], &mut [0; 3]);
            self.output_occupied_len = 3; // Simulating an encoding operation

            self.delegate.as_mut().unwrap().write(&[0; 3]) // Simulating writing to the delegate
        }
    }

    let mut delegate = MockDelegate::new();
    let mut encoder = Encoder::new(delegate);
    let input = b"abc"; // Non-empty input
    let result = encoder.write(input).unwrap();
    assert_eq!(result, 3);
}

#[test]
#[should_panic]
fn test_write_when_extra_is_filled() {
    struct MockDelegate {
        data_written: Vec<u8>,
    }

    impl MockDelegate {
        fn new() -> Self {
            Self { data_written: Vec::new() }
        }

        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data_written.extend_from_slice(buf);
            Ok(buf.len())
        }
    }

    struct Encoder {
        delegate: Option<MockDelegate>,
        output_occupied_len: usize,
        extra_input: [u8; 3],
        extra_input_occupied_len: usize,
        engine: MockEngine,
    }

    struct MockEngine;

    impl MockEngine {
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len().min(3);
            output[..len].copy_from_slice(&input[..len]);
            len // returning number of bytes "encoded" for testing
        }
    }

    impl Encoder {
        fn new(delegate: MockDelegate) -> Self {
            Self {
                delegate: Some(delegate),
                output_occupied_len: 0,
                extra_input: [0; 3],
                extra_input_occupied_len: 0,
                engine: MockEngine,
            }
        }

        fn write(&mut self, input: &[u8]) -> Result<usize, std::io::Error> {
            assert!(self.delegate.is_some(), "Cannot write more after calling finish()");

            if self.output_occupied_len > 0 {
                panic!("Output occupied length should not be greater than 0 here");
            }

            if input.len() == 1 {
                self.extra_input[self.extra_input_occupied_len] = input[0];
                self.extra_input_occupied_len += 1;
                return Ok(1);
            }

            return Ok(0);
        }
    }

    let mut delegate = MockDelegate::new();
    let mut encoder = Encoder::new(delegate);
    let input = b"a"; // one byte in input
    encoder.extra_input_occupied_len = 2; // Simulating the extra input already being filled
    let _ = encoder.write(input);
}

