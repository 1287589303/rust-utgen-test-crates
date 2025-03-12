// Answer 0

#[test]
fn test_dot_any_byte_except_0() {
    let byte = 0;
    let hir = Hir::dot(Dot::AnyByteExcept(byte));
}

#[test]
fn test_dot_any_byte_except_255() {
    let byte = 255;
    let hir = Hir::dot(Dot::AnyByteExcept(byte));
}

#[test]
fn test_dot_any_byte_except_128() {
    let byte = 128;
    let hir = Hir::dot(Dot::AnyByteExcept(byte));
}

#[test]
fn test_dot_any_byte_except_64() {
    let byte = 64;
    let hir = Hir::dot(Dot::AnyByteExcept(byte));
}

#[test]
fn test_dot_any_byte_except_100() {
    let byte = 100;
    let hir = Hir::dot(Dot::AnyByteExcept(byte));
}

