// Answer 0

#[test]
fn test_delim_span() {
    use proc_macro2::extra::DelimSpan;
    use proc_macro2::Span;

    let span1 = Span::call_site();
    let span2 = Span::from_span(1, 2);
    let delim_span = DelimSpan::new(span1, span2);
    
    assert_eq!(delim_span.__span(), Span::join(span1, span2));
}

#[test]
fn test_empty_delim_span() {
    use proc_macro2::extra::DelimSpan;
    use proc_macro2::Span;

    let empty_span = Span::call_site();
    let delim_span = DelimSpan::new(empty_span, empty_span);
    
    assert_eq!(delim_span.__span(), empty_span);
}

