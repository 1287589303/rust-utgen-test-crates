// Answer 0

#[test]
fn test_fmt_repetition_one_or_more_greedy() {
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
    let ast = ast::Repetition {
        span: Span::default(),
        op: RepetitionOp { kind: RepetitionKind::OneOrMore },
        greedy: true,
        ast: Box::new(Ast::default()),
    };
    
    let result = writer.fmt_repetition(&ast);
    let _ = result; // Calling the function
}

#[test]
fn test_fmt_repetition_one_or_more_not_greedy() {
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
    let ast = ast::Repetition {
        span: Span::default(),
        op: RepetitionOp { kind: RepetitionKind::OneOrMore },
        greedy: false,
        ast: Box::new(Ast::default()),
    };
    
    let result = writer.fmt_repetition(&ast);
    let _ = result; // Calling the function
}

