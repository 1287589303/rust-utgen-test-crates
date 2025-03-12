// Answer 0

#[test]
fn test_fmt_space_byte() {
    let byte = super::Byte(b' ');
    let mut output = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut output);
}

#[test]
fn test_fmt_byte_a() {
    let byte = super::Byte(b'a');
    let mut output = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut output);
}

#[test]
fn test_fmt_byte_f() {
    let byte = super::Byte(b'f');
    let mut output = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut output);
}

#[test]
fn test_fmt_byte_other() {
    let byte = super::Byte(b'z');
    let mut output = core::fmt::Formatter::new();
    let _ = byte.fmt(&mut output);
}

