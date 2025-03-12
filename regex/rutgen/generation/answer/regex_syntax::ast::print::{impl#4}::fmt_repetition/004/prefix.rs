// Answer 0

#[test]
fn test_fmt_repetition_bounded() {
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
        op: RepetitionOp { kind: RepetitionKind::Range(RepetitionRange::Bounded(0, 10)) },
        greedy: false,
        ast: Box::new(Ast::default()),
    };

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_bounded_max_values() {
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
        op: RepetitionOp { kind: RepetitionKind::Range(RepetitionRange::Bounded(99, 100)) },
        greedy: false,
        ast: Box::new(Ast::default()),
    };

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_bounded_zero() {
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
        op: RepetitionOp { kind: RepetitionKind::Range(RepetitionRange::Bounded(0, 0)) },
        greedy: false,
        ast: Box::new(Ast::default()),
    };

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_at_least() {
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
        op: RepetitionOp { kind: RepetitionKind::Range(RepetitionRange::AtLeast(1)) },
        greedy: false,
        ast: Box::new(Ast::default()),
    };

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.fmt_repetition(&ast);
}

