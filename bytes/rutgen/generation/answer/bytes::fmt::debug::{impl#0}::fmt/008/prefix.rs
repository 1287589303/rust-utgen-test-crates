// Answer 0

#[test]
fn test_fmt_with_newline_and_return() {
    let bytes = BytesRef(&[b'\n', b'\r', b'\0', 0x21, 0x7e, 0x80]);
    let mut buffer = Vec::new();
    let mut formatter = Formatter::new(&mut buffer);
    let _ = bytes.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_non_printable() {
    let bytes = BytesRef(&[b'\n', b'\r', 0x10, 0x1f, 0x21, 0x7e]);
    let mut buffer = Vec::new();
    let mut formatter = Formatter::new(&mut buffer);
    let _ = bytes.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_printable_and_non_printable() {
    let bytes = BytesRef(&[b'\n', b'\r', 0x1f, 0x23, 0x7e]);
    let mut buffer = Vec::new();
    let mut formatter = Formatter::new(&mut buffer);
    let _ = bytes.fmt(&mut formatter);
}

