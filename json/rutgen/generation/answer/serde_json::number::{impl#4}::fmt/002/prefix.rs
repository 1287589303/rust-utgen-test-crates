// Answer 0

#[test]
fn test_fmt_neg_int_neg_one() {
    let number = Number { n: N::NegInt(-1) };
    let mut formatter = core::fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_neg_int_neg_hundred() {
    let number = Number { n: N::NegInt(-100) };
    let mut formatter = core::fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_neg_int_neg_two_billion() {
    let number = Number { n: N::NegInt(-2147483648) };
    let mut formatter = core::fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

