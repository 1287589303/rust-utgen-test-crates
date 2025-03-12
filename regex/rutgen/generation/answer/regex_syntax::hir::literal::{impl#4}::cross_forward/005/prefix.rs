// Answer 0

#[test]
fn test_cross_forward_with_exact_literals() {
    let mut seq1 = Seq::new(vec![
        Literal::exact("foo"),
        Literal::exact("bar"),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact("baz"),
        Literal::exact("quux"),
    ]);
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_mixture_of_exact_and_inexact() {
    let mut seq1 = Seq::new(vec![
        Literal::exact("foo"),
        Literal::inexact("bar"),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::inexact("quux"),
        Literal::exact("baz"),
    ]);
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_multiple_exact_literals() {
    let mut seq1 = Seq::new(vec![
        Literal::exact("apple"),
        Literal::exact("banana"),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact("carrot"),
        Literal::exact("date"),
    ]);
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_exact_and_empty_inexact() {
    let mut seq1 = Seq::new(vec![
        Literal::exact("hello"),
        Literal::inexact(""),
    ]);
    let mut seq2 = Seq::new(vec![
        Literal::exact("world"),
    ]);
    seq1.cross_forward(&mut seq2);
}

