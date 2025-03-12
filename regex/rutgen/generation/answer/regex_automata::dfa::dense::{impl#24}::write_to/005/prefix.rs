// Answer 0

#[derive(Clone, Debug)]
struct TestEndian;
impl Endian for TestEndian {
    fn write_u32(value: u32, dst: &mut [u8]) {
        dst[..4].copy_from_slice(&value.to_le_bytes());
    }
}

#[test]
fn test_write_to_with_exact_buffer_size() {
    let slices = vec![1u32, 2u32, 3u32, 4u32];
    let pattern_ids = vec![5u32, 6u32];
    let pattern_len = 2;

    let match_states = MatchStates {
        slices: slices.clone(),
        pattern_ids: pattern_ids.clone(),
        pattern_len,
    };

    let nwrite = match_states.write_to_len();
    let mut buffer = vec![0u8; nwrite];

    match_states.write_to::<TestEndian>(&mut buffer).unwrap();
}

#[test]
fn test_write_to_with_even_slices_length() {
    let slices = vec![1u32, 2u32, 3u32, 4u32];
    let pattern_ids = vec![5u32, 6u32];
    let pattern_len = 2;

    let match_states = MatchStates {
        slices: slices.clone(),
        pattern_ids: pattern_ids.clone(),
        pattern_len,
    };

    let nwrite = match_states.write_to_len();
    let mut buffer = vec![0u8; nwrite];

    match_states.write_to::<TestEndian>(&mut buffer).unwrap();
}

#[test]
fn test_write_to_with_non_empty_slices_and_pattern_ids() {
    let slices = vec![10u32, 20u32, 30u32, 40u32];
    let pattern_ids = vec![50u32];
    let pattern_len = 1;

    let match_states = MatchStates {
        slices: slices.clone(),
        pattern_ids: pattern_ids.clone(),
        pattern_len,
    };

    let nwrite = match_states.write_to_len();
    let mut buffer = vec![0u8; nwrite];

    match_states.write_to::<TestEndian>(&mut buffer).unwrap();
}

