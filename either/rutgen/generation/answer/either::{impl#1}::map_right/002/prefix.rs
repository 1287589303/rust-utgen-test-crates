// Answer 0

#[test]
fn test_map_right_with_left_variant() {
    let left: Either<&str, u32> = Left("test");
    let result = left.map_right(|x| x * 2);
}

#[test]
fn test_map_right_with_right_variant() {
    let right: Either<u32, f64> = Right(123.0);
    let result = right.map_right(|x| x * 2.0);
}

