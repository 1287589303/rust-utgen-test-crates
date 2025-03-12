// Answer 0

#[test]
fn test_as_ref_with_vec() {
    let slices: Vec<u32> = vec![0, 1, 2];
    let pattern_ids: Vec<u32> = vec![100, 101, 102, 103];
    let pattern_len: usize = 4;

    let match_states = MatchStates {
        slices,
        pattern_ids,
        pattern_len,
    };

    let result = match_states.as_ref();
}

#[test]
fn test_as_ref_with_slice() {
    let slices: &[u32] = &[0, 1, 2];
    let pattern_ids: &[u32] = &[100, 101, 102, 103];
    let pattern_len: usize = 4;

    let match_states = MatchStates {
        slices: slices.to_vec(),
        pattern_ids: pattern_ids.to_vec(),
        pattern_len,
    };

    let result = match_states.as_ref();
}

#[test]
fn test_as_ref_with_boundary_condition() {
    let slices: Vec<u32> = vec![0];
    let pattern_ids: Vec<u32> = vec![200];
    let pattern_len: usize = 1;

    let match_states = MatchStates {
        slices,
        pattern_ids,
        pattern_len,
    };

    let result = match_states.as_ref();
}

