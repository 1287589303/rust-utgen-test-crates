// Answer 0

#[test]
fn test_cross_forward_with_exact_self_and_exact_other() {
    let mut seq1 = Seq::new(vec![
        Literal::exact("foo"),
        Literal::exact("bar"),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact("baz"),
    ]);
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_exact_self_and_inexact_other() {
    let mut seq1 = Seq::new(vec![
        Literal::exact("foo"),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::inexact("baz"),
        Literal::exact("quux"),
    ]);
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_inexact_self_and_exact_other() {
    let mut seq1 = Seq::new(vec![
        Literal::inexact("foo"),
        Literal::exact("bar"),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact("baz"),
    ]);
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_exact_self_single_literal() {
    let mut seq1 = Seq::new(vec![
        Literal::exact("foo"),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact("bar"),
    ]);
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_multiple_inexact_self_literals() {
    let mut seq1 = Seq::new(vec![
        Literal::inexact("foo"),
        Literal::inexact("bar"),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact("baz"),
    ]);
    seq1.cross_forward(&mut seq2);
}

