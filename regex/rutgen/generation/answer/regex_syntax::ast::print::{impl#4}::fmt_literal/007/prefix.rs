// Answer 0

#[test]
fn test_fmt_literal_special_bell() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Bell),
        c: '\u{07}',
    };
    let mut writer_instance = Writer { wtr: &mut writer };
    
    let _ = writer_instance.fmt_literal(&literal);
}

#[test]
fn test_fmt_literal_special_form_feed() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::FormFeed),
        c: '\u{0C}',
    };
    let mut writer_instance = Writer { wtr: &mut writer };

    let _ = writer_instance.fmt_literal(&literal);
}

#[test]
fn test_fmt_literal_special_line_feed() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed),
        c: '\n',
    };
    let mut writer_instance = Writer { wtr: &mut writer };

    let _ = writer_instance.fmt_literal(&literal);
}

#[test]
fn test_fmt_literal_special_vertical_tab() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::VerticalTab),
        c: '\u{0B}',
    };
    let mut writer_instance = Writer { wtr: &mut writer };

    let _ = writer_instance.fmt_literal(&literal);
}

#[test]
fn test_fmt_literal_special_space() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Space),
        c: ' ',
    };
    let mut writer_instance = Writer { wtr: &mut writer };

    let _ = writer_instance.fmt_literal(&literal);
}

#[test]
fn test_fmt_literal_special_carriage_return() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };
    let mut writer_instance = Writer { wtr: &mut writer };

    let _ = writer_instance.fmt_literal(&literal);
}

#[test]
fn test_fmt_literal_special_tab() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Tab),
        c: '\t',
    };
    let mut writer_instance = Writer { wtr: &mut writer };

    let _ = writer_instance.fmt_literal(&literal);
}

#[test]
fn test_fmt_literal_verbatim() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    };
    let mut writer_instance = Writer { wtr: &mut writer };

    let _ = writer_instance.fmt_literal(&literal);
}

