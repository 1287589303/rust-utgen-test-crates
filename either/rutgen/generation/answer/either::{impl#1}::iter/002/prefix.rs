// Answer 0

#[test]
fn test_iter_left() {
    let left: Either<Vec<u32>, Vec<u32>> = Left(vec![1, 2, 3]);
    let right: Either<Vec<u32>, Vec<u32>> = Right(vec![4, 5, 6]);
    let mut iter = left.iter();
    let result: Vec<u32> = iter.collect();
}

#[test]
fn test_iter_right() {
    let left: Either<Vec<u32>, Vec<u32>> = Left(vec![7, 8, 9]);
    let right: Either<Vec<u32>, Vec<u32>> = Right(vec![10, 11, 12]);
    let mut iter = right.iter();
    let result: Vec<u32> = iter.collect();
}

#[test]
fn test_iter_both() {
    let left: Either<Vec<u32>, Vec<u32>> = Left(vec![13, 14]);
    let right: Either<Vec<u32>, Vec<u32>> = Right(vec![15, 16]);
    let mut all = Vec::new();
    all.extend(left.iter());
    all.extend(right.iter());
}

#[test]
fn test_empty_left() {
    let left: Either<Vec<u32>, Vec<u32>> = Left(vec![]);
    let right: Either<Vec<u32>, Vec<u32>> = Right(vec![17, 18]);
    let mut all = Vec::new();
    all.extend(left.iter());
    all.extend(right.iter());
}

#[test]
fn test_empty_right() {
    let left: Either<Vec<u32>, Vec<u32>> = Left(vec![19, 20]);
    let right: Either<Vec<u32>, Vec<u32>> = Right(vec![]);
    let mut all = Vec::new();
    all.extend(left.iter());
    all.extend(right.iter());
}

