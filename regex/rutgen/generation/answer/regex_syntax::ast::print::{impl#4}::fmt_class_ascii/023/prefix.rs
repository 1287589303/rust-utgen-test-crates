// Answer 0

#[test]
fn test_fmt_class_ascii_negated() {
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

    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Ascii,
        negated: true,
    };

    let _ = writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_non_negated() {
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

    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Ascii,
        negated: false,
    };

    let _ = writer.fmt_class_ascii(&ast);
}

