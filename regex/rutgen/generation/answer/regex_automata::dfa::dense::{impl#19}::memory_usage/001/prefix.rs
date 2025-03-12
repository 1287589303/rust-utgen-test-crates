// Answer 0

#[test]
fn test_memory_usage_empty_table() {
    let start_table = StartTable {
        table: vec![],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::default(); 256] },
        stride: 0,
        pattern_len: None,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    start_table.memory_usage();
}

#[test]
fn test_memory_usage_one_entry_table() {
    let start_table = StartTable {
        table: vec![0u32],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::default(); 256] },
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    start_table.memory_usage();
}

#[test]
fn test_memory_usage_max_entry_table() {
    let max_entries: usize = (u32::MAX as usize) / size_of::<u32>(); // this adjusts to fit within usize constraints
    let start_table = StartTable {
        table: vec![0u32; max_entries],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::default(); 256] },
        stride: max_entries,
        pattern_len: Some(max_entries),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    start_table.memory_usage();
}

