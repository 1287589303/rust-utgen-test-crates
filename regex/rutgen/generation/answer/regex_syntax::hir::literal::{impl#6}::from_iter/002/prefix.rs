// Answer 0

#[test]
fn test_from_iter_empty_iterator() {
    let empty_iter: Vec<Literal> = vec![];
    let seq = Seq::from_iter(empty_iter);
}

#[test]
fn test_from_iter_empty_vec() {
    let seq: Seq = Seq::from_iter(vec![].into_iter());
}

#[test]
fn test_from_iter_iter_no_literals() {
    let no_literals: std::iter::Empty<Literal> = std::iter::empty();
    let seq = Seq::from_iter(no_literals);
}

