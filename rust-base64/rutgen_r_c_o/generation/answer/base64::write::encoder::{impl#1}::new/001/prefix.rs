// Answer 0

#[test]
fn test_new_encoder_with_file() {
    use std::fs::File;
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            todo!()
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let file = File::create("testfile.txt").unwrap();
    let engine = TestEngine;
    let encoder = EncoderWriter::new(file, &engine);
}

#[test]
fn test_new_encoder_with_buffered_writer() {
    use std::io::BufWriter;
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            todo!()
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let buffer = Vec::new();
    let buffered_writer = BufWriter::new(buffer);
    let engine = TestEngine;
    let encoder = EncoderWriter::new(buffered_writer, &engine);
}

#[test]
fn test_new_encoder_with_memory_writer() {
    use std::io::Cursor;
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            todo!()
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let cursor = Cursor::new(Vec::new());
    let engine = TestEngine;
    let encoder = EncoderWriter::new(cursor, &engine);
}

