// Answer 0

#[test]
fn test_has_subexprs_repetition_valid_min_max() {
    let span = Span { start: Position(0), end: Position(5) };
    let sub_expression = Box::new(Ast::Literal(Box::new(Literal { span, kind: LiteralKind::Unicode, c: 'a' })));
    let repetition = Repetition {
        min: 1,
        max: Some(3),
        greedy: true,
        sub: sub_expression,
    };
    let ast = Ast::Repetition(Box::new(repetition));
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_repetition_min_zero() {
    let span = Span { start: Position(0), end: Position(5) };
    let sub_expression = Box::new(Ast::Dot(Box::new(span)));
    let repetition = Repetition {
        min: 0,
        max: None,
        greedy: false,
        sub: sub_expression,
    };
    let ast = Ast::Repetition(Box::new(repetition));
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_repetition_equal_min_max() {
    let span = Span { start: Position(1), end: Position(4) };
    let sub_expression = Box::new(Ast::ClassPerl(Box::new(ClassPerl { span, kind: ClassPerlKind::Digit, negated: false })));
    let repetition = Repetition {
        min: 2,
        max: Some(2),
        greedy: true,
        sub: sub_expression,
    };
    let ast = Ast::Repetition(Box::new(repetition));
    ast.has_subexprs();
}

