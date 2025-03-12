// Answer 0

#[test]
fn test_is_f64_float_value() {
    let number = Number { n: N::Float(1.0) };
    let result = number.is_f64();
}

#[test]
fn test_is_f64_negative_float_value() {
    let number = Number { n: N::Float(-1.5) };
    let result = number.is_f64();
}

#[test]
fn test_is_f64_large_float_value() {
    let number = Number { n: N::Float(1.7e308) };
    let result = number.is_f64();
}

#[test]
fn test_is_f64_small_float_value() {
    let number = Number { n: N::Float(-1.7e308) };
    let result = number.is_f64();
}

#[test]
fn test_is_f64_zero_float_value() {
    let number = Number { n: N::Float(0.0) };
    let result = number.is_f64();
}

