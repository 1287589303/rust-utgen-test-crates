// Answer 0

#[test]
fn test_fmt_class_ascii_graph_non_negated() {
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
    let mut writer_instance = Writer { wtr: &mut writer };
    
    let ast = ast::ClassAscii {
        span: Span::default(), // Assuming Span has a default implementation
        kind: ClassAsciiKind::Graph,
        negated: false,
    };
    
    let _ = writer_instance.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_graph_negated() {
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
    let mut writer_instance = Writer { wtr: &mut writer };
    
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Graph,
        negated: true,
    };
    
    let _ = writer_instance.fmt_class_ascii(&ast);
}

