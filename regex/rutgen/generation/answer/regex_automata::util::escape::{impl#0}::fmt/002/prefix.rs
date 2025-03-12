// Answer 0

#[test]
fn test_fmt_for_space() {
    let debug_byte = DebugByte(b' ');
    let mut formatter = core::fmt::Formatter::new();
    let _ = debug_byte.fmt(&mut formatter);
}

#[test]
fn test_fmt_for_exclamation() {
    let debug_byte = DebugByte(b'!');
    let mut formatter = core::fmt::Formatter::new();
    let _ = debug_byte.fmt(&mut formatter);
}

#[test]
fn test_fmt_for_tilde() {
    let debug_byte = DebugByte(b'~');
    let mut formatter = core::fmt::Formatter::new();
    let _ = debug_byte.fmt(&mut formatter);
}

#[test]
fn test_fmt_for_lowercase_a() {
    let debug_byte = DebugByte(b'a');
    let mut formatter = core::fmt::Formatter::new();
    let _ = debug_byte.fmt(&mut formatter);
}

#[test]
fn test_fmt_for_lowercase_f() {
    let debug_byte = DebugByte(b'f');
    let mut formatter = core::fmt::Formatter::new();
    let _ = debug_byte.fmt(&mut formatter);
}

