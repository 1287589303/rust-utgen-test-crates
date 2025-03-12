// Answer 0

#[test]
fn test_map_with_right_variant_identity() {
    let value: Either<i32, i32> = Right(42);
    let other = value.map(|x| x);
}

#[test]
fn test_map_with_right_variant_increment() {
    let value: Either<i32, i32> = Right(0);
    let other = value.map(|x| x + 1);
}

#[test]
fn test_map_with_right_variant_max_integer() {
    let value: Either<i32, i32> = Right(2147483647);
    let other = value.map(|x| x);
}

#[test]
fn test_map_with_right_variant_negative() {
    let value: Either<i32, i32> = Right(-1);
    let other = value.map(|x| x * 2);
}

#[test]
fn test_map_with_right_variant_large_number() {
    let value: Either<i32, i32> = Right(100);
    let other = value.map(|x| x * 3);
}

