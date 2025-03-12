// Answer 0

#[test]
fn test_reverse_literals_non_empty() {
    let mut seq = Seq {
        literals: Some(vec![
            Literal::exact(vec![b'f', b'o', b'o']),
            Literal::exact(vec![b'r', b'a', b'b']),
        ]),
    };
    seq.reverse_literals();
}

#[test]
fn test_reverse_literals_empty() {
    let mut seq = Seq {
        literals: Some(vec![]),
    };
    seq.reverse_literals();
}

#[test]
fn test_reverse_literals_empty_literal() {
    let mut seq = Seq {
        literals: Some(vec![Literal::exact(vec![])]),
    };
    seq.reverse_literals();
}

