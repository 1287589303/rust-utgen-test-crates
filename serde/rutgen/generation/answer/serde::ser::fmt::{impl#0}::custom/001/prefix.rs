// Answer 0

#[test]
fn test_custom_with_string() {
    let error = fmt::Error::custom("test string");
}

#[test]
fn test_custom_with_number() {
    let error = fmt::Error::custom(123);
}

#[test]
fn test_custom_with_empty_string() {
    let error = fmt::Error::custom("");
}

#[test]
fn test_custom_with_custom_struct() {
    struct MyStruct;
    impl std::fmt::Display for MyStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "MyStruct display");
        }
    }

    let my_struct = MyStruct;
    let error = fmt::Error::custom(my_struct);
}

