// Answer 0

#[test]
fn test_try_reserve_positive_capacity() {
    use hashbrown::HashMap;
    
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.try_reserve(10).expect("failed to reserve capacity for 10 elements");
}

#[test]
fn test_try_reserve_zero_capacity() {
    use hashbrown::HashMap;
    
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.try_reserve(0).expect("failed to reserve capacity for 0 elements");
}

#[test]
#[should_panic]
fn test_try_reserve_capacity_overflow() {
    use hashbrown::HashMap;
    use hashbrown::TryReserveError;
    
    let mut map: HashMap<i32, i32> = HashMap::new();

    match map.try_reserve(usize::MAX) {
        Err(error) => {
            match error {
                TryReserveError::CapacityOverflow => {}
                _ => panic!("expected TryReserveError::CapacityOverflow, got different error"),
            }
        }
        _ => panic!("expected an error due to overflow"),
    }
}

