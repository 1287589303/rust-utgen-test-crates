// Answer 0

#[test]
fn test_unwrap_left_with_i32() {
    let left: Either<i32, ()> = Left(42);
    let _ = left.unwrap_left();
}

#[test]
fn test_unwrap_left_with_string() {
    let left: Either<String, ()> = Left(String::from("Hello"));
    let _ = left.unwrap_left();
}

#[test]
fn test_unwrap_left_with_float() {
    let left: Either<f64, ()> = Left(3.14);
    let _ = left.unwrap_left();
}

#[test]
fn test_unwrap_left_with_tuple() {
    let left: Either<(i32, i32), ()> = Left((1, 2));
    let _ = left.unwrap_left();
}

