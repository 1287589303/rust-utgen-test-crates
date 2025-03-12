// Answer 0

#[test]
fn validate_success_case() {
    let seen = {
        let mut set = alloc::collections::BTreeSet::new();
        set.insert(StateID(1));
        set.insert(StateID(2));
        set.insert(StateID(3));
        Seen { set }
    };

    let start_map = StartByteMap { map: [Start::NonWordByte; 256] };

    let start_table = StartTable {
        table: vec![0; 8 + 3 * 8], // 8 entries for the starts, 3 patterns with 8 IDs each, just as an example
        kind: StartKind::Both,
        start_map,
        stride: 3,
        pattern_len: Some(3),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let special = Special {
        max: StateID(5),
        quit_id: StateID(0),
        min_match: StateID(100),
        max_match: StateID(200),
        min_accel: StateID(300),
        max_accel: StateID(400),
        min_start: StateID(1),
        max_start: StateID(3),
    };

    start_table.validate(&special, &seen).unwrap();
}

#[test]
fn validate_with_empty_seens() {
    let seen = Seen {
        set: alloc::collections::BTreeSet::new(),
    };

    let start_map = StartByteMap { map: [Start::NonWordByte; 256] };

    let start_table = StartTable {
        table: vec![0; 8], // Minimum 8 entries
        kind: StartKind::Both,
        start_map,
        stride: 0,
        pattern_len: None,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let special = Special {
        max: StateID(0),
        quit_id: StateID(0),
        min_match: StateID(0),
        max_match: StateID(0),
        min_accel: StateID(0),
        max_accel: StateID(0),
        min_start: StateID(0),
        max_start: StateID(0),
    };

    start_table.validate(&special, &seen).unwrap();
}

#[test]
fn validate_some_match_states() {
    let seen = {
        let mut set = alloc::collections::BTreeSet::new();
        set.insert(StateID(1));
        set.insert(StateID(2));
        set.insert(StateID(3));
        set.insert(StateID(4));
        Seen { set }
    };

    let start_map = StartByteMap { map: [Start::NonWordByte; 256] };

    let start_table = StartTable {
        table: vec![0; 8 + 4 * 8],
        kind: StartKind::Both,
        start_map,
        stride: 4,
        pattern_len: Some(4),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let special = Special {
        max: StateID(5),
        quit_id: StateID(0),
        min_match: StateID(1),
        max_match: StateID(2),
        min_accel: StateID(3),
        max_accel: StateID(4),
        min_start: StateID(0),
        max_start: StateID(2),
    };

    start_table.validate(&special, &seen).unwrap();
}

