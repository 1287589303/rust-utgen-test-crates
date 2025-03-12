// Answer 0

#[test]
fn test_set_len_lower_boundary() {
    let mut accels = Accels::<Vec<AccelTy>>::empty();
    accels.set_len(0);
}

#[test]
fn test_set_len_upper_boundary() {
    let mut accels = Accels::<Vec<AccelTy>>::empty();
    accels.set_len(8);
}

#[should_panic]
fn test_set_len_invalid_case() {
    let mut accels = Accels::<Vec<AccelTy>>::empty();
    accels.set_len(9);
}

