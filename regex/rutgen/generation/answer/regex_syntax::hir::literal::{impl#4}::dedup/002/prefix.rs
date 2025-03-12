// Answer 0

#[test]
fn test_dedup_with_exact_and_inexact_literals() {
    let mut seq = Seq {
        literals: Some(vec![
            Literal::exact(b"foo"),
            Literal::inexact(b"foo"),
        ]),
    };
    seq.dedup();
}

#[test]
fn test_dedup_with_multiple_literals() {
    let mut seq = Seq {
        literals: Some(vec![
            Literal::exact(b"foo"),
            Literal::exact(b"foo"),
            Literal::inexact(b"bar"),
            Literal::exact(b"foo"),
            Literal::inexact(b"bar"),
        ]),
    };
    seq.dedup();
}

#[test]
fn test_dedup_with_no_duplicates() {
    let mut seq = Seq {
        literals: Some(vec![
            Literal::exact(b"foo"),
            Literal::inexact(b"bar"),
        ]),
    };
    seq.dedup();
}

#[test]
fn test_dedup_with_empty_literals() {
    let mut seq = Seq {
        literals: Some(vec![]),
    };
    seq.dedup();
}

#[test]
fn test_dedup_with_single_literal() {
    let mut seq = Seq {
        literals: Some(vec![
            Literal::exact(b"foo"),
        ]),
    };
    seq.dedup();
}

#[test]
fn test_dedup_with_infinite_seq() {
    let mut seq = Seq::infinite();
    seq.dedup();
}

