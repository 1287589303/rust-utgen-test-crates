// Answer 0

#[test]
fn test_fmt_class_bracketed_post_valid() {
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
    let ast = ast::ClassBracketed { span: Span::new(0, 1), negated: false, kind: ClassSet::Normal };
    
    writer.fmt_class_bracketed_post(&ast).unwrap();
}

#[test]
fn test_fmt_class_bracketed_post_empty() {
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
    let ast = ast::ClassBracketed { span: Span::new(0, 0), negated: false, kind: ClassSet::Empty };

    writer.fmt_class_bracketed_post(&ast).unwrap();
}

#[test]
fn test_fmt_class_bracketed_post_multiple_invocations() {
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
    let ast1 = ast::ClassBracketed { span: Span::new(0, 1), negated: false, kind: ClassSet::Normal };
    let ast2 = ast::ClassBracketed { span: Span::new(1, 2), negated: true, kind: ClassSet::Normal };

    writer.fmt_class_bracketed_post(&ast1).unwrap();
    writer.fmt_class_bracketed_post(&ast2).unwrap();
}

