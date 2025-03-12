// Answer 0

#[test]
fn test_parse_uncounted_repetition_with_question_mark() {
    let position = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(position, position);
    let ast_item = Ast::literal(ast::Literal { /* initialization */ });
    let mut concat = Concat {
        span: span.clone(),
        asts: vec![ast_item],
    };
    let parser_instance = ParserI {
        parser: /* implementation of P */,
        pattern: "a?b*",
    };
    // Simulate conditions
    parser_instance.char = || '?';
    parser_instance.bump = || true;
    parser_instance.pos = || position;
    let _ = parser_instance.parse_uncounted_repetition(concat, RepetitionKind::ZeroOrOne);
}

#[test]
fn test_parse_uncounted_repetition_with_asterisk() {
    let position = Position { offset: 2, line: 1, column: 3 };
    let span = Span::new(position, position);
    let ast_item = Ast::literal(ast::Literal { /* initialization */ });
    let mut concat = Concat {
        span: span.clone(),
        asts: vec![ast_item],
    };
    let parser_instance = ParserI {
        parser: /* implementation of P */,
        pattern: "a*b+c",
    };
    // Simulate conditions
    parser_instance.char = || '*';
    parser_instance.bump = || true;
    parser_instance.pos = || position;
    let _ = parser_instance.parse_uncounted_repetition(concat, RepetitionKind::ZeroOrMore);
}

