// Answer 0

#[test]
fn test_to_owned_with_both_kind() {
    let table: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let start_map = StartByteMap { 
        map: [Start::default(); 256] 
    };
    let stride = 4;
    let pattern_len = Some(2);
    let universal_start_unanchored = Some(StateID(1));
    let universal_start_anchored = Some(StateID(2));
    
    let start_table = StartTable {
        table,
        kind: StartKind::Both,
        start_map,
        stride,
        pattern_len,
        universal_start_unanchored,
        universal_start_anchored,
    };
    
    let owned_table = start_table.to_owned();
}

#[test]
fn test_to_owned_with_unanchored_kind() {
    let table: Vec<u32> = vec![9, 10, 11, 12, 13, 14, 15, 16];
    let start_map = StartByteMap { 
        map: [Start::default(); 256] 
    };
    let stride = 3;
    let pattern_len = Some(0);
    let universal_start_unanchored = Some(StateID(3));
    let universal_start_anchored = None;
    
    let start_table = StartTable {
        table,
        kind: StartKind::Unanchored,
        start_map,
        stride,
        pattern_len,
        universal_start_unanchored,
        universal_start_anchored,
    };
    
    let owned_table = start_table.to_owned();
}

#[test]
fn test_to_owned_with_anchored_kind() {
    let table: Vec<u32> = vec![17, 18, 19, 20, 21, 22, 23, 24];
    let start_map = StartByteMap { 
        map: [Start::default(); 256] 
    };
    let stride = 2;
    let pattern_len = None;
    let universal_start_unanchored = None;
    let universal_start_anchored = Some(StateID(4));
    
    let start_table = StartTable {
        table,
        kind: StartKind::Anchored,
        start_map,
        stride,
        pattern_len,
        universal_start_unanchored,
        universal_start_anchored,
    };
    
    let owned_table = start_table.to_owned();
}

