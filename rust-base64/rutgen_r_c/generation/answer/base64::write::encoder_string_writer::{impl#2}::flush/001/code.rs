// Answer 0

#[test]
fn test_encoder_string_writer_flush() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
            input.len()
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }

        fn config(&self) -> &Self::Config {
            &()
        }

        fn encode<T: AsRef<[u8]>>(&self, _: T) -> String {
            String::new()
        }

        fn encode_string<T: AsRef<[u8]>>(&self, _: T, _: &mut String) {}

        fn encode_slice<T: AsRef<[u8]>>(&self, _: T, _: &mut [u8]) -> Result<usize, EncodeSliceError> {
            Ok(0)
        }

        fn decode<T: AsRef<[u8]>>(&self, _: T) -> Result<Vec<u8>, DecodeError> {
            Ok(vec![])
        }

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

    struct MockStrConsumer;

    impl StrConsumer for MockStrConsumer {
        fn consume(&mut self, _: &str) {}
    }

    let engine = MockEngine;
    let consumer = MockStrConsumer;
    let mut writer = EncoderStringWriter {
        encoder: EncoderWriter {
            engine: &engine,
            delegate: None,
            extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
            extra_input_occupied_len: 0,
            output: [0; BUF_SIZE],
            output_occupied_len: 0,
            panicked: false,
        },
    };

    let result = writer.flush();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_encoder_string_writer_flush_panic_on_flush() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }

        fn config(&self) -> &Self::Config {
            &()
        }

        fn encode<T: AsRef<[u8]>>(&self, _: T) -> String {
            String::new()
        }

        fn encode_string<T: AsRef<[u8]>>(&self, _: T, _: &mut String) {}

        fn encode_slice<T: AsRef<[u8]>>(&self, _: T, _: &mut [u8]) -> Result<usize, EncodeSliceError> {
            Ok(0)
        }

        fn decode<T: AsRef<[u8]>>(&self, _: T) -> Result<Vec<u8>, DecodeError> {
            Ok(vec![])
        }

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

    struct MockStrConsumer;

    impl StrConsumer for MockStrConsumer {
        fn consume(&mut self, _: &str) {
            panic!("Flush failed");
        }
    }

    let engine = MockEngine;
    let consumer = MockStrConsumer;
    let mut writer = EncoderStringWriter {
        encoder: EncoderWriter {
            engine: &engine,
            delegate: None,
            extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
            extra_input_occupied_len: 0,
            output: [0; BUF_SIZE],
            output_occupied_len: 0,
            panicked: false,
        },
    };

    writer.flush();
}

