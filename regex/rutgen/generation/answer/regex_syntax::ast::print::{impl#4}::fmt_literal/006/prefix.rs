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

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed),
        c: '\n',
    };
    let mut w = Writer { wtr: &mut writer };
    w.fmt_literal(&ast).unwrap();
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

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::VerticalTab),
        c: '\x0B',
    };
    let mut w = Writer { wtr: &mut writer };
    w.fmt_literal(&ast).unwrap();
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

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Space),
        c: ' ',
    };
    let mut w = Writer { wtr: &mut writer };
    w.fmt_literal(&ast).unwrap();
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

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Bell),
        c: '\x07',
    };
    let mut w = Writer { wtr: &mut writer };
    w.fmt_literal(&ast).unwrap();
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

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::FormFeed),
        c: '\x0C',
    };
    let mut w = Writer { wtr: &mut writer };
    w.fmt_literal(&ast).unwrap();
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

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Tab),
        c: '\t',
    };
    let mut w = Writer { wtr: &mut writer };
    w.fmt_literal(&ast).unwrap();
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

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::CarriageReturn),
        c: '\r',
    };
    let mut w = Writer { wtr: &mut writer };
    w.fmt_literal(&ast).unwrap();
}

#[test]
fn test_fmt_literal_meta() {
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
    let ast = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Meta,
        c: 'a',
    };
    let mut w = Writer { wtr: &mut writer };
    w.fmt_literal(&ast).unwrap();
}

