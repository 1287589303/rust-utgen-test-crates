// Answer 0

#[test]
fn test_write_with_one_byte_input() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..4].copy_from_slice(&input[0..1]);
            4
        }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }
        fn config(&self) -> &Self::Config {
            &()
        }
    }
    
    let engine = TestEngine;
    let mut output_buf = [0u8; BUF_SIZE];
    let mut writer = EncoderWriter::new(io::sink(), &engine);
    let input: &[u8] = b"a";
    let res = writer.write(input);
}

#[test]
fn test_write_with_two_bytes_input() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..4].copy_from_slice(&input[0..2]);
            4
        }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }
        fn config(&self) -> &Self::Config {
            &()
        }
    }
    
    let engine = TestEngine;
    let mut output_buf = [0u8; BUF_SIZE];
    let mut writer = EncoderWriter::new(io::sink(), &engine);
    let input: &[u8] = b"ab";
    let res = writer.write(input);
}

