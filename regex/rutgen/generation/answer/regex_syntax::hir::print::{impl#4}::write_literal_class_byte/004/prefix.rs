// Answer 0

#[test]
fn test_write_literal_class_byte_b_less_than_0() {
    struct TestWriter(String);
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut output = TestWriter(String::new());
    let mut writer = Writer { wtr: output };
    let _ = writer.write_literal_class_byte(0x00);
}

#[test]
fn test_write_literal_class_byte_b_eq_0x80() {
    struct TestWriter(String);
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut output = TestWriter(String::new());
    let mut writer = Writer { wtr: output };
    let _ = writer.write_literal_class_byte(0x80);
}

#[test]
fn test_write_literal_class_byte_b_eq_0xFF() {
    struct TestWriter(String);
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut output = TestWriter(String::new());
    let mut writer = Writer { wtr: output };
    let _ = writer.write_literal_class_byte(0xFF);
}

