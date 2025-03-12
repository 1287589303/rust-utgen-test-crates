// Answer 0

#[test]
fn test_fmt_class_perl_digit_negated() {
    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: DummyWriter { output: String::new() } };
    let ast = ast::ClassPerl { span: Span::default(), kind: ClassPerlKind::Digit, negated: true };
    writer.fmt_class_perl(&ast).unwrap();
}

#[test]
fn test_fmt_class_perl_digit_non_negated() {
    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: DummyWriter { output: String::new() } };
    let ast = ast::ClassPerl { span: Span::default(), kind: ClassPerlKind::Digit, negated: false };
    writer.fmt_class_perl(&ast).unwrap();
}

#[test]
fn test_fmt_class_perl_space_negated() {
    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: DummyWriter { output: String::new() } };
    let ast = ast::ClassPerl { span: Span::default(), kind: ClassPerlKind::Space, negated: true };
    writer.fmt_class_perl(&ast).unwrap();
}

#[test]
fn test_fmt_class_perl_space_non_negated() {
    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: DummyWriter { output: String::new() } };
    let ast = ast::ClassPerl { span: Span::default(), kind: ClassPerlKind::Space, negated: false };
    writer.fmt_class_perl(&ast).unwrap();
}

#[test]
fn test_fmt_class_perl_word_negated() {
    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: DummyWriter { output: String::new() } };
    let ast = ast::ClassPerl { span: Span::default(), kind: ClassPerlKind::Word, negated: true };
    writer.fmt_class_perl(&ast).unwrap();
}

#[test]
fn test_fmt_class_perl_word_non_negated() {
    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: DummyWriter { output: String::new() } };
    let ast = ast::ClassPerl { span: Span::default(), kind: ClassPerlKind::Word, negated: false };
    writer.fmt_class_perl(&ast).unwrap();
}

