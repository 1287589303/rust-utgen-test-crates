// Answer 0

#[test]
fn test_max_literal_len_with_single_literal() {
    let lit = Literal(vec![b'a']).len();
    let seq = Seq {
        literals: Some(vec![lit]),
    };
    seq.max_literal_len();
}

#[test]
fn test_max_literal_len_with_multiple_literals() {
    let lit1 = Literal(vec![b'a', b'b']).len();
    let lit2 = Literal(vec![b'c']).len();
    let lit3 = Literal(vec![b'd', b'e', b'f']).len();
    let seq = Seq {
        literals: Some(vec![lit1, lit2, lit3]),
    };
    seq.max_literal_len();
}

#[test]
fn test_max_literal_len_with_varying_lengths() {
    let lit1 = Literal(vec![b'a']).len();
    let lit2 = Literal(vec![b'b', b'c', b'd']).len();
    let lit3 = Literal(vec![b'e']).len();
    let seq = Seq {
        literals: Some(vec![lit1, lit2, lit3]),
    };
    seq.max_literal_len();
}

#[test]
fn test_max_literal_len_with_empty_vec() {
    let seq = Seq {
        literals: Some(vec![]),
    };
    seq.max_literal_len();
}

#[test]
fn test_max_literal_len_with_large_literals() {
    let lit1 = Literal(vec![b'a'; 100]).len();
    let lit2 = Literal(vec![b'b'; 200]).len();
    let seq = Seq {
        literals: Some(vec![lit1, lit2]),
    };
    seq.max_literal_len();
}

