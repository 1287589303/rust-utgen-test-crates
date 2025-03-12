// Answer 0

#[test]
fn test_keep_last_bytes_normal_case() {
    let mut lit = Literal::exact(b"abcdef");
    lit.keep_last_bytes(4);
}

#[test]
fn test_keep_last_bytes_boundary_case() {
    let mut lit = Literal::exact(b"abcdef");
    lit.keep_last_bytes(5);
}

#[test]
fn test_keep_last_bytes_non_empty() {
    let mut lit = Literal::exact(b"hello");
    lit.keep_last_bytes(3);
}

#[test]
fn test_keep_last_bytes_multiple() {
    let mut lit = Literal::exact(b"rustlang");
    lit.keep_last_bytes(6);
}

#[test]
fn test_keep_last_bytes_short_length() {
    let mut lit = Literal::exact(b"abcdefgh");
    lit.keep_last_bytes(2);
}

