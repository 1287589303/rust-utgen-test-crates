// Answer 0

#[cfg(test)]
fn test_fmt_flags_negation() {
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
        kind: ast::FlagsItemKind::Negation,
    };

    let flags = ast::Flags {
        span: Span::default(),
        items: vec![flags_item],
    };
    
    writer.fmt_flags(&flags).unwrap();
}

#[cfg(test)]
fn test_fmt_flags_multiple_negations() {
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
            kind: ast::FlagsItemKind::Negation,
        },
        ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Negation,
        },
    ];

    let flags = ast::Flags {
        span: Span::default(),
        items: flags_items,
    };
    
    writer.fmt_flags(&flags).unwrap();
}

#[cfg(test)]
fn test_fmt_flags_with_flags() {
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
            kind: ast::FlagsItemKind::Negation,
        },
        ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
        },
    ];

    let flags = ast::Flags {
        span: Span::default(),
        items: flags_items,
    };
    
    writer.fmt_flags(&flags).unwrap();
}

#[cfg(test)]
fn test_fmt_flags_empty() {
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

    let flags = ast::Flags {
        span: Span::default(),
        items: vec![],
    };
    
    writer.fmt_flags(&flags).unwrap();
}

#[cfg(test)]
fn test_fmt_flags_only_flags() {
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
            kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
        },
        ast::FlagsItem {
            span: Span::default(),
            kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
        },
    ];

    let flags = ast::Flags {
        span: Span::default(),
        items: flags_items,
    };
    
    writer.fmt_flags(&flags).unwrap();
}

