// Answer 0

#[test]
fn test_finish_success() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { data: Vec::new() }
        }
        
        fn get_data(self) -> Vec<u8> {
            self.data
        }
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

    let mut encoder = Encoder::new(MockWriter::new());
    // Simulate writing some data to the encoder here...
    
    let result = encoder.finish();
    assert!(result.is_ok());
    // Add assertions for the final state if necessary...
}

#[test]
#[should_panic]
fn test_finish_already_called() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { data: Vec::new() }
        }
        
        fn get_data(self) -> Vec<u8> {
            self.data
        }
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

    let mut encoder = Encoder::new(MockWriter::new());
    encoder.finish(); // Call finish the first time

    // This should panic as finish is already called once
    encoder.finish(); // Call finish the second time
}

