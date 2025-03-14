// Answer 0

#[test]
fn test_flush_success() {
    struct MockEncoder {
        flush_called: bool,
    }

    impl MockEncoder {
        fn flush(&mut self) -> io::Result<()> {
            self.flush_called = true;
            Ok(())
        }
    }

    struct EncoderStringWriter {
        encoder: MockEncoder,
    }

    let mut writer = EncoderStringWriter {
        encoder: MockEncoder { flush_called: false },
    };

    let result = writer.flush();
    assert!(result.is_ok());
    assert!(writer.encoder.flush_called);
}

#[test]
#[should_panic]
fn test_flush_panic() {
    struct MockEncoder;

    impl MockEncoder {
        fn flush(&mut self) -> io::Result<()> {
            panic!("Flush failed due to panic.");
        }
    }

    struct EncoderStringWriter {
        encoder: MockEncoder,
    }

    let mut writer = EncoderStringWriter { encoder: MockEncoder };

    writer.flush();
}

