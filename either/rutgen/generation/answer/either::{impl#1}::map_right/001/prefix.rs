// Answer 0

#[test]
fn test_map_right_with_positive_integer() {
    let right: Either<u32, _> = Right(500);
    let result = right.map_right(|x| x * 2);
}

#[test]
fn test_map_right_boundary_case_max() {
    let right: Either<u32, _> = Right(1000);
    let result = right.map_right(|x| x + 1);
}

#[test]
fn test_map_right_boundary_case_min() {
    let right: Either<u32, _> = Right(1);
    let result = right.map_right(|x| x - 1);
}

#[test]
fn test_map_right_with_zero_case() {
    let right: Either<u32, _> = Right(0);
    let result = right.map_right(|x| x + 5);
}

#[test]
fn test_map_right_with_large_integer() {
    let right: Either<u32, _> = Right(999);
    let result = right.map_right(|x| x * 10);
}

