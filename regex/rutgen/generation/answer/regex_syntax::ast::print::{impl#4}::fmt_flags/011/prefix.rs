// Answer 0

#[test]
fn test_fmt_flags_single_multiline() {
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

    let flags_item = ast::FlagsItem {
        span: Span::default(),
        kind: ast::FlagsItemKind::Flag(Flag::MultiLine),
    };

    let flags = ast::Flags {
        span: Span::default(),
        items: vec![flags_item],
    };

    writer.fmt_flags(&flags).unwrap();
}

#[test]
fn test_fmt_flags_multiple_items_with_multiline() {
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

    let flags_items = vec![
        ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Flag(Flag::MultiLine),
        },
        ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Flag(Flag::CaseInsensitive),
        },
    ];

    let flags = ast::Flags {
        span: Span::default(),
        items: flags_items,
    };

    writer.fmt_flags(&flags).unwrap();
}

#[test]
fn test_fmt_flags_err_condition() {
    struct ErrWriter;

    impl fmt::Write for ErrWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut writer = ErrWriter;

    let flags_item = ast::FlagsItem {
        span: Span::default(),
        kind: ast::FlagsItemKind::Flag(Flag::MultiLine),
    };

    let flags = ast::Flags {
        span: Span::default(),
        items: vec![flags_item],
    };

    writer.fmt_flags(&flags).unwrap_err();
}

