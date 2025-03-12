// Answer 0

#[test]
fn test_factor_none_right_none() {
    let right: Either<Option<i32>, Option<i32>> = Right(None);
    right.factor_none();
}

#[test]
fn test_factor_none_right_some_unit() {
    let right: Either<Option<()>, Option<()>> = Right(Some(()));
    right.factor_none();
}

#[test]
fn test_factor_none_right_some_small_primitive() {
    let right: Either<Option<i32>, Option<i32>> = Right(Some(42));
    right.factor_none();
}

#[test]
fn test_factor_none_right_some_string() {
    let right: Either<Option<String>, Option<String>> = Right(Some(String::from("hello")));
    right.factor_none();
}

#[test]
fn test_factor_none_right_some_vec() {
    let right: Either<Option<Vec<u8>>, Option<Vec<u8>>> = Right(Some(vec![1, 2, 3]));
    right.factor_none();
}

