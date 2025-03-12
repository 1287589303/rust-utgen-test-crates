// Answer 0

#[test]
fn test_fmt_literal_special_line_feed() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed),
        c: '\n',
    };
    let _ = writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_vertical_tab() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::VerticalTab),
        c: '\x0b',
    };
    let _ = writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_space() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Space),
        c: ' ',
    };
    let _ = writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_bell() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Bell),
        c: '\x07',
    };
    let _ = writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_form_feed() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::FormFeed),
        c: '\x0c',
    };
    let _ = writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_tab() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Tab),
        c: '\t',
    };
    let _ = writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_special_carriage_return() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };
    let _ = writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_short() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeShort),
        c: 'A',
    };
    let _ = writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_fixed_x() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::X),
        c: 'B',
    };
    let _ = writer.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_hex_fixed_unicode_long() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeLong),
        c: 'C',
    };
    let _ = writer.fmt_literal(&ast);
}

