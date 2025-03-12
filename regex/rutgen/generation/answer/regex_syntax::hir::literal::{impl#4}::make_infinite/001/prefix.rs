// Answer 0

#[test]
fn test_make_infinite_with_some_multiple_literals() {
    let mut seq = Seq {
        literals: Some(vec![Literal(vec![b'a']), Literal(vec![b'b']), Literal(vec![b'c'])]),
    };
    seq.make_infinite();
}

#[test]
fn test_make_infinite_with_some_empty_literals() {
    let mut seq = Seq {
        literals: Some(vec![]),
    };
    seq.make_infinite();
}

#[test]
fn test_make_infinite_with_none_literals() {
    let mut seq = Seq {
        literals: None,
    };
    seq.make_infinite();
}

