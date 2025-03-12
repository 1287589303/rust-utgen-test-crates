// Answer 0

#[test]
fn test_build_from_dfas_valid() {
    let fwd = DFA {
        config: dfa::Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };
    let rev = DFA {
        config: dfa::Config {
            match_kind: MatchKind::All,
            ..dfa::Config::default()
        },
        nfa: thompson::NFA::new(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };
    let regex = Builder::new().build_from_dfas(fwd, rev);
}

#[test]
fn test_build_from_dfas_reverse_dfa_only() {
    let fwd = DFA {
        config: dfa::Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };
    let rev = DFA {
        config: dfa::Config {
            match_kind: MatchKind::All,
            ..dfa::Config::default()
        },
        nfa: thompson::NFA::new(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };
    let regex = Builder::new().build_from_dfas(fwd, rev);
}

#[test]
fn test_build_from_dfas_empty_dfa() {
    let fwd = DFA {
        config: dfa::Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 0,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };
    let rev = DFA {
        config: dfa::Config {
            match_kind: MatchKind::All,
            ..dfa::Config::default()
        },
        nfa: thompson::NFA::new(),
        stride2: 0,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };
    let regex = Builder::new().build_from_dfas(fwd, rev);
}

#[test]
#[should_panic]
fn test_build_from_dfas_mismatch_config() {
    let fwd = DFA {
        config: dfa::Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };
    let rev = DFA {
        config: dfa::Config {
            match_kind: MatchKind::Any,
            ..dfa::Config::default()
        },
        nfa: thompson::NFA::new(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };
    let regex = Builder::new().build_from_dfas(fwd, rev);
}

