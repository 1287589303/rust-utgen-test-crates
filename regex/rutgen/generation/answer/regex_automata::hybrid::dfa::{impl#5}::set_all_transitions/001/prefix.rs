// Answer 0

#[test]
fn test_set_all_transitions_valid() {
    let mut cache = Cache { trans: vec![LazyStateID(0); 512], ..Default::default() };
    let dfa = DFA {
        classes: ByteClasses([0; 256]),
        ..Default::default()
    };
    
    let mut lazy = Lazy::new(&dfa, &mut cache);
    let from = LazyStateID(1);
    let to = LazyStateID(2);
    lazy.set_all_transitions(from, to);
}

#[test]
#[should_panic]
fn test_set_all_transitions_invalid_unit() {
    let mut cache = Cache { trans: vec![LazyStateID(0); 512], ..Default::default() };
    let dfa = DFA {
        classes: ByteClasses([0; 256]),
        ..Default::default()
    };
    
    let mut lazy = Lazy::new(&dfa, &mut cache);
    let from = LazyStateID(1);
    let to = LazyStateID(2);
    lazy.set_transition(from, alphabet::Unit(256), to); // Invalid byte
}

