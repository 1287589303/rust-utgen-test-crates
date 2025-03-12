// Answer 0

#[test]
fn test_is_empty_with_none() {
    let seq = Seq { literals: None };
    seq.is_empty();
}

#[test]
fn test_is_empty_with_empty_vec() {
    let seq = Seq { literals: Some(vec![]) };
    seq.is_empty();
}

#[test]
fn test_is_empty_with_non_empty_vec() {
    let lit = Literal(vec![b'a']);
    let seq = Seq { literals: Some(vec![lit]) };
    seq.is_empty();
}

#[test]
fn test_is_empty_with_singleton() {
    let lit = Literal(vec![b'x']);
    let seq = Seq::singleton(lit);
    seq.is_empty();
}

#[test]
fn test_is_empty_with_new() {
    let seq_empty = Seq::new(vec![]); 
    seq_empty.is_empty();

    let lit = Literal(vec![b'a']);
    let seq_single = Seq::new(vec![lit]);
    seq_single.is_empty();
}

