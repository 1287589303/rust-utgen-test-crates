// Answer 0

#[test]
fn test_factor_err_right_ok() {
    let right: Either<Result<String, u32>, Result<i32, u32>> = Right(Ok(String::from("test")));
    let _ = right.factor_err();
}

#[test]
fn test_factor_err_right_err() {
    let right: Either<Result<String, u32>, Result<i32, u32>> = Right(Err(42));
    let _ = right.factor_err();
}

#[test]
fn test_factor_err_right_mixed() {
    let right: Either<Result<String, u32>, Result<i32, u32>> = Right(Ok(String::from("mixed")));
    let left: Either<Result<i32, u32>, Result<String, u32>> = Left(Err(43));
    let _ = right.factor_err();
    let _ = left.factor_err();
}

