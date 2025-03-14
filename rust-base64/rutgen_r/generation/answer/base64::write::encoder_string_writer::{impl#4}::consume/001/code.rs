// Answer 0

#[derive(Default)]
struct StringWriter {
    content: String,
}

impl StringWriter {
    fn push_str(&mut self, s: &str) {
        self.content.push_str(s);
    }

    fn get_content(&self) -> &str {
        &self.content
    }
}

#[test]
fn test_consume_empty_string() {
    let mut writer = StringWriter::default();
    writer.consume("");
    assert_eq!(writer.get_content(), "");
}

#[test]
fn test_consume_single_character() {
    let mut writer = StringWriter::default();
    writer.consume("a");
    assert_eq!(writer.get_content(), "a");
}

#[test]
fn test_consume_multiple_characters() {
    let mut writer = StringWriter::default();
    writer.consume("hello");
    assert_eq!(writer.get_content(), "hello");
}

#[test]
fn test_consume_overwriting() {
    let mut writer = StringWriter::default();
    writer.consume("first");
    writer.consume("second");
    assert_eq!(writer.get_content(), "firstsecond");
}

#[test]
fn test_consume_large_string() {
    let mut writer = StringWriter::default();
    let large_input = "A".repeat(1000);
    writer.consume(&large_input);
    assert_eq!(writer.get_content(), &large_input);
}

#[test]
fn test_consume_multiple_empty_strings() {
    let mut writer = StringWriter::default();
    writer.consume("");
    writer.consume("");
    assert_eq!(writer.get_content(), "");
}

