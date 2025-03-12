// Answer 0

#[test]
fn test_is_left_right_variant() {
    let value: Either<i32, &str> = Right("the right value");
    let result = value.is_left();
}

#[test]
fn test_is_left_right_variant_with_different_types() {
    let value: Either<f64, bool> = Right(true);
    let result = value.is_left();
}

#[test]
fn test_is_left_empty_string_right_variant() {
    let value: Either<&str, String> = Right(String::from(""));
    let result = value.is_left();
}

#[test]
fn test_is_left_right_variant_with_number() {
    let value: Either<u32, f64> = Right(3.14);
    let result = value.is_left();
}

#[test]
fn test_is_left_complex_right_variant() {
    let value: Either<(i32, i32), Vec<i32>> = Right(vec![1, 2, 3]);
    let result = value.is_left();
}

