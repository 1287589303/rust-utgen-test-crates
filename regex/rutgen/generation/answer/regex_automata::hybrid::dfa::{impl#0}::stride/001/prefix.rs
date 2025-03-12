// Answer 0

#[test]
fn test_stride_zero() {
    let dfa = DFA {
        config: Config { look_behind: None, anchored: Anchored::default() },
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap { map: [Start::default(); 256] },
        classes: ByteClasses([0; 256]),
        quitset: ByteSet([false; 256]),
        cache_capacity: 0,
    };
    dfa.stride();
}

#[test]
fn test_stride_one() {
    let dfa = DFA {
        config: Config { look_behind: None, anchored: Anchored::default() },
        nfa: thompson::NFA::default(),
        stride2: 1,
        start_map: StartByteMap { map: [Start::default(); 256] },
        classes: ByteClasses([0; 256]),
        quitset: ByteSet([false; 256]),
        cache_capacity: 0,
    };
    dfa.stride();
}

#[test]
fn test_stride_twelve() {
    let dfa = DFA {
        config: Config { look_behind: None, anchored: Anchored::default() },
        nfa: thompson::NFA::default(),
        stride2: 12,
        start_map: StartByteMap { map: [Start::default(); 256] },
        classes: ByteClasses([0; 256]),
        quitset: ByteSet([false; 256]),
        cache_capacity: 0,
    };
    dfa.stride();
}

#[test]
fn test_stride_sixty_four() {
    let dfa = DFA {
        config: Config { look_behind: None, anchored: Anchored::default() },
        nfa: thompson::NFA::default(),
        stride2: 64,
        start_map: StartByteMap { map: [Start::default(); 256] },
        classes: ByteClasses([0; 256]),
        quitset: ByteSet([false; 256]),
        cache_capacity: 0,
    };
    dfa.stride();
}

