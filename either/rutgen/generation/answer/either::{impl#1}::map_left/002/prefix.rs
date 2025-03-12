// Answer 0

#[test]
fn test_map_left_basic() {
    let left: Either<i32, u32> = Left(123);
    let result = left.map_left(|x| x * 2);
}

#[test]
fn test_map_left_negative() {
    let left: Either<i32, u32> = Left(-50);
    let result = left.map_left(|x| x * 3);
}

#[test]
fn test_map_left_zero() {
    let left: Either<i32, u32> = Left(0);
    let result = left.map_left(|x| x + 1);
}

#[test]
fn test_map_left_string() {
    let left: Either<&str, u32> = Left("Hello");
    let result = left.map_left(|x| format!("{} World", x));
}

#[test]
fn test_map_left_float() {
    let left: Either<f32, u32> = Left(3.14);
    let result = left.map_left(|x| x * 2.0);
}

