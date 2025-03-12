// Answer 0

#[test]
fn test_parse_uncounted_repetition_with_question_mark() {
    let pattern = "abc?";
    let position = Position { offset: 3, line: 1, column: 4 };
    let span = Span::new(position, position);
    let ast = Concat { span, asts: vec![Ast::literal(ast::Literal { span })] };
    let parser = ParserI { parser: Parser { /* initialize necessary fields */ }, pattern };
    
    let result = parser.parse_uncounted_repetition(ast, ast::RepetitionKind::ZeroOrOne);
}

#[test]
fn test_parse_uncounted_repetition_with_asterisk() {
    let pattern = "abc*";
    let position = Position { offset: 3, line: 1, column: 4 };
    let span = Span::new(position, position);
    let ast = Concat { span, asts: vec![Ast::literal(ast::Literal { span })] };
    let parser = ParserI { parser: Parser { /* initialize necessary fields */ }, pattern };
    
    let result = parser.parse_uncounted_repetition(ast, ast::RepetitionKind::ZeroOrMore);
}

