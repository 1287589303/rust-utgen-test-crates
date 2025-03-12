// Answer 0

#[test]
fn test_from_ne_bytes_zero() {
    let bytes: [u8; 4] = [0, 0, 0, 0];
    let _ = SmallIndex::from_ne_bytes(bytes);
}

#[test]
fn test_from_ne_bytes_at_max() {
    let max_value = SmallIndex::MAX.as_u32();
    let bytes: [u8; 4] = max_value.to_ne_bytes();
    let _ = SmallIndex::from_ne_bytes(bytes);
}

#[test]
fn test_from_ne_bytes_mid_value() {
    let mid_value = SmallIndex::MAX.as_u32() / 2;
    let bytes: [u8; 4] = mid_value.to_ne_bytes();
    let _ = SmallIndex::from_ne_bytes(bytes);
}

#[test]
fn test_from_ne_bytes_near_max() {
    let near_max_value = SmallIndex::MAX.as_u32() - 1;
    let bytes: [u8; 4] = near_max_value.to_ne_bytes();
    let _ = SmallIndex::from_ne_bytes(bytes);
}

#[test]
fn test_from_ne_bytes_ranged_value() {
    let ranged_value = 123456;
    let bytes: [u8; 4] = ranged_value.to_ne_bytes();
    let _ = SmallIndex::from_ne_bytes(bytes);
}

