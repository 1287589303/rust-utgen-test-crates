// Answer 0

#[test]
fn test_start_unanchored_with_valid_start_index_zero() {
    let table = vec![0u8; StateID::SIZE * 12]; // assume stride > 0, here it's 4 patterns
    let start_map = StartByteMap { map: [Start::NonWordByte; 256] };
    let start_table = StartTable {
        table,
        kind: StartKind::Both,
        start_map,
        stride: 4,
        pattern_len: Some(4),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let result = start_table.start(Anchored::No, Start::from_usize(0));
}

#[test]
fn test_start_unanchored_with_valid_start_index_five() {
    let table = vec![0u8; StateID::SIZE * 12]; // assume stride > 0, here it's 4 patterns
    let start_map = StartByteMap { map: [Start::NonWordByte; 256] };
    let start_table = StartTable {
        table,
        kind: StartKind::Both,
        start_map,
        stride: 4,
        pattern_len: Some(4),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let result = start_table.start(Anchored::No, Start::from_usize(5));
}

#[test]
fn test_start_unanchored_with_valid_start_index_seven() {
    let table = vec![0u8; StateID::SIZE * 12]; // assume stride > 0, here it's 4 patterns
    let start_map = StartByteMap { map: [Start::NonWordByte; 256] };
    let start_table = StartTable {
        table,
        kind: StartKind::Both,
        start_map,
        stride: 4,
        pattern_len: Some(4),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let result = start_table.start(Anchored::No, Start::from_usize(7));
}

#[test]
fn test_start_unanchored_when_kind_is_unanchored() {
    let table = vec![0u8; StateID::SIZE * 12]; // assume stride > 0, here it's 4 patterns
    let start_map = StartByteMap { map: [Start::NonWordByte; 256] };
    let start_table = StartTable {
        table,
        kind: StartKind::Unanchored,
        start_map,
        stride: 4,
        pattern_len: Some(4),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let result = start_table.start(Anchored::No, Start::from_usize(0));
}

