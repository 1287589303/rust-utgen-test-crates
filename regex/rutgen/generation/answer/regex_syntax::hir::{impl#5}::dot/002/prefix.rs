// Answer 0

#[test]
fn test_dot_any_char() {
    let result = Hir::dot(Dot::AnyChar);
}

#[test]
fn test_dot_any_byte() {
    let result = Hir::dot(Dot::AnyByte);
}

#[test]
fn test_dot_any_char_except() {
    let result = Hir::dot(Dot::AnyCharExcept('\0'));
    let result2 = Hir::dot(Dot::AnyCharExcept('\u{10FFFF}'));
}

#[test]
fn test_dot_any_char_except_lf() {
    let result = Hir::dot(Dot::AnyCharExceptLF);
}

#[test]
fn test_dot_any_char_except_crlf() {
    let result = Hir::dot(Dot::AnyCharExceptCRLF);
}

#[test]
fn test_dot_any_byte_except() {
    let result = Hir::dot(Dot::AnyByteExcept(0));
    let result2 = Hir::dot(Dot::AnyByteExcept(255));
}

#[test]
fn test_dot_any_byte_except_lf() {
    let result = Hir::dot(Dot::AnyByteExceptLF);
}

#[test]
fn test_dot_any_byte_except_crlf() {
    let result = Hir::dot(Dot::AnyByteExceptCRLF);
}

