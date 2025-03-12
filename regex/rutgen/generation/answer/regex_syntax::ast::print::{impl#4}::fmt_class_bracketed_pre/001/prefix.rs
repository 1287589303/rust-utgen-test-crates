// Answer 0

#[test]
fn test_fmt_class_bracketed_pre_negated() {
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
    let ast = ast::ClassBracketed {
        span: Span { /* initialize with valid data */ },
        negated: true,
        kind: ClassSet::Union { /* initialize with valid ClassSet data */ },
    };

    writer.fmt_class_bracketed_pre(&ast).unwrap();
}

#[test]
fn test_fmt_class_bracketed_pre_not_negated() {
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
    let ast = ast::ClassBracketed {
        span: Span { /* initialize with valid data */ },
        negated: false,
        kind: ClassSet::Union { /* initialize with valid ClassSet data */ },
    };

    writer.fmt_class_bracketed_pre(&ast).unwrap();
}

