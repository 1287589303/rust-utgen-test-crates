// Answer 0

#[test]
fn test_union_with_infinite_seq() {
    let mut seq1 = Seq::new(&["a", "b", "c"]);
    let mut seq2 = Seq::infinite();
    seq1.union(&mut seq2);
}

#[test]
fn test_union_with_empty_seq() {
    let mut seq1 = Seq::new(&["x", "y"]);
    let mut seq2 = Seq::empty();
    seq1.union(&mut seq2);
}

#[test]
fn test_union_with_another_non_empty_seq() {
    let mut seq1 = Seq::new(&["foo", "bar"]);
    let mut seq2 = Seq::new(&["baz", "quux"]);
    seq1.union(&mut seq2);
}

#[test]
fn test_union_with_nil_other() {
    let mut seq1 = Seq::new(&["hello", "world"]);
    let mut seq2 = Seq::infinite();
    seq2.make_infinite(); // ensuring seq2 is infinite, matching the precondition
    seq1.union(&mut seq2);
}

