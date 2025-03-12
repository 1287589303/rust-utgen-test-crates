// Answer 0

#[test]
fn test_either_into_u8() {
    let right: Either<u16, u32> = Right(255u32);
    let result: u8 = right.either_into();
}

#[test]
fn test_either_into_i32() {
    let right: Either<u16, u32> = Right(123456789u32);
    let result: i32 = right.either_into();
}

#[test]
fn test_either_into_string() {
    let right: Either<&str, &str> = Right("test");
    let result: String = right.either_into();
}

#[test]
fn test_either_into_empty_vec() {
    let right: Either<Vec<u8>, Vec<u8>> = Right(Vec::new());
    let result: Vec<u8> = right.either_into();
}

#[test]
fn test_either_into_large_number() {
    let right: Either<u16, u64> = Right(1_000_000_000_000_000_000u64);
    let result: u64 = right.either_into();
}

