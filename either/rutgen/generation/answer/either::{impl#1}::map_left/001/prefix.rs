// Answer 0

#[test]
fn test_map_left_with_right_variant() {
    let right: Either<u32, &str> = Right("value");
    let result = right.map_left(|x| x * 2);
}

#[test]
fn test_map_left_with_right_variant_integer() {
    let right: Either<u32, u32> = Right(42);
    let result = right.map_left(|x| x * 2);
}

#[test]
fn test_map_left_with_right_variant_string() {
    let right: Either<String, &str> = Right("test");
    let result = right.map_left(|x| x.len());
}

#[test]
fn test_map_left_with_right_variant_floating_point() {
    let right: Either<f64, f64> = Right(3.14);
    let result = right.map_left(|x| x * 2.0);
}

#[test]
fn test_map_left_with_right_variant_empty_string() {
    let right: Either<u32, &str> = Right("");
    let result = right.map_left(|x| x * 3);
}

