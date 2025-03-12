// Answer 0

#[test]
fn test_fmt_class_ascii_lower_negated() {
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
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Lower,
        negated: true,
    };
    let _ = writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_upper_negated() {
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
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Upper,
        negated: true,
    };
    let _ = writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_lower_not_negated() {
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
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Lower,
        negated: false,
    };
    let _ = writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_upper_not_negated() {
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
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Upper,
        negated: false,
    };
    let _ = writer.fmt_class_ascii(&ast);
}

