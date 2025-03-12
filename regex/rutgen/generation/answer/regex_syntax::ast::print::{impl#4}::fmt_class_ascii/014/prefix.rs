// Answer 0

#[test]
fn test_fmt_class_ascii_lower_not_negated() {
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
        span: Span::default(),
        kind: ClassAsciiKind::Lower,
        negated: false,
    };

    let _ = writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_upper_not_negated() {
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
        span: Span::default(),
        kind: ClassAsciiKind::Upper,
        negated: false,
    };

    let _ = writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_lower_negated() {
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
        span: Span::default(),
        kind: ClassAsciiKind::Lower,
        negated: true,
    };

    let _ = writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_upper_negated() {
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
        span: Span::default(),
        kind: ClassAsciiKind::Upper,
        negated: true,
    };

    let _ = writer.fmt_class_ascii(&ast);
}

