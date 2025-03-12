// Answer 0

#[test]
fn test_ident_fragment_span() {
    struct TestIdentFragment;

    impl IdentFragment for TestIdentFragment {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    let instance = TestIdentFragment;
    assert_eq!(instance.span(), None);
}

