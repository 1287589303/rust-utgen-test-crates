// Answer 0

fn test_write_all_encoded_output_zero_length() {
    struct MockWriter {
        output_occupied_len: usize,
    }

    impl MockWriter {
        fn new() -> Self {
            Self {
                output_occupied_len: 0,
            }
        }

        fn write_all_encoded_output(&mut self) -> Result<()> {
            while self.output_occupied_len > 0 {
                let remaining_len = self.output_occupied_len;
                match self.write_to_delegate(remaining_len) {
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                    Ok(()) => {}
                };
            }
            debug_assert_eq!(0, self.output_occupied_len);
            Ok(())
        }

        fn write_to_delegate(&self, _length: usize) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter::new();
    let result = writer.write_all_encoded_output();
    assert!(result.is_ok());
}

fn test_write_all_encoded_output_non_zero_length() {
    struct MockWriter {
        output_occupied_len: usize,
    }

    impl MockWriter {
        fn new(length: usize) -> Self {
            Self {
                output_occupied_len: length,
            }
        }

        fn write_all_encoded_output(&mut self) -> Result<()> {
            while self.output_occupied_len > 0 {
                let remaining_len = self.output_occupied_len;
                match self.write_to_delegate(remaining_len) {
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                    Ok(()) => {}
                };
            }
            debug_assert_eq!(0, self.output_occupied_len);
            Ok(())
        }

        fn write_to_delegate(&self, _length: usize) -> Result<()> {
            // Simulate an operation that does not change output_occupied_len
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Test error"))
        }
    }

    let mut writer = MockWriter::new(1);
    let result = writer.write_all_encoded_output();
    assert!(result.is_err());
}

