// Answer 0

#[test]
fn test_cross_reverse_with_self_empty_and_other_finite() {
    let mut seq1 = Seq::empty();
    let mut seq2 = Seq::new(vec![
        Literal::exact("foo"),
        Literal::inexact("bar"),
    ]);
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_self_finite_and_other_finite() {
    let mut seq1 = Seq::new(vec![
        Literal::exact("foo"),
        Literal::exact("baz"),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact("quux"),
        Literal::inexact("bar"),
    ]);
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_self_zero_length_and_other_finite() {
    let mut seq1 = Seq::new(vec![
        Literal::exact(""),
        Literal::exact("baz"),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact("quux"),
        Literal::inexact(""),
    ]);
    seq1.cross_reverse(&mut seq2);
} 

#[test]
fn test_cross_reverse_with_self_exact_literals_and_other_finite() {
    let mut seq1 = Seq::new(vec![
        Literal::exact("hello"),
        Literal::exact("world"),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact("foo"),
        Literal::inexact("bar"),
    ]);
    seq1.cross_reverse(&mut seq2);
} 

#[test]
#[should_panic] 
fn test_cross_reverse_with_infinite_self_and_finite_other() {
    let mut seq1 = Seq::infinite();
    let mut seq2 = Seq::new(vec![
        Literal::exact("foo"),
        Literal::inexact("bar"),
    ]);
    seq1.cross_reverse(&mut seq2);
}

