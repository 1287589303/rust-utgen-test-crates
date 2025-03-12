// Answer 0

#[test]
fn test_debug_haystack_with_null_byte() {
    let data: &[u8] = &[0x00];
    let haystack = DebugHaystack(data);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

#[test]
fn test_debug_haystack_with_control_characters() {
    let data: &[u8] = &[0x01, 0x02, 0x03, 0x04];
    let haystack = DebugHaystack(data);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

#[test]
fn test_debug_haystack_with_delimiter() {
    let data: &[u8] = &[0x7f];
    let haystack = DebugHaystack(data);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

#[test]
fn test_debug_haystack_with_invalid_utf8() {
    let data: &[u8] = &[0x80, 0xff];
    let haystack = DebugHaystack(data);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

#[test]
fn test_debug_haystack_with_valid_utf8_sequence() {
    let data: &[u8] = &[0x61, 0x62, 0x63]; // "abc"
    let haystack = DebugHaystack(data);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

