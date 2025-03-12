// Answer 0

#[test]
fn test_is_finite_with_none() {
    let seq = Seq { literals: None };
    seq.is_finite();
}

#[test]
fn test_is_finite_with_empty_vec() {
    let seq = Seq { literals: Some(vec![]) };
    seq.is_finite();
}

#[test]
fn test_is_finite_with_empty_string() {
    let literal = Literal { bytes: vec![0] };
    let seq = Seq { literals: Some(vec![literal]) };
    seq.is_finite();
}

#[test]
fn test_is_finite_with_multiple_literals() {
    let literal_a = Literal { bytes: vec![b'a'] };
    let literal_b = Literal { bytes: vec![b'b'] };
    let seq = Seq { literals: Some(vec![literal_a, literal_b]) };
    seq.is_finite();
}

#[test]
fn test_is_finite_with_single_literal() {
    let literal_c = Literal { bytes: vec![b'c'] };
    let seq = Seq { literals: Some(vec![literal_c]) };
    seq.is_finite();
}

