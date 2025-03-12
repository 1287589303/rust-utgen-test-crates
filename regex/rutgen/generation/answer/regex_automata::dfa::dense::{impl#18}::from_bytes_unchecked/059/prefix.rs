// Answer 0

#[test]
fn test_from_bytes_unchecked_valid_input() {
    let kind_value: u8 = 0; // StartKind::Both
    let start_map_bytes: [u8; 256] = [0; 256]; // valid StartByteMap
    let stride: u32 = 6; // valid stride
    let pattern_len: u32 = 5; // valid pattern length
    let universal_unanchored: u32 = 1; // valid StateID, not MAX
    let universal_anchored: u32 = 2; // valid StateID, not MAX

    let valid_start_ids: [u32; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]; // 12 valid StateIDs

    let mut slice: Vec<u8> = Vec::new();
    slice.push(kind_value);
    slice.extend_from_slice(&start_map_bytes);
    slice.extend_from_slice(&stride.to_le_bytes());
    slice.extend_from_slice(&pattern_len.to_le_bytes());
    slice.extend_from_slice(&universal_unanchored.to_le_bytes());
    slice.extend_from_slice(&universal_anchored.to_le_bytes());
    slice.extend_from_slice(&valid_start_ids.concat()); // concatenate into valid slice

    let result = unsafe { StartTable::<&[u32]>::from_bytes_unchecked(&mut slice) };
}

#[test]
fn test_from_bytes_unchecked_valid_input_max_pattern_len() {
    let kind_value: u8 = 1; // StartKind::Unanchored
    let start_map_bytes: [u8; 256] = [1; 256]; // valid StartByteMap
    let stride: u32 = 6; // valid stride
    let pattern_len: u32 = u32::MAX - 1; // valid pattern length within limits
    let universal_unanchored: u32 = 1; // valid StateID, not MAX
    let universal_anchored: u32 = 2; // valid StateID, not MAX

    let valid_start_ids: [u32; 12] = [12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]; // 12 valid StateIDs

    let mut slice: Vec<u8> = Vec::new();
    slice.push(kind_value);
    slice.extend_from_slice(&start_map_bytes);
    slice.extend_from_slice(&stride.to_le_bytes());
    slice.extend_from_slice(&pattern_len.to_le_bytes());
    slice.extend_from_slice(&universal_unanchored.to_le_bytes());
    slice.extend_from_slice(&universal_anchored.to_le_bytes());
    slice.extend_from_slice(&valid_start_ids.concat()); // concatenate into valid slice

    let result = unsafe { StartTable::<&[u32]>::from_bytes_unchecked(&mut slice) };
}

#[test]
fn test_from_bytes_unchecked_valid_input_no_patterns() {
    let kind_value: u8 = 2; // StartKind::Anchored
    let start_map_bytes: [u8; 256] = [2; 256]; // valid StartByteMap
    let stride: u32 = 6; // valid stride
    let pattern_len: u32 = 0; // no patterns
    let universal_unanchored: u32 = 1; // valid StateID, not MAX
    let universal_anchored: u32 = 2; // valid StateID, not MAX

    let valid_start_ids: [u32; 12] = [24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35]; // 12 valid StateIDs

    let mut slice: Vec<u8> = Vec::new();
    slice.push(kind_value);
    slice.extend_from_slice(&start_map_bytes);
    slice.extend_from_slice(&stride.to_le_bytes());
    slice.extend_from_slice(&pattern_len.to_le_bytes());
    slice.extend_from_slice(&universal_unanchored.to_le_bytes());
    slice.extend_from_slice(&universal_anchored.to_le_bytes());
    slice.extend_from_slice(&valid_start_ids.concat()); // concatenate into valid slice

    let result = unsafe { StartTable::<&[u32]>::from_bytes_unchecked(&mut slice) };
}

