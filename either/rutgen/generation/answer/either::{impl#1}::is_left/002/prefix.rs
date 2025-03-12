// Answer 0

#[test]
fn test_is_left_with_integer() {
    let instance = Left(42);
    instance.is_left();
}

#[test]
fn test_is_left_with_string() {
    let instance = Left("left value".to_string());
    instance.is_left();
}

#[test]
fn test_is_left_with_float() {
    let instance = Left(3.14);
    instance.is_left();
}

#[test]
fn test_is_left_with_tuple() {
    let instance = Left((1, "value"));
    instance.is_left();
}

#[test]
fn test_is_left_with_vec() {
    let instance = Left(vec![1, 2, 3]);
    instance.is_left();
}

