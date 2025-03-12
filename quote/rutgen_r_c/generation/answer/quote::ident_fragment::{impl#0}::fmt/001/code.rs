// Answer 0

#[test]
fn test_fmt_on_ident_fragment() {
    struct TestIdentFragment;
    
    impl IdentFragment for TestIdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestIdentFragment")
        }
    }

    let test_instance = TestIdentFragment;
    let mut output = String::new();
    let result = test_instance.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "TestIdentFragment");
}

#[test]
fn test_span_on_ident_fragment() {
    struct TestIdentFragment;

    impl IdentFragment for TestIdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestIdentFragment")
        }
    }

    let test_instance = TestIdentFragment;
    assert_eq!(test_instance.span(), None);
}

#[should_panic]
fn test_fmt_panics_on_invalid_state() {
    struct InvalidIdentFragment;

    impl IdentFragment for InvalidIdentFragment {
        fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
            panic!("Invalid state");
        }
    }

    let invalid_instance = InvalidIdentFragment;
    let _ = invalid_instance.fmt(&mut String::new());
}

