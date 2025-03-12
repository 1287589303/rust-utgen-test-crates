// Answer 0

#[test]
fn test_into_inner_valid_writer() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let vec_writer = Vec::new();
    let engine = TestEngine;
    let mut encoder_writer = EncoderWriter::new(vec_writer, &engine);
    encoder_writer.write_all_encoded_output().unwrap();
    let inner_writer = encoder_writer.into_inner();
}

#[test]
#[should_panic]
fn test_into_inner_after_finish() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let vec_writer = Vec::new();
    let engine = TestEngine;
    let mut encoder_writer = EncoderWriter::new(vec_writer, &engine);
    encoder_writer.finish().unwrap();
    let _ = encoder_writer.into_inner();
}

#[test]
fn test_into_inner_with_large_writer() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let mut large_writer = vec![0u8; 2048];
    let engine = TestEngine;
    let mut encoder_writer = EncoderWriter::new(large_writer, &engine);
    encoder_writer.write_all_encoded_output().unwrap();
    let inner_writer = encoder_writer.into_inner();
}

