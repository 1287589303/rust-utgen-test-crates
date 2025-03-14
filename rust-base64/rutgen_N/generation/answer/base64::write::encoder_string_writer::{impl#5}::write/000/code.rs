// Answer 0

#[test]
fn test_write_valid_utf8() {
    struct TestConsumer {
        output: String,
    }

    impl TestConsumer {
        fn consume(&mut self, s: &str) {
            self.output.push_str(s);
        }
    }

    struct EncoderStringWriter {
        str_consumer: TestConsumer,
    }

    impl EncoderStringWriter {
        fn new() -> Self {
            Self {
                str_consumer: TestConsumer {
                    output: String::new(),
                },
            }
        }

        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let s = std::str::from_utf8(buf).expect("Input must be valid UTF-8");
            self.str_consumer.consume(s);
            Ok(buf.len())
        }
    }

    let mut writer = EncoderStringWriter::new();
    let input = b"Valid UTF-8 string";
    let result = writer.write(input).unwrap();
    
    assert_eq!(result, input.len());
    assert_eq!(writer.str_consumer.output, "Valid UTF-8 string");
}

#[test]
#[should_panic]
fn test_write_invalid_utf8() {
    struct TestConsumer {
        output: String,
    }

    impl TestConsumer {
        fn consume(&mut self, s: &str) {
            self.output.push_str(s);
        }
    }

    struct EncoderStringWriter {
        str_consumer: TestConsumer,
    }

    impl EncoderStringWriter {
        fn new() -> Self {
            Self {
                str_consumer: TestConsumer {
                    output: String::new(),
                },
            }
        }

        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let s = std::str::from_utf8(buf).expect("Input must be valid UTF-8");
            self.str_consumer.consume(s);
            Ok(buf.len())
        }
    }

    let mut writer = EncoderStringWriter::new();
    let input: &[u8] = &[0xFF]; // Invalid UTF-8 byte
    writer.write(input).unwrap(); // This should panic
}

