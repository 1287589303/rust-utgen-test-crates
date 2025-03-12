// Answer 0

#[test]
fn test_factor_ok_left_ok() {
    let left: Either<i32, Result<u32, String>> = Left(Ok(42));
    let result = left.factor_ok();
}

#[test]
fn test_factor_ok_left_err() {
    let left: Either<i32, Result<u32, String>> = Left(Err("error".to_string()));
    let result = left.factor_ok();
}

#[test]
fn test_factor_ok_right_ok() {
    let right: Either<Result<u32, i32>, String> = Right(Ok(42));
    let result = right.factor_ok();
}

#[test]
fn test_factor_ok_right_err() {
    let right: Either<Result<u32, i32>, String> = Right(Err("error".to_string()));
    let result = right.factor_ok();
}

