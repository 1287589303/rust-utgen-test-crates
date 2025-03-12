// Answer 0

#[test]
fn test_push_when_literals_is_none() {
    let mut seq = Seq::empty();
    let lit = Literal(Box::new([b'a', b'b', b'c']));

    seq.push(lit);
}

#[test]
fn test_push_when_literals_is_empty() {
    let mut seq = Seq {
        literals: Some(vec![]),
    };
    let lit = Literal(Box::new([b'a', b'b', b'c']));

    seq.push(lit);
}

#[test]
fn test_push_same_literal() {
    let mut seq = Seq {
        literals: Some(vec![Literal(Box::new([b'a', b'b', b'c']))]),
    };
    let lit = Literal(Box::new([b'a', b'b', b'c']));

    seq.push(lit);
}

#[test]
fn test_push_different_literal() {
    let mut seq = Seq {
        literals: Some(vec![Literal(Box::new([b'a', b'b', b'c']))]),
    };
    let lit = Literal(Box::new([b'd', b'e', b'f']));

    seq.push(lit);
}

