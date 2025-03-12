// Answer 0

#[test]
fn test_len_with_four_bytes() {
    let table: &[u8] = &[0, 1, 2, 3];
    let start_map = StartByteMap { map: [Start::default(); 256] };
    let start_table = StartTable {
        table,
        kind: StartKind::Both,
        start_map,
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let _ = start_table.len();
}

#[test]
fn test_len_with_eight_bytes() {
    let table: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7];
    let start_map = StartByteMap { map: [Start::default(); 256] };
    let start_table = StartTable {
        table,
        kind: StartKind::Both,
        start_map,
        stride: 2,
        pattern_len: Some(2),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let _ = start_table.len();
}

#[test]
fn test_len_with_twelve_bytes() {
    let table: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let start_map = StartByteMap { map: [Start::default(); 256] };
    let start_table = StartTable {
        table,
        kind: StartKind::Both,
        start_map,
        stride: 3,
        pattern_len: Some(3),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let _ = start_table.len();
}

#[test]
fn test_len_with_nine_bytes() {
    let table: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8];
    let start_map = StartByteMap { map: [Start::default(); 256] };
    let start_table = StartTable {
        table,
        kind: StartKind::Both,
        start_map,
        stride: 2,
        pattern_len: None,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let _ = start_table.len();
}

#[test]
fn test_len_with_sixteen_bytes() {
    let table: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let start_map = StartByteMap { map: [Start::default(); 256] };
    let start_table = StartTable {
        table,
        kind: StartKind::Both,
        start_map,
        stride: 4,
        pattern_len: Some(4),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let _ = start_table.len();
}

