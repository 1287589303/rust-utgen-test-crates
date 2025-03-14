// Answer 0

#[test]
fn test_try_from_success() {
    struct Base64;
    impl Base64 {
        fn new(value: &str) -> Result<Self, String> {
            if value.is_empty() {
                return Err("Empty string".to_string());
            }
            Ok(Base64)
        }

        type Error = String;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Self::new(value)
        }
    }

    let result = Base64::try_from("test_value");
    assert!(result.is_ok());
}

#[test]
fn test_try_from_empty_string() {
    struct Base64;
    impl Base64 {
        fn new(value: &str) -> Result<Self, String> {
            if value.is_empty() {
                return Err("Empty string".to_string());
            }
            Ok(Base64)
        }

        type Error = String;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Self::new(value)
        }
    }

    let result = Base64::try_from("");
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Empty string");
}

