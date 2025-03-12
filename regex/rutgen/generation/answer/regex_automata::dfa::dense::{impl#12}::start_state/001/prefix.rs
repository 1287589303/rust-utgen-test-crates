// Answer 0

#[test]
fn test_start_state_look_behind_zero() {
    struct TestDFA {
        quitset: ByteSet,
        st: StartTable<Vec<u32>>,
    }

    let quitset = ByteSet::empty();
    let st = StartTable {
        table: vec![StateID(0), StateID(1)],
        kind: StartKind::both(),
        start_map: StartByteMap::new(),
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let dfa = TestDFA { quitset, st };

    let config = start::Config::new().look_behind(Some(0));
    let _ = dfa.start_state(&config);
}

#[test]
fn test_start_state_look_behind_mid_range() {
    struct TestDFA {
        quitset: ByteSet,
        st: StartTable<Vec<u32>>,
    }

    let quitset = ByteSet::empty();
    let st = StartTable {
        table: vec![StateID(0), StateID(1)],
        kind: StartKind::both(),
        start_map: StartByteMap::new(),
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let dfa = TestDFA { quitset, st };

    let config = start::Config::new().look_behind(Some(127));
    let _ = dfa.start_state(&config);
}

#[test]
fn test_start_state_look_behind_max_range() {
    struct TestDFA {
        quitset: ByteSet,
        st: StartTable<Vec<u32>>,
    }

    let quitset = ByteSet::empty();
    let st = StartTable {
        table: vec![StateID(0), StateID(1)],
        kind: StartKind::both(),
        start_map: StartByteMap::new(),
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let dfa = TestDFA { quitset, st };

    let config = start::Config::new().look_behind(Some(255));
    let _ = dfa.start_state(&config);
}

