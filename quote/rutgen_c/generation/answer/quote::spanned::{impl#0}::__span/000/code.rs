// Answer 0

#[test]
fn test_span_return_value() {
    let span = Span::call_site();
    assert_eq!(span.__span(), span);
}

#[test]
fn test_span_identity() {
    let span = Span::mixed_site();
    assert_eq!(span.__span(), span);
}

