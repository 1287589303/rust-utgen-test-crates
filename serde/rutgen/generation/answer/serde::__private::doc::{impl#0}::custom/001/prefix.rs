// Answer 0

#[test]
fn test_custom_with_string() {
    let err = Error::custom("test string");
}

#[test]
fn test_custom_with_integer() {
    let err = Error::custom(42);
}

#[test]
fn test_custom_with_struct() {
    struct CustomDisplay;
    impl std::fmt::Display for CustomDisplay {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "CustomDisplay structure")
        }
    }
    let err = Error::custom(CustomDisplay);
}

#[should_panic]
fn test_custom_with_non_display() {
    struct NonDisplay;
    let err = Error::custom(NonDisplay);
}

