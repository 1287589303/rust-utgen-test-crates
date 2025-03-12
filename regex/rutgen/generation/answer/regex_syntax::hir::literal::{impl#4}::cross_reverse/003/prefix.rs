// Answer 0

#[test]
fn test_cross_reverse_with_exact_and_inexact_literals() {
    let mut seq1 = Seq::new(vec![
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
fn test_cross_reverse_with_empty_literal() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"".to_vec()),
        Literal::exact(b"foo".to_vec()),
        Literal::inexact(b"bar".to_vec()),
    ]);
    
    let mut seq2 = Seq::new(vec![
        Literal::exact(b"baz".to_vec()),
    ]);
    
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_no_exact_in_seq1() {
    let mut seq1 = Seq::new(vec![
        Literal::inexact(b"bar".to_vec()),
    ]);
    
    let mut seq2 = Seq::new(vec![
        Literal::exact(b"baz".to_vec()),
        Literal::exact(b"quux".to_vec()),
    ]);
    
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_sequential_inexact_literals() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"foo".to_vec()),
        Literal::inexact(b"bar".to_vec()),
        Literal::inexact(b"xyz".to_vec()),
    ]);
    
    let mut seq2 = Seq::new(vec![
        Literal::exact(b"baz".to_vec()),
        Literal::exact(b"quux".to_vec()),
    ]);
    
    seq1.cross_reverse(&mut seq2);
}

