// Answer 0

#[test]
fn test_fmt_class_ascii_word_negated() {
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
        kind: ast::ClassAsciiKind::Word,
        negated: true,
    };
    let _ = writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_word_non_negated() {
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
        kind: ast::ClassAsciiKind::Word,
        negated: false,
    };
    let _ = writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_xdigit_negated() {
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
        kind: ast::ClassAsciiKind::Xdigit,
        negated: true,
    };
    let _ = writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_xdigit_non_negated() {
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
        kind: ast::ClassAsciiKind::Xdigit,
        negated: false,
    };
    let _ = writer.fmt_class_ascii(&ast);
}

