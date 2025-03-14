// Answer 0

#[test]
fn test_write_with_full_input_and_encoding() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            // Assume it encodes 3 bytes into 4 bytes
            output[..4].copy_from_slice(&[0; 4]); // Simple mock encoding
            4
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let mut output_buf = [0u8; BUF_SIZE];
    let mut extra_input = [0u8; MIN_ENCODE_CHUNK_SIZE];
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(vec![]),
        extra_input,
        extra_input_occupied_len: 0,
        output: output_buf,
        output_occupied_len: 0,
        panicked: false,
    };

    let input = [1, 2, 3]; // input.len() == 3
    let result = encoder_writer.write(&input).unwrap();

    assert_eq!(result, 3);
    // Confirm that the output buffer contains the expected encoded data
}

#[test]
fn test_write_with_leftover_encoded_data() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..4].copy_from_slice(&[0; 4]); // Simple mock encoding
            4
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let mut output_buf = [0u8; BUF_SIZE];
    let mut extra_input = [0u8; MIN_ENCODE_CHUNK_SIZE];
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(vec![]),
        extra_input,
        extra_input_occupied_len: 0,
        output: output_buf,
        output_occupied_len: 4, // Simulate that 4 bytes are already encoded
        panicked: false,
    };

    let input = [4, 5, 6]; // input.len() > 3
    let result = encoder_writer.write(&input).unwrap();

    assert_eq!(result, 3);
    // Confirm that the output buffer contains the expected encoded data after including leftovers
}

#[test]
fn test_write_with_input_fitting_into_extra() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..4].copy_from_slice(&[0; 4]); // Simple mock encoding
            4
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let mut output_buf = [0u8; BUF_SIZE];
    let mut extra_input = [0u8; MIN_ENCODE_CHUNK_SIZE];
    let input = [7]; // input.len() == 1
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(vec![]),
        extra_input,
        extra_input_occupied_len: 0, // Starting empty
        output: output_buf,
        output_occupied_len: 0,
        panicked: false,
    };

    let result = encoder_writer.write(&input).unwrap();

    assert_eq!(result, 1);
    // Confirm the extra_input is correctly populated
    assert_eq!(encoder_writer.extra_input[0], 7);
    assert_eq!(encoder_writer.extra_input_occupied_len, 1);
}

