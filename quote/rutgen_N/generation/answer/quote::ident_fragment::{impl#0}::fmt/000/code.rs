// Answer 0

#[test]
fn test_fmt() {
    use std::fmt::{self, Write};

    struct IdentFragment(u32);

    impl IdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "IdentFragment({})", self.0)
        }
    }

    let fragment = IdentFragment(42);
    let mut output = String::new();
    let result = fragment.fmt(&mut fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(output, "IdentFragment(42)");
}

