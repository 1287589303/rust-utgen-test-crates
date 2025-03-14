// Answer 0

#[test]
fn test_decoder_reader_new_with_non_empty_reader() {
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

        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let input_data = b"Hello, World!";
    let reader = &input_data[..];
    
    let decoder_reader = DecoderReader::new(reader, &engine);

    assert_eq!(decoder_reader.engine as *const _, &engine as *const _);
    assert_eq!(decoder_reader.inner, reader);
    assert_eq!(decoder_reader.b64_buffer, [0; BUF_SIZE]);
    assert_eq!(decoder_reader.b64_offset, 0);
    assert_eq!(decoder_reader.b64_len, 0);
    assert_eq!(decoder_reader.decoded_chunk_buffer, [0; DECODED_CHUNK_SIZE]);
    assert_eq!(decoder_reader.decoded_offset, 0);
    assert_eq!(decoder_reader.decoded_len, 0);
    assert_eq!(decoder_reader.input_consumed_len, 0);
    assert_eq!(decoder_reader.padding_offset, None);
}

#[test]
fn test_decoder_reader_new_with_empty_reader() {
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
        
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let input_data: &[u8] = &[];
    let reader = &input_data[..];

    let decoder_reader = DecoderReader::new(reader, &engine);

    assert_eq!(decoder_reader.engine as *const _, &engine as *const _);
    assert_eq!(decoder_reader.inner, reader);
    assert_eq!(decoder_reader.b64_buffer, [0; BUF_SIZE]);
    assert_eq!(decoder_reader.b64_offset, 0);
    assert_eq!(decoder_reader.b64_len, 0);
    assert_eq!(decoder_reader.decoded_chunk_buffer, [0; DECODED_CHUNK_SIZE]);
    assert_eq!(decoder_reader.decoded_offset, 0);
    assert_eq!(decoder_reader.decoded_len, 0);
    assert_eq!(decoder_reader.input_consumed_len, 0);
    assert_eq!(decoder_reader.padding_offset, None);
}

