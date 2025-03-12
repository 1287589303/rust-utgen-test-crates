// Answer 0

#[test]
fn test_cross_reverse_with_exact_literalls() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"foo".to_vec()),
        Literal::inexact(b"bar".to_vec())
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact(b"baz".to_vec()),
        Literal::exact(b"quux".to_vec())
    ]);
    
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_empty_literal() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"foo".to_vec()),
        Literal::exact(b"".to_vec()),
        Literal::inexact(b"bar".to_vec())
    ]);
    let mut seq2 = Seq::infinite();
    
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_inexact_selflit() {
    let mut seq1 = Seq::new(vec![
        Literal::inexact(b"baz".to_vec()),
        Literal::exact(b"foo".to_vec())
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact(b"quux".to_vec()),
        Literal::exact(b"baz".to_vec())
    ]);
    
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_infinite_other() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(b"foo".to_vec()),
        Literal::inexact(b"bar".to_vec()),
    ]);
    let mut seq2 = Seq::infinite();
    
    seq1.cross_reverse(&mut seq2);
}

