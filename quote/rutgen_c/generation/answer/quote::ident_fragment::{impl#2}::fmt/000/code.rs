// Answer 0

#[test]
fn test_fmt_with_normal_identifier() {
    use proc_macro2::Ident;
    use core::fmt::Formatter;

    struct TestIdent {
        ident: Ident,
    }

    let id = Ident::new("my_ident", Span::call_site());
    let test_ident = TestIdent { ident: id };

    let mut output = String::new();
    let mut formatter = Formatter::new(&mut output);

    let result = test_ident.ident.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(output, "my_ident");
}

#[test]
fn test_fmt_with_raw_identifier() {
    use proc_macro2::Ident;
    use core::fmt::Formatter;

    struct TestIdent {
        ident: Ident,
    }

    let id = Ident::new("r#my_ident", Span::call_site());
    let test_ident = TestIdent { ident: id };

    let mut output = String::new();
    let mut formatter = Formatter::new(&mut output);

    let result = test_ident.ident.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(output, "my_ident");
}

