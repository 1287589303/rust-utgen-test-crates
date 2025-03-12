// Answer 0

#[test]
fn test_into_inner_left_i32() {
    let left: Either<i32, i32> = Left(123);
    let _result = left.into_inner();
}

#[test]
fn test_into_inner_right_i32() {
    let right: Either<i32, i32> = Right(123);
    let _result = right.into_inner();
}

#[test]
fn test_into_inner_left_float() {
    let left: Either<f64, f64> = Left(45.67);
    let _result = left.into_inner();
}

#[test]
fn test_into_inner_right_float() {
    let right: Either<f64, f64> = Right(45.67);
    let _result = right.into_inner();
}

#[test]
fn test_into_inner_left_bool() {
    let left: Either<bool, bool> = Left(true);
    let _result = left.into_inner();
}

#[test]
fn test_into_inner_right_bool() {
    let right: Either<bool, bool> = Right(true);
    let _result = right.into_inner();
}

#[test]
fn test_into_inner_left_string() {
    let left: Either<String, String> = Left(String::from("Hello"));
    let _result = left.into_inner();
}

#[test]
fn test_into_inner_right_string() {
    let right: Either<String, String> = Right(String::from("Hello"));
    let _result = right.into_inner();
}

#[test]
fn test_into_inner_left_empty_string() {
    let left: Either<String, String> = Left(String::from(""));
    let _result = left.into_inner();
}

#[test]
fn test_into_inner_right_empty_string() {
    let right: Either<String, String> = Right(String::from(""));
    let _result = right.into_inner();
}

#[test]
fn test_into_inner_left_unit() {
    let left: Either<(), ()> = Left(());
    let _result = left.into_inner();
}

#[test]
fn test_into_inner_right_unit() {
    let right: Either<(), ()> = Right(());
    let _result = right.into_inner();
}

#[test]
fn test_into_inner_left_zero() {
    let left: Either<u32, u32> = Left(0);
    let _result = left.into_inner();
}

#[test]
fn test_into_inner_right_zero() {
    let right: Either<u32, u32> = Right(0);
    let _result = right.into_inner();
}

