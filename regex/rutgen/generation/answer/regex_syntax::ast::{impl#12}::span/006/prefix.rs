// Answer 0

#[test]
fn test_span_class_perl_non_negated() {
    let span = Span {
        start: Position(0),
        end: Position(10),
    };
    let class_perl = ClassPerl {
        span: span.clone(),
        kind: ClassPerlKind::Digit,
        negated: false,
    };
    let ast = Ast::ClassPerl(Box::new(class_perl));
    let _ = ast.span();
}

#[test]
fn test_span_class_perl_negated() {
    let span = Span {
        start: Position(5),
        end: Position(15),
    };
    let class_perl = ClassPerl {
        span: span.clone(),
        kind: ClassPerlKind::Word,
        negated: true,
    };
    let ast = Ast::ClassPerl(Box::new(class_perl));
    let _ = ast.span();
}

#[test]
fn test_span_class_perl_minimal_values() {
    let span = Span {
        start: Position(0),
        end: Position(1),
    };
    let class_perl = ClassPerl {
        span: span.clone(),
        kind: ClassPerlKind::Word,
        negated: false,
    };
    let ast = Ast::ClassPerl(Box::new(class_perl));
    let _ = ast.span();
}

#[test]
fn test_span_class_perl_large_values() {
    let span = Span {
        start: Position(100),
        end: Position(200),
    };
    let class_perl = ClassPerl {
        span: span.clone(),
        kind: ClassPerlKind::Space,
        negated: false,
    };
    let ast = Ast::ClassPerl(Box::new(class_perl));
    let _ = ast.span();
}

