// Answer 0

#[test]
fn test_fmt_float_normal() {
    let value = Unexpected::Float(3.14);
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_float_zero() {
    let value = Unexpected::Float(0.0);
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_float_negative() {
    let value = Unexpected::Float(-2.71);
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_float_positive_infinity() {
    let value = Unexpected::Float(f64::INFINITY);
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_float_negative_infinity() {
    let value = Unexpected::Float(f64::NEG_INFINITY);
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_float_nan() {
    let value = Unexpected::Float(f64::NAN);
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

