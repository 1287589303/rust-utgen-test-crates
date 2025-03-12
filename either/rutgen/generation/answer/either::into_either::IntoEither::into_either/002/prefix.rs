// Answer 0

#[test]
fn test_into_either_with_integer() {
    let x = 42;
    let result = x.into_either(false);
}

#[test]
fn test_into_either_with_negative_integer() {
    let x = -1;
    let result = x.into_either(false);
}

#[test]
fn test_into_either_with_float() {
    let x = 3.14;
    let result = x.into_either(false);
}

#[test]
fn test_into_either_with_boolean() {
    let x = true;
    let result = x.into_either(false);
}

#[test]
fn test_into_either_with_string() {
    let x = String::from("Hello");
    let result = x.into_either(false);
}

#[test]
fn test_into_either_with_vec() {
    let x = vec![1, 2, 3];
    let result = x.into_either(false);
}

