// Answer 0

#[test]
fn test_has_subexprs_class_bracketed_negated() {
    let span = Span { start: Position::default(), end: Position::default() };
    let class_bracketed = ClassBracketed {
        span,
        negated: true,
        kind: ClassSet::default(),
    };
    let ast = Ast::class_bracketed(class_bracketed);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_class_bracketed_non_negated() {
    let span = Span { start: Position::default(), end: Position::default() };
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::default(),
    };
    let ast = Ast::class_bracketed(class_bracketed);
    ast.has_subexprs();
}

