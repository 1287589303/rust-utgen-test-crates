// Answer 0

#[test]
fn test_fmt_class_unicode_one_letter() {
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
    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('a'),
    };
    
    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.fmt_class_unicode(&ast);
}

#[test]
fn test_fmt_class_unicode_named_value_equal() {
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
    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::Equal,
            name: String::from("name"),
            value: String::from("value"),
        },
    };

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.fmt_class_unicode(&ast);
}

#[test]
fn test_fmt_class_unicode_named_value_not_equal() {
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
    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::NotEqual,
            name: String::from("name"),
            value: String::from("value"),
        },
    };

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.fmt_class_unicode(&ast);
}

#[test]
fn test_fmt_class_unicode_named_value_colon() {
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
    let ast = ast::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue {
            op: ast::ClassUnicodeOpKind::Colon,
            name: String::from("name"),
            value: String::from("value"),
        },
    };

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.fmt_class_unicode(&ast);
}

