// Answer 0

#[test]
fn test_byte_classes_single_class() {
    let byte_set = ByteSet([true; 256]);
    let classes = ByteClasses([0; 256]);
    let config = Config { byte_classes: Some(true), ..Default::default() };
    let dfa = DFA {
        config,
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap { map: [Default::default(); 256] },
        classes,
        quitset: byte_set,
        cache_capacity: 0,
    };
    let _result = dfa.byte_classes();
}

#[test]
fn test_byte_classes_multiple_classes() {
    let byte_set = ByteSet([true; 256]);
    let classes = ByteClasses([1; 256]);
    let config = Config { byte_classes: Some(true), ..Default::default() };
    let dfa = DFA {
        config,
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap { map: [Default::default(); 256] },
        classes,
        quitset: byte_set,
        cache_capacity: 0,
    };
    let _result = dfa.byte_classes();
}

#[test]
fn test_byte_classes_all_classes_enabled() {
    let byte_set = ByteSet([true; 256]);
    let classes = ByteClasses([0; 256]);
    let config = Config { byte_classes: Some(true), ..Default::default() };
    let dfa = DFA {
        config,
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap { map: [Default::default(); 256] },
        classes,
        quitset: byte_set,
        cache_capacity: 0,
    };
    let _result = dfa.byte_classes();
}

#[test]
fn test_byte_classes_disabled() {
    let byte_set = ByteSet([true; 256]);
    let classes = ByteClasses([0; 256]);
    let config = Config { byte_classes: Some(false), ..Default::default() };
    let dfa = DFA {
        config,
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap { map: [Default::default(); 256] },
        classes,
        quitset: byte_set,
        cache_capacity: 0,
    };
    let _result = dfa.byte_classes();
}

#[test]
fn test_byte_classes_edge_case_empty() {
    let byte_set = ByteSet([false; 256]);
    let classes = ByteClasses([0; 256]);
    let config = Config { byte_classes: Some(true), ..Default::default() };
    let dfa = DFA {
        config,
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap { map: [Default::default(); 256] },
        classes,
        quitset: byte_set,
        cache_capacity: 0,
    };
    let _result = dfa.byte_classes();
}

