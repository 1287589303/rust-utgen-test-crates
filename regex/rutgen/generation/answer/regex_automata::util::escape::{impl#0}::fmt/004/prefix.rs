// Answer 0

#[test]
fn test_debug_byte_space() {
    let debug_byte = DebugByte(b' ');
    let mut output = core::fmt::Formatter::new();
    let _ = debug_byte.fmt(&mut output);
}

#[test]
fn test_debug_byte_control_characters() {
    let control_chars = [b'\x00', b'\x01', b'\x02', b'\x0A', b'\x1F'];
    for &byte in &control_chars {
        let debug_byte = DebugByte(byte);
        let mut output = core::fmt::Formatter::new();
        let _ = debug_byte.fmt(&mut output);
    }
}

#[test]
fn test_debug_byte_regular_ascii() {
    let regular_ascii = [b'\x21', b'\x22', b'\x23', b'\x24', b'\x7F'];
    for &byte in &regular_ascii {
        let debug_byte = DebugByte(byte);
        let mut output = core::fmt::Formatter::new();
        let _ = debug_byte.fmt(&mut output);
    }
}

