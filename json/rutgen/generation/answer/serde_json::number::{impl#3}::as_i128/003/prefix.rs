// Answer 0

#[test]
fn test_as_i128_positive_integer_min() {
    let number = Number { n: N::PosInt(0) };
    let _ = number.as_i128();
}

#[test]
fn test_as_i128_positive_integer_mid() {
    let number = Number { n: N::PosInt(12345) };
    let _ = number.as_i128();
}

#[test]
fn test_as_i128_positive_integer_max() {
    let number = Number { n: N::PosInt(9_223_372_036_854_775_807) };
    let _ = number.as_i128();
}

