// Answer 0

#[test]
fn test_is_exact_none() {
    let seq = Seq { literals: None };
    let _ = seq.is_exact();
}

#[test]
fn test_is_exact_empty() {
    let seq = Seq { literals: Some(vec![]) };
    let _ = seq.is_exact();
}

#[test]
fn test_is_exact_single_exact() {
    let literal = Literal { exact: true, bytes: vec![b'a'], };
    let seq = Seq { literals: Some(vec![literal]) };
    let _ = seq.is_exact();
}

#[test]
fn test_is_exact_single_inexact() {
    let literal = Literal { exact: false, bytes: vec![b'a'], };
    let seq = Seq { literals: Some(vec![literal]) };
    let _ = seq.is_exact();
}

#[test]
fn test_is_exact_mixed_literals() {
    let exact_literal = Literal { exact: true, bytes: vec![b'a'], };
    let inexact_literal = Literal { exact: false, bytes: vec![b'b'], };
    let seq = Seq { literals: Some(vec![inexact_literal, exact_literal]) };
    let _ = seq.is_exact();
}

