// Answer 0

#[test]
fn test_fmt_class_ascii_alpha() {
    struct MockWriter;
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter };
    let ast = ast::ClassAscii {
        span: Span::default(), // assuming Span has a default implementation
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    };

    let _ = writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_alpha_negated() {
    struct MockWriter;

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Alpha,
        negated: true,
    };

    let _ = writer.fmt_class_ascii(&ast);
}

