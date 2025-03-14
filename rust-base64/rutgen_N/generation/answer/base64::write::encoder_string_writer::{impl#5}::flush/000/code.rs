// Answer 0

#[test]
fn test_flush() {
    struct DummyWriter;

    impl DummyWriter {
        fn new() -> Self {
            DummyWriter
        }
    }

    impl std::io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            // no op for the purpose of this test
            Ok(())
        }
    }

    let mut writer = DummyWriter::new();
    let result = writer.flush();
    assert!(result.is_ok());
}

