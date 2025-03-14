// Answer 0

#[test]
fn test_write_encoded_bytes_success() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Encoder {
        f: TestWriter,
    }

    impl Encoder {
        type Error = std::fmt::Error;

        fn write_encoded_bytes(&mut self, encoded: &[u8]) -> Result<(), Self::Error> {
            self.f.write_str(std::str::from_utf8(encoded).expect("base64 data was not utf8"))
        }
    }

    let mut encoder = Encoder { f: TestWriter::new() };
    let encoded_data = b"Hello, World!";
    let result = encoder.write_encoded_bytes(encoded_data);
    assert!(result.is_ok());
    assert_eq!(encoder.f.output, "Hello, World!");
}

#[test]
#[should_panic(expected = "base64 data was not utf8")]
fn test_write_encoded_bytes_invalid_utf8() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Encoder {
        f: TestWriter,
    }

    impl Encoder {
        type Error = std::fmt::Error;

        fn write_encoded_bytes(&mut self, encoded: &[u8]) -> Result<(), Self::Error> {
            self.f.write_str(std::str::from_utf8(encoded).expect("base64 data was not utf8"))
        }
    }

    let mut encoder = Encoder { f: TestWriter::new() };
    let invalid_encoded_data = &[0xFF, 0xFE]; // Invalid UTF-8 bytes
    encoder.write_encoded_bytes(invalid_encoded_data).unwrap();
}

