// Answer 0

#[test]
fn test_into_ast_empty_case() {
    let span = Span { start: Position(0), end: Position(0) };
    let concat = Concat { span, asts: Vec::new() };
    let _result = concat.into_ast();
}

#[test]
fn test_into_ast_single_empty_span() {
    let span = Span { start: Position(0), end: Position(0) };
    let concat = Concat { span, asts: Vec::new() };
    let _result = concat.into_ast();
}

