// Answer 0

#[test]
fn test_into_inner() {
    struct TestEngine;
    struct TestConfig;
    struct TestDecodeEstimate;
    
    impl Engine for TestEngine {
        type Config = TestConfig;
        type DecodeEstimate = TestDecodeEstimate;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            TestDecodeEstimate
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
            &TestConfig
        }
    }

    struct TestStrConsumer {
        content: String,
    }

    impl StrConsumer for TestStrConsumer {
        fn consume(&mut self, buf: &str) {
            self.content.push_str(buf);
        }
    }

    let engine = TestEngine;
    let consumer = TestStrConsumer {
        content: String::new(),
    };

    let mut encoder_string_writer = EncoderStringWriter::from_consumer(consumer, &engine);
    encoder_string_writer.into_inner();

    assert_eq!(encoder_string_writer.encoder.delegate.is_none(), true);
}

