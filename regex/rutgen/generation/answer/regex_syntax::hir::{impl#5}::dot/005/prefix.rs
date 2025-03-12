// Answer 0

#[test]
fn test_dot_any_char_except_lf() {
    let dot = Dot::AnyCharExceptLF;
    let hir = Hir::dot(dot);
    let expected = Hir::class(Class::Unicode(ClassUnicode::new([
        ClassUnicodeRange::new('\0', '\x09'),
        ClassUnicodeRange::new('\x0B', '\u{10FFFF}'),
    ])));
}

#[test]
fn test_dot_any_char_except_crlf() {
    let dot = Dot::AnyCharExceptCRLF;
    let hir = Hir::dot(dot);
    let expected = Hir::class(Class::Unicode(ClassUnicode::new([
        ClassUnicodeRange::new('\0', '\x09'),
        ClassUnicodeRange::new('\x0B', '\x0C'),
        ClassUnicodeRange::new('\x0E', '\u{10FFFF}'),
    ])));
}

#[test]
fn test_dot_any_byte_except() {
    let byte = b'a';
    let dot = Dot::AnyByteExcept(byte);
    let hir = Hir::dot(dot);
    let expected = Hir::class(Class::Bytes(ClassBytes::new([
        ClassBytesRange::new(b'\0', b'\xFF'),
    ])));
}

#[test]
fn test_dot_any_byte_except_lf() {
    let dot = Dot::AnyByteExceptLF;
    let hir = Hir::dot(dot);
    let expected = Hir::class(Class::Bytes(ClassBytes::new([
        ClassBytesRange::new(b'\0', b'\x09'),
        ClassBytesRange::new(b'\x0B', b'\xFF'),
    ])));
}

#[test]
fn test_dot_any_byte_except_crlf() {
    let dot = Dot::AnyByteExceptCRLF;
    let hir = Hir::dot(dot);
    let expected = Hir::class(Class::Bytes(ClassBytes::new([
        ClassBytesRange::new(b'\0', b'\x09'),
        ClassBytesRange::new(b'\x0B', b'\x0C'),
        ClassBytesRange::new(b'\x0E', b'\xFF'),
    ])));
}

