// Answer 0

#[test]
fn test_fmt_repetition_one_or_more_greedy_false() {
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

    let ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_one_or_more_greedy_true() {
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

    let ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::OneOrMore },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_zero_or_more_greedy_false() {
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

    let ast = ast::Repetition {
        span: Span::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::ZeroOrMore },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
}

