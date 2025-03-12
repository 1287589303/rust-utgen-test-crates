// Answer 0

#[test]
fn test_into_ast_perl_negated() {
    let span = Span { start: Position(0), end: Position(1) };
    let perl_class = ClassPerl { span, kind: ClassPerlKind::SomeKind, negated: true };
    let primitive = Primitive::Perl(perl_class);
    let _ = primitive.into_ast();
}

#[test]
fn test_into_ast_perl_non_negated() {
    let span = Span { start: Position(1), end: Position(2) };
    let perl_class = ClassPerl { span, kind: ClassPerlKind::OtherKind, negated: false };
    let primitive = Primitive::Perl(perl_class);
    let _ = primitive.into_ast();
}

#[test]
fn test_into_ast_perl_different_span() {
    let span = Span { start: Position(2), end: Position(3) };
    let perl_class = ClassPerl { span, kind: ClassPerlKind::DifferentKind, negated: false };
    let primitive = Primitive::Perl(perl_class);
    let _ = primitive.into_ast();
}

