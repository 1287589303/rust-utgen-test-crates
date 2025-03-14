// Answer 0

#[derive(Debug)]
struct MockWriter {
    output: String,
}

impl MockWriter {
    fn new() -> Self {
        MockWriter {
            output: String::new(),
        }
    }

    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        self.output.push_str(s);
        Ok(())
    }
}

struct Base64Display<'a> {
    f: &'a mut MockWriter,
}

impl<'a> Base64Display<'a> {
    fn write_encoded_bytes(&mut self, encoded: &[u8]) -> Result<(), std::fmt::Error> {
        self.f
            .write_str(std::str::from_utf8(encoded).expect("base64 data was not utf8"))
    }
}

#[test]
fn test_write_encoded_bytes_basic() {
    let mut writer = MockWriter::new();
    let mut display = Base64Display { f: &mut writer };
    let encoded: &[u8] = b"SGVsbG8gV29ybGQ="; // "Hello World" in base64

    assert!(display.write_encoded_bytes(encoded).is_ok());
    assert_eq!(writer.output, "SGVsbG8gV29ybGQ=");
}

#[test]
fn test_write_encoded_bytes_empty() {
    let mut writer = MockWriter::new();
    let mut display = Base64Display { f: &mut writer };
    let encoded: &[u8] = b""; // empty base64

    assert!(display.write_encoded_bytes(encoded).is_ok());
    assert_eq!(writer.output, "");
}

#[test]
#[should_panic(expected = "base64 data was not utf8")]
fn test_write_encoded_bytes_invalid_utf8() {
    let mut writer = MockWriter::new();
    let mut display = Base64Display { f: &mut writer };
    let encoded: &[u8] = &[0xff]; // invalid UTF-8 byte

    display.write_encoded_bytes(encoded).unwrap();
}

