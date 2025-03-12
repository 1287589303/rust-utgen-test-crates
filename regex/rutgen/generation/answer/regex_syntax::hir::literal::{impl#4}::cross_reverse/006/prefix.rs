// Answer 0

#[test]
fn test_cross_reverse_with_non_empty_exact_literals_in_self_and_other() {
    let mut seq1 = Seq::new(vec![Literal::exact("foo"), Literal::exact("baz")]);
    let mut seq2 = Seq::new(vec![Literal::exact("bar")]);
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_empty_seq1_and_exact_literals_in_other() {
    let mut seq1 = Seq::empty();
    let mut seq2 = Seq::new(vec![Literal::exact("bar"), Literal::exact("baz")]);
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_infinite_seq1_and_exact_literals_in_other() {
    let mut seq1 = Seq::infinite();
    let mut seq2 = Seq::new(vec![Literal::exact("bar"), Literal::exact("baz")]);
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_seq1_having_exact_and_inexact_literls_and_non_empty_exact_literals_in_other() {
    let mut seq1 = Seq::new(vec![Literal::exact("foo"), Literal::inexact("quux")]);
    let mut seq2 = Seq::new(vec![Literal::exact("bar")]);
    seq1.cross_reverse(&mut seq2);
}

#[test]
fn test_cross_reverse_with_empty_literals_in_seq1_and_infinite_seq2() {
    let mut seq1 = Seq::new(vec![Literal::exact("foo"), Literal::exact("")]);
    let mut seq2 = Seq::infinite();
    seq1.cross_reverse(&mut seq2);
}

