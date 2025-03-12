// Answer 0

#[test]
fn test_fmt_class_ascii_cnrtrl_not_negated() {
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
    let mut fmt_writer = Writer { wtr: writer };

    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Cntrl,
        negated: false,
    };

    let _ = fmt_writer.fmt_class_ascii(&ast);
}

