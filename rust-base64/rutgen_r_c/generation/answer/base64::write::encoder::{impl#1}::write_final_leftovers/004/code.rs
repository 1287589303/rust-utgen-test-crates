// Answer 0

#[test]
fn test_write_final_leftovers_with_encoded_data() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = (); // Assume for simplicity in testing
        
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate {
            ()
        }

        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let engine = MockEngine;
    let mock_writer = MockWriter { buffer: Vec::new() };

    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(mock_writer),
        extra_input: [1, 2, 3], // Non-zero extra input
        extra_input_occupied_len: 3,
        output: [0; BUF_SIZE],
        output_occupied_len: 0, // Output is initially empty
        panicked: false,
    };

    // First call to write_all_encoded_output should be successful
    encoder_writer.write_all_encoded_output().unwrap();

    // Call write_final_leftovers and expect it to return Ok(())
    let result = encoder_writer.write_final_leftovers();
    
    // Validate that the result is Ok(())
    assert!(result.is_ok());
} 

#[test]
fn test_write_final_leftovers_with_non_empty_output() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = (); // Assume for simplicity in testing

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate {
            ()
        }

        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let engine = MockEngine;
    let mock_writer = MockWriter { buffer: Vec::new() };

    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(mock_writer),
        extra_input: [5, 6, 7], // Non-zero extra input
        extra_input_occupied_len: 3,
        output: [8, 9, 10, 11, 12, 13, 14, 15], // Non empty output
        output_occupied_len: 8,
        panicked: false,
    };

    // Execute write_all_encoded_output once to simulate successful prior writes
    encoder_writer.write_all_encoded_output().unwrap();

    // Call write_final_leftovers and expect it to return Ok(())
    let result = encoder_writer.write_final_leftovers();
    
    // Validate that the result is Ok(())
    assert!(result.is_ok());
}

