// Answer 0

#[test]
fn test_union_into_empty_with_empty_other() {
    let mut seq1 = Seq::new(&["a", "", "f"]);
    let mut seq2 = Seq::new(&[]);

    seq1.union_into_empty(&mut seq2);
}

#[test]
fn test_union_into_empty_with_empty_lit_in_self() {
    let mut seq1 = Seq::new(&["", "b", "c"]);
    let mut seq2 = Seq::new(&[]);

    seq1.union_into_empty(&mut seq2);
}

#[test]
fn test_union_into_empty_with_non_empty_other() {
    let mut seq1 = Seq::new(&["x", "", "y"]);
    let mut seq2 = Seq::new(&["z", "w"]);

    seq1.union_into_empty(&mut seq2);
}

