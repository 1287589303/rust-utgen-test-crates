// Answer 0

#[test]
fn test_dedup_single_exact_and_inexact_lit() {
    let mut seq = Seq::new(vec![
        Literal::exact(b"foo"),
        Literal::inexact(b"foo"),
    ]);
    seq.dedup();
}

#[test]
fn test_dedup_multiple_lits_with_exact_and_inexact() {
    let mut seq = Seq::new(vec![
        Literal::exact(b"foo"),
        Literal::inexact(b"foo"),
        Literal::exact(b"bar"),
        Literal::inexact(b"bar"),
    ]);
    seq.dedup();
}

#[test]
fn test_dedup_with_mixed_exacts_and_inexact() {
    let mut seq = Seq::new(vec![
        Literal::exact(b"foo"),
        Literal::exact(b"foo"),
        Literal::inexact(b"bar"),
        Literal::exact(b"bar"),
    ]);
    seq.dedup();
}

#[test]
fn test_dedup_all_exact_lits() {
    let mut seq = Seq::new(vec![
        Literal::exact(b"foo"),
        Literal::exact(b"foo"),
        Literal::exact(b"bar"),
    ]);
    seq.dedup();
}

#[test]
fn test_dedup_all_inexact_lits() {
    let mut seq = Seq::new(vec![
        Literal::inexact(b"foo"),
        Literal::inexact(b"foo"),
        Literal::inexact(b"bar"),
    ]);
    seq.dedup();
}

#[test]
fn test_dedup_with_no_duplicates() {
    let mut seq = Seq::new(vec![
        Literal::exact(b"foo"),
        Literal::inexact(b"baz"),
    ]);
    seq.dedup();
}

#[test]
fn test_dedup_multiple_duplicates() {
    let mut seq = Seq::new(vec![
        Literal::exact(b"foo"),
        Literal::inexact(b"foo"),
        Literal::inexact(b"foo"),
        Literal::exact(b"bar"),
        Literal::inexact(b"bar"),
    ]);
    seq.dedup();
}

