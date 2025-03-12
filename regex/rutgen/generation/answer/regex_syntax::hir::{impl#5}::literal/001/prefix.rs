// Answer 0

#[test]
fn test_literal_empty_slice() {
    let result = Hir::literal(&[]);
}

#[test]
fn test_literal_empty_vec() {
    let result = Hir::literal(Vec::<u8>::new());
}

#[test]
fn test_literal_empty_string() {
    let result = Hir::literal("");
}

