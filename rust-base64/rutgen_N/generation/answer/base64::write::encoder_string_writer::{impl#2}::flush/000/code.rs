// Answer 0

#[test]
fn test_flush_success() -> Result<(), std::io::Error> {
    struct MockEncoder {
        should_flush_succeed: bool,
    }

    impl MockEncoder {
        fn flush(&mut self) -> io::Result<()> {
            if self.should_flush_succeed {
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "Flush failed"))
            }
        }
    }

    struct EncoderStringWriter {
        encoder: MockEncoder,
    }

    impl EncoderStringWriter {
        fn flush(&mut self) -> io::Result<()> {
            self.encoder.flush()
        }
    }

    let mut writer = EncoderStringWriter {
        encoder: MockEncoder {
            should_flush_succeed: true,
        },
    };

    assert_eq!(writer.flush().is_ok(), true);

    Ok(())
}

#[test]
#[should_panic(expected = "Flush failed")]
fn test_flush_failure() {
    struct MockEncoder {
        should_flush_succeed: bool,
    }

    impl MockEncoder {
        fn flush(&mut self) -> io::Result<()> {
            if self.should_flush_succeed {
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "Flush failed"))
            }
        }
    }

    struct EncoderStringWriter {
        encoder: MockEncoder,
    }

    impl EncoderStringWriter {
        fn flush(&mut self) -> io::Result<()> {
            self.encoder.flush()
        }
    }

    let mut writer = EncoderStringWriter {
        encoder: MockEncoder {
            should_flush_succeed: false,
        },
    };

    writer.flush().expect("Flush should succeed");
}

