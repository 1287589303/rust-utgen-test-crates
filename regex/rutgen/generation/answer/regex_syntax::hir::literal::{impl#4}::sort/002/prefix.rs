// Answer 0

#[test]
fn test_sort_distinct_literals() {
    let mut seq = Seq::new(&["foo", "quux", "bar"]);
    seq.sort();
}

#[test]
fn test_sort_identical_literals() {
    let mut seq = Seq::new(&["foo", "foo", "foo"]);
    seq.sort();
}

#[test]
fn test_sort_varying_lengths() {
    let mut seq = Seq::new(&["a", "ab", "abc"]);
    seq.sort();
}

#[test]
fn test_sort_already_sorted() {
    let mut seq = Seq::new(&["a", "b", "c"]);
    seq.sort();
}

#[test]
fn test_sort_prefix_relationships() {
    let mut seq = Seq::new(&["samwise", "sam"]);
    seq.sort();
}

#[test]
fn test_sort_single_literal() {
    let mut seq = Seq::new(&["onlyone"]);
    seq.sort();
}

