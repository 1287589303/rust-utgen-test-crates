// Answer 0

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
        c: 'A',
    };

    let mut formatter = Writer { wtr: &mut writer };
    let _ = formatter.fmt_literal(&ast);
}

#[test]
fn test_fmt_literal_superfluous() {
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
        kind: ast::LiteralKind::Superfluous,
        c: 'B',
    };

    let mut formatter = Writer { wtr: &mut writer };
    let _ = formatter.fmt_literal(&ast);
}

