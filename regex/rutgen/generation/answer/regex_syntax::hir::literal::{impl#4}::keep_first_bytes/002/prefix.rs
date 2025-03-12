// Answer 0

#[test]
fn test_keep_first_bytes_zero_length() {
    let mut seq = Seq::new(vec![Literal::exact(b"a"), Literal::inexact(b"foo"), Literal::exact(b"quux")]);
    seq.keep_first_bytes(0);
}

#[test]
fn test_keep_first_bytes_smaller_than_literals() {
    let mut seq = Seq::new(vec![Literal::exact(b"a"), Literal::inexact(b"foo"), Literal::exact(b"quux")]);
    seq.keep_first_bytes(1);
}

#[test]
fn test_keep_first_bytes_equal_to_literal_lengths() {
    let mut seq = Seq::new(vec![Literal::exact(b"a"), Literal::inexact(b"foo"), Literal::exact(b"quux")]);
    seq.keep_first_bytes(3);
}

#[test]
fn test_keep_first_bytes_greater_than_longest_literal() {
    let mut seq = Seq::new(vec![Literal::exact(b"a"), Literal::inexact(b"foo"), Literal::exact(b"quux")]);
    seq.keep_first_bytes(5);
}

#[test]
fn test_keep_first_bytes_at_max_literal_len() {
    let mut seq = Seq::new(vec![Literal::exact(b"a"), Literal::inexact(b"foo"), Literal::exact(b"quux")]);
    let max_len = seq.max_literal_len().unwrap();
    seq.keep_first_bytes(max_len);
}

