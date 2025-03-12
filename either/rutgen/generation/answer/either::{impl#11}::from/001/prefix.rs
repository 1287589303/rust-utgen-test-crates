// Answer 0

#[test]
fn test_from_either_right_unit() {
    let val = Either::Right(());
    let result: Result<(), _> = Result::from(val);
}

#[test]
fn test_from_either_right_integer() {
    let val = Either::Right(42);
    let result: Result<i32, _> = Result::from(val);
}

#[test]
fn test_from_either_right_string() {
    let val = Either::Right(String::from("test"));
    let result: Result<String, _> = Result::from(val);
}

#[test]
fn test_from_either_right_custom_type() {
    struct CustomType {
        value: i32,
    }
    
    let val = Either::Right(CustomType { value: 5 });
    let result: Result<CustomType, _> = Result::from(val);
}

#[test]
fn test_from_either_right_floating_point() {
    let val = Either::Right(3.14);
    let result: Result<f64, _> = Result::from(val);
}

