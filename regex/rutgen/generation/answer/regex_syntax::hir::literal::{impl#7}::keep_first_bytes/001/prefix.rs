// Answer 0

#[test]
fn test_keep_first_bytes_equal_length() {
    let mut literal = Literal::exact(vec![1, 2, 3]);
    let len = literal.len();
    literal.keep_first_bytes(len);
}

#[test]
fn test_keep_first_bytes_zero_length() {
    let mut literal = Literal::exact(vec![1, 2, 3]);
    let len = 0;
    literal.keep_first_bytes(len);
}

#[test]
fn test_keep_first_bytes_exceeding_length() {
    let mut literal = Literal::exact(vec![1, 2, 3]);
    let len = literal.len() + 1;
    literal.keep_first_bytes(len);
}

