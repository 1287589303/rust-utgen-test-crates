// Answer 0

#[test]
fn test_dot_any_char() {
    let hir = Hir::dot(Dot::AnyChar);
}

#[test]
fn test_dot_any_byte() {
    let hir = Hir::dot(Dot::AnyByte);
}

#[test]
fn test_dot_any_char_except() {
    let hir = Hir::dot(Dot::AnyCharExcept('a'));
}

#[test]
fn test_dot_any_char_except_lf() {
    let hir = Hir::dot(Dot::AnyCharExceptLF);
}

#[test]
fn test_dot_any_char_except_crlf() {
    let hir = Hir::dot(Dot::AnyCharExceptCRLF);
}

#[test]
fn test_dot_any_byte_except() {
    let hir = Hir::dot(Dot::AnyByteExcept(100));
}

#[test]
fn test_dot_any_byte_except_lf() {
    let hir = Hir::dot(Dot::AnyByteExceptLF);
}

#[test]
fn test_dot_any_byte_except_crlf() {
    let hir = Hir::dot(Dot::AnyByteExceptCRLF);
}

