// Answer 0

#[test]
fn test_cross_reverse_with_exact_and_inexact_literals() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"foo".to_vec()),
        Literal::inexact(b"bar".to_vec()),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::inexact(b"quux".to_vec()),
        Literal::exact(b"baz".to_vec()),
    ]);

    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_only_inexact_literals_in_self() {
    let mut seq1 = Seq::new(vec![
        Literal::inexact(b"bar".to_vec()),
        Literal::exact(b"foo".to_vec()),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::inexact(b"quux".to_vec()),
        Literal::exact(b"baz".to_vec()),
    ]);

    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_empty_exact_literal_in_self() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"".to_vec()), 
        Literal::exact(b"foo".to_vec()),
        Literal::inexact(b"bar".to_vec()),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact(b"baz".to_vec()),
        Literal::inexact(b"quux".to_vec()),
    ]);

    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_single_inexact_and_exact() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"exact".to_vec()),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::inexact(b"inexact".to_vec()),
    ]);

    seq1.cross_reverse(&mut seq2);
}

