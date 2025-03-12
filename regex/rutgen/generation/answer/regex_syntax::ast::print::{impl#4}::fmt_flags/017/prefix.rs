// Answer 0

#[test]
fn test_fmt_flags_empty_items() {
    struct WriterMock {
        output: String,
    }
    
    impl fmt::Write for WriterMock {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let mut writer = WriterMock { output: String::new() };
    let flags = ast::Flags {
        span: Span { start: 0, end: 0 },
        items: Vec::new(),
    };
    
    writer.fmt_flags(&flags).unwrap();
}

#[test]
fn test_fmt_flags_negation_item() {
    struct WriterMock {
        output: String,
    }
    
    impl fmt::Write for WriterMock {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let mut writer = WriterMock { output: String::new() };
    let negation_item = ast::FlagsItem {
        span: Span { start: 0, end: 1 },
        kind: ast::FlagsItemKind::Negation,
    };
    let flags = ast::Flags {
        span: Span { start: 0, end: 1 },
        items: vec![negation_item],
    };
    
    writer.fmt_flags(&flags).unwrap();
}

