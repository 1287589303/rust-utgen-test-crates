// Answer 0

#[test]
fn test_flush_decoded_buf_full_copy() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { Ok(DecodeMetadata { decoded_len: 3 }) }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;
    let mut decoder = super::DecoderReader::new(std::io::empty(), &engine);
    // Set up the state to satisfy preconditions
    decoder.decoded_chunk_buffer = [1, 2, 3];
    decoder.decoded_len = 3; // self.decoded_len > 0
    decoder.decoded_offset = 0;

    let mut output_buf = [0; 3];
    let result = decoder.flush_decoded_buf(&mut output_buf).unwrap();

    assert_eq!(result, 3);
    assert_eq!(&output_buf, &[1, 2, 3]);
    assert_eq!(decoder.decoded_len, 0); // Ensure decoded_len is now 0
    assert_eq!(decoder.decoded_offset, 3); // Ensure decoded_offset updated correctly
}

#[test]
fn test_flush_decoded_buf_partial_copy() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { Ok(DecodeMetadata { decoded_len: 3 }) }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;
    let mut decoder = super::DecoderReader::new(std::io::empty(), &engine);
    // Set up the state to satisfy preconditions
    decoder.decoded_chunk_buffer = [1, 2, 3];
    decoder.decoded_len = 3; // self.decoded_len > 0
    decoder.decoded_offset = 0;

    let mut output_buf = [0; 2]; // Smaller buffer
    let result = decoder.flush_decoded_buf(&mut output_buf).unwrap();

    assert_eq!(result, 2); // Only two bytes copied
    assert_eq!(&output_buf, &[1, 2]);
    assert_eq!(decoder.decoded_len, 1); // Ensure decoded_len updated correctly
    assert_eq!(decoder.decoded_offset, 2); // Ensure decoded_offset updated correctly
}

