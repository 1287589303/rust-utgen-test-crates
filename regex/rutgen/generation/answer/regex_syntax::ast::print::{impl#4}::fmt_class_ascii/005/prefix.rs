// Answer 0

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

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Upper,
        negated: true,
    };

    let mut result_writer = Writer { wtr: writer };
    let _ = result_writer.fmt_class_ascii(&ast);
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

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Upper,
        negated: false,
    };

    let mut result_writer = Writer { wtr: writer };
    let _ = result_writer.fmt_class_ascii(&ast);
}

