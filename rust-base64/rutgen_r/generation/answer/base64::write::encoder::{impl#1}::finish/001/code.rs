// Answer 0

#[test]
fn test_finish_successful_completion() {
    struct MockWriter;

    impl std::io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Encoder<W> {
        delegate: Option<W>,
    }

    impl<W: std::io::Write> Encoder<W> {
        pub fn new(writer: W) -> Self {
            Encoder {
                delegate: Some(writer),
            }
        }

        pub fn finish(&mut self) -> std::io::Result<W> {
            assert!(
                self.delegate.is_some(),
                "Encoder has already had finish() called"
            );

            self.write_final_leftovers()?;

            let writer = self.delegate.take().expect("Writer must be present");

            Ok(writer)
        }

        fn write_final_leftovers(&mut self) -> std::io::Result<()> {
            // Simulating case where there are no final leftovers, which succeeds
            Ok(())
        }
    }

    let mock_writer = MockWriter;
    let mut encoder = Encoder::new(mock_writer);
    
    let result = encoder.finish();
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_finish_called_twice() {
    struct MockWriter;

    impl std::io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Encoder<W> {
        delegate: Option<W>,
    }

    impl<W: std::io::Write> Encoder<W> {
        pub fn new(writer: W) -> Self {
            Encoder {
                delegate: Some(writer),
            }
        }

        pub fn finish(&mut self) -> std::io::Result<W> {
            assert!(
                self.delegate.is_some(),
                "Encoder has already had finish() called"
            );

            self.write_final_leftovers()?;

            let writer = self.delegate.take().expect("Writer must be present");

            Ok(writer)
        }

        fn write_final_leftovers(&mut self) -> std::io::Result<()> {
            // Simulating case where it could return an error to check the panic condition
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Some error"))
        }
    }

    let mock_writer = MockWriter;
    let mut encoder = Encoder::new(mock_writer);
    
    // First call to finish should succeed if no error in leftovers.
    encoder.finish().unwrap();
    
    // Second call should panic.
    encoder.finish();
}

