// Answer 0

#[test]
fn test_map_left_with_identity_function() {
    let value: Either<i32, i32> = Left(10);
    let other = value.map(|x| x);
}

#[test]
fn test_map_left_with_integer_function() {
    let value: Either<i32, i32> = Left(5);
    let other = value.map(|x| x * 3);
}

#[test]
fn test_map_left_with_string_function() {
    let value: Either<String, i32> = Left(String::from("test"));
    let other = value.map(|s| s.len());
}

#[test]
fn test_map_left_with_float_function() {
    let value: Either<f64, f64> = Left(2.5);
    let other = value.map(|x| x * 4.0);
}

#[test]
fn test_map_left_with_complex_function() {
    let value: Either<i32, f64> = Left(7);
    let other = value.map(|x| x + 10);
}

