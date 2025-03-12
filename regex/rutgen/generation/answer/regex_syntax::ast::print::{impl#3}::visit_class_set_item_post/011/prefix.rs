// Answer 0

#[test]
fn test_visit_class_set_item_post_empty() {
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
    let item = ast::ClassSetItem::Empty(ast::Span::new(0, 0));
    writer.visit_class_set_item_post(&item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_literal() {
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
    let span = ast::Span::new(0, 1);
    let literal = ast::Literal {
        span,
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    };
    let item = ast::ClassSetItem::Literal(literal);
    writer.visit_class_set_item_post(&item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_range() {
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
    let span = ast::Span::new(0, 2);
    let start = ast::Literal {
        span,
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    };
    let end = ast::Literal {
        span,
        kind: ast::LiteralKind::Verbatim,
        c: 'z',
    };
    let item = ast::ClassSetItem::Range(ast::ClassSetRange { span, start, end });
    writer.visit_class_set_item_post(&item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_ascii() {
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
    let ascii_class = ast::ClassAscii {
        span: ast::Span::new(0, 1),
        kind: ast::ClassAsciiKind::Alnum,
        negated: false,
    };
    let item = ast::ClassSetItem::Ascii(ascii_class);
    writer.visit_class_set_item_post(&item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_unicode() {
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
    let unicode_class = ast::ClassUnicode {
        span: ast::Span::new(0, 1),
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('a'),
    };
    let item = ast::ClassSetItem::Unicode(unicode_class);
    writer.visit_class_set_item_post(&item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_perl() {
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
    let perl_class = ast::ClassPerl {
        span: ast::Span::new(0, 1),
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };
    let item = ast::ClassSetItem::Perl(perl_class);
    writer.visit_class_set_item_post(&item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_bracketed() {
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
    let bracketed_class = ast::ClassBracketed {
        span: ast::Span::new(0, 1),
        negated: false,
        kind: ast::ClassSet::Normal,
    };
    let item = ast::ClassSetItem::Bracketed(Box::new(bracketed_class));
    writer.visit_class_set_item_post(&item).unwrap();
}

