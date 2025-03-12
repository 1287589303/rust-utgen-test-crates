// Answer 0

#[test]
fn test_fmt_repetition_zero_or_more_greedy_true() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let ast = ast::Repetition {
        span: Span::default(),
        op: RepetitionOp { kind: RepetitionKind::ZeroOrMore },
        greedy: true,
        ast: Box::new(Ast::default()),
    };
    
    let _ = writer.fmt_repetition(&ast);
}

#[test]
fn test_fmt_repetition_zero_or_more_greedy_false() {
    struct MockWriter {
        output: String,
    }
    
    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let ast = ast::Repetition {
        span: Span::default(),
        op: RepetitionOp { kind: RepetitionKind::ZeroOrMore },
        greedy: false,
        ast: Box::new(Ast::default()),
    };
    
    let _ = writer.fmt_repetition(&ast);
}

