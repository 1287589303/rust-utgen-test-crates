// Answer 0

#[test]
fn test_to_owned_with_vec_u8() {
    let start_table = StartTable {
        table: vec![1, 2, 3, 4, 5, 6, 7, 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::default(); 256] },
        stride: 2,
        pattern_len: Some(3),
        universal_start_unanchored: Some(StateID(1)),
        universal_start_anchored: Some(StateID(2)),
    };

    let _owned_table = start_table.to_owned();
}

#[test]
fn test_to_owned_with_slice() {
    let start_table = StartTable {
        table: &[1, 2, 3, 4, 5, 6, 7, 8],
        kind: StartKind::Unanchored,
        start_map: StartByteMap { map: [Start::default(); 256] },
        stride: 1,
        pattern_len: None,
        universal_start_unanchored: None,
        universal_start_anchored: Some(StateID(2)),
    };

    let _owned_table = start_table.to_owned();
}

#[test]
fn test_to_owned_empty_table() {
    let start_table = StartTable {
        table: vec![],
        kind: StartKind::Anchored,
        start_map: StartByteMap { map: [Start::default(); 256] },
        stride: 0,
        pattern_len: Some(0),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let _owned_table = start_table.to_owned();
}

