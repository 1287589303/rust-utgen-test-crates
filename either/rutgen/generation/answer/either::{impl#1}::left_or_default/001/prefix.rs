// Answer 0

#[test]
fn test_left_or_default_right_case_1() {
    let right: Either<String, u32> = Right(42);
    let result = right.left_or_default();
}

#[test]
fn test_left_or_default_right_case_2() {
    let right: Either<String, u32> = Right(100);
    let result = right.left_or_default();
}

#[test]
fn test_left_or_default_right_case_3() {
    let right: Either<String, u32> = Right(0);
    let result = right.left_or_default();
}

#[test]
fn test_left_or_default_right_case_4() {
    let right: Either<String, u32> = Right(std::u32::MAX);
    let result = right.left_or_default();
}

#[test]
fn test_left_or_default_right_case_5() {
    let right: Either<String, u32> = Right(7);
    let result = right.left_or_default();
}

