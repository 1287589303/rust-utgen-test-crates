// Answer 0

#[test]
fn test_read_with_full_buffer() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 3, padding_offset: None })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let mock_engine = MockEngine;
    let data = b"SGVsbG8gd29ybGQh"; // Base64 for "Hello world!"
    let mut cursor = std::io::Cursor::new(data);
    let mut decoder_reader = DecoderReader::new(cursor, &mock_engine);
    let mut buf = [0u8; 3];

    decoder_reader.b64_offset = BUF_SIZE;
    decoder_reader.b64_len = 0;

    let result = decoder_reader.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3);
    assert_eq!(&buf, b"Hel");
}

#[test]
fn test_read_with_partial_read() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 2, padding_offset: None })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let mock_engine = MockEngine;
    let data = b"SGVsbG8gd29ybGQh"; // Base64 for "Hello world!"
    let mut cursor = std::io::Cursor::new(data);
    let mut decoder_reader = DecoderReader::new(cursor, &mock_engine);
    let mut buf = [0u8; 2];

    decoder_reader.b64_offset = BUF_SIZE;
    decoder_reader.b64_len = 0;

    let result = decoder_reader.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);
    assert_eq!(&buf, b"He");
}

#[test]
fn test_read_when_eof() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Err(DecodeSliceError::OutputSliceTooSmall)
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let mock_engine = MockEngine;
    let data = b""; // Empty input
    let mut cursor = std::io::Cursor::new(data);
    let mut decoder_reader = DecoderReader::new(cursor, &mock_engine);
    let mut buf = [0u8; 4];

    decoder_reader.b64_offset = BUF_SIZE;
    decoder_reader.b64_len = 0;

    let result = decoder_reader.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

