// Answer 0

#[test]
fn test_fmt_unsigned_zero() {
    let unexpected = Unexpected::Unsigned(0);
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected.fmt(&mut formatter);
}

#[test]
fn test_fmt_unsigned_half_max() {
    let unexpected = Unexpected::Unsigned(9223372036854775808); // This is just over half of u64::MAX
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected.fmt(&mut formatter);
}

#[test]
fn test_fmt_unsigned_max() {
    let unexpected = Unexpected::Unsigned(18446744073709551615); // u64::MAX
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected.fmt(&mut formatter);
}

