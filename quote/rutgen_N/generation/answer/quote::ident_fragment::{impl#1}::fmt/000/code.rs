// Answer 0

#[test]
fn test_fmt_ident_fragment() {
    use std::fmt;

    struct IdentFragment(u32);

    impl fmt::Display for IdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "IdentFragment({})", self.0)
        }
    }

    let ident_fragment = IdentFragment(10);
    let mut output = String::new();
    let result = write!(&mut output, "{}", ident_fragment);
    assert!(result.is_ok());
    assert_eq!(output, "IdentFragment(10)");
}

