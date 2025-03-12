// Answer 0

#[test]
fn test_memory_usage_one_byte() {
    let table = vec![0u8];
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
    let _ = start_table.memory_usage();
}

#[test]
fn test_memory_usage_maximum_length() {
    let table = vec![0u8; 256];
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
    let _ = start_table.memory_usage();
}

#[test]
fn test_memory_usage_multiple_bytes() {
    let table = vec![0u8; 128];
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
    let _ = start_table.memory_usage();
}

