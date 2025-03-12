// Answer 0

#[test]
fn test_span_assertion_valid() {
    let start_position = Position(0);
    let end_position = Position(5);
    let span = Span { start: start_position, end: end_position };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::WordBoundary };
    let ast = Ast::Assertion(Box::new(assertion));
    let result = ast.span();
}

#[test]
fn test_span_assertion_boundary() {
    let start_position = Position(0);
    let end_position = Position(0);
    let span = Span { start: start_position, end: end_position };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::BeginningOfLine };
    let ast = Ast::Assertion(Box::new(assertion));
    let result = ast.span();
}

#[test]
fn test_span_assertion_non_negative() {
    let start_position = Position(3);
    let end_position = Position(7);
    let span = Span { start: start_position, end: end_position };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::EndOfLine };
    let ast = Ast::Assertion(Box::new(assertion));
    let result = ast.span();
}

#[test]
fn test_span_assertion_start_equals_end() {
    let start_position = Position(4);
    let end_position = Position(4);
    let span = Span { start: start_position, end: end_position };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::WordBoundary };
    let ast = Ast::Assertion(Box::new(assertion));
    let result = ast.span();
}

