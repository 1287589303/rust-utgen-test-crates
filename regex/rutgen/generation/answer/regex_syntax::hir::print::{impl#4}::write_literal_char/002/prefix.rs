// Answer 0

#[test]
fn test_write_literal_char_meta_character() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };

    let meta_characters = vec!['\\', '.', '+', '*', '?', '(', ')', '|', '[', ']', '{', '}', '^', '$', '#', '&', '-', '~'];

    for c in meta_characters {
        let mut w = Writer { wtr: writer };
        let _ = w.write_literal_char(c);
    }
}

#[test]
fn test_write_literal_char_with_special_characters() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };

    let special_chars = vec!['$', '&', '~'];

    for c in special_chars {
        let mut w = Writer { wtr: writer };
        let _ = w.write_literal_char(c);
    }
}

