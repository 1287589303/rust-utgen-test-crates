// Answer 0

#[test]
fn test_from_bytes_unchecked_valid_start_kind() {
    let input: Vec<u8> = vec![
        0, 0, 0, 0, // StartKind::Both (0)
    ];
    let start_map_bytes = vec![0; 256]; // Valid StartByteMap
    let stride_bytes = vec![6u32.to_le_bytes().to_vec()];
    let pattern_len_bytes = vec![0u32.to_le_bytes().to_vec()];
    let universal_unanchored_bytes = vec![u32::MAX.to_le_bytes().to_vec()];
    let universal_anchored_bytes = vec![u32::MAX.to_le_bytes().to_vec()];
    
    let mut slice = input.clone();
    slice.extend(start_map_bytes);
    slice.extend(stride_bytes);
    slice.extend(pattern_len_bytes);
    slice.extend(universal_unanchored_bytes);
    slice.extend(universal_anchored_bytes);

    let _ = unsafe { StartTable::from_bytes_unchecked(&mut slice) };
}

#[test]
#[should_panic]
fn test_from_bytes_unchecked_stride_error() {
    let input: Vec<u8> = vec![
        1, 0, 0, 0, // StartKind::Unanchored (1)
    ];
    let start_map_bytes = vec![0; 256]; // Valid StartByteMap
    let stride_bytes = vec![7u32.to_le_bytes().to_vec()]; // Invalid stride
    let pattern_len_bytes = vec![0u32.to_le_bytes().to_vec()];
    let universal_unanchored_bytes = vec![u32::MAX.to_le_bytes().to_vec()];
    let universal_anchored_bytes = vec![u32::MAX.to_le_bytes().to_vec()];
    
    let mut slice = input.clone();
    slice.extend(start_map_bytes);
    slice.extend(stride_bytes);
    slice.extend(pattern_len_bytes);
    slice.extend(universal_unanchored_bytes);
    slice.extend(universal_anchored_bytes);

    let _ = unsafe { StartTable::from_bytes_unchecked(&mut slice) };
}

#[test]
fn test_from_bytes_unchecked_valid_with_no_patterns() {
    let input: Vec<u8> = vec![
        2, 0, 0, 0, // StartKind::Anchored (2)
    ];
    let start_map_bytes = vec![0; 256]; // Valid StartByteMap
    let stride_bytes = vec![6u32.to_le_bytes().to_vec()]; // Correct stride
    let pattern_len_bytes = vec![u32::MAX.to_le_bytes().to_vec()]; // No patterns (None)
    let universal_unanchored_bytes = vec![u32::MAX.to_le_bytes().to_vec()];
    let universal_anchored_bytes = vec![u32::MAX.to_le_bytes().to_vec()];
    
    let mut slice = input.clone();
    slice.extend(start_map_bytes);
    slice.extend(stride_bytes);
    slice.extend(pattern_len_bytes);
    slice.extend(universal_unanchored_bytes);
    slice.extend(universal_anchored_bytes);

    let _ = unsafe { StartTable::from_bytes_unchecked(&mut slice) };
}

#[test]
fn test_from_bytes_unchecked_valid_with_some_patterns() {
    let input: Vec<u8> = vec![
        0, 0, 0, 0, // StartKind::Both (0)
    ];
    let start_map_bytes = vec![0; 256]; // Valid StartByteMap
    let stride_bytes = vec![6u32.to_le_bytes().to_vec()]; // Correct stride
    let pattern_len_bytes = vec![5u32.to_le_bytes().to_vec()]; // Valid pattern length
    let universal_unanchored_bytes = vec![1u32.to_le_bytes().to_vec()]; // Valid StateID
    let universal_anchored_bytes = vec![2u32.to_le_bytes().to_vec()]; // Valid StateID
    
    let mut slice = input.clone();
    slice.extend(start_map_bytes);
    slice.extend(stride_bytes);
    slice.extend(pattern_len_bytes);
    slice.extend(universal_unanchored_bytes);
    slice.extend(universal_anchored_bytes);

    let _ = unsafe { StartTable::from_bytes_unchecked(&mut slice) };
}

