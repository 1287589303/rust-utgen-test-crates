// Answer 0

#[test]
fn test_keep_last_bytes_equal_length() {
    let mut lit = Literal::exact(vec![1, 2, 3]);
    let len = lit.len();
    lit.keep_last_bytes(len);
}

#[test]
fn test_keep_last_bytes_zero_length() {
    let mut lit = Literal::exact(vec![5, 6, 7]);
    let len = lit.len();
    lit.keep_last_bytes(0);
}

#[test]
fn test_keep_last_bytes_empty() {
    let mut lit = Literal::exact(vec![]);
    let len = lit.len();
    lit.keep_last_bytes(len);
}

