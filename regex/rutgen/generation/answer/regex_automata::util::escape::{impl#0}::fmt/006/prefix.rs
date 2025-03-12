// Answer 0

#[test]
fn test_debug_byte_space() {
    let byte = DebugByte(b' ');
    let mut formatter = core::fmt::Formatter::new();
    byte.fmt(&mut formatter).unwrap();
}

#[test]
fn test_debug_byte_non_printable() {
    let byte = DebugByte(0x00);
    let mut formatter = core::fmt::Formatter::new();
    byte.fmt(&mut formatter).unwrap();
}

#[test]
fn test_debug_byte_printable() {
    let byte = DebugByte(b'A');
    let mut formatter = core::fmt::Formatter::new();
    byte.fmt(&mut formatter).unwrap();
}

#[test]
fn test_debug_byte_max_value() {
    let byte = DebugByte(0xFF);
    let mut formatter = core::fmt::Formatter::new();
    byte.fmt(&mut formatter).unwrap();
}

