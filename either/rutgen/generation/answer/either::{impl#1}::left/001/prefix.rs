// Answer 0

#[test]
fn test_left_with_right_variant_string() {
    let right_variant: Either<String, i32> = Right(42);
    let _ = right_variant.left();
}

#[test]
fn test_left_with_right_variant_integer() {
    let right_variant: Either<i32, f64> = Right(3.14);
    let _ = right_variant.left();
}

#[test]
fn test_left_with_right_variant_tuple() {
    let right_variant: Either<(i32, i32), char> = Right('a');
    let _ = right_variant.left();
}

#[test]
fn test_left_with_right_variant_struct() {
    struct SampleStruct;
    let right_variant: Either<SampleStruct, i32> = Right(100);
    let _ = right_variant.left();
}

#[test]
fn test_left_with_right_variant_unit() {
    let right_variant: Either<(), f64> = Right(2.718);
    let _ = right_variant.left();
}

