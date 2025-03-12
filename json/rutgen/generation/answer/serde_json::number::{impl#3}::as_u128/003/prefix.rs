// Answer 0

#[test]
fn test_as_u128_positive_integer() {
    let number = Number { n: N::PosInt(1) };
    let _result = number.as_u128();
}

#[test]
fn test_as_u128_positive_integer_max() {
    let number = Number { n: N::PosInt(u64::MAX) };
    let _result = number.as_u128();
}

#[test]
fn test_as_u128_positive_integer_middle() {
    let number = Number { n: N::PosInt(123456789) };
    let _result = number.as_u128();
}

