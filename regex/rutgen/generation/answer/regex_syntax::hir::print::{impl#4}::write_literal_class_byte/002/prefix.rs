// Answer 0

#[test]
fn test_write_literal_class_byte_space() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    writer.write_literal_class_byte(0x20).unwrap();
}

#[test]
fn test_write_literal_class_byte_tilde() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    writer.write_literal_class_byte(0x7E).unwrap();
}

