// Answer 0

#[test]
fn test_fmt_with_r_prefix() {
    struct MyIdent(Ident);
    
    let id = Ident::new("r#example", Span::call_site());
    let my_ident = MyIdent(id);
    let mut output = String::new();
    let result = my_ident.fmt(&mut fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(output, "example");
}

#[test]
fn test_fmt_without_r_prefix() {
    struct MyIdent(Ident);
    
    let id = Ident::new("example", Span::call_site());
    let my_ident = MyIdent(id);
    let mut output = String::new();
    let result = my_ident.fmt(&mut fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(output, "example");
}

