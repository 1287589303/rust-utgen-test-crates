// Answer 0

#[test]
fn drop_panicked_should_not_call_write_final_leftovers() {
    use std::io::Cursor;

    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(
            &self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = TestEngine;
    let writer = Cursor::new(Vec::new());

    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(writer),
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: true,  // simulate the panicked state
    };

    // Call drop implicitly by letting 'encoder_writer' go out of scope 
}

