// Answer 0

#[test]
fn test_output_slice_too_small_fmt() {
    struct TestStruct;

    impl std::fmt::Display for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Output slice too small")
        }
    }
    
    let test_value = TestStruct;
    let mut output = String::new();
    let result = write!(&mut output, "{}", test_value);
    
    assert!(result.is_ok());
    assert_eq!(output, "Output slice too small");
}

