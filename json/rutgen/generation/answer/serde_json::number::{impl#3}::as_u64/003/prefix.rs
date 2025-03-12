// Answer 0

#[test]
fn test_as_u64_positive_integer_max() {
    let number = Number { n: N::PosInt(u64::MAX) };
    let _result = number.as_u64();
}

#[test]
fn test_as_u64_positive_integer_min() {
    let number = Number { n: N::PosInt(0) };
    let _result = number.as_u64();
}

#[test]
fn test_as_u64_positive_integer_small_value() {
    let number = Number { n: N::PosInt(1) };
    let _result = number.as_u64();
}

#[test]
fn test_as_u64_positive_integer_median_value() {
    let number = Number { n: N::PosInt(123456789) };
    let _result = number.as_u64();
}

