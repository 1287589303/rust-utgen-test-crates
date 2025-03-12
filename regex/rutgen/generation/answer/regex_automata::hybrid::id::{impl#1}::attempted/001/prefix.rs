// Answer 0

#[test]
fn test_lazy_state_id_error_attempted_zero() {
    let error = LazyStateIDError { attempted: 0 };
    let _ = error.attempted();
}

#[test]
fn test_lazy_state_id_error_attempted_one() {
    let error = LazyStateIDError { attempted: 1 };
    let _ = error.attempted();
}

#[test]
fn test_lazy_state_id_error_attempted_boundary_low() {
    let error = LazyStateIDError { attempted: 2u64.pow(63) - 1 };
    let _ = error.attempted();
}

#[test]
fn test_lazy_state_id_error_attempted_max() {
    let error = LazyStateIDError { attempted: u64::MAX };
    let _ = error.attempted();
}

