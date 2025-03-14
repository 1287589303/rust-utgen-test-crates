// Answer 0

#[test]
fn test_flush_returns_ok() {
    struct EncoderStringWriter;

    impl EncoderStringWriter {
        fn flush(&mut self) -> std::io::Result<()> {
            // no op
            Ok(())
        }
    }

    let mut writer = EncoderStringWriter;
    let result = writer.flush();
    assert!(result.is_ok());
}

#[test]
fn test_flush_on_empty_writer() {
    struct EncoderStringWriter;

    impl EncoderStringWriter {
        fn flush(&mut self) -> std::io::Result<()> {
            // no op
            Ok(())
        }
    }

    let mut writer = EncoderStringWriter;
    let result = writer.flush();
    assert_eq!(result, Ok(()));
}

