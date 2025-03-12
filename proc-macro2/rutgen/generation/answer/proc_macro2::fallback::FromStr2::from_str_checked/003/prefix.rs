// Answer 0

#[test]
fn test_from_str_checked_valid_input_success() {
    struct TestType;

    impl FromStr2 for TestType {
        fn valid(src: &str) -> bool {
            src.starts_with("valid_token")
        }

        fn from_str(src: &str) -> Result<Self, proc_macro::LexError> {
            if src.contains("valid_token") {
                Ok(TestType)
            } else {
                Err(proc_macro::LexError)
            }
        }
    }

    let valid_input = "valid_token_example";
    let result = TestType::from_str_checked(valid_input);
}

#[test]
fn test_from_str_checked_valid_input_failure() {
    struct TestType;

    impl FromStr2 for TestType {
        fn valid(src: &str) -> bool {
            src.starts_with("valid_token")
        }

        fn from_str(src: &str) -> Result<Self, proc_macro::LexError> {
            if src == "valid_token_with_error" {
                Err(proc_macro::LexError)
            } else {
                Ok(TestType)
            }
        }
    }

    let valid_input = "valid_token_with_error";
    let result = TestType::from_str_checked(valid_input);
}

#[test]
fn test_from_str_checked_another_valid_input() {
    struct TestType;

    impl FromStr2 for TestType {
        fn valid(src: &str) -> bool {
            src.starts_with("valid")
        }

        fn from_str(src: &str) -> Result<Self, proc_macro::LexError> {
            if src == "valid_token_2" {
                Ok(TestType)
            } else {
                Err(proc_macro::LexError)
            }
        }
    }

    let valid_input = "valid_token_2";
    let result = TestType::from_str_checked(valid_input);
}

