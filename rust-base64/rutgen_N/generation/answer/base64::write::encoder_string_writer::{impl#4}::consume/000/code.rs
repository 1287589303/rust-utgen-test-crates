// Answer 0

#[derive(Default)]
struct EncoderStringWriter {
    value: String,
}

impl EncoderStringWriter {
    fn push_str(&mut self, buf: &str) {
        self.value.push_str(buf);
    }

    fn consume(&mut self, buf: &str) {
        self.push_str(buf);
    }
}

#[test]
fn test_consume_empty_string() {
    let mut writer = EncoderStringWriter::default();
    writer.consume("");
    assert_eq!(writer.value, "");
}

#[test]
fn test_consume_non_empty_string() {
    let mut writer = EncoderStringWriter::default();
    writer.consume("Hello, ");
    assert_eq!(writer.value, "Hello, ");

    writer.consume("world!");
    assert_eq!(writer.value, "Hello, world!");
}

#[test]
fn test_consume_multiple_times() {
    let mut writer = EncoderStringWriter::default();
    writer.consume("First ");
    writer.consume("Second ");
    writer.consume("Third");
    assert_eq!(writer.value, "First Second Third");
}

#[test]
fn test_consume_large_string() {
    let mut writer = EncoderStringWriter::default();
    let large_string = "A".repeat(1000);
    writer.consume(&large_string);
    assert_eq!(writer.value, large_string);
}

