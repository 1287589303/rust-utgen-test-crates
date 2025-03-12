// Answer 0

#[test]
fn test_as_i64_negative_one() {
    let number = Number { n: N::NegInt(-1) };
    number.as_i64();
}

#[test]
fn test_as_i64_negative_hundred() {
    let number = Number { n: N::NegInt(-100) };
    number.as_i64();
}

#[test]
fn test_as_i64_negative_int_min() {
    let number = Number { n: N::NegInt(i64::MIN) };
    number.as_i64();
}

#[test]
fn test_as_i64_large_negative() {
    let number = Number { n: N::NegInt(-9223372036854775808) };
    number.as_i64();
}

