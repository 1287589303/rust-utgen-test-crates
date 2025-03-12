// Answer 0

#[test]
fn test_either_into_left() {
    let left: Either<u16, u32> = Left(3u16);
    let result: u64 = left.either_into();
}

#[test]
fn test_either_into_right() {
    let right: Either<u16, u32> = Right(7u32);
    let result: u64 = right.either_into();
}

#[test]
fn test_either_into_left_min_value() {
    let left: Either<u16, u32> = Left(0u16);
    let result: u64 = left.either_into();
}

#[test]
fn test_either_into_right_min_value() {
    let right: Either<u16, u32> = Right(0u32);
    let result: u64 = right.either_into();
}

#[test]
fn test_either_into_left_max_value() {
    let left: Either<u16, u32> = Left(u16::MAX);
    let result: u64 = left.either_into();
}

#[test]
fn test_either_into_right_max_value() {
    let right: Either<u16, u32> = Right(u32::MAX);
    let result: u64 = right.either_into();
}

