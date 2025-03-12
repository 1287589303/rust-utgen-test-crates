// Answer 0

#[test]
fn test_fmt_pos_int_zero() {
    let number = Number { n: N::PosInt(0) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_pos_int_small() {
    let number = Number { n: N::PosInt(1) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_pos_int_large() {
    let number = Number { n: N::PosInt(18446744073709551615) }; // MAX_U64
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_neg_int() {
    let number = Number { n: N::NegInt(-1) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_float() {
    let number = Number { n: N::Float(3.14) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_float_negative() {
    let number = Number { n: N::Float(-2.71) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

