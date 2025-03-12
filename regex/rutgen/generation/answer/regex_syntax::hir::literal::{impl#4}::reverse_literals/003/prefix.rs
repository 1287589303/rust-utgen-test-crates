// Answer 0

#[test]
fn test_reverse_literals_non_empty_exact_literals() {
    let mut seq = Seq {
        literals: Some(vec![
            Literal::exact(b"abc".to_vec()),
            Literal::exact(b"def".to_vec()),
        ]),
    };
    seq.reverse_literals();
}

#[test]
fn test_reverse_literals_single_literal() {
    let mut seq = Seq {
        literals: Some(vec![
            Literal::exact(b"xyz".to_vec()),
        ]),
    };
    seq.reverse_literals();
}

#[test]
fn test_reverse_literals_multiple_literals() {
    let mut seq = Seq {
        literals: Some(vec![
            Literal::exact(b"hello".to_vec()),
            Literal::exact(b"world".to_vec()),
            Literal::exact(b"rust".to_vec()),
        ]),
    };
    seq.reverse_literals();
}

#[test]
fn test_reverse_literals_prefer_exact() {
    let mut seq = Seq {
        literals: Some(vec![
            Literal::exact(b"one".to_vec()),
            Literal::exact(b"two".to_vec()),
            Literal::exact(b"three".to_vec()),
        ]),
    };
    seq.reverse_literals();
}

#[test]
fn test_reverse_literals_empty_vector() {
    let mut seq = Seq {
        literals: Some(vec![
            Literal::exact(b"".to_vec()),
        ]),
    };
    seq.reverse_literals();
}

