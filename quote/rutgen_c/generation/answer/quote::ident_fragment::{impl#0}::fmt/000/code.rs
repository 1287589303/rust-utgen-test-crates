// Answer 0

#[test]
fn test_fmt_for_ident_fragment() {
    struct MyIdentFragment;

    impl IdentFragment for MyIdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "MyIdentFragment")
        }
    }

    let fragment = MyIdentFragment;
    let mut output = String::new();
    let result = fragment.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "MyIdentFragment");
}

#[test]
fn test_fmt_for_mut_ident_fragment() {
    struct MyIdentFragment;

    impl IdentFragment for MyIdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "MyIdentFragment")
        }
    }

    let mut fragment = MyIdentFragment;
    let mut output = String::new();
    let result = IdentFragment::fmt(&mut fragment, &mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "MyIdentFragment");
}

