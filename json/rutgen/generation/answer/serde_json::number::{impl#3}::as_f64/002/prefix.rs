// Answer 0

#[test]
fn test_as_f64_negative_one() {
    let number = Number { n: N::NegInt(-1) };
    let _result = number.as_f64();
}

#[test]
fn test_as_f64_negative_max_i64() {
    let number = Number { n: N::NegInt(i64::MIN) }; // -9223372036854775808
    let _result = number.as_f64();
}

#[test]
fn test_as_f64_negative_large_integer() {
    let number = Number { n: N::NegInt(-12345678901234) };
    let _result = number.as_f64();
}

#[test]
fn test_as_f64_negative_small_integer() {
    let number = Number { n: N::NegInt(-42) };
    let _result = number.as_f64();
}

