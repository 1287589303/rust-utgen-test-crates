// Answer 0

#[test]
fn test_fmt_class_ascii_space_negated() {
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
        span: ast::Span::default(), // Example span initialization
        kind: ast::ClassAsciiKind::Space,
        negated: true,
    };

    let _result = Writer { wtr: &mut writer }.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_blank_negated() {
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
        span: ast::Span::default(), // Example span initialization
        kind: ast::ClassAsciiKind::Blank,
        negated: true,
    };

    let _result = Writer { wtr: &mut writer }.fmt_class_ascii(&ast);
}

