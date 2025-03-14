// Answer 0

fn test_read_buf_not_empty_zero_decoded_len() -> Result<(), Box<dyn std::error::Error>> {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            output.copy_from_slice(&input[0..3]); // assert successful decoding
            Ok(DecodeMetadata { decoded_len: 3, padding_offset: None })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let mut reader = DecoderReader::new(&mut b"SGVsbG8gV29ybGQ=".as_ref(), &engine);
    let mut buf = [0u8; 1]; // buf.len() < DECODED_CHUNK_SIZE

    let bytes_read = reader.read(&mut buf)?;

    assert_eq!(bytes_read, 1);
    assert_eq!(buf[0], b'H'); // Check decoded byte

    Ok(())
}

fn test_read_buf_empty() -> Result<(), Box<dyn std::error::Error>> {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Err(DecodeSliceError::OutputSliceTooSmall)
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let mut reader = DecoderReader::new(&mut b"SGVsbG8gV29ybGQ=".as_ref(), &engine);
    
    let mut buf = [0u8; 1024];
    let bytes_read = reader.read(&mut buf)?;

    assert_eq!(bytes_read, 0); // No bytes should have been read

    Ok(())
}

