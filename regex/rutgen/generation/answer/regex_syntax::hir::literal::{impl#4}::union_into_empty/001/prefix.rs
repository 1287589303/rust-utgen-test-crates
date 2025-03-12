// Answer 0

#[test]
fn test_union_into_empty_with_single_empty_and_non_empty_literal() {
    let mut seq1 = Seq::new(&["a", "", "b"]);
    let mut seq2 = Seq::new(&["foo"]);
    seq1.union_into_empty(&mut seq2);
}

#[test]
fn test_union_into_empty_with_multiple_empty_literals() {
    let mut seq1 = Seq::new(&["", "", "c"]);
    let mut seq2 = Seq::new(&["bar"]);
    seq1.union_into_empty(&mut seq2);
}

#[test]
fn test_union_into_empty_with_empty_and_non_empty_literlas() {
    let mut seq1 = Seq::new(&["", "d", "e"]);
    let mut seq2 = Seq::new(&["baz", "qux"]);
    seq1.union_into_empty(&mut seq2);
}

#[test]
fn test_union_into_empty_with_no_empty_literals_in_other() {
    let mut seq1 = Seq::new(&["x", "", "y"]);
    let mut seq2 = Seq::new(&["alpha", "beta"]);
    seq1.union_into_empty(&mut seq2);
}

#[test]
fn test_union_into_empty_with_multiple_non_empty_literlas() {
    let mut seq1 = Seq::new(&["", "m", "n"]);
    let mut seq2 = Seq::new(&["gamma", "delta", "epsilon"]);
    seq1.union_into_empty(&mut seq2);
}

