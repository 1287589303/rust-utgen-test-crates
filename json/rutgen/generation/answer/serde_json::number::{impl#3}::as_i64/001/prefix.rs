// Answer 0

#[test]
fn test_as_i64_float_zero() {
    let number = Number { n: N::Float(0.0) };
    number.as_i64();
}

#[test]
fn test_as_i64_float_pos() {
    let number = Number { n: N::Float(3.14) };
    number.as_i64();
}

#[test]
fn test_as_i64_float_neg() {
    let number = Number { n: N::Float(-2.71) };
    number.as_i64();
}

#[test]
fn test_as_i64_float_small() {
    let number = Number { n: N::Float(1e-10) };
    number.as_i64();
}

#[test]
fn test_as_i64_float_large() {
    let number = Number { n: N::Float(1e10) };
    number.as_i64();
}

