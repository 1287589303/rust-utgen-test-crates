// Answer 0

#[test]
fn test_is_u64_neg_int_negative_one() {
    let number = Number { n: N::NegInt(-1) };
    number.is_u64();
}

#[test]
fn test_is_u64_neg_int_min_i64() {
    let number = Number { n: N::NegInt(i64::MIN) };
    number.is_u64();
}

#[test]
fn test_is_u64_neg_int_large_negative() {
    let number = Number { n: N::NegInt(-2147483648) };
    number.is_u64();
}

