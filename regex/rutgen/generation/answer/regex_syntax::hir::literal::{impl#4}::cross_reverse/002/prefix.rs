// Answer 0

#[test]
fn test_cross_reverse_with_exact_self_and_inexact_other() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"foo".to_vec()),
        Literal::exact(b"bar".to_vec()),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::inexact(b"quux".to_vec()),
        Literal::exact(b"baz".to_vec()),
    ]);
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_multiple_exact_self_and_inexact_other() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"apple".to_vec()),
        Literal::exact(b"orange".to_vec()),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::inexact(b"banana".to_vec()),
        Literal::exact(b"grape".to_vec()),
    ]);
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_empty_literal_in_self() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"hello".to_vec()),
        Literal::exact(b"".to_vec()), // empty literal
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::inexact(b"world".to_vec()),
        Literal::exact(b"test".to_vec()),
    ]);
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_first_literal_inexact() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"foo".to_vec()),
        Literal::exact(b"bar".to_vec()),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::inexact(b"quux".to_vec()),
        Literal::inexact(b"baz".to_vec()),
    ]);
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_exact_self_and_exact_other() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"cat".to_vec()),
        Literal::exact(b"dog".to_vec()),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact(b"fish".to_vec()),
        Literal::inexact(b"bird".to_vec()),
    ]);
    seq1.cross_reverse(&mut seq2);
}

