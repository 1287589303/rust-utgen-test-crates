// Answer 0

#[test]
fn test_debug_byte_space() {
    let debug_byte = DebugByte(b' ');
    let mut output = core::fmt::Formatter::new();
    let _ = debug_byte.fmt(&mut output);
}

#[test]
fn test_debug_byte_a() {
    let debug_byte = DebugByte(b'a');
    let mut output = core::fmt::Formatter::new();
    let _ = debug_byte.fmt(&mut output);
}

#[test]
fn test_debug_byte_f() {
    let debug_byte = DebugByte(b'f');
    let mut output = core::fmt::Formatter::new();
    let _ = debug_byte.fmt(&mut output);
}

#[test]
fn test_debug_byte_below_a() {
    let debug_byte = DebugByte(b'\x0f');
    let mut output = core::fmt::Formatter::new();
    let _ = debug_byte.fmt(&mut output);
}

#[test]
fn test_debug_byte_above_f() {
    let debug_byte = DebugByte(b'g');
    let mut output = core::fmt::Formatter::new();
    let _ = debug_byte.fmt(&mut output);
}

#[test]
fn test_debug_byte_invalid_utf8() {
    let debug_byte = DebugByte(0xFF);
    let mut output = core::fmt::Formatter::new();
    let _ = debug_byte.fmt(&mut output);
}

