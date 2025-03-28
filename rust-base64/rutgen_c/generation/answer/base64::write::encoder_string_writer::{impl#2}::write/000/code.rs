// Answer 0

#[test]
fn test_encoder_string_writer_write() {
    struct TestEngine;

    impl Engine for TestEngine {
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
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    struct TestStrConsumer {
        buffer: String,
    }

    impl StrConsumer for TestStrConsumer {
        fn consume(&mut self, buf: &str) {
            self.buffer.push_str(buf);
        }
    }

    let engine = TestEngine;
    let str_consumer = TestStrConsumer { buffer: String::new() };
    let mut writer = EncoderStringWriter {
        encoder: EncoderWriter {
            engine: &engine,
            delegate: None,
            extra_input: [0; 16],
            extra_input_occupied_len: 0,
            output: [0; 16],
            output_occupied_len: 0,
            panicked: false,
        },
        str_consumer,
    };

    let data = b"Test data";
    let result = writer.write(data).unwrap();

    assert_eq!(result, data.len());
}

