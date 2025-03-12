// Answer 0

#[test]
fn test_write_literal_byte_b_eq_0x7F() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let b: u8 = 0x7F;
    // The call to the function under test
    let _ = writer.write_literal_byte(b);
}

#[test]
fn test_write_literal_byte_b_eq_0x20() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let b: u8 = 0x20;
    // The call to the function under test
    let _ = writer.write_literal_byte(b);
}

#[test]
fn test_write_literal_byte_b_eq_0x2F() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let b: u8 = 0x2F; // ASCII '/'
    // The call to the function under test
    let _ = writer.write_literal_byte(b);
}

#[test]
fn test_write_literal_byte_b_eq_0x3A() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }

        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let b: u8 = 0x3A; // ASCII ':'
    // The call to the function under test
    let _ = writer.write_literal_byte(b);
}

