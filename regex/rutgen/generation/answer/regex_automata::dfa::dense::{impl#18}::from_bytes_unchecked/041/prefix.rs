// Answer 0

#[test]
fn test_from_bytes_unchecked_invalid_pattern_count() {
    let invalid_pattern_len: u32 = PatternID::LIMIT as u32 + 1; // Boundary just over LIMIT
    let stride: usize = 7; // Not equal to 6
    let kind: u32 = 0; // Valid StartKind
    let start_byte_map: [u8; 256] = [0; 256]; // Valid StartByteMap

    let mut slice = Vec::with_capacity(256 + 4 + 4 + 4 + 4 + 4);
    slice.extend_from_slice(&kind.to_le_bytes()); // Insert StartKind
    slice.extend_from_slice(&start_byte_map); // Insert StartByteMap
    slice.extend_from_slice(&stride.try_into().unwrap()); // Insert stride
    slice.extend_from_slice(&invalid_pattern_len.to_le_bytes()); // Insert invalid pattern length
    slice.extend_from_slice(&0); // Insert the universal start for unanchored
    slice.extend_from_slice(&0); // Insert the universal start for anchored

    let pattern_table_size = std::mem::size_of::<u32>() * stride * (PatternID::LIMIT + 1);
    let total_size = slice.len() + pattern_table_size;

    slice.resize(total_size, 0); // Ensure the slice is large enough
    let result = unsafe { StartTable::from_bytes_unchecked(&mut slice) };

    drop(result);
}

#[test]
fn test_from_bytes_unchecked_invalid_stride() {
    let invalid_stride: usize = 5; // Not equal to 6
    let kind: u32 = 1; // Valid StartKind
    let start_byte_map: [u8; 256] = [0; 256]; // Valid StartByteMap
    let valid_pattern_len: u32 = 10; // Valid u32

    let mut slice = Vec::with_capacity(256 + 4 + 4 + 4 + 4 + 4);
    slice.extend_from_slice(&kind.to_le_bytes()); // Insert StartKind
    slice.extend_from_slice(&start_byte_map); // Insert StartByteMap
    slice.extend_from_slice(&invalid_stride.to_le_bytes()); // Insert invalid stride
    slice.extend_from_slice(&valid_pattern_len.to_le_bytes()); // Insert valid pattern length
    slice.extend_from_slice(&0); // Insert the universal start for unanchored
    slice.extend_from_slice(&0); // Insert the universal start for anchored

    let pattern_table_size = std::mem::size_of::<u32>() * invalid_stride * (PatternID::LIMIT + 1);
    let total_size = slice.len() + pattern_table_size;

    slice.resize(total_size, 0); // Ensure the slice is large enough
    let result = unsafe { StartTable::from_bytes_unchecked(&mut slice) };

    drop(result);
}

