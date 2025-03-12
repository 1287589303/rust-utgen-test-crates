// Answer 0

#[test]
fn test_fmt_repetition_range_bounded_zero_zero() {
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

    let ast = ast::Repetition {
        span: Default::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(0, 0)) },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    let _ = writer.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_range_at_least_zero() {
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

    let ast = ast::Repetition {
        span: Default::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(0)) },
        greedy: false,
        ast: Box::new(ast::Ast::default()),
    };

    let _ = writer.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_range_exactly_zero() {
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

    let ast = ast::Repetition {
        span: Default::default(),
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(0)) },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    let _ = writer.fmt_repetition(&ast);
}

