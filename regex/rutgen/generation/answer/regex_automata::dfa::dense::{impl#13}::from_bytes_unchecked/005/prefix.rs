// Answer 0

#[test]
fn test_from_bytes_unchecked_stride2_too_small() {
    let state_len: u32 = 1;
    let stride2: u32 = 0;
    let byte_classes: [u8; 256] = [0; 256];
    let mut slice: [u8; 512] = [0; 512];

    slice[0..4].copy_from_slice(&state_len.to_le_bytes());
    slice[4..8].copy_from_slice(&stride2.to_le_bytes());
    slice[8..264].copy_from_slice(&byte_classes);

    let result = unsafe { from_bytes_unchecked(&mut slice) };
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_unchecked_stride2_maximum() {
    let state_len: u32 = 1;
    let stride2: u32 = 9;
    let byte_classes: [u8; 256] = [0; 256];
    let mut slice: [u8; 512] = [0; 512];

    slice[0..4].copy_from_slice(&state_len.to_le_bytes());
    slice[4..8].copy_from_slice(&stride2.to_le_bytes());
    slice[8..264].copy_from_slice(&byte_classes);

    let result = unsafe { from_bytes_unchecked(&mut slice) };
    assert!(result.is_err());
}

