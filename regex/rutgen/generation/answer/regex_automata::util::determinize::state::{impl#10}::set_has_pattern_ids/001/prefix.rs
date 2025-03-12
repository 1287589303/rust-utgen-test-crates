// Answer 0

#[test]
fn test_set_has_pattern_ids() {
    let mut vec = vec![0u8; 1];
    {
        let mut repr_vec = ReprVec(&mut vec);
        repr_vec.set_has_pattern_ids();
    }
    assert_eq!(vec[0], 2); // This validates that bit 1 is set.
}

#[test]
fn test_set_has_pattern_ids_boundary() {
    let mut vec = vec![0u8; 2];
    {
        let mut repr_vec = ReprVec(&mut vec);
        repr_vec.set_has_pattern_ids();
    }
    assert_eq!(vec[0], 2); // Again checks that first byte's bit 1 is set.
}

