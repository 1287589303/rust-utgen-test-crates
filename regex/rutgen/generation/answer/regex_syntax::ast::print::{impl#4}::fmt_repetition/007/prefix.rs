// Answer 0

#[test]
fn test_fmt_repetition_zero_or_more_greedy() {
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
    let repetition = ast::Repetition {
        span: Span::default(), // Assuming Span has a default implementation
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrMore,
        },
        greedy: true,
        ast: Box::new(ast::some_valid_ast()), // Provide a valid ast instance
    };

    writer.fmt_repetition(&repetition).unwrap();
}

#[test]
fn test_fmt_repetition_zero_or_more_nongreedy() {
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
    let repetition = ast::Repetition {
        span: Span::default(), // Assuming Span has a default implementation
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrMore,
        },
        greedy: false,
        ast: Box::new(ast::some_valid_ast()), // Provide a valid ast instance
    };

    writer.fmt_repetition(&repetition).unwrap();
} 

#[test]
fn test_fmt_repetition_one_or_more_greedy() {
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
    let repetition = ast::Repetition {
        span: Span::default(), // Assuming Span has a default implementation
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: true,
        ast: Box::new(ast::some_valid_ast()), // Provide a valid ast instance
    };

    writer.fmt_repetition(&repetition).unwrap();
}

#[test]
fn test_fmt_repetition_one_or_more_nongreedy() {
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
    let repetition = ast::Repetition {
        span: Span::default(), // Assuming Span has a default implementation
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: false,
        ast: Box::new(ast::some_valid_ast()), // Provide a valid ast instance
    };

    writer.fmt_repetition(&repetition).unwrap();
} 

#[test]
fn test_fmt_repetition_range_bounded() {
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
    let repetition = ast::Repetition {
        span: Span::default(), // Assuming Span has a default implementation
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(2, 5)),
        },
        greedy: true,
        ast: Box::new(ast::some_valid_ast()), // Provide a valid ast instance
    };

    writer.fmt_repetition(&repetition).unwrap();
} 

#[test]
fn test_fmt_repetition_range_exactly() {
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
    let repetition = ast::Repetition {
        span: Span::default(), // Assuming Span has a default implementation
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(3)),
        },
        greedy: true,
        ast: Box::new(ast::some_valid_ast()), // Provide a valid ast instance
    };

    writer.fmt_repetition(&repetition).unwrap();
} 

#[test]
fn test_fmt_repetition_range_at_least() {
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
    let repetition = ast::Repetition {
        span: Span::default(), // Assuming Span has a default implementation
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(4)),
        },
        greedy: true,
        ast: Box::new(ast::some_valid_ast()), // Provide a valid ast instance
    };

    writer.fmt_repetition(&repetition).unwrap();
}

