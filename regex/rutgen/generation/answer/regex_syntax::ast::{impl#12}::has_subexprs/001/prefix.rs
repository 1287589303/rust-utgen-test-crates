// Answer 0

#[test]
fn test_has_subexprs_concat_non_empty() {
    let span = Span { start: Position(0), end: Position(10) };
    let ast_elements = vec![
        Ast::literal(Box::new(Literal { span: span.clone(), kind: LiteralKind::Simple, c: 'a' })),
        Ast::literal(Box::new(Literal { span: span.clone(), kind: LiteralKind::Simple, c: 'b' })),
        Ast::class_unicode(Box::new(ClassUnicode { span: span.clone(), negated: false, kind: ClassUnicodeKind::Letter })),
    ];
    let concat = Ast::concat(Box::new(Concat { span, asts: ast_elements }));
    let result = concat.has_subexprs();
}

#[test]
fn test_has_subexprs_concat_with_repetition() {
    let span = Span { start: Position(0), end: Position(15) };
    let repetition = Box::new(Repetition { min: 1, max: Some(3), greedy: true, sub: Box::new(Ast::literal(Box::new(Literal { span: span.clone(), kind: LiteralKind::Simple, c: 'c' }))) });
    let ast_elements = vec![
        Ast::class_perl(Box::new(ClassPerl { span: span.clone(), kind: ClassPerlKind::Digit, negated: false })),
        Ast::repetition(repetition),
    ];
    let concat = Ast::concat(Box::new(Concat { span, asts: ast_elements }));
    let result = concat.has_subexprs();
} 

#[test]
fn test_has_subexprs_concat_with_group() {
    let span = Span { start: Position(0), end: Position(20) };
    let group = Box::new(Group { span: span.clone(), kind: GroupKind::Capture, ast: Box::new(Ast::literal(Box::new(Literal { span: span.clone(), kind: LiteralKind::Simple, c: 'd' }))) });
    let ast_elements = vec![
        Ast::class_bracketed(Box::new(ClassBracketed { span: span.clone(), negated: false, kind: ClassSet::Union })),
        Ast::group(group),
    ];
    let concat = Ast::concat(Box::new(Concat { span, asts: ast_elements }));
    let result = concat.has_subexprs();
}

#[test]
fn test_has_subexprs_concat_with_alternation() {
    let span = Span { start: Position(0), end: Position(25) };
    let alternation = Box::new(Alternation { span: span.clone(), asts: vec![Ast::dot(Box::new(span.clone())), Ast::assertion(Box::new(Assertion { span: span.clone(), kind: AssertionKind::WordBoundary }))] });
    let ast_elements = vec![
        Ast::literal(Box::new(Literal { span: span.clone(), kind: LiteralKind::Simple, c: 'e' })),
        Ast::alternation(alternation),
    ];
    let concat = Ast::concat(Box::new(Concat { span, asts: ast_elements }));
    let result = concat.has_subexprs();
}

