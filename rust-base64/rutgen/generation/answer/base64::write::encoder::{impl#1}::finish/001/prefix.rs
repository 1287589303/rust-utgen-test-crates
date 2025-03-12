// Answer 0

#[test]
fn test_finish_with_no_extra_input() {
    struct MockEngine;
    struct MockWriter;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<(), std::io::Error> { Ok(()) }
        fn config(&self) -> &Self::Config { &() }
        fn encode_slice<T: AsRef<[u8]>>(&self, _input: T, _output_buf: &mut [u8]) -> Result<usize, std::io::Error> { Ok(0) }
    }

    let engine = MockEngine;
    let writer = MockWriter;

    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(writer),
        extra_input: [0u8; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: [0u8; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let _result = encoder_writer.finish();
}

#[test]
fn test_finish_with_partial_extra_input() {
    struct MockEngine;
    struct MockWriter;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<(), std::io::Error> { Ok(()) }
        fn config(&self) -> &Self::Config { &() }
        fn encode_slice<T: AsRef<[u8]>>(&self, _input: T, _output_buf: &mut [u8]) -> Result<usize, std::io::Error> { Ok(1) }
    }

    let engine = MockEngine;
    let writer = MockWriter;

    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(writer),
        extra_input: [1, 2, 3],
        extra_input_occupied_len: 3,
        output: [0u8; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let _result = encoder_writer.finish();
}

#[test]
fn test_finish_with_max_extra_input() {
    struct MockEngine;
    struct MockWriter;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<(), std::io::Error> { Ok(()) }
        fn config(&self) -> &Self::Config { &() }
        fn encode_slice<T: AsRef<[u8]>>(&self, _input: T, _output_buf: &mut [u8]) -> Result<usize, std::io::Error> { Ok(3) }
    }

    let engine = MockEngine;
    let writer = MockWriter;

    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(writer),
        extra_input: [4, 5, 6],
        extra_input_occupied_len: MIN_ENCODE_CHUNK_SIZE,
        output: [0u8; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let _result = encoder_writer.finish();
}

