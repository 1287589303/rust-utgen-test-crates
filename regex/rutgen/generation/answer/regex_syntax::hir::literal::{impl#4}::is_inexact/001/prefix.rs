// Answer 0

#[test]
fn test_is_inexact_empty_sequence() {
    let seq = Seq::empty();
    seq.is_inexact();
}

#[test]
fn test_is_inexact_some_exact_literals() {
    let literal_exact = Literal::new(Box::from("exact" as &[u8]), true);
    let seq = Seq::new(vec![literal_exact]);
    seq.is_inexact();
}

#[test]
fn test_is_inexact_some_inexact_literals() {
    let literal_inexact = Literal::new(Box::from("inexact" as &[u8]), false);
    let seq = Seq::new(vec![literal_inexact]);
    seq.is_inexact();
}

#[test]
fn test_is_inexact_all_inexact_literals() {
    let literal_inexact1 = Literal::new(Box::from("inexact1" as &[u8]), false);
    let literal_inexact2 = Literal::new(Box::from("inexact2" as &[u8]), false);
    let seq = Seq::new(vec![literal_inexact1, literal_inexact2]);
    seq.is_inexact();
}

#[test]
fn test_is_inexact_all_possible_literals() {
    let seq = Seq { literals: None };
    seq.is_inexact();
}

#[test]
fn test_is_inexact_infinite_sequence() {
    let seq = Seq::infinite();
    seq.is_inexact();
}

