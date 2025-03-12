// Answer 0

#[test]
fn test_primitive_into_ast_literal() {
    let span = Span { start: 0, end: 5 };
    let lit = Literal { span: span.clone(), kind: LiteralKind::Char, c: 'a' };
    let primitive = Primitive::Literal(lit);
    let _ast = primitive.into_ast();
}

#[test]
fn test_primitive_into_ast_assertion() {
    let span = Span { start: 0, end: 1 };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::WordBoundary };
    let primitive = Primitive::Assertion(assertion);
    let _ast = primitive.into_ast();
}

#[test]
fn test_primitive_into_ast_dot() {
    let span = Span { start: 0, end: 1 };
    let primitive = Primitive::Dot(span);
    let _ast = primitive.into_ast();
}

#[test]
fn test_primitive_into_ast_perl() {
    let span = Span { start: 0, end: 2 };
    let perl_class = ClassPerl { span: span.clone(), kind: ClassPerlKind::Digit, negated: false };
    let primitive = Primitive::Perl(perl_class);
    let _ast = primitive.into_ast();
}

#[test]
fn test_primitive_into_ast_unicode() {
    let span = Span { start: 0, end: 3 };
    let unicode_class = ClassUnicode { span: span.clone(), negated: false, kind: ClassUnicodeKind::Letter };
    let primitive = Primitive::Unicode(unicode_class);
    let _ast = primitive.into_ast();
}

