// Answer 0

#[test]
fn test_encoder_new() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl std::io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockEngine;

    let mut mock_writer = MockWriter { data: Vec::new() };
    let mock_engine = MockEngine;

    let encoder = base64::new(&mut mock_writer, &mock_engine);

    assert!(encoder.delegate.is_some());
    assert_eq!(encoder.extra_input.len(), base64::MIN_ENCODE_CHUNK_SIZE);
    assert_eq!(encoder.output.len(), base64::BUF_SIZE);
    assert_eq!(encoder.output_occupied_len, 0);
    assert_eq!(encoder.extra_input_occupied_len, 0);
    assert!(!encoder.panicked);
}

