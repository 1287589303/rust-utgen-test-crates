// Answer 0

#[test]
fn test_write_to_success_case() {
    let mut dst = vec![0u8; 64];
    let table = StartTable {
        table: vec![0u8; 32],
        kind: StartKind::Both,
        start_map: StartByteMap::new(&LookMatcher::default()),
        stride: 4,
        pattern_len: Some(1),
        universal_start_unanchored: Some(StateID(0)),
        universal_start_anchored: Some(StateID(1)),
    };
    let _ = table.write_to::<EndianType>(&mut dst).unwrap();
}

#[test]
fn test_write_to_length_case() {
    let mut dst = vec![0u8; 60];
    let table = StartTable {
        table: vec![0u8; 32],
        kind: StartKind::Unanchored,
        start_map: StartByteMap::new(&LookMatcher::default()),
        stride: 4,
        pattern_len: Some(0),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let nwrite = table.write_to::<EndianType>(&mut dst).unwrap();
    assert_eq!(dst.len(), nwrite);
}

#[test]
#[should_panic]
fn test_write_to_too_small_dst() {
    let mut dst = vec![0u8; 10];
    let table = StartTable {
        table: vec![0u8; 40],
        kind: StartKind::Anchored,
        start_map: StartByteMap::new(&LookMatcher::default()),
        stride: 8,
        pattern_len: Some(2),
        universal_start_unanchored: None,
        universal_start_anchored: Some(StateID(2)),
    };
    let _ = table.write_to::<EndianType>(&mut dst).unwrap();
}

#[test]
fn test_write_to_no_start_ids_case() {
    let mut dst = vec![0u8; 64];
    let table = StartTable {
        table: vec![0u8; 32],
        kind: StartKind::Both,
        start_map: StartByteMap::new(&LookMatcher::default()),
        stride: 0,
        pattern_len: None,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let result = table.write_to::<EndianType>(&mut dst).unwrap();
    assert_eq!(result, table.write_to_len());
}

