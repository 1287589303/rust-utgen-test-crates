// Answer 0

#[test]
fn test_fmt_flags_with_single_swap_greed_flag() {
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
        kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
    };
    let flags = ast::Flags {
        span: Span::default(),
        items: vec![flags_item],
    };
    let mut fmt_writer = Writer { wtr: writer };

    let _ = fmt_writer.fmt_flags(&flags);
}

#[test]
fn test_fmt_flags_with_multiple_flags_including_swap_greed() {
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
        ast::FlagsItem { span: Span::default(), kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) },
        ast::FlagsItem { span: Span::default(), kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed) },
        ast::FlagsItem { span: Span::default(), kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode) },
    ];
    let flags = ast::Flags {
        span: Span::default(),
        items: flags_items,
    };
    let mut fmt_writer = Writer { wtr: writer };

    let _ = fmt_writer.fmt_flags(&flags);
}

#[test]
fn test_fmt_flags_with_empty_flags() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let flags = ast::Flags {
        span: Span::default(),
        items: Vec::new(),
    };
    let mut fmt_writer = Writer { wtr: writer };

    let _ = fmt_writer.fmt_flags(&flags);
}

#[test]
fn test_fmt_flags_with_multiple_swap_greed_flags() {
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
        ast::FlagsItem { span: Span::default(), kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed) },
        ast::FlagsItem { span: Span::default(), kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed) },
    ];
    let flags = ast::Flags {
        span: Span::default(),
        items: flags_items,
    };
    let mut fmt_writer = Writer { wtr: writer };

    let _ = fmt_writer.fmt_flags(&flags);
}

