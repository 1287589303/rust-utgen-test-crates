// Answer 0

#[test]
fn test_fmt_class_ascii_xdigit_negated() {
    struct MockWriter;
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }
    let mut writer = Writer { wtr: MockWriter };
    let ast = ast::ClassAscii {
        span: Span::new(0, 1),
        kind: ClassAsciiKind::Xdigit,
        negated: true,
    };
    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_xdigit_not_negated() {
    struct MockWriter;
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }
    let mut writer = Writer { wtr: MockWriter };
    let ast = ast::ClassAscii {
        span: Span::new(0, 1),
        kind: ClassAsciiKind::Xdigit,
        negated: false,
    };
    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_digit_negated() {
    struct MockWriter;
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }
    let mut writer = Writer { wtr: MockWriter };
    let ast = ast::ClassAscii {
        span: Span::new(0, 1),
        kind: ClassAsciiKind::Digit,
        negated: true,
    };
    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_digit_not_negated() {
    struct MockWriter;
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }
    let mut writer = Writer { wtr: MockWriter };
    let ast = ast::ClassAscii {
        span: Span::new(0, 1),
        kind: ClassAsciiKind::Digit,
        negated: false,
    };
    writer.fmt_class_ascii(&ast).unwrap();
}

