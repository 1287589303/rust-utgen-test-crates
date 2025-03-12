// Answer 0

#[test]
fn test_as_f64_float_positive() {
    let number = Number { n: N::Float(1.0) };
    let _result = number.as_f64();
}

#[test]
fn test_as_f64_float_negative() {
    let number = Number { n: N::Float(-1.0) };
    let _result = number.as_f64();
}

#[test]
fn test_as_f64_float_max() {
    let number = Number { n: N::Float(1.7976931348623157E+308) };
    let _result = number.as_f64();
}

#[test]
fn test_as_f64_float_min() {
    let number = Number { n: N::Float(-1.7976931348623157E+308) };
    let _result = number.as_f64();
}

#[test]
fn test_as_f64_float_zero() {
    let number = Number { n: N::Float(0.0) };
    let _result = number.as_f64();
}

