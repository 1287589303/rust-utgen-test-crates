// Answer 0

#[test]
fn test_fmt_ascii_space() {
    let byte_instance = Byte(b' ');
    let mut formatter = core::fmt::Formatter::default();
    let _ = byte_instance.fmt(&mut formatter);
}

#[test]
fn test_fmt_non_ascii() {
    let byte_instance = Byte(0xFF);
    let mut formatter = core::fmt::Formatter::default();
    let _ = byte_instance.fmt(&mut formatter);
}

