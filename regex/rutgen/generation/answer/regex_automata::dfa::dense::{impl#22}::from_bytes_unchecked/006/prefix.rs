// Answer 0

#[test]
fn test_from_bytes_unchecked_success() {
    let data: [u8; 32] = [
        0x01, 0x00, 0x00, 0x00, // state_len (1)
        0x02, 0x00, 0x00, 0x00, // pair_len = 2 * state_len (2)
        0x04, 0x00, 0x00, 0x00, // slices bytes length (4 bytes for 1 pair)
        0x00, 0x00, 0x00, 0x00, // offset 0
        0x01, 0x00, 0x00, 0x00, // length 1
        0x01, 0x00, 0x00, 0x00, // pattern_len (1)
        0x01, 0x00, 0x00, 0x00, // idlen (1)
        0x01, 0x00, 0x00, 0x00, // pattern_id 0
    ];
    
    let slice = &data[..];
    let _ = unsafe { MatchStates::from_bytes_unchecked(slice) };
}

#[test]
fn test_from_bytes_unchecked_edge_length() {
    let data: [u8; 16] = [
        0x01, 0x00, 0x00, 0x00, // state_len (1)
        0x02, 0x00, 0x00, 0x00, // pair_len = 2 * state_len (2)
        0x04, 0x00, 0x00, 0x00, // slices bytes length (4 bytes for 1 pair)
        0x00, 0x00, 0x00, 0x00, // offset 0
        0x01, 0x00, 0x00, 0x00, // length 1
        0x01, 0x00, 0x00, 0x00, // pattern_len (1)
        0x01, 0x00, 0x00, 0x00, // idlen (1)
    ];
    
    let slice = &data[..];
    let _ = unsafe { MatchStates::from_bytes_unchecked(slice) };
}

#[test]
#[should_panic] // Expecting a panic due to pattern_length being erroneous
fn test_from_bytes_unchecked_invalid_pattern_length() {
    let data: [u8; 24] = [
        0x01, 0x00, 0x00, 0x00, // state_len (1)
        0x02, 0x00, 0x00, 0x00, // pair_len = 2 * state_len (2)
        0x04, 0x00, 0x00, 0x00, // slice bytes length
        0x00, 0x00, 0x00, 0x00, // offset 0
        0x01, 0x00, 0x00, 0x00, // length 1
        0x01, 0x00, 0x00, 0x00, // pattern_len (1)
        0x02, 0x00, 0x00, 0x00, // invalid idlen (2), should cause failure
    ];

    let slice = &data[..];
    let _ = unsafe { MatchStates::from_bytes_unchecked(slice) };
} 

#[test]
fn test_from_bytes_unchecked_maximum_valid_size() {
    let data: [u8; 512] = [
        // Header
        0x7F, 0x00, 0x00, 0x00, // state_len (127)
        0xFE, 0x00, 0x00, 0x00, // pair_len = 2 * state_len (254)
    ];
    
    // Fill in the slice offsets and lengths for the maximum valid case
    let mut buffer = data.to_vec();
    for i in 0..127 {
        buffer.extend_from_slice(&[(i * 4) as u32 as u8; 4]); // Offset
        buffer.extend_from_slice(&[1u32 as u8; 4]); // Length
    }
    buffer.extend_from_slice(&[0x7F, 0x00, 0x00, 0x00]); // pattern_len (127)
    buffer.extend_from_slice(&[127u32 as u8; 4]); // idlen (127)
    for i in 0..127 {
        buffer.extend_from_slice(&[i as u8; 4]); // pattern_ids
    }

    let slice = &buffer[..];
    let _ = unsafe { MatchStates::from_bytes_unchecked(slice) };
}

