// Answer 0

#[test]
fn test_keep_first_bytes_with_length_zero() {
    let mut lit = Literal::exact(vec![1, 2, 3]);
    lit.keep_first_bytes(0);
}

#[test]
fn test_keep_first_bytes_with_length_one() {
    let mut lit = Literal::exact(vec![1, 2, 3]);
    lit.keep_first_bytes(1);
}

#[test]
fn test_keep_first_bytes_with_length_two() {
    let mut lit = Literal::exact(vec![1, 2, 3]);
    lit.keep_first_bytes(2);
}

#[test]
fn test_keep_first_bytes_with_length_three() {
    let mut lit = Literal::exact(vec![1, 2, 3]);
    lit.keep_first_bytes(3);
}

#[test]
fn test_keep_first_bytes_with_length_four() {
    let mut lit = Literal::exact(vec![1, 2, 3, 4]);
    lit.keep_first_bytes(3);
}

#[test]
fn test_keep_first_bytes_with_length_equal_to_current_length() {
    let mut lit = Literal::exact(vec![1, 2, 3]);
    lit.keep_first_bytes(3);
}

#[test]
fn test_keep_first_bytes_with_length_greater_than_current_length() {
    let mut lit = Literal::exact(vec![1, 2]);
    lit.keep_first_bytes(3);
}

#[test]
fn test_keep_first_bytes_with_empty_literal() {
    let mut lit = Literal::exact(vec![]);
    lit.keep_first_bytes(1);
}

