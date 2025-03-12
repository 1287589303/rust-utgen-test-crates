// Answer 0

#[test]
fn test_eq_posint_negint() {
    let a = N::PosInt(5);
    let b = N::NegInt(-3);
    a.eq(&b);
}

#[test]
fn test_eq_posint_float() {
    let a = N::PosInt(10);
    let b = N::Float(3.14);
    a.eq(&b);
}

#[test]
fn test_eq_negint_posint() {
    let a = N::NegInt(-1);
    let b = N::PosInt(1);
    a.eq(&b);
}

#[test]
fn test_eq_negint_float() {
    let a = N::NegInt(-4);
    let b = N::Float(-2.71);
    a.eq(&b);
}

#[test]
fn test_eq_float_posint() {
    let a = N::Float(2.5);
    let b = N::PosInt(2);
    a.eq(&b);
}

#[test]
fn test_eq_float_negint() {
    let a = N::Float(-3.5);
    let b = N::NegInt(-2);
    a.eq(&b);
}

