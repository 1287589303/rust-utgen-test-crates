// Answer 0

#[test]
fn test_flush_with_write_all_encoded_output_error() {
    struct MockDelegate {
        should_fail_flush: bool,
    }

    impl std::io::Write for MockDelegate {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            if self.should_fail_flush {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Flush error"));
            }
            Ok(())
        }
    }

    struct Encoder {
        delegate: Option<MockDelegate>,
    }

    impl Encoder {
        fn write_all_encoded_output(&mut self) -> Result<()> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Encoding error"))
        }

        fn flush(&mut self) -> Result<()> {
            self.write_all_encoded_output()?;
            self.delegate
                .as_mut()
                .expect("Writer must be present")
                .flush()
        }
    }

    let mut encoder = Encoder {
        delegate: Some(MockDelegate { should_fail_flush: false }),
    };

    let result = encoder.flush();
    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "Writer must be present")]
fn test_flush_without_delegate() {
    struct Encoder {
        delegate: Option<MockDelegate>,
    }

    impl Encoder {
        fn write_all_encoded_output(&mut self) -> Result<()> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Encoding error"))
        }

        fn flush(&mut self) -> Result<()> {
            self.write_all_encoded_output()?;
            self.delegate
                .as_mut()
                .expect("Writer must be present")
                .flush()
        }
    }

    let mut encoder = Encoder {
        delegate: None,
    };

    let _ = encoder.flush();
}

