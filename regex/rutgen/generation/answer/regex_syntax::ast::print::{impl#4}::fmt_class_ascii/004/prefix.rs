// Answer 0

#[test]
fn test_fmt_class_ascii_word_non_negated() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let class_ascii = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Word,
        negated: false,
    };

    let mut fmt_writer = Writer { wtr: &mut writer };
    let _ = fmt_writer.fmt_class_ascii(&class_ascii);
}

#[test]
fn test_fmt_class_ascii_word_negated() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let class_ascii = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Word,
        negated: true,
    };

    let mut fmt_writer = Writer { wtr: &mut writer };
    let _ = fmt_writer.fmt_class_ascii(&class_ascii);
}

