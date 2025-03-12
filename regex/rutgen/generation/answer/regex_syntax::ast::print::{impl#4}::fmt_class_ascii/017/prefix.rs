// Answer 0

#[test]
fn test_fmt_class_ascii_digit_negated() {
    struct DummyWriter;
    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }
    
    let mut writer = Writer { wtr: DummyWriter };
    let ast = ast::ClassAscii {
        span: Default::default(),
        kind: ast::ClassAsciiKind::Digit,
        negated: true,
    };
    let _ = writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_digit_not_negated() {
    struct DummyWriter;
    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }
    
    let mut writer = Writer { wtr: DummyWriter };
    let ast = ast::ClassAscii {
        span: Default::default(),
        kind: ast::ClassAsciiKind::Digit,
        negated: false,
    };
    let _ = writer.fmt_class_ascii(&ast);
}

