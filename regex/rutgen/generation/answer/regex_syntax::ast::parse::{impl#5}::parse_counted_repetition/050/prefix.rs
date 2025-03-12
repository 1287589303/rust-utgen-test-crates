// Answer 0

#[test]
fn test_parse_counted_repetition_unexpected_character() {
    struct TestParser {
        char_position: usize,
        concat: ast::Concat,
        ast: ast::Ast,
    }

    let start_position = Position { offset: 0, line: 1, column: 1 };
    let end_position = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(start_position, end_position);

    let repetition_ast = ast::Repetition {
        span: span.clone(),
        op: ast::RepetitionOp {
            span: span.clone(),
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: true,
        ast: Box::new(ast::Ast::Empty(Box::new(span))),
    };

    let concat = ast::Concat {
        span: span.clone(),
        asts: vec![repetition_ast],
    };

    let parser = TestParser {
        char_position: 0, // Not at '{'
        concat,
        ast: ast::Ast::Flags(Box::new(ast::Flags { span })),
    };

    // Simulate a call to `parse_counted_repetition` with the setup
    let _result = parser.parse_counted_repetition(parser.concat);
}

#[test]
fn test_parse_counted_repetition_non_eof() {
    struct TestParser {
        char_position: usize,
        concat: ast::Concat,
        ast: ast::Ast,
    }

    let start_position = Position { offset: 0, line: 1, column: 1 };
    let end_position = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(start_position, end_position);

    let repetition_ast = ast::Repetition {
        span: span.clone(),
        op: ast::RepetitionOp {
            span: span.clone(),
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: true,
        ast: Box::new(ast::Ast::Literal(Box::new(ast::Literal { span }))),
    };

    let concat = ast::Concat {
        span: span.clone(),
        asts: vec![repetition_ast],
    };

    let parser = TestParser {
        char_position: '{', // Not at '{', simulating an unexpected input character
        concat,
        ast: ast::Ast::Literal(Box::new(ast::Literal { span })),
    };

    // Simulate a call to `parse_counted_repetition` with the setup
    let _result = parser.parse_counted_repetition(parser.concat);
}

