// Answer 0

#[test]
fn test_union_into_empty_with_non_empty_lits1_and_non_empty_lits2() {
    let mut seq1 = Seq::new(&[b"a", b"b", b"c"]);
    let mut seq2 = Seq::new(&[b"x", b"y", b"z"]);
    seq1.union_into_empty(&mut seq2);
}

#[test]
fn test_union_into_empty_with_non_empty_lits1_and_single_literal_lits2() {
    let mut seq1 = Seq::new(&[b"one", b"two"]);
    let mut seq2 = Seq::new(&[b"three"]);
    seq1.union_into_empty(&mut seq2);
}

#[test]
fn test_union_into_empty_with_non_empty_lits1_and_repeated_literal_lits2() {
    let mut seq1 = Seq::new(&[b"alpha", b"beta"]);
    let mut seq2 = Seq::new(&[b"beta", b"gamma"]);
    seq1.union_into_empty(&mut seq2);
}

