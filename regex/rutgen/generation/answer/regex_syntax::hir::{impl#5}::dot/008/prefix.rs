// Answer 0

#[test]
fn test_dot_any_char() {
    let dot = Dot::AnyChar;
    let hir = Hir::dot(dot);
}

#[test]
fn test_dot_any_byte() {
    let dot = Dot::AnyByte;
    let hir = Hir::dot(dot);
}

#[test]
fn test_dot_any_char_except() {
    let dot = Dot::AnyCharExcept('\u{0}');
    let hir = Hir::dot(dot);
}

#[test]
fn test_dot_any_char_except_lf() {
    let dot = Dot::AnyCharExceptLF;
    let hir = Hir::dot(dot);
}

#[test]
fn test_dot_any_char_except_crlf() {
    let dot = Dot::AnyCharExceptCRLF;
    let hir = Hir::dot(dot);
}

#[test]
fn test_dot_any_byte_except() {
    let dot = Dot::AnyByteExcept(0);
    let hir = Hir::dot(dot);
}

#[test]
fn test_dot_any_byte_except_lf() {
    let dot = Dot::AnyByteExceptLF;
    let hir = Hir::dot(dot);
}

#[test]
fn test_dot_any_byte_except_crlf() {
    let dot = Dot::AnyByteExceptCRLF;
    let hir = Hir::dot(dot);
}

