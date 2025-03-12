// Answer 0

#[test]
fn test_write_with_extra_data_present() {
    struct DummyEngine;
    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = ();
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            // dummy encoding implementation
            let len = input.len().min(3); // simulate encoding of at most 3 bytes
            output[..len].copy_from_slice(&input[..len]);
            4 // return fixed length for the sake of the example
        }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {}
        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {}
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let mut buffer = [0u8; BUF_SIZE];
    let engine = DummyEngine;
    let mut encoder_writer = EncoderWriter::new(&mut buffer as &mut dyn io::Write, &engine);
    encoder_writer.output_occupied_len = 4; // precondition: output_occupied_len > 0
    encoder_writer.extra_input_occupied_len = 1; // some leftover data
    encoder_writer.extra_input[0] = 42; // leftover byte

    let input: &[u8] = &[1, 2, 3]; // valid non-empty input
    let result = encoder_writer.write(input);
}

#[test]
fn test_write_with_exactly_buffer_size() {
    struct DummyEngine;
    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = ();
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len().min(3);
            output[..len].copy_from_slice(&input[..len]);
            4
        }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {}
        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {}
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let mut buffer = [0u8; BUF_SIZE];
    let engine = DummyEngine;
    let mut encoder_writer = EncoderWriter::new(&mut buffer as &mut dyn io::Write, &engine);
    encoder_writer.output_occupied_len = 4; // precondition: output_occupied_len > 0
    encoder_writer.extra_input_occupied_len = 2; // some leftover data
    encoder_writer.extra_input[0] = 42; // leftover byte
    encoder_writer.extra_input[1] = 43; // leftover byte

    let input: &[u8] = &[10, 20, 30]; // valid non-empty input
    let result = encoder_writer.write(input);
}

#[test]
fn test_write_with_multiple_rotated_leftovers() {
    struct DummyEngine;
    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = ();
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len().min(3);
            output[..len].copy_from_slice(&input[..len]);
            4
        }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {}
        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {}
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let mut buffer = [0u8; BUF_SIZE];
    let engine = DummyEngine;
    let mut encoder_writer = EncoderWriter::new(&mut buffer as &mut dyn io::Write, &engine);
    encoder_writer.output_occupied_len = 4; // precondition: output_occupied_len > 0
    encoder_writer.extra_input_occupied_len = 2; // some leftover data
    encoder_writer.extra_input[0] = 99; // leftover byte
    encoder_writer.extra_input[1] = 100; // leftover byte

    let input: &[u8] = &[30, 40, 50, 60]; // valid non-empty input
    let result = encoder_writer.write(input);
}

