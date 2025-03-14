// Answer 0

#[test]
fn test_into_inner_success() {
    struct TestWriter {
        content: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { content: Vec::new() }
        }

        fn write(&mut self, data: &[u8]) {
            self.content.extend_from_slice(data);
        }
    }

    struct EncoderWriter<W> {
        delegate: Option<W>,
    }

    impl<W> EncoderWriter<W> {
        fn new(writer: W) -> Self {
            EncoderWriter { delegate: Some(writer) }
        }

        fn finish(mut self) -> W {
            self.delegate.take().expect("Encoder has already been finished")
        }

        fn into_inner(mut self) -> W {
            self.delegate
                .take()
                .expect("Encoder has already had finish() called")
        }
    }

    let writer = TestWriter::new();
    let encoder = EncoderWriter::new(writer);
    
    let inner_writer = encoder.into_inner();

    assert!(inner_writer.content.is_empty());
}

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_into_inner_after_finish() {
    struct TestWriter {
        content: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { content: Vec::new() }
        }
    }

    struct EncoderWriter<W> {
        delegate: Option<W>,
    }

    impl<W> EncoderWriter<W> {
        fn new(writer: W) -> Self {
            EncoderWriter { delegate: Some(writer) }
        }

        fn finish(mut self) -> W {
            self.delegate.take().expect("Encoder has already been finished")
        }

        fn into_inner(mut self) -> W {
            self.delegate
                .take()
                .expect("Encoder has already had finish() called")
        }
    }

    let writer = TestWriter::new();
    let encoder = EncoderWriter::new(writer);
    let _inner_writer = encoder.finish();
    
    encoder.into_inner(); // This should panic.
}

