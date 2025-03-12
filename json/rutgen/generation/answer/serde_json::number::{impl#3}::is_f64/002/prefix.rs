// Answer 0

#[test]
fn test_is_f64_neg_int() {
    let number = Number { n: N::NegInt(-1) };
    number.is_f64();
}

#[test]
fn test_is_f64_neg_int_large() {
    let number = Number { n: N::NegInt(-1000000) };
    number.is_f64();
}

#[test]
fn test_is_f64_neg_int_min() {
    let number = Number { n: N::NegInt(i64::MIN) };
    number.is_f64();
}

