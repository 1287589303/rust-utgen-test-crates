// Answer 0

#[test]
fn test_fmt_ident_fragment() {
    struct TestIdentFragment;

    impl IdentFragment for TestIdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestIdentFragment")
        }
    }

    let id_fragment = TestIdentFragment;
    let mut output = String::new();
    let result = id_fragment.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "TestIdentFragment");
}

#[test]
fn test_fmt_ident_fragment_mut() {
    struct TestIdentFragment;

    impl IdentFragment for TestIdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestIdentFragment")
        }
    }

    let mut id_fragment = TestIdentFragment;
    let mut output = String::new();
    let result = (&mut id_fragment).fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "TestIdentFragment");
}

