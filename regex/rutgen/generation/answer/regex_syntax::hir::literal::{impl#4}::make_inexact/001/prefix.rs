// Answer 0

#[test]
fn test_make_inexact_with_non_empty_vec() {
    let mut seq = Seq {
        literals: Some(vec![Literal::exact(vec![b'a']), Literal::exact(vec![b'b'])]),
    };
    seq.make_inexact();
}

#[test]
fn test_make_inexact_with_empty_vec() {
    let mut seq = Seq {
        literals: Some(vec![]),
    };
    seq.make_inexact();
}

#[test]
fn test_make_inexact_with_none() {
    let mut seq = Seq {
        literals: None,
    };
    seq.make_inexact();
}

