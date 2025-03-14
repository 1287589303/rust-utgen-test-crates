// Answer 0

fn test_write_all_encoded_output_error() -> Result<()> {
    use std::io::{self, ErrorKind};

    struct MockWriter {
        output_occupied_len: usize,
        write_result: Result<(), io::Error>,
    }

    impl MockWriter {
        fn new(output_occupied_len: usize, write_result: Result<(), io::Error>) -> Self {
            Self {
                output_occupied_len,
                write_result,
            }
        }

        fn write_to_delegate(&mut self, _: usize) -> Result<(), io::Error> {
            self.write_result.clone()
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
    }

    // Test case: Simulate an IO error that is not interrupted
    let mut writer = MockWriter::new(10, Err(io::Error::new(ErrorKind::Other, "failed write")));
    let result = writer.write_all_encoded_output();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Other);

    Ok(())
}

fn test_write_all_encoded_output_interrupted() -> Result<()> {
    use std::io::{self, ErrorKind};

    struct MockWriter {
        output_occupied_len: usize,
        write_result: Result<(), io::Error>,
    }

    impl MockWriter {
        fn new(output_occupied_len: usize, write_result: Result<(), io::Error>) -> Self {
            Self {
                output_occupied_len,
                write_result,
            }
        }

        fn write_to_delegate(&mut self, _: usize) -> Result<(), io::Error> {
            self.write_result.clone()
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
    }

    // Test case: Simulate an interrupted error
    let mut writer = MockWriter::new(10, Err(io::Error::new(ErrorKind::Interrupted, "interrupted")));
    let result = writer.write_all_encoded_output();
    assert!(result.is_ok());

    Ok(())
}

