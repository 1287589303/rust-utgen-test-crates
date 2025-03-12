// Answer 0

#[test]
fn test_fmt_literal_hex_brace_unicode_long() {
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
    let hex_literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeLong),
        c: '\u{1F600}',  // 😀
    };
    let mut fmt_writer = Writer { wtr: &mut writer };
    fmt_writer.fmt_literal(&hex_literal).unwrap();
}

#[test]
fn test_fmt_literal_hex_brace_unicode_short() {
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
    let hex_literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::UnicodeShort),
        c: '\u{0061}',  // 'a'
    };
    let mut fmt_writer = Writer { wtr: &mut writer };
    fmt_writer.fmt_literal(&hex_literal).unwrap();
}

#[test]
fn test_fmt_literal_hex_brace_x() {
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
    let hex_literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::X),
        c: '\u{0042}',  // 'B'
    };
    let mut fmt_writer = Writer { wtr: &mut writer };
    fmt_writer.fmt_literal(&hex_literal).unwrap();
}

#[test]
fn test_fmt_literal_verbatim() {
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
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Verbatim,
        c: '\n', // newline character
    };
    let mut fmt_writer = Writer { wtr: &mut writer };
    fmt_writer.fmt_literal(&literal).unwrap();
}

