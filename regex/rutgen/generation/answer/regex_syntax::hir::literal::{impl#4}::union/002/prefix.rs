// Answer 0

#[test]
fn test_union_with_some_literals_into_infinite() {
    let mut seq1 = Seq::infinite();
    let lit1 = Literal(Box::from(b"foo"));
    let lit2 = Literal(Box::from(b"bar"));
    let mut seq2 = Seq::new(vec![lit1.clone(), lit2.clone()]);
    seq1.union(&mut seq2);
}

#[test]
fn test_union_with_multiple_literals_into_infinite() {
    let mut seq1 = Seq::infinite();
    let lit1 = Literal(Box::from(b"baz"));
    let lit2 = Literal(Box::from(b"qux"));
    let lit3 = Literal(Box::from(b"quux"));
    let mut seq2 = Seq::new(vec![lit1.clone(), lit2.clone(), lit3.clone()]);
    seq1.union(&mut seq2);
}

#[test]
fn test_union_with_empty_literals_into_infinite() {
    let mut seq1 = Seq::infinite();
    let mut seq2 = Seq::new(vec![]);
    seq1.union(&mut seq2);
}

