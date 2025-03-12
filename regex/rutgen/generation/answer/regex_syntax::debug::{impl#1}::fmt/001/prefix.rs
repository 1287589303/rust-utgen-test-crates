// Answer 0

#[test]
fn test_fmt_empty_bytes() {
    let bytes = Bytes(&[]);
    let _ = core::fmt::write(&mut core::fmt::Formatter, |f| bytes.fmt(f));
}

#[test]
fn test_fmt_non_utf8_single_byte() {
    let bytes = Bytes(&[0x80]);
    let _ = core::fmt::write(&mut core::fmt::Formatter, |f| bytes.fmt(f));
}

#[test]
fn test_fmt_non_utf8_two_bytes() {
    let bytes = Bytes(&[0x80, 0x81]);
    let _ = core::fmt::write(&mut core::fmt::Formatter, |f| bytes.fmt(f));
}

#[test]
fn test_fmt_non_utf8_three_bytes() {
    let bytes = Bytes(&[0xED, 0xA0, 0x80]); // invalid UTF-8
    let _ = core::fmt::write(&mut core::fmt::Formatter, |f| bytes.fmt(f));
}

#[test]
fn test_fmt_non_utf8_four_bytes() {
    let bytes = Bytes(&[0xF8, 0x88, 0x80, 0x80]); // invalid UTF-8
    let _ = core::fmt::write(&mut core::fmt::Formatter, |f| bytes.fmt(f));
}

#[test]
fn test_fmt_control_byte() {
    let bytes = Bytes(&[0x01, 0x00]);
    let _ = core::fmt::write(&mut core::fmt::Formatter, |f| bytes.fmt(f));
}

