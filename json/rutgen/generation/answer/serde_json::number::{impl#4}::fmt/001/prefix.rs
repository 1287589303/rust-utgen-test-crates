// Answer 0

#[test]
fn test_fmt_float_positive() {
    let number = Number { n: N::Float(1.0) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_float_negative() {
    let number = Number { n: N::Float(-1.0) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_float_small() {
    let number = Number { n: N::Float(0.000001) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_float_large() {
    let number = Number { n: N::Float(1.7976931348623157E+308) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_float_small_negative() {
    let number = Number { n: N::Float(-1.7976931348623157E+308) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

