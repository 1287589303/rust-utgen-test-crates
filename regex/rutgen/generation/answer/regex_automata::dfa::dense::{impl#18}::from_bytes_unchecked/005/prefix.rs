// Answer 0

#[test]
fn test_from_bytes_unchecked_success_case() {
    let slice: &[u8] = &[
        // StartKind::Both (0)
        0, 0, 0, 0,
        // Valid StartByteMap entries (first 256 bytes)
        [0u8; 256].map(|x| (x % 6) as u8).as_slice().to_vec(),
        // Stride (6)
        6, 0, 0, 0,
        // Patterns (0)
        0, 0, 0, 0,
    ].concat();

    let result = unsafe { StartTable::from_bytes_unchecked(&slice) };
    let _ = result.unwrap();
}

#[test]
fn test_from_bytes_unchecked_patterns_none_case() {
    let slice: &[u8] = &[
        // StartKind::Unanchored (1)
        1, 0, 0, 0,
        // Valid StartByteMap entries (first 256 bytes)
        [1u8; 256].map(|x| (x % 6) as u8).as_slice().to_vec(),
        // Stride (6)
        6, 0, 0, 0,
        // Patterns (None, represented by u32::MAX)
        255, 255, 255, 255,
    ].concat();

    let result = unsafe { StartTable::from_bytes_unchecked(&slice) };
    let _ = result.unwrap();
}

#[test]
fn test_from_bytes_unchecked_wrong_stride() {
    let slice: &[u8] = &[
        // StartKind::Anchored (2)
        2, 0, 0, 0,
        // Valid StartByteMap entries (first 256 bytes)
        [2u8; 256].map(|x| (x % 6) as u8).as_slice().to_vec(),
        // Wrong Stride (4)
        4, 0, 0, 0,
        // Patterns (None, represented by u32::MAX)
        255, 255, 255, 255,
    ].concat();

    let result = unsafe { StartTable::from_bytes_unchecked(&slice) };
    assert!(result.is_err());
}

