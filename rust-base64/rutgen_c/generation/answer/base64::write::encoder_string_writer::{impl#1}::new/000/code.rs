// Answer 0

#[test]
fn test_encoder_string_writer_new() {
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
        ) -> Result<(), ()> {
            Ok(())
        }

        fn config(&self) -> &Self::Config {
            &()
        }

        fn encode<T: AsRef<[u8]>>(&self, _input: T) -> String {
            String::new()
        }

        fn encode_string<T: AsRef<[u8]>>(&self, _input: T, _output_buf: &mut String) {}

        fn encode_slice<T: AsRef<[u8]>>(
            &self,
            _input: T,
            _output_buf: &mut [u8],
        ) -> Result<usize, ()> {
            Ok(0)
        }

        fn decode<T: AsRef<[u8]>>(&self, _input: T) -> Result<Vec<u8>, ()> {
            Ok(vec![])
        }

        fn decode_vec<T: AsRef<[u8]>>(
            &self,
            _input: T,
            _buffer: &mut Vec<u8>,
        ) -> Result<(), ()> {
            Ok(())
        }

        fn decode_slice<T: AsRef<[u8]>>(
            &self,
            _input: T,
            _output: &mut [u8],
        ) -> Result<usize, ()> {
            Ok(0)
        }

        fn decode_slice_unchecked<T: AsRef<[u8]>>(
            &self,
            _input: T,
            _output: &mut [u8],
        ) -> Result<usize, ()> {
            Ok(0)
        }
    }

    let engine = MockEngine;
    let writer = EncoderStringWriter::new(&engine);
    assert_eq!(writer.encoder.delegate, None);
}

