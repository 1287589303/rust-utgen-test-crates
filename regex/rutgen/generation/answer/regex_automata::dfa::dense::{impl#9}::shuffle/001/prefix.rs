// Answer 0

#[test]
fn test_shuffle_with_two_states() {
    let mut dfa = OwnedDFA::default();
    dfa.special.quit_id = StateID(1);
    dfa.add_empty_state().expect("Should be able to add a state");
    dfa.add_empty_state().expect("Should be able to add a state");
    
    let matches: BTreeMap<StateID, Vec<PatternID>> = BTreeMap::new();
    
    let result = dfa.shuffle(matches);
    result.unwrap();
}

#[test]
fn test_shuffle_with_two_non_dead_start_states() {
    let mut dfa = OwnedDFA::default();
    dfa.special.quit_id = StateID(1);
    dfa.add_empty_state().expect("Should be able to add a state");
    dfa.add_empty_state().expect("Should be able to add a state");
    
    let matches: BTreeMap<StateID, Vec<PatternID>> = BTreeMap::new();

    dfa.set_start_state(Anchored::Yes, Start::Text, StateID(1));
    
    let result = dfa.shuffle(matches);
    result.unwrap();
}

