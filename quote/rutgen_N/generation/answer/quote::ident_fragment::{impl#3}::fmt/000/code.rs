// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct TestStruct;

    impl fmt::Display for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestStruct")
        }
    }

    let test_instance = TestStruct;
    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        assert!(test_instance.fmt(&mut formatter).is_ok());
    }
    assert_eq!(output, "TestStruct");
}

