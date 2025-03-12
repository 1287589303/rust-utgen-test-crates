// Answer 0

#[test]
fn test_fmt_with_valid_ident_fragment() {
    struct ValidIdentFragment;

    impl IdentFragment for ValidIdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "ValidIdentFragment")
        }
    }

    let fragment = &mut ValidIdentFragment;
    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new();
        fragment.fmt(&mut formatter).unwrap();
        output = formatter.to_string();
    }
    assert_eq!(output, "ValidIdentFragment");
}

#[test]
fn test_fmt_with_reference() {
    struct RefIdentFragment;

    impl IdentFragment for RefIdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "RefIdentFragment")
        }
    }

    let fragment = &mut RefIdentFragment;
    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new();
        fragment.fmt(&mut formatter).unwrap();
        output = formatter.to_string();
    }
    assert_eq!(output, "RefIdentFragment");
}

#[should_panic]
fn test_fmt_should_panic() {
    struct PanicIdentFragment;

    impl IdentFragment for PanicIdentFragment {
        fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
            panic!("This should panic!");
        }
    }

    let fragment = &mut PanicIdentFragment;
    let mut formatter = fmt::Formatter::new();
    fragment.fmt(&mut formatter).unwrap();
}

