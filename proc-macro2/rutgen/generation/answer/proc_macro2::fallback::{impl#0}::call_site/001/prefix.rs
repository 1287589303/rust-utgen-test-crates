// Answer 0

#[test]
fn test_lex_error_call_site() {
    let lex_error = LexError::call_site();
    let span = lex_error.span();
    // The properties of span are implicitly checked by the expected return value
    // which is Span::call_site(), but we can construct it for consistency in context.
    assert_eq!(span, Span::call_site());
}

#[test]
fn test_lex_error_call_site_properties() {
    let lex_error = LexError::call_site();
    let span = lex_error.span();
    // We ensure that the span contains the correct boundary values
    // as defined by the call_site method.
    #[cfg(span_locations)]
    {
        assert_eq!(span.lo, 0);
        assert_eq!(span.hi, 0);
    }
}

