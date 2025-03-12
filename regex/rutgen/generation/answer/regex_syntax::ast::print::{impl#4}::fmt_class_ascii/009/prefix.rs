// Answer 0

#[test]
fn test_fmt_class_ascii_punct_negated() {
    struct DummyWriter;
    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = Writer { wtr: DummyWriter };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Punct,
        negated: true,
    };

    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_punct_non_negated() {
    struct DummyWriter;
    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = Writer { wtr: DummyWriter };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Punct,
        negated: false,
    };

    writer.fmt_class_ascii(&ast).unwrap();
}

