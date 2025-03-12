// Answer 0

#[test]
fn test_fmt_class_unicode_negated_named_value_equal() {
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
    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: true,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::Equal,
            name: String::from("name"),
            value: String::from("value1"),
        },
    };
    let _ = writer.fmt_class_unicode(&ast);
}

#[test]
fn test_fmt_class_unicode_negated_named_value_not_equal() {
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
    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: true,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::NotEqual,
            name: String::from("name"),
            value: String::from("value2"),
        },
    };
    let _ = writer.fmt_class_unicode(&ast);
}

#[test]
fn test_fmt_class_unicode_negated_named_value_colon() {
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
    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: true,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::Colon,
            name: String::from("anotherName"),
            value: String::from("value2"),
        },
    };
    let _ = writer.fmt_class_unicode(&ast);
}

#[test]
fn test_fmt_class_unicode_negated_named() {
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
    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: true,
        kind: ast::ClassUnicodeKind::Named(String::from("name1")),
    };
    let _ = writer.fmt_class_unicode(&ast);
}

