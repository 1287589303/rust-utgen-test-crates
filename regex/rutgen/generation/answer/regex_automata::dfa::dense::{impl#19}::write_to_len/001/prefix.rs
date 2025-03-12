// Answer 0

#[test]
fn test_write_to_len_both() {
    let kind = StartKind::Both;
    let start_map = StartByteMap::new(&LookMatcher::default());
    let stride: usize = 4;
    let pattern_len: Option<usize> = Some(2);
    let universal_start_unanchored: Option<StateID> = None;
    let universal_start_anchored: Option<StateID> = None;
    let table = vec![0; 12]; // 12 is an arbitrary number to represent StateID instances

    let start_table = StartTable {
        table: table,
        kind: kind,
        start_map: start_map,
        stride: stride,
        pattern_len: pattern_len,
        universal_start_unanchored: universal_start_unanchored,
        universal_start_anchored: universal_start_anchored,
    };

    start_table.write_to_len();
}

#[test]
fn test_write_to_len_unanchored() {
    let kind = StartKind::Unanchored;
    let start_map = StartByteMap::new(&LookMatcher::default());
    let stride: usize = 2;
    let pattern_len: Option<usize> = Some(1);
    let universal_start_unanchored: Option<StateID> = Some(StateID::default());
    let universal_start_anchored: Option<StateID> = None;
    let table = vec![0; 8];

    let start_table = StartTable {
        table: table,
        kind: kind,
        start_map: start_map,
        stride: stride,
        pattern_len: pattern_len,
        universal_start_unanchored: universal_start_unanchored,
        universal_start_anchored: universal_start_anchored,
    };

    start_table.write_to_len();
}

#[test]
fn test_write_to_len_anchored() {
    let kind = StartKind::Anchored;
    let start_map = StartByteMap::new(&LookMatcher::default());
    let stride: usize = 1;
    let pattern_len: Option<usize> = None;
    let universal_start_unanchored: Option<StateID> = None;
    let universal_start_anchored: Option<StateID> = Some(StateID::default());
    let table = vec![0; 4];

    let start_table = StartTable {
        table: table,
        kind: kind,
        start_map: start_map,
        stride: stride,
        pattern_len: pattern_len,
        universal_start_unanchored: universal_start_unanchored,
        universal_start_anchored: universal_start_anchored,
    };

    start_table.write_to_len();
}

#[test]
fn test_write_to_len_empty_table() {
    let kind = StartKind::Both;
    let start_map = StartByteMap::new(&LookMatcher::default());
    let stride: usize = 0;
    let pattern_len: Option<usize> = Some(0);
    let universal_start_unanchored: Option<StateID> = None;
    let universal_start_anchored: Option<StateID> = None;
    let table: Vec<u32> = vec![];

    let start_table = StartTable {
        table: table,
        kind: kind,
        start_map: start_map,
        stride: stride,
        pattern_len: pattern_len,
        universal_start_unanchored: universal_start_unanchored,
        universal_start_anchored: universal_start_anchored,
    };

    start_table.write_to_len();
}

