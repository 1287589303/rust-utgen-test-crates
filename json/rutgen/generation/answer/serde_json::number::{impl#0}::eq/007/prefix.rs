// Answer 0

#[test]
fn test_eq_pos_int_equal() {
    let a = N::PosInt(0);
    let b = N::PosInt(0);
    let _ = a.eq(&b);
}

#[test]
fn test_eq_pos_int_equal_max() {
    let a = N::PosInt(u64::MAX);
    let b = N::PosInt(u64::MAX);
    let _ = a.eq(&b);
}

#[test]
fn test_eq_pos_int_near_boundary() {
    let a = N::PosInt(1);
    let b = N::PosInt(1);
    let _ = a.eq(&b);
}

