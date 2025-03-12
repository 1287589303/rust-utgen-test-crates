// Answer 0

#[test]
fn test_write_to_success() {
    let kind = StartKind::Both;
    let start_map = StartByteMap::new(&LookMatcher::default());
    let stride = 4;
    let pattern_len = Some(1);
    let universal_start_unanchored = None;
    let universal_start_anchored = None;
    let table = vec![StateID::default(), StateID::default(), StateID::default()];
    let start_table = StartTable {
        table,
        kind,
        start_map,
        stride,
        pattern_len,
        universal_start_unanchored,
        universal_start_anchored,
    };

    let nwrite = start_table.write_to_len();
    let mut dst = vec![0u8; nwrite];

    let _ = start_table.write_to::<Endian>(&mut dst).unwrap();
}

#[test]
fn test_write_to_with_empty_table() {
    let kind = StartKind::Both;
    let start_map = StartByteMap::new(&LookMatcher::default());
    let stride = 4;
    let pattern_len = Some(0);
    let universal_start_unanchored = None;
    let universal_start_anchored = None;
    let table: Vec<StateID> = vec![];
    let start_table = StartTable {
        table,
        kind,
        start_map,
        stride,
        pattern_len,
        universal_start_unanchored,
        universal_start_anchored,
    };

    let nwrite = start_table.write_to_len();
    let mut dst = vec![0u8; nwrite];

    let result = start_table.write_to::<Endian>(&mut dst);
    assert!(result.is_err());
}

