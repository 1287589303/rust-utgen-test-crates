// Answer 0

#[test]
fn test_factor_iter_left() {
    let left: Either<&[&str; 1], Vec<u8>> = Left(["hello"]);
    let iter = left.factor_iter();
    let result: Vec<_> = iter.collect();
}

#[test]
fn test_factor_iter_right() {
    let right: Either<[&str; 2], Vec<u8>> = Right(vec![0, 1]);
    let iter = right.factor_iter();
    let result: Vec<_> = iter.collect();
}

#[test]
fn test_factor_iter_combined() {
    let both: Either<&[&str; 2], Vec<u8>> = Left(["hello", "world"]);
    let iter = both.factor_iter();
    let result: Vec<_> = iter.collect();
}

