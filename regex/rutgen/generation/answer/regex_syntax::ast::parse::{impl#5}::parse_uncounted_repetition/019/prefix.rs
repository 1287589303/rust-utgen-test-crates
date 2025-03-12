// Answer 0

#[test]
fn test_parse_uncounted_repetition_question_mark() {
    let pos_start = Position { offset: 0, line: 1, column: 1 };
    let pos_end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(pos_start, pos_end);

    let ast_element = Ast::literal(Box::new(/* Your Literal here */));
    let concat = ast::Concat {
        span: span.clone(),
        asts: vec![ast_element],
    };

    let kind = ast::RepetitionKind::ZeroOrOne;
    
    let parser = ParserI {
        parser: /* Your Parser here */,
        pattern: "a?",
    };

    parser.parse_uncounted_repetition(concat, kind).ok();
}

#[test]
fn test_parse_uncounted_repetition_asterisk() {
    let pos_start = Position { offset: 0, line: 1, column: 1 };
    let pos_end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(pos_start, pos_end);

    let ast_element = Ast::literal(Box::new(/* Your Literal here */));
    let concat = ast::Concat {
        span: span.clone(),
        asts: vec![ast_element],
    };

    let kind = ast::RepetitionKind::ZeroOrMore;
    
    let parser = ParserI {
        parser: /* Your Parser here */,
        pattern: "a*",
    };

    parser.parse_uncounted_repetition(concat, kind).ok();
}

#[test]
fn test_parse_uncounted_repetition_plus() {
    let pos_start = Position { offset: 0, line: 1, column: 1 };
    let pos_end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(pos_start, pos_end);

    let ast_element = Ast::literal(Box::new(/* Your Literal here */));
    let concat = ast::Concat {
        span: span.clone(),
        asts: vec![ast_element],
    };

    let kind = ast::RepetitionKind::OneOrMore;
    
    let parser = ParserI {
        parser: /* Your Parser here */,
        pattern: "a+",
    };

    parser.parse_uncounted_repetition(concat, kind).ok();
}

