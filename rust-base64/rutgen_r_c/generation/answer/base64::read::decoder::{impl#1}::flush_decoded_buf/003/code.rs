// Answer 0

#[test]
fn test_flush_decoded_buf_full_copy() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { len: 0 })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let mut decoder = DecoderReader::new(std::io::empty(), &engine);
    decoder.decoded_chunk_buffer = [1, 2, 3];
    decoder.decoded_offset = 0;
    decoder.decoded_len = DECODED_CHUNK_SIZE;

    let mut buf = [0u8; 3];
    let result = decoder.flush_decoded_buf(&mut buf).unwrap();

    assert_eq!(result, 3);
    assert_eq!(&buf, &[1, 2, 3]);
    assert_eq!(decoder.decoded_offset, 3);
    assert_eq!(decoder.decoded_len, 0);
}

#[test]
fn test_flush_decoded_buf_partial_copy() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { len: 0 })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let mut decoder = DecoderReader::new(std::io::empty(), &engine);
    decoder.decoded_chunk_buffer = [1, 2, 3];
    decoder.decoded_offset = 0;
    decoder.decoded_len = 2;

    let mut buf = [0u8; 2];
    let result = decoder.flush_decoded_buf(&mut buf).unwrap();

    assert_eq!(result, 2);
    assert_eq!(&buf, &[1, 2]);
    assert_eq!(decoder.decoded_offset, 2);
    assert_eq!(decoder.decoded_len, 1);
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_empty_output() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { len: 0 })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let mut decoder = DecoderReader::new(std::io::empty(), &engine);
    decoder.decoded_chunk_buffer = [1, 2, 3];
    decoder.decoded_offset = 0;
    decoder.decoded_len = 2;

    let mut buf = [];
    decoder.flush_decoded_buf(&mut buf).unwrap();
}

