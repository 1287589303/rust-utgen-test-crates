// Answer 0

#[test]
fn test_encode_remaining_data_no_partial_input() {
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
    }

    struct MockStrConsumer {
        consumed: String,
    }

    impl StrConsumer for MockStrConsumer {
        fn consume(&mut self, buf: &str) {
            self.consumed.push_str(buf);
        }
    }

    let engine = MockEngine;
    let consumer = MockStrConsumer {
        consumed: String::new(),
    };
    let encoder_string_writer = EncoderStringWriter::from_consumer(consumer, &engine);
    let final_consumer = encoder_string_writer.into_inner();

    assert!(final_consumer.consumed.is_empty());
}

#[test]
fn test_encode_remaining_data_with_partial_input() {
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
    }

    struct MockStrConsumer {
        consumed: String,
    }

    impl StrConsumer for MockStrConsumer {
        fn consume(&mut self, buf: &str) {
            self.consumed.push_str(buf);
        }
    }

    let engine = MockEngine;
    let mut consumer = MockStrConsumer {
        consumed: String::new(),
    };
    let encoder_string_writer = EncoderStringWriter::from_consumer(consumer, &engine);
    
    // Simulate some buffered data being added
    consumer.consume("Hello, "); // Simulating partial input
    let final_consumer = encoder_string_writer.into_inner();

    assert_eq!(final_consumer.consumed, "Hello, ");
}

