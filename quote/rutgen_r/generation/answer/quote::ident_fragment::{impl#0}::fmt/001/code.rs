// Answer 0

#[test]
fn test_fmt_valid_ident_fragment() {
    struct TestIdentFragment(u32);
    
    impl std::fmt::Display for TestIdentFragment {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "IdentFragment {}", self.0)
        }
    }

    let fragment = TestIdentFragment(42);
    let mut output = String::new();
    let result = std::fmt::write(&mut output, format_args!("{}", fragment));
    
    assert!(result.is_ok());
    assert_eq!(output, "IdentFragment 42");
}

#[test]
#[should_panic(expected = "attempt to multiply with overflow")]
fn test_fmt_overflow_ident_fragment() {
    struct OverflowIdentFragment(u32);
    
    impl std::fmt::Display for OverflowIdentFragment {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let _ = self.0.checked_mul(2).expect("attempt to multiply with overflow");
            write!(f, "IdentFragment {}", self.0)
        }
    }

    let fragment = OverflowIdentFragment(u32::MAX);
    let _ = format!("{}", fragment);
}

