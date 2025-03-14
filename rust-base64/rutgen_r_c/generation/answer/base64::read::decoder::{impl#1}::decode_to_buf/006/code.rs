// Answer 0

#[derive(Default)]
struct TestEngine;

impl Engine for TestEngine {
    type Config = ();
    type DecodeEstimate = usize;

    fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
        0 // stub implementation
    }

    fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
        0 // stub implementation
    }

    fn internal_decode(
        &self,
        _input: &[u8],
        _output: &mut [u8],
        _decode_estimate: Self::DecodeEstimate,
    ) -> Result<DecodeMetadata, DecodeSliceError> {
        Ok(DecodeMetadata { decoded_len: 0, padding_offset: None }) // stub implementation
    }

    fn config(&self) -> &Self::Config {
        &() // stub implementation
    }
}

#[test]
fn test_decode_to_buf_empty_buf() {
    let engine = TestEngine::default();
    let mut buf = vec![0u8; 3];
    let mut decoder = DecoderReader::new(&b"SGVsbG8="[..], &engine);

    decoder.b64_len = 4; // equal to b64_len_to_decode
    decoder.b64_offset = BUF_SIZE - 4; // total is BUF_SIZE
    decoder.padding_offset = Some(0); // some padding to trigger condition
    let result = decoder.decode_to_buf(4, &mut buf);

    assert_eq!(result, Ok(0));
}

#[test]
fn test_decode_to_buf_non_empty_buf() {
    let engine = TestEngine::default();
    let mut buf = vec![0u8; 3]; // valid size
    let mut decoder = DecoderReader::new(&b"SGVsbG8="[..], &engine);

    decoder.b64_len = 4; // equal to b64_len_to_decode
    decoder.b64_offset = BUF_SIZE - 4; // total is BUF_SIZE
    decoder.padding_offset = Some(0); // some padding to trigger condition
    let result = decoder.decode_to_buf(4, &mut buf);

    assert_eq!(result, Ok(0));
}

