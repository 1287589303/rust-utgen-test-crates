// Answer 0

#[test]
fn test_fmt_class_ascii_blank_negated() {
    struct MockWriter;
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter };
    let ast = ast::ClassAscii {
        span: Default::default(),
        kind: ast::ClassAsciiKind::Blank,
        negated: true,
    };
    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_blank_non_negated() {
    struct MockWriter;
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter };
    let ast = ast::ClassAscii {
        span: Default::default(),
        kind: ast::ClassAsciiKind::Blank,
        negated: false,
    };
    writer.fmt_class_ascii(&ast).unwrap();
}

