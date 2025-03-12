// Answer 0

#[test]
fn test_as_i64_with_large_positive_integer() {
    let number = Number {
        n: N::PosInt(u64::MAX), // u64::MAX > i64::MAX as u64
    };
    number.as_i64();
}

#[test]
fn test_as_i64_with_large_positive_integer_plus_one() {
    let number = Number {
        n: N::PosInt(u64::MAX - 1), // still > i64::MAX as u64
    };
    number.as_i64();
}

