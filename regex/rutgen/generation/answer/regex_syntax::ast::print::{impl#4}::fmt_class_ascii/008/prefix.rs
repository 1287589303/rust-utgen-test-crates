// Answer 0

#[test]
fn test_fmt_class_ascii_space_not_negated() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, src: &str) -> fmt::Result {
            self.output.push_str(src);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::ClassAscii {
        span: Span::default(), // Assuming there is a default implementation
        kind: ClassAsciiKind::Space,
        negated: false,
    };

    let mut visitor = Writer { wtr: writer };
    visitor.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_space_negated() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, src: &str) -> fmt::Result {
            self.output.push_str(src);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::ClassAscii {
        span: Span::default(), // Assuming there is a default implementation
        kind: ClassAsciiKind::Space,
        negated: true,
    };

    let mut visitor = Writer { wtr: writer };
    visitor.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_space_edge_case() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, src: &str) -> fmt::Result {
            self.output.push_str(src);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::ClassAscii {
        span: Span::default(), // Assuming there is a default implementation
        kind: ClassAsciiKind::Space,
        negated: false,
    };

    let mut visitor = Writer { wtr: writer };
    visitor.fmt_class_ascii(&ast).unwrap();
}

