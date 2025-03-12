// Answer 0

#[test]
fn test_has_subexprs_flags() {
    let span = Span { start: Position(0), end: Position(1) };
    let flags = SetFlags { span, flags: Flags::new() };
    let ast = Ast::flags(flags);
    let result = ast.has_subexprs();
}

#[test]
fn test_has_subexprs_empty() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::empty(span);
    let result = ast.has_subexprs();
}

#[test]
fn test_has_subexprs_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal { span, kind: LiteralKind::default(), c: 'a' };
    let ast = Ast::literal(literal);
    let result = ast.has_subexprs();
}

#[test]
fn test_has_subexprs_dot() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::dot(span);
    let result = ast.has_subexprs();
}

#[test]
fn test_has_subexprs_assertion() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = Assertion { span, kind: AssertionKind::default() };
    let ast = Ast::assertion(assertion);
    let result = ast.has_subexprs();
}

#[test]
fn test_has_subexprs_class_unicode() {
    let span = Span { start: Position(0), end: Position(1) };
    let class_unicode = ClassUnicode { span, negated: false, kind: ClassUnicodeKind::default() };
    let ast = Ast::class_unicode(class_unicode);
    let result = ast.has_subexprs();
}

#[test]
fn test_has_subexprs_class_perl() {
    let span = Span { start: Position(0), end: Position(1) };
    let class_perl = ClassPerl { span, kind: ClassPerlKind::default(), negated: false };
    let ast = Ast::class_perl(class_perl);
    let result = ast.has_subexprs();
}

