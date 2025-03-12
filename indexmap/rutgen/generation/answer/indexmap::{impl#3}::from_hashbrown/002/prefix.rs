// Answer 0

#[test]
fn test_from_hashbrown_capacity_overflow() {
    let error = hashbrown::TryReserveError::CapacityOverflow;
    let result = TryReserveError::from_hashbrown(error);
}

#[test]
fn test_from_hashbrown_alloc_error() {
    let layout = alloc::alloc::Layout::from_size_align(1, 1).unwrap();
    let error = hashbrown::TryReserveError::AllocError { layout };
    let result = TryReserveError::from_hashbrown(error);
}

