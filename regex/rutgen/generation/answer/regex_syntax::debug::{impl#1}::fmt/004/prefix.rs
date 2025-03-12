// Answer 0

#[test]
fn test_empty_bytes() {
    let bytes = Bytes(&[]);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| bytes.fmt(f));
}

#[test]
fn test_invalid_utf8_single_byte() {
    let bytes = Bytes(&[0x80]);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| bytes.fmt(f));
}

#[test]
fn test_invalid_utf8_double_bytes() {
    let bytes = Bytes(&[0xC0, 0x80]);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| bytes.fmt(f));
}

#[test]
fn test_control_character() {
    let bytes = Bytes(&[0x01]);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| bytes.fmt(f));
}

#[test]
fn test_valid_utf8_character() {
    let bytes = Bytes(&[0xC2, 0xA9]);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| bytes.fmt(f));
}

#[test]
fn test_multiple_control_characters() {
    let bytes = Bytes(&[0x01, 0x02, 0x03]);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| bytes.fmt(f));
}

