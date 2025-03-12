// Answer 0

#[test]
fn test_insufficient_state_id_capacity_zero() {
    let err = LazyStateIDError { attempted: 0 };
    let result = BuildError::insufficient_state_id_capacity(err);
}

#[test]
fn test_insufficient_state_id_capacity_mid_value() {
    let err = LazyStateIDError { attempted: 12345 };
    let result = BuildError::insufficient_state_id_capacity(err);
}

#[test]
fn test_insufficient_state_id_capacity_max_value() {
    let err = LazyStateIDError { attempted: u64::MAX };
    let result = BuildError::insufficient_state_id_capacity(err);
}

