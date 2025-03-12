// Answer 0

#[test]
fn test_check_alignment_u8() {
    let slice: &[u8] = &[1, 2, 3]; // Address alignment may cause misalignment.
    let result = check_alignment::<u8>(slice);
}

#[test]
fn test_check_alignment_u16() {
    let slice: &[u8] = &[1, 2, 3, 4, 5, 6, 7]; // Address is not aligned for u16.
    let result = check_alignment::<u16>(slice);
}

#[test]
fn test_check_alignment_u32() {
    let slice: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9]; // Address not aligned for u32.
    let result = check_alignment::<u32>(slice);
}

#[test]
fn test_check_alignment_u64() {
    let slice: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]; // Address not aligned for u64.
    let result = check_alignment::<u64>(slice);
}

#[test]
fn test_check_alignment_u128() {
    let slice: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]; // Address not aligned for u128.
    let result = check_alignment::<u128>(slice);
}

