// Answer 0

#[test]
fn test_eq_neg_int_equal() {
    let a = N::NegInt(-1);
    let b = N::NegInt(-1);
    let _ = a.eq(&b);
}

#[test]
fn test_eq_neg_int_different() {
    let a = N::NegInt(-1);
    let b = N::NegInt(-2);
    let _ = a.eq(&b);
}

#[test]
fn test_eq_neg_int_boundary_min() {
    let a = N::NegInt(i64::min_value());
    let b = N::NegInt(i64::min_value());
    let _ = a.eq(&b);
}

#[test]
fn test_eq_neg_int_boundary_max() {
    let a = N::NegInt(-1);
    let b = N::NegInt(i64::min_value() + 1);
    let _ = a.eq(&b);
}

