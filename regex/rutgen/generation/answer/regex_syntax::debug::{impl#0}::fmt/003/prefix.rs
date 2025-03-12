// Answer 0

#[test]
fn test_fmt_for_space_byte() {
    let byte = Byte(b' ');
    let mut formatter = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut formatter);
}

#[test]
fn test_fmt_for_a_byte() {
    let byte = Byte(b'A');
    let mut formatter = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut formatter);
}

#[test]
fn test_fmt_for_f_byte() {
    let byte = Byte(b'F');
    let mut formatter = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut formatter);
}

#[test]
fn test_fmt_for_0_byte() {
    let byte = Byte(b'0');
    let mut formatter = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut formatter);
}

#[test]
fn test_fmt_for_9_byte() {
    let byte = Byte(b'9');
    let mut formatter = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut formatter);
}

#[test]
fn test_fmt_for_ascii_space_byte() {
    let byte = Byte(b'\x20');
    let mut formatter = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut formatter);
}

