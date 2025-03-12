// Answer 0

#[test]
fn test_is_anchored_with_anchored_yes() {
    let nfa = thompson::NFA::always_match().unwrap();
    let forward_dfa = DFA {
        config: Config {},
        nfa,
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 0,
    };
    
    let regex = Regex {
        forward: forward_dfa.clone(),
        reverse: forward_dfa,
    };
    
    let input = Input::new(&[b'a', b'b', b'c']).anchored(Anchored::Yes);
    
    regex.is_anchored(&input);
}

#[test]
fn test_is_anchored_with_pattern() {
    let nfa = thompson::NFA::always_match().unwrap();
    let forward_dfa = DFA {
        config: Config {},
        nfa,
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 0,
    };
    
    let regex = Regex {
        forward: forward_dfa.clone(),
        reverse: forward_dfa,
    };
    
    let input = Input::new(&[b'a', b'b', b'c']).anchored(Anchored::Pattern(PatternID::default()));
    
    regex.is_anchored(&input);
}

