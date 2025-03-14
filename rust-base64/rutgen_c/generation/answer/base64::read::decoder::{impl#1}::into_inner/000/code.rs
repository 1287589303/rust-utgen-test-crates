// Answer 0

#[test]
fn test_into_inner() {
    struct MockEngine;

    impl Engine for MockEngine {
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
            Err(DecodeSliceError::InvalidBase64)
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let input_data: &[u8] = b"dummy base64 encoded input";
    let reader = io::Cursor::new(input_data);
    let engine = MockEngine;

    let decoder = DecoderReader::new(reader, &engine);
    let inner_reader = decoder.into_inner();

    assert_eq!(inner_reader.get_ref(), input_data);
}

#[test]
fn test_into_inner_empty_input() {
    struct MockEngine;

    impl Engine for MockEngine {
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
            Err(DecodeSliceError::InvalidBase64)
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let input_data: &[u8] = &[];
    let reader = io::Cursor::new(input_data);
    let engine = MockEngine;

    let decoder = DecoderReader::new(reader, &engine);
    let inner_reader = decoder.into_inner();

    assert_eq!(inner_reader.get_ref(), input_data);
}

