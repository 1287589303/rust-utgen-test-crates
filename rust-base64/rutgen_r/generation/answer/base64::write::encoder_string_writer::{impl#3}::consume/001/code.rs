// Answer 0

#[test]
fn test_consume_with_non_empty_buffer() {
    struct EncoderStringWriter {
        data: String,
    }

    impl EncoderStringWriter {
        fn consume(&mut self, buf: &str) {
            self.data.push_str(buf);
        }
    }

    let mut writer = EncoderStringWriter { data: String::new() };
    writer.consume("Hello, ");
    writer.consume("world!");
    assert_eq!(writer.data, "Hello, world!");
}

#[test]
fn test_consume_with_empty_buffer() {
    struct EncoderStringWriter {
        data: String,
    }

    impl EncoderStringWriter {
        fn consume(&mut self, buf: &str) {
            self.data.push_str(buf);
        }
    }

    let mut writer = EncoderStringWriter { data: String::new() };
    writer.consume("");
    assert_eq!(writer.data, "");
}

#[test]
fn test_consume_multiple_calls() {
    struct EncoderStringWriter {
        data: String,
    }

    impl EncoderStringWriter {
        fn consume(&mut self, buf: &str) {
            self.data.push_str(buf);
        }
    }

    let mut writer = EncoderStringWriter { data: String::new() };
    writer.consume("first ");
    writer.consume("second ");
    writer.consume("third");
    assert_eq!(writer.data, "first second third");
}

#[test]
fn test_consume_boundary_condition() {
    struct EncoderStringWriter {
        data: String,
    }

    impl EncoderStringWriter {
        fn consume(&mut self, buf: &str) {
            self.data.push_str(buf);
        }
    }

    let mut writer = EncoderStringWriter { data: String::new() };
    let long_string = "x".repeat(1000); // Test with a long string
    writer.consume(&long_string);
    assert_eq!(writer.data, long_string);
}

