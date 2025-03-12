// Answer 0

#[test]
fn test_cross_forward_empty_other() {
    let mut seq1 = Seq::new(vec![Literal::exact(b"foo".to_vec())]);
    let mut seq2 = Seq::empty();
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_zero_length_literal() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"".to_vec()), 
        Literal::exact(b"foo".to_vec())
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact(b"bar".to_vec())
    ]);
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_non_empty_other() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"foo".to_vec()),
        Literal::exact(b"".to_vec())
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact(b"baz".to_vec()),
        Literal::exact(b"qux".to_vec())
    ]);
    seq1.cross_forward(&mut seq2);
}

