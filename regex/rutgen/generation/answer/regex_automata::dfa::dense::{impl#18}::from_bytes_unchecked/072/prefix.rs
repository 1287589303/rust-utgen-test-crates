// Answer 0

#[test]
fn test_from_bytes_unchecked_valid_with_both() {
    let slice: &[u8] = &[
        0, 0, 0, 0, // StartKind::Both
        0, 0, 0, 0, // StartByteMap (valid)
        6, 0, 0, 0, // stride (valid)
        0, 0, 0, 0, // maybe_pattern_len (0)
        1, 1, 1, 1, // universal_unanchored (valid)
        2, 2, 2, 2, // universal_anchored (valid)
        0; 240 // Padding to reach 256 bytes
    ];
    let result = unsafe { StartTable::from_bytes_unchecked(slice) };
}

#[test]
fn test_from_bytes_unchecked_valid_with_unanchored() {
    let slice: &[u8] = &[
        1, 0, 0, 0, // StartKind::Unanchored
        0, 0, 0, 0, // StartByteMap (valid)
        6, 0, 0, 0, // stride (valid)
        5, 0, 0, 0, // maybe_pattern_len (5)
        1, 1, 1, 1, // universal_unanchored (valid)
        3, 3, 3, 3, // universal_anchored (valid)
        0; 240 // Padding to reach 256 bytes
    ];
    let result = unsafe { StartTable::from_bytes_unchecked(slice) };
}

#[test]
fn test_from_bytes_unchecked_valid_with_anchored() {
    let slice: &[u8] = &[
        2, 0, 0, 0, // StartKind::Anchored
        0, 0, 0, 0, // StartByteMap (valid)
        6, 0, 0, 0, // stride (valid)
        4, 0, 0, 0, // maybe_pattern_len (4)
        1, 1, 1, 1, // universal_unanchored (valid)
        5, 5, 5, 5, // universal_anchored (valid)
        0; 240 // Padding to reach 256 bytes
    ];
    let result = unsafe { StartTable::from_bytes_unchecked(slice) };
}

