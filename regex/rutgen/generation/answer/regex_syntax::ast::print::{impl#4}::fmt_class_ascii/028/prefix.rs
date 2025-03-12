// Answer 0

#[test]
fn test_fmt_class_ascii_alnum_not_negated() {
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
        span: Default::default(),
        kind: ast::ClassAsciiKind::Alnum,
        negated: false,
    };

    let mut fmt_writer = Writer { wtr: writer };
    fmt_writer.fmt_class_ascii(&ast).unwrap();
}

