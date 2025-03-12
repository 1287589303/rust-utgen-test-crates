// Answer 0

#[test]
fn test_into_ast_with_assertion_kind_b() {
    let assertion = Assertion {
        span: Span { start: Position(0), end: Position(10) },
        kind: AssertionKind::B,
    };
    let primitive = Primitive::Assertion(assertion);
    let _ = primitive.into_ast();
}

#[test]
fn test_into_ast_with_assertion_kind_not_b() {
    let assertion = Assertion {
        span: Span { start: Position(0), end: Position(10) },
        kind: AssertionKind::NotB,
    };
    let primitive = Primitive::Assertion(assertion);
    let _ = primitive.into_ast();
}

#[test]
fn test_into_ast_with_assertion_kind_start() {
    let assertion = Assertion {
        span: Span { start: Position(0), end: Position(1) },
        kind: AssertionKind::Start,
    };
    let primitive = Primitive::Assertion(assertion);
    let _ = primitive.into_ast();
}

#[test]
fn test_into_ast_with_assertion_kind_end() {
    let assertion = Assertion {
        span: Span { start: Position(10), end: Position(20) },
        kind: AssertionKind::End,
    };
    let primitive = Primitive::Assertion(assertion);
    let _ = primitive.into_ast();
}

