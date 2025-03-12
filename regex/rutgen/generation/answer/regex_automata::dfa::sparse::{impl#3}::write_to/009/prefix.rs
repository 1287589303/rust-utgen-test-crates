// Answer 0

#[test]
fn test_write_to_little_endian() {
    use crate::util::wire::LittleEndian;
    let flags = Flags {
        has_empty: true,
        is_utf8: true,
        is_always_start_anchored: false,
    };
    let transitions = Transitions {
        sparse: vec![0; 10],
        classes: ByteClasses::new(),
        state_len: 1,
        pattern_len: 1,
    };
    let start_table = StartTable {
        table: vec![0u8; 20],
        kind: StartKind::Both,
        start_map: StartByteMap::new(),
        stride: 4,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let special = Special {
        max: 5,
        quit_id: 1,
        min_match: 0,
        max_match: 2,
        min_accel: 2,
        max_accel: 4,
        min_start: 0,
        max_start: 3,
    };
    let quitset = ByteSet::empty();
    
    let dfa = DFA {
        tt: transitions,
        st: start_table,
        special,
        pre: None,
        quitset,
        flags,
    };
    
    let mut buffer = vec![0; 100];
    let result = dfa.write_to::<LittleEndian>(&mut buffer);
}

#[test]
fn test_write_to_big_endian() {
    use crate::util::wire::BigEndian;
    let flags = Flags {
        has_empty: false,
        is_utf8: false,
        is_always_start_anchored: true,
    };
    let transitions = Transitions {
        sparse: vec![1; 15],
        classes: ByteClasses::new(),
        state_len: 2,
        pattern_len: 3,
    };
    let start_table = StartTable {
        table: vec![0u8; 40],
        kind: StartKind::Unanchored,
        start_map: StartByteMap::new(),
        stride: 5,
        pattern_len: Some(2),
        universal_start_unanchored: Some(3),
        universal_start_anchored: None,
    };
    let special = Special {
        max: 6,
        quit_id: 2,
        min_match: 1,
        max_match: 3,
        min_accel: 2,
        max_accel: 5,
        min_start: 1,
        max_start: 4,
    };
    let quitset = ByteSet::empty();
    
    let dfa = DFA {
        tt: transitions,
        st: start_table,
        special,
        pre: None,
        quitset,
        flags,
    };
    
    let mut buffer = vec![0; 100];
    let result = dfa.write_to::<BigEndian>(&mut buffer);
}

#[test]
fn test_write_to_native_endian() {
    use crate::util::wire::NativeEndian;
    let flags = Flags {
        has_empty: true,
        is_utf8: false,
        is_always_start_anchored: true,
    };
    let transitions = Transitions {
        sparse: vec![2; 12],
        classes: ByteClasses::new(),
        state_len: 3,
        pattern_len: 2,
    };
    let start_table = StartTable {
        table: vec![0u8; 30],
        kind: StartKind::Anchored,
        start_map: StartByteMap::new(),
        stride: 3,
        pattern_len: None,
        universal_start_unanchored: Some(2),
        universal_start_anchored: None,
    };
    let special = Special {
        max: 7,
        quit_id: 3,
        min_match: 0,
        max_match: 5,
        min_accel: 3,
        max_accel: 6,
        min_start: 2,
        max_start: 5,
    };
    let quitset = ByteSet::empty();
    
    let dfa = DFA {
        tt: transitions,
        st: start_table,
        special,
        pre: None,
        quitset,
        flags,
    };
    
    let mut buffer = vec![0; 100];
    let result = dfa.write_to::<NativeEndian>(&mut buffer);
}

