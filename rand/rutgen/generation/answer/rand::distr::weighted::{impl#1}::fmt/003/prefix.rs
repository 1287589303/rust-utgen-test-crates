// Answer 0

#[test]
fn test_fmt_invalid_weight_negative() {
    let error = Error::InvalidWeight;
    let mut formatter = fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_invalid_weight_large() {
    let error = Error::InvalidWeight;
    let mut formatter = fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_invalid_weight_non_number() {
    let error = Error::InvalidWeight;
    let mut formatter = fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

