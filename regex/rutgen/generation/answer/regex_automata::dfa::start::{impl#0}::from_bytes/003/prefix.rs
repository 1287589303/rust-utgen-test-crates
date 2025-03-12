// Answer 0

#[test]
fn test_from_bytes_both() {
    let slice: &[u8] = &[0, 0, 0, 0];
    let result = StartKind::from_bytes(slice);
}

#[test]
fn test_from_bytes_unanchored() {
    let slice: &[u8] = &[1, 0, 0, 0];
    let result = StartKind::from_bytes(slice);
}

#[test]
fn test_from_bytes_anchored() {
    let slice: &[u8] = &[2, 0, 0, 0];
    let result = StartKind::from_bytes(slice);
}

#[test]
fn test_from_bytes_unrecognized() {
    let slice: &[u8] = &[3, 0, 0, 0];
    let result = StartKind::from_bytes(slice);
}

