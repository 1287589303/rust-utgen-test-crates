// Answer 0

#[test]
fn test_from_bytes_unchecked_valid_case() {
    let state_len: u32 = 1; // one match state
    let pair_len: usize = (state_len.checked_mul(2).unwrap()) as usize; // two pairs for state_len
    let pattern_id_len: u32 = 1; // one pattern ID

    let slices_bytes_len = (pair_len * 4) as usize; // each pattern ID is 4 bytes
    let pattern_ids_len = (pattern_id_len * 4) as usize;

    let mut bytes: Vec<u8> = vec![
        // Mock slice containing the data.
    ];

    bytes.extend_from_slice(&state_len.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes()); // nr for state length
    bytes.resize(bytes.len() + slices_bytes_len + pattern_ids_len, 0);

    // Fill slices
    let slices: Vec<u32> = vec![0, 4]; // offsets
    for (i, &b) in slices.iter().enumerate() {
        bytes[8 + (i * 4)..8 + (i * 4) + 4].copy_from_slice(&b.to_le_bytes());
    }

    bytes.extend_from_slice(&(1 as u32).to_le_bytes()); // pattern length
    bytes.extend_from_slice(&0_u32.to_le_bytes()); // nr for pattern length

    bytes.extend_from_slice(&(pattern_id_len as u32).to_le_bytes()); // pattern ID length
    bytes.extend_from_slice(&0_u32.to_le_bytes()); // nr for pattern ID length

    // Fill pattern ids
    let pattern_ids: Vec<u32> = vec![0];
    for (i, &b) in pattern_ids.iter().enumerate() {
        let offset = 8 + slices_bytes_len + (i * 4);
        bytes[offset..offset + 4].copy_from_slice(&b.to_le_bytes());
    }

    let (match_states, _) = unsafe { MatchStates::from_bytes_unchecked(&bytes) }.unwrap();
}

#[test]
#[should_panic]
fn test_from_bytes_unchecked_buffer_too_small_case() {
    let mut bytes: Vec<u8> = vec![0; 5]; // Not enough data to read state length

    unsafe { MatchStates::from_bytes_unchecked(&bytes) }.unwrap();
}

#[test]
#[should_panic]
fn test_from_bytes_unchecked_invalid_length_case() {
    let state_len: u32 = 1; // one match state
    let pair_len: usize = (state_len.checked_mul(2).unwrap()) as usize; // two pairs for state_len
    let pattern_id_len: u32 = 1; // one pattern ID

    let slices_bytes_len = (pair_len * 4) as usize; // each pattern ID is 4 bytes
    let pattern_ids_len = (pattern_id_len * 4) as usize;

    let mut bytes: Vec<u8> = vec![
        // Mock slice containing the data.
    ];

    bytes.extend_from_slice(&state_len.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes()); // nr for state length
    bytes.resize(bytes.len() + slices_bytes_len - 1, 0); // Intentional under-sizing

    let (match_states, _) = unsafe { MatchStates::from_bytes_unchecked(&bytes) }.unwrap();
}

#[test]
fn test_from_bytes_unchecked_empty_slices_case() {
    let state_len: u32 = 0; // No match states
    let bytes: Vec<u8> = state_len.to_le_bytes().to_vec();

    let (match_states, _) = unsafe { MatchStates::from_bytes_unchecked(&bytes) }.unwrap();
    assert_eq!(match_states.pattern_len, 0);
}

