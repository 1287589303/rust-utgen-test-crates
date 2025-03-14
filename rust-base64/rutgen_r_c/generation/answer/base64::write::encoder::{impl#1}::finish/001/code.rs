// Answer 0

#[test]
fn test_finish_success() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }
        fn config(&self) -> &Self::Config { &() }

        #[cfg(any(feature = "alloc", test))]
        fn encode<T: AsRef<[u8]>>(&self, _: T) -> String { String::new() }
        
        #[cfg(any(feature = "alloc", test))]
        fn encode_string<T: AsRef<[u8]>>(&self, _: T, _: &mut String) {}

        #[cfg(any(feature = "alloc", test))]
        fn encode_slice<T: AsRef<[u8]>>(&self, _: T, _: &mut [u8]) -> Result<usize, EncodeSliceError> {
            Ok(0)
        }

        #[cfg(any(feature = "alloc", test))]
        fn decode<T: AsRef<[u8]>>(&self, _: T) -> Result<Vec<u8>, DecodeError> {
            Ok(Vec::new())
        }

        #[cfg(any(feature = "alloc", test))]
        fn decode_vec<T: AsRef<[u8]>>(&self, _: T, _: &mut Vec<u8>) -> Result<(), DecodeError> {
            Ok(())
        }

        fn decode_slice<T: AsRef<[u8]>>(&self, _: T, _: &mut [u8]) -> Result<usize, DecodeSliceError> {
            Ok(0)
        }
        
        fn decode_slice_unchecked<T: AsRef<[u8]>>(&self, _: T, _: &mut [u8]) -> Result<usize, DecodeError> {
            Ok(0)
        }
    }

    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let engine = MockEngine;
    let writer = MockWriter;
    let mut encoder = EncoderWriter::new(writer, &engine);

    assert!(encoder.delegate.is_some());
    let result = encoder.finish();
    assert!(result.is_ok());
}

#[test]
fn test_finish_write_final_leftovers_error() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }
        fn config(&self) -> &Self::Config { &() }

        #[cfg(any(feature = "alloc", test))]
        fn encode<T: AsRef<[u8]>>(&self, _: T) -> String { String::new() }
        
        #[cfg(any(feature = "alloc", test))]
        fn encode_string<T: AsRef<[u8]>>(&self, _: T, _: &mut String) {}

        #[cfg(any(feature = "alloc", test))]
        fn encode_slice<T: AsRef<[u8]>>(&self, _: T, _: &mut [u8]) -> Result<usize, EncodeSliceError> {
            Ok(0)
        }

        #[cfg(any(feature = "alloc", test))]
        fn decode<T: AsRef<[u8]>>(&self, _: T) -> Result<Vec<u8>, DecodeError> {
            Ok(Vec::new())
        }

        #[cfg(any(feature = "alloc", test))]
        fn decode_vec<T: AsRef<[u8]>>(&self, _: T, _: &mut Vec<u8>) -> Result<(), DecodeError> {
            Ok(())
        }

        fn decode_slice<T: AsRef<[u8]>>(&self, _: T, _: &mut [u8]) -> Result<usize, DecodeSliceError> {
            Ok(0)
        }
        
        fn decode_slice_unchecked<T: AsRef<[u8]>>(&self, _: T, _: &mut [u8]) -> Result<usize, DecodeError> {
            Ok(0)
        }
    }

    struct MockFaultyWriter;

    impl io::Write for MockFaultyWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Err(io::Error::new(ErrorKind::Other, "Write error"))
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let engine = MockEngine;
    let writer = MockFaultyWriter;
    let mut encoder = EncoderWriter::new(writer, &engine);

    assert!(encoder.delegate.is_some());
    let result = encoder.finish();
    assert!(result.is_err());
}

