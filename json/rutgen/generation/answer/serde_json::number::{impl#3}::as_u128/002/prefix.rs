// Answer 0

#[test]
fn test_as_u128_neg_int_one() {
    let number = Number {
        n: N::NegInt(-1),
    };
    number.as_u128();
}

#[test]
fn test_as_u128_neg_int_large() {
    let number = Number {
        n: N::NegInt(-42),
    };
    number.as_u128();
}

#[test]
fn test_as_u128_neg_int_negative_large() {
    let number = Number {
        n: N::NegInt(-1000000),
    };
    number.as_u128();
}

