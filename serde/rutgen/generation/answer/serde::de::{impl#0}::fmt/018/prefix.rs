// Answer 0

#[test]
fn test_fmt_bool_true() {
    let input = Unexpected::Bool(true);
    let mut formatter = std::fmt::Formatter::new();
    let _ = input.fmt(&mut formatter);
}

#[test]
fn test_fmt_bool_false() {
    let input = Unexpected::Bool(false);
    let mut formatter = std::fmt::Formatter::new();
    let _ = input.fmt(&mut formatter);
}

