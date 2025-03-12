// Answer 0

#[test]
fn test_from_either_left_integer() {
    let val = Left(42);
    let result: Result<i32, i32> = Result::from(val);
}

#[test]
fn test_from_either_left_string() {
    let val = Left(String::from("error"));
    let result: Result<String, String> = Result::from(val);
}

#[test]
fn test_from_either_left_custom_type() {
    #[derive(Debug)]
    struct CustomType {
        value: i32,
    }

    let val = Left(CustomType { value: 10 });
    let result: Result<CustomType, CustomType> = Result::from(val);
}

#[test]
fn test_from_either_left_empty_string() {
    let val = Left(String::from(""));
    let result: Result<String, String> = Result::from(val);
}

#[test]
fn test_from_either_left_zero() {
    let val = Left(0);
    let result: Result<i32, i32> = Result::from(val);
}

