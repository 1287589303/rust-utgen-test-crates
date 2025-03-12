// Answer 0

#[test]
fn test_left_or_with_right_variant() {
    let right_value: Either<i32, i32> = Right(5);
    let default_value: i32 = 10;
    let result = right_value.left_or(default_value);
}

#[test]
fn test_left_or_with_right_variant_string() {
    let right_value: Either<&str, &str> = Right("right");
    let default_value: &str = "left";
    let result = right_value.left_or(default_value);
}

#[test]
fn test_left_or_with_right_variant_float() {
    let right_value: Either<f32, f32> = Right(3.14);
    let default_value: f32 = 1.61;
    let result = right_value.left_or(default_value);
}

#[test]
fn test_left_or_with_right_variant_empty_string() {
    let right_value: Either<&str, &str> = Right("right");
    let default_value: &str = "";
    let result = right_value.left_or(default_value);
}

#[test]
fn test_left_or_with_right_variant_char() {
    let right_value: Either<char, char> = Right('y');
    let default_value: char = 'x';
    let result = right_value.left_or(default_value);
}

