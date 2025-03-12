// Answer 0

#[test]
fn test_has_subexprs_group_with_subexpressions() {
    let span = Span { start: Position(0), end: Position(10) };
    let sub_expr = Ast::literal(Box::new(Literal { span: span.clone(), kind: LiteralKind::Character, c: 'a' }));
    let group = Ast::group(Box::new(Group { span, kind: GroupKind::Capturing, ast: Box::new(sub_expr) }));
    let result = group.has_subexprs();
}

#[test]
fn test_has_subexprs_group_with_multiple_subexpressions() {
    let span = Span { start: Position(0), end: Position(20) };
    let sub_expr1 = Ast::literal(Box::new(Literal { span: span.clone(), kind: LiteralKind::Character, c: 'b' }));
    let sub_expr2 = Ast::literal(Box::new(Literal { span: span.clone(), kind: LiteralKind::Character, c: 'c' }));
    let group = Ast::group(Box::new(Group { span, kind: GroupKind::Capturing, ast: Box::new(Ast::alternation(Box::new(Alternation { span, asts: vec![Box::new(sub_expr1), Box::new(sub_expr2)] }))) }));
    let result = group.has_subexprs();
}

#[test]
fn test_has_subexprs_group_with_empty_subexpression() {
    let span = Span { start: Position(0), end: Position(15) };
    let empty_sub_expr = Ast::empty(span.clone());
    let group = Ast::group(Box::new(Group { span, kind: GroupKind::Capturing, ast: Box::new(empty_sub_expr) }));
    let result = group.has_subexprs();
}

