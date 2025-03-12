// Answer 0

#[test]
fn test_memory_usage_no_patterns() {
    use crate::hybrid::dfa::DFA;
    
    let dfa = DFA {
        config: Default::default(),
        nfa: crate::nfa::thompson::NFA::default(),
        stride2: 0,
        start_map: crate::util::start::StartByteMap { map: [Default::default(); 256] },
        classes: crate::util::alphabet::ByteClasses([0; 256]),
        quitset: crate::util::primitives::ByteSet([false; 256]),
        cache_capacity: 0,
    };
    dfa.memory_usage();
}

#[test]
fn test_memory_usage_with_empty_nfa() {
    use crate::hybrid::dfa::DFA;

    let dfa = DFA {
        config: Default::default(),
        nfa: crate::nfa::thompson::NFA::default(),
        stride2: 0,
        start_map: crate::util::start::StartByteMap { map: [Default::default(); 256] },
        classes: crate::util::alphabet::ByteClasses([0; 256]),
        quitset: crate::util::primitives::ByteSet([false; 256]),
        cache_capacity: 0,
    };
    dfa.memory_usage();
}

