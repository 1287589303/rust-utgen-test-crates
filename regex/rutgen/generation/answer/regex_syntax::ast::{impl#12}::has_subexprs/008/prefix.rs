// Answer 0

#[test]
fn test_has_subexprs_empty() {
    let span = Span { start: Position::default(), end: Position::default() };
    let ast = Ast::empty(span);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_flags() {
    let span = Span { start: Position::default(), end: Position::default() };
    let flags = SetFlags { span, flags: Flags::default() };
    let ast = Ast::flags(flags);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_literal() {
    let span = Span { start: Position::default(), end: Position::default() };
    let literal = Literal { span, kind: LiteralKind::default(), c: 'a' };
    let ast = Ast::literal(literal);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_dot() {
    let span = Span { start: Position::default(), end: Position::default() };
    let ast = Ast::dot(span);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_assertion() {
    let span = Span { start: Position::default(), end: Position::default() };
    let assertion = Assertion { span, kind: AssertionKind::default() };
    let ast = Ast::assertion(assertion);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_class_unicode() {
    let span = Span { start: Position::default(), end: Position::default() };
    let class_unicode = ClassUnicode { span, negated: false, kind: ClassUnicodeKind::default() };
    let ast = Ast::class_unicode(class_unicode);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_class_perl() {
    let span = Span { start: Position::default(), end: Position::default() };
    let class_perl = ClassPerl { span, kind: ClassPerlKind::default(), negated: false };
    let ast = Ast::class_perl(class_perl);
    ast.has_subexprs();
}

