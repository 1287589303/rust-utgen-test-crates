// Answer 0

#[test]
fn test_write_to_both_kind_with_small_buffer() {
    let start_kind = StartKind::Both;
    let mut dst: [u8; 2] = [0; 2]; // length less than size_of::<u32>()
    let result = start_kind.write_to::<wire::LittleEndian>(&mut dst);
}

#[test]
fn test_write_to_unanchored_kind_with_small_buffer() {
    let start_kind = StartKind::Unanchored;
    let mut dst: [u8; 2] = [0; 2]; // length less than size_of::<u32>()
    let result = start_kind.write_to::<wire::LittleEndian>(&mut dst);
}

#[test]
fn test_write_to_anchored_kind_with_small_buffer() {
    let start_kind = StartKind::Anchored;
    let mut dst: [u8; 2] = [0; 2]; // length less than size_of::<u32>()
    let result = start_kind.write_to::<wire::LittleEndian>(&mut dst);
}

