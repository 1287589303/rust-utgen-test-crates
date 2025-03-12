// Answer 0

#[test]
fn test_span_returns_some_for_ident() {
    use proc_macro2::Ident;

    let ident = Ident::new("test_ident", Span::call_site());
    assert!(ident.span().is_some());
}

#[test]
fn test_span_returns_correct_span_for_ident() {
    use proc_macro2::{Ident, Span};

    let ident = Ident::new("test_ident", Span::call_site());
    assert_eq!(ident.span(), Some(Span::call_site()));
}

