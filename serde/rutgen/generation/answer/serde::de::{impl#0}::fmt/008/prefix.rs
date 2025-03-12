// Answer 0

#[test]
fn test_fmt_seq() {
    let unexpected_value = Unexpected::Seq;
    let mut formatter = ::std::fmt::Formatter::new();
    let _ = unexpected_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_seq_boundary() {
    let unexpected_value = Unexpected::Seq;
    let mut formatter = ::std::fmt::Formatter::new();
    let _ = unexpected_value.fmt(&mut formatter);
}

