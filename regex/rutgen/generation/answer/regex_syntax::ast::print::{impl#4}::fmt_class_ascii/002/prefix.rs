// Answer 0

#[test]
fn test_fmt_class_ascii_xdigit_not_negated() {
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
    let mut ast_class_ascii = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Xdigit,
        negated: false,
    };

    let mut fmt_writer = Writer { wtr: writer };
    fmt_writer.fmt_class_ascii(&ast_class_ascii).unwrap();
}

