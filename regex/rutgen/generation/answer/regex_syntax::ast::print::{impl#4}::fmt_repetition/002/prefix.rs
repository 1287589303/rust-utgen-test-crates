// Answer 0

#[test]
fn test_fmt_repetition_with_exactly() {
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
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(0)) },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_with_at_least() {
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
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(5)) },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_with_bounded() {
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
        op: ast::RepetitionOp { kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(2, 5)) },
        greedy: true,
        ast: Box::new(ast::Ast::default()),
    };

    writer.fmt_repetition(&ast).unwrap();
}

