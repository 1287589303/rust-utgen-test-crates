// Answer 0

#[test]
fn test_fmt_class_ascii_blank_not_negated() {
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
    let ast = ast::ClassAscii {
        span: Span::default(), // Assuming default implementation is available
        kind: ClassAsciiKind::Blank,
        negated: false,
    };

    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_blank_negated() {
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
    let ast = ast::ClassAscii {
        span: Span::default(), // Assuming default implementation is available
        kind: ClassAsciiKind::Blank,
        negated: true,
    };

    writer.fmt_class_ascii(&ast).unwrap();
}

