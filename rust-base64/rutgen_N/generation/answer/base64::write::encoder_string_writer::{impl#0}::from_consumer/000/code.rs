// Answer 0

#[test]
fn test_from_consumer() {
    struct MockStrConsumer {
        output: String,
    }

    impl MockStrConsumer {
        fn new() -> Self {
            MockStrConsumer { output: String::new() }
        }
    }

    struct MockEngine;

    struct EncoderWriter {
        consumer: Utf8SingleCodeUnitWriter<MockStrConsumer>,
        engine: MockEngine,
    }

    struct Utf8SingleCodeUnitWriter<S> {
        str_consumer: S,
    }

    struct EncoderStringWriter<S, E> {
        encoder: EncoderWriter,
    }

    impl<S, E> EncoderStringWriter<S, E> {
        pub fn from_consumer(str_consumer: S, engine: &'_ E) -> Self {
            EncoderStringWriter {
                encoder: EncoderWriter::new(Utf8SingleCodeUnitWriter { str_consumer }, engine),
            }
        }
    }

    impl EncoderWriter {
        fn new(consumer: Utf8SingleCodeUnitWriter<MockStrConsumer>, engine: &MockEngine) -> Self {
            EncoderWriter { consumer, engine: engine.clone() }
        }
    }

    let mock_consumer = MockStrConsumer::new();
    let mock_engine = MockEngine;
    let encoder_string_writer = EncoderStringWriter::from_consumer(mock_consumer, &mock_engine);

    assert!(encoder_string_writer.encoder.consumer.str_consumer.output.is_empty());
}

