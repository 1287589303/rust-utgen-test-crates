// Answer 0

#[test]
fn test_fmt_class_ascii_graph_negated() {
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
        span: Span::default(), // Assuming a default span exists
        kind: ClassAsciiKind::Graph,
        negated: true,
    };

    let mut visitor = Writer { wtr: writer };
    let _ = visitor.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_graph_non_negated() {
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
        span: Span::default(), // Assuming a default span exists
        kind: ClassAsciiKind::Graph,
        negated: false,
    };

    let mut visitor = Writer { wtr: writer };
    let _ = visitor.fmt_class_ascii(&ast);
}

