// Answer 0

#[test]
fn test_literals_none() {
    let seq = Seq { literals: None };
    let result = seq.literals();
}

#[test]
fn test_literals_empty_vec() {
    let seq = Seq { literals: Some(vec![]) };
    let result = seq.literals();
}

#[test]
fn test_literals_singleton() {
    let literal = Literal { bytes: vec![0], exact: true };
    let seq = Seq { literals: Some(vec![literal]) };
    let result = seq.literals();
}

#[test]
fn test_literals_multiple() {
    let literal1 = Literal { bytes: vec![0], exact: true };
    let literal2 = Literal { bytes: vec![1], exact: false };
    let seq = Seq { literals: Some(vec![literal1, literal2]) };
    let result = seq.literals();
}

#[test]
fn test_literals_infinite() {
    let seq = Seq::infinite();
    let result = seq.literals();
}

