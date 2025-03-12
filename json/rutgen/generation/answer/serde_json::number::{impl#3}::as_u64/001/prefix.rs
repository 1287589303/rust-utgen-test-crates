// Answer 0

#[test]
fn test_as_u64_float_positive() {
    let number = Number { n: N::Float(3.14) };
    let _ = number.as_u64();
}

#[test]
fn test_as_u64_float_negative() {
    let number = Number { n: N::Float(-2.71) };
    let _ = number.as_u64();
}

#[test]
fn test_as_u64_float_zero() {
    let number = Number { n: N::Float(0.0) };
    let _ = number.as_u64();
}

#[test]
fn test_as_u64_float_nan() {
    let number = Number { n: N::Float(f64::NAN) };
    let _ = number.as_u64();
}

#[test]
fn test_as_u64_float_infinity() {
    let number = Number { n: N::Float(f64::INFINITY) };
    let _ = number.as_u64();
}

#[test]
fn test_as_u64_float_negative_infinity() {
    let number = Number { n: N::Float(f64::NEG_INFINITY) };
    let _ = number.as_u64();
}

