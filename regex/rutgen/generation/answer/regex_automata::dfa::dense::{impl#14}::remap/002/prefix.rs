// Answer 0

#[test]
fn test_remap_invalid_state_id() {
    let classes = ByteClasses([0; 256]);
    let mut table = TransitionTable {
        table: vec![0; 512],
        classes,
        stride2: 9,
    };
    let invalid_id = StateID(SmallIndex::from_usize(512)); // out of valid range
    let map = |_: StateID| StateID(SmallIndex::from_usize(256)); // returns an invalid StateID
    table.remap(invalid_id, map);
}

#[test]
fn test_remap_boundary_state_id() {
    let classes = ByteClasses([0; 256]);
    let mut table = TransitionTable {
        table: vec![0; 512],
        classes,
        stride2: 9,
    };
    let boundary_id = StateID(SmallIndex::from_usize(511)); // maximum valid range
    let map = |_: StateID| StateID(SmallIndex::from_usize(512)); // again returns an invalid StateID
    table.remap(boundary_id, map);
}

