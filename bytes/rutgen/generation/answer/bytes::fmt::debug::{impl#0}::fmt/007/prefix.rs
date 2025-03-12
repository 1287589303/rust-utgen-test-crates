// Answer 0

#[test]
fn test_fmt_with_newline_and_printable() {
    let data = BytesRef(&[b'P', b'y', b't', b'h', b'o', b'n', b' ', b'\n', b'A', b'r', b'e', b' ', b't', b'h', b'e', b' ', b'b'est']);
    let _ = core::fmt::write(&mut String::new().into_bytes(), |f| data.fmt(f));
}

#[test]
fn test_fmt_with_newline_and_non_printable() {
    let data = BytesRef(&[b'A', b'B', b'\n', b'\x01', b'\x1F']);
    let _ = core::fmt::write(&mut String::new().into_bytes(), |f| data.fmt(f));
}

#[test]
fn test_fmt_with_multiple_newlines_and_printable() {
    let data = BytesRef(&[b'H', b'e', b'l', b'l', b'o', b'!', b'\n', b'T', b'h', b'i', b's', b' ', b'a', b' ', b't', b'e', b's', b't']);
    let _ = core::fmt::write(&mut String::new().into_bytes(), |f| data.fmt(f));
}

#[test]
fn test_fmt_with_newline_and_ascii_non_printable() {
    let data = BytesRef(&[b'\n', b'A', b'\x10', b'b', b'\x1e']);
    let _ = core::fmt::write(&mut String::new().into_bytes(), |f| data.fmt(f));
}

