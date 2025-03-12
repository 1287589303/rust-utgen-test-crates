// Answer 0

#[test]
fn test_cross_forward_exact_and_inexact_literals() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"foo".to_vec()),
        Literal::inexact(b"bar".to_vec()),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::inexact(b"quux".to_vec()),
        Literal::exact(b"baz".to_vec()),
    ]);
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_infinite_other() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"foo".to_vec()),
        Literal::inexact(b"bar".to_vec()),
    ]);
    let mut seq2 = Seq::infinite();
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_empty_string_literal() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"foo".to_vec()),
        Literal::exact(b"".to_vec()),
        Literal::inexact(b"bar".to_vec()),
    ]);
    let mut seq2 = Seq::infinite();
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_no_exact_literals_in_other() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"foo".to_vec()),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::inexact(b"bar".to_vec()),
        Literal::inexact(b"baz".to_vec()),
    ]);
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_zero_length_literal() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"foo".to_vec()),
        Literal::exact(b"".to_vec()), // zero-length literal
        Literal::inexact(b"bar".to_vec()),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::inexact(b"quux".to_vec()),
        Literal::exact(b"baz".to_vec()),
    ]);
    seq1.cross_forward(&mut seq2);
}

