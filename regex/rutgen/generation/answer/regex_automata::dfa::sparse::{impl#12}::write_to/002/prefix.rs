// Answer 0

#[test]
fn test_write_to_with_both_kind() {
    let start_map = StartByteMap::new(&LookMatcher::default());
    let start_table = StartTable {
        table: vec![0u8; 32],
        kind: StartKind::Both,
        start_map,
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: Some(StateID(0)),
        universal_start_anchored: Some(StateID(1)),
    };
    let mut dst = vec![0u8; start_table.write_to_len()];
    let _ = start_table.write_to::<LittleEndian>(&mut dst);
}

#[test]
fn test_write_to_with_unanchored_kind() {
    let start_map = StartByteMap::new(&LookMatcher::default());
    let start_table = StartTable {
        table: vec![0u8; 32],
        kind: StartKind::Unanchored,
        start_map,
        stride: 2,
        pattern_len: Some(2),
        universal_start_unanchored: None,
        universal_start_anchored: Some(StateID(2)),
    };
    let mut dst = vec![0u8; start_table.write_to_len()];
    let _ = start_table.write_to::<LittleEndian>(&mut dst);
}

#[test]
fn test_write_to_with_anchored_kind() {
    let start_map = StartByteMap::new(&LookMatcher::default());
    let start_table = StartTable {
        table: vec![0u8; 32],
        kind: StartKind::Anchored,
        start_map,
        stride: 0,
        pattern_len: None,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let mut dst = vec![0u8; start_table.write_to_len()];
    let _ = start_table.write_to::<LittleEndian>(&mut dst);
}

#[test]
#[should_panic]
fn test_write_to_with_invalid_size_for_both_kind() {
    let start_map = StartByteMap::new(&LookMatcher::default());
    let start_table = StartTable {
        table: vec![0u8; 32], // Assuming this results in nwrite > 32
        kind: StartKind::Both,
        start_map,
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: Some(StateID(0)),
        universal_start_anchored: Some(StateID(1)),
    };
    let mut dst = vec![0u8; 16]; // Intentionally smaller
    let _ = start_table.write_to::<LittleEndian>(&mut dst);
}

#[test]
#[should_panic]
fn test_write_to_with_invalid_size_for_unanchored_kind() {
    let start_map = StartByteMap::new(&LookMatcher::default());
    let start_table = StartTable {
        table: vec![0u8; 32],
        kind: StartKind::Unanchored,
        start_map,
        stride: 2,
        pattern_len: Some(2),
        universal_start_unanchored: None,
        universal_start_anchored: Some(StateID(2)),
    };
    let mut dst = vec![0u8; 16]; // Intentionally smaller
    let _ = start_table.write_to::<LittleEndian>(&mut dst);
}

#[test]
#[should_panic]
fn test_write_to_with_invalid_size_for_anchored_kind() {
    let start_map = StartByteMap::new(&LookMatcher::default());
    let start_table = StartTable {
        table: vec![0u8; 32],
        kind: StartKind::Anchored,
        start_map,
        stride: 0,
        pattern_len: None,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let mut dst = vec![0u8; 16]; // Intentionally smaller
    let _ = start_table.write_to::<LittleEndian>(&mut dst);
}

