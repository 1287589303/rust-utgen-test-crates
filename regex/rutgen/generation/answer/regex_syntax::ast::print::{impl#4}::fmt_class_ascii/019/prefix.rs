// Answer 0

#[test]
fn test_fmt_class_ascii_cntrl_negated() {
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
        kind: ClassAsciiKind::Cntrl,
        negated: true,
    };

    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_cntrl_non_negated() {
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
        kind: ClassAsciiKind::Cntrl,
        negated: false,
    };

    writer.fmt_class_ascii(&ast).unwrap();
}

