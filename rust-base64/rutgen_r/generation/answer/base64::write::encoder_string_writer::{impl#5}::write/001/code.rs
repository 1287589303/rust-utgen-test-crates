// Answer 0

#[test]
fn test_write_valid_utf8() {
    struct StrConsumer {
        output: String,
    }

    impl StrConsumer {
        fn new() -> Self {
            StrConsumer { output: String::new() }
        }
        
        fn consume(&mut self, s: &str) {
            self.output.push_str(s);
        }
    }

    struct EncoderStringWriter {
        str_consumer: StrConsumer,
    }

    impl EncoderStringWriter {
        fn new(consumer: StrConsumer) -> Self {
            EncoderStringWriter { str_consumer: consumer }
        }
        
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let s = std::str::from_utf8(buf).expect("Input must be valid UTF-8");
            self.str_consumer.consume(s);
            Ok(buf.len())
        }
    }

    let consumer = StrConsumer::new();
    let mut writer = EncoderStringWriter::new(consumer);
    let buf = b"valid utf8 string";

    let result = writer.write(buf).unwrap();
    
    assert_eq!(result, buf.len());
}

#[test]
fn test_write_empty_string() {
    struct StrConsumer {
        output: String,
    }

    impl StrConsumer {
        fn new() -> Self {
            StrConsumer { output: String::new() }
        }
        
        fn consume(&mut self, s: &str) {
            self.output.push_str(s);
        }
    }

    struct EncoderStringWriter {
        str_consumer: StrConsumer,
    }

    impl EncoderStringWriter {
        fn new(consumer: StrConsumer) -> Self {
            EncoderStringWriter { str_consumer: consumer }
        }
        
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let s = std::str::from_utf8(buf).expect("Input must be valid UTF-8");
            self.str_consumer.consume(s);
            Ok(buf.len())
        }
    }

    let consumer = StrConsumer::new();
    let mut writer = EncoderStringWriter::new(consumer);
    let buf: &[u8] = b"";

    let result = writer.write(buf).unwrap();
    
    assert_eq!(result, buf.len());
}

