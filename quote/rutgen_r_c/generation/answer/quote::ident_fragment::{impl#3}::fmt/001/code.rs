// Answer 0

#[test]
fn test_ident_fragment_fmt() {
    struct MyIdentFragment;

    impl fmt::Display for MyIdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "MyIdentFragment")
        }
    }

    impl IdentFragment for MyIdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "MyIdentFragment")
        }
    }

    let my_ident = Cow::Owned(MyIdentFragment);
    let mut output = String::new();
    let result = my_ident.fmt(&mut fmt::Formatter::new());
    assert!(result.is_ok());
    assert_eq!(output, "MyIdentFragment");
}

#[test]
fn test_ident_fragment_span() {
    struct MyIdentFragment;

    impl IdentFragment for MyIdentFragment {}

    let my_ident = Cow::Owned(MyIdentFragment);
    let span = my_ident.span();
    assert!(span.is_none());
}

