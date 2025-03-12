// Answer 0

#[test]
fn test_write_to_exact_length_dst() {
    let slices: Vec<u32> = vec![1, 2, 3, 4]; // even length for self.slices()
    let pattern_ids: Vec<u32> = vec![10, 20];
    let match_states = MatchStates {
        slices,
        pattern_ids,
        pattern_len: 2,
    };
    let mut dst = vec![0; match_states.write_to_len()]; // dst length matches nwrite
    match_states.write_to::<Endian>(&mut dst).unwrap();
}

#[test]
fn test_write_to_small_pattern_ids() {
    let slices: Vec<u32> = vec![1, 2, 3, 4]; // even length for self.slices()
    let pattern_ids: Vec<u32> = vec![10]; // one pattern_id to check false case
    let match_states = MatchStates {
        slices,
        pattern_ids,
        pattern_len: 1,
    };
    let mut dst = vec![0; match_states.write_to_len()]; // dst length matches nwrite
    match_states.write_to::<Endian>(&mut dst).unwrap();
}

#[test]
fn test_write_to_empty_slices() {
    let slices: Vec<u32> = vec![]; // zero length for self.slices()
    let pattern_ids: Vec<u32> = vec![10]; // one pattern_id to ensure true case
    let match_states = MatchStates {
        slices,
        pattern_ids,
        pattern_len: 1,
    };
    let mut dst = vec![0; match_states.write_to_len()]; // dst length matches nwrite
    match_states.write_to::<Endian>(&mut dst).unwrap();
}

#[test]
#[should_panic]
fn test_write_to_odd_length_slices() {
    let slices: Vec<u32> = vec![1, 2, 3]; // odd length for self.slices() to trigger failure
    let pattern_ids: Vec<u32> = vec![10, 20]; // has pattern_ids
    let match_states = MatchStates {
        slices,
        pattern_ids,
        pattern_len: 2,
    };
    let mut dst = vec![0; match_states.write_to_len()]; // dst length matching nwrite
    match_states.write_to::<Endian>(&mut dst).unwrap();
}

#[test]
fn test_write_to_no_pattern_ids() {
    let slices: Vec<u32> = vec![1, 2, 3, 4]; // even length for self.slices()
    let pattern_ids: Vec<u32> = vec![]; // no pattern ids to ensure false case
    let match_states = MatchStates {
        slices,
        pattern_ids,
        pattern_len: 0,
    };
    let mut dst = vec![0; match_states.write_to_len()]; // dst length matches nwrite
    match_states.write_to::<Endian>(&mut dst).unwrap();
}

