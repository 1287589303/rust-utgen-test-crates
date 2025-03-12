// Answer 0

#[test]
fn test_from_bytes_unchecked_valid_case() {
    let state_len: u32 = 2;
    let pattern_len: u32 = 3;
    let idlen: u32 = 4;
    let slice: Vec<u8> = vec![
        // match state length (4 bytes)
        (state_len).to_le_bytes().to_vec(),
        // state pairs: 2 pairs of (offset, length), each 4 bytes
        0, 0, 0, 0,   // offset for first state
        4, 0, 0, 0,   // length for first state
        4, 0, 0, 0,   // offset for second state
        4, 0, 0, 0,   // length for second state
        // pattern length (4 bytes)
        (pattern_len).to_le_bytes().to_vec(),
        // ID length (4 bytes) - will cause failure in the ID length read
        (idlen).to_le_bytes().to_vec(),
        // pattern ID data - ensuring length + alignments are right
        0, 1, 2, 3, // 4 pattern IDs
    ].concat();

    let result = unsafe { MatchStates::from_bytes_unchecked(&slice) };
}

#[test]
#[should_panic]
fn test_from_bytes_unchecked_invalid_slice() {
    let state_len: u32 = 1;
    let slice: Vec<u8> = vec![
        // match state length (4 bytes)
        (state_len).to_le_bytes().to_vec(),
        // state pairs: 1 pairs of (offset, length), each 4 bytes
        0, 0, 0, 0,   // offset for first state
        4, 0, 0, 0,   // length for first state
        // pattern length (4 bytes)
        0, 0, 0, 0,   // pattern length
        // ID length (invalid, lesser than required: 4 bytes expected)
        // empty byte slice
    ].concat();

    let _ = unsafe { MatchStates::from_bytes_unchecked(&slice) };
}

#[test]
fn test_from_bytes_unchecked_boundary_case() {
    let state_len: u32 = 1;
    let pattern_len: u32 = 1;
    let idlen: u32 = 0; // will trigger an error for empty pattern ID

    let slice: Vec<u8> = vec![
        // match state length (4 bytes)
        (state_len).to_le_bytes().to_vec(),
        // state pairs: 1 pairs of (offset, length), each 4 bytes
        0, 0, 0, 0,   // offset for first state
        4, 0, 0, 0,   // length for first state
        // pattern length (4 bytes)
        (pattern_len).to_le_bytes().to_vec(),
        // ID length (4 bytes)
        (idlen).to_le_bytes().to_vec(),
        // no pattern ID data - triggering check slice length failure
    ].concat();

    let result = unsafe { MatchStates::from_bytes_unchecked(&slice) };
}

