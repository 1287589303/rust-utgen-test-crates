// Answer 0

#[test]
fn test_factor_none_left_some() {
    let left: Either<Option<Vec<u8>>, Option<String>> = Left(Some(vec![1, 2, 3]));
    let _ = left.factor_none();
}

#[test]
fn test_factor_none_left_none() {
    let left: Either<Option<Vec<u8>>, Option<String>> = Left(None);
    let _ = left.factor_none();
}

#[test]
fn test_factor_none_right_some() {
    let right: Either<Option<Vec<u8>>, Option<String>> = Right(Some(String::from("hello")));
    let _ = right.factor_none();
}

#[test]
fn test_factor_none_right_none() {
    let right: Either<Option<Vec<u8>>, Option<String>> = Right(None);
    let _ = right.factor_none();
}

