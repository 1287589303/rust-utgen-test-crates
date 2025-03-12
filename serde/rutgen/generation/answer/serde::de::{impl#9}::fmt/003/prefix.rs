// Answer 0

#[test]
fn test_fmt_finite_without_decimal_point() {
    let value = WithDecimalPoint(2.0);
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_finite_with_decimal_point() {
    let value = WithDecimalPoint(1.0);
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_fmt_non_finite_nan() {
    let value = WithDecimalPoint(f64::NAN);
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_fmt_non_finite_infinity() {
    let value = WithDecimalPoint(f64::INFINITY);
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_integer_without_decimal_point() {
    let value = WithDecimalPoint(3.0);
    let mut formatter = std::fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

