// Answer 0

#[test]
fn test_span() {
    struct TestIdent(Ident);

    let ident = Ident::new("test_ident", Span::call_site());
    let test_ident = TestIdent(ident);

    let result = test_ident.span();
    assert!(result.is_some());
}

#[test]
fn test_span_none() {
    struct EmptyIdent(Ident);

    let empty_ident = Ident::new("", Span::call_site());
    let test_empty_ident = EmptyIdent(empty_ident);

    let result = test_empty_ident.span();
    assert!(result.is_some());
}

#[test]
fn test_fmt() {
    struct FmtIdent(Ident);

    let ident = Ident::new("r#test_ident", Span::call_site());
    let mut buffer = String::new();
    let mut formatter = fmt::Formatter::new(&mut buffer);
    let fmt_ident = FmtIdent(ident);

    // Testing format without panic
    assert!(fmt_ident.fmt(&mut formatter).is_ok());
    assert_eq!(buffer, "test_ident");
}

