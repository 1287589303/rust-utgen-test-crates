// Answer 0

#[test]
fn test_parse_counted_repetition_valid() {
    let concat = ast::Concat {
        span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 10, line: 1, column: 11 }),
        asts: vec![ast::Ast::literal(Box::new(ast::Literal { span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }), value: 'a' }))],
    };
    let parser = ParserI {
        parser: /* instantiate parser with initial state */,
        pattern: "{1,2}abc",
    };
    parser.parse_counted_repetition(concat).unwrap();
}

#[test]
fn test_parse_counted_repetition_at_least() {
    let concat = ast::Concat {
        span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 10, line: 1, column: 11 }),
        asts: vec![ast::Ast::literal(Box::new(ast::Literal { span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }), value: 'b' }))],
    };
    let parser = ParserI {
        parser: /* instantiate parser with initial state */,
        pattern: "{3,}abc",
    };
    parser.parse_counted_repetition(concat).unwrap();
}

#[test]
fn test_parse_counted_repetition_exactly() {
    let concat = ast::Concat {
        span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 10, line: 1, column: 11 }),
        asts: vec![ast::Ast::literal(Box::new(ast::Literal { span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }), value: 'c' }))],
    };
    let parser = ParserI {
        parser: /* instantiate parser with initial state */,
        pattern: "{4}abc",
    };
    parser.parse_counted_repetition(concat).unwrap();
}

#[test]
fn test_parse_counted_repetition_bounded() {
    let concat = ast::Concat {
        span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 10, line: 1, column: 11 }),
        asts: vec![ast::Ast::literal(Box::new(ast::Literal { span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }), value: 'd' }))],
    };
    let parser = ParserI {
        parser: /* instantiate parser with initial state */,
        pattern: "{2,5}abc",
    };
    parser.parse_counted_repetition(concat).unwrap();
}

#[test]
fn test_parse_counted_repetition_with_optional_greedy() {
    let concat = ast::Concat {
        span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 10, line: 1, column: 11 }),
        asts: vec![ast::Ast::literal(Box::new(ast::Literal { span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }), value: 'e' }))],
    };
    let parser = ParserI {
        parser: /* instantiate parser with initial state that expects following character to be '?' */,
        pattern: "{1,3}?abc",
    };
    parser.parse_counted_repetition(concat).unwrap();
} 

#[test]
#[should_panic]
fn test_parse_counted_repetition_empty_concat() {
    let concat = ast::Concat {
        span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 10, line: 1, column: 11 }),
        asts: Vec::new(),
    };
    let parser = ParserI {
        parser: /* instantiate parser with initial state */,
        pattern: "{1,2}abc",
    };
    parser.parse_counted_repetition(concat).unwrap();
}

#[test]
#[should_panic]
fn test_parse_counted_repetition_invalid_range() {
    let concat = ast::Concat {
        span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 10, line: 1, column: 11 }),
        asts: vec![ast::Ast::literal(Box::new(ast::Literal { span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }), value: 'f' }))],
    };
    let parser = ParserI {
        parser: /* instantiate parser with initial state */,
        pattern: "{3,2}abc", // invalid as 3 < 2
    };
    parser.parse_counted_repetition(concat).unwrap();
}

