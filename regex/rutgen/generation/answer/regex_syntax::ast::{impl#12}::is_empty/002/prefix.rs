// Answer 0

#[test]
fn test_ast_empty() {
    let span = Span { start: Position(0), end: Position(0) };
    let ast = Ast::empty(span);
    assert!(ast.is_empty());
}

#[test]
fn test_ast_empty_with_non_zero_span() {
    let span = Span { start: Position(1), end: Position(2) };
    let ast = Ast::empty(span);
    assert!(ast.is_empty());
}

#[test]
fn test_nested_empty_ast() {
    let span = Span { start: Position(0), end: Position(0) };
    let inner_ast = Ast::empty(span);
    let group = Group { span, kind: GroupKind::SomeKind, ast: Box::new(inner_ast) };
    let ast = Ast::group(group);
    assert!(ast.is_empty());
}

