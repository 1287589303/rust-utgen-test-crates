// Answer 0

#[test]
fn test_fmt_with_rust_keyword_prefix() {
    use proc_macro2::Ident;
    use core::fmt::Write;

    let test_ident = Ident::new("r#async", Span::call_site());
    let mut output = String::new();
    let result = test_ident.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "async");
}

#[test]
fn test_fmt_with_rust_keyword_with_leading_space() {
    use proc_macro2::Ident;
    use core::fmt::Write;

    let test_ident = Ident::new("r#type", Span::call_site());
    let mut output = String::new();
    let result = test_ident.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "type");
}

