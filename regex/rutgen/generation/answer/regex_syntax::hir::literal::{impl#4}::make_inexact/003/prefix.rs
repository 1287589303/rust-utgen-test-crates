// Answer 0

#[test]
fn test_make_inexact_with_none_literals() {
    let mut seq = Seq { literals: None };
    seq.make_inexact();
}

#[test]
fn test_make_inexact_with_empty_literals() {
    let mut seq = Seq { literals: Some(vec![]) };
    seq.make_inexact();
}

