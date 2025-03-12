// Answer 0

#[test]
fn test_fmt_class_ascii_print_not_negated() {
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
    let mut class_ascii = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Print,
        negated: false,
    };

    let result = writer.fmt_class_ascii(&class_ascii);
}

#[test]
fn test_fmt_class_ascii_print_negated() {
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
    let mut class_ascii = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Print,
        negated: true,
    };

    let result = writer.fmt_class_ascii(&class_ascii);
}

